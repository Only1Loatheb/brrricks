use crate::builder::finalized_process::{FinalizedProcess, FlowingFinalizedProcess};
use crate::builder::split_process::SplitProcessSplitter;
use crate::builder::*;
use crate::hlist_concat::Concat;
use crate::hlist_transform_to::TransformTo;
use crate::param_list::ParamList;
use crate::step::step::{Entry, Final, Linear, Splitter};
use anyhow::anyhow;
use frunk_core::coproduct::Coproduct;
use frunk_core::hlist::HNil;
use serde_value::Value;
use std::future::Future;
use std::hint::unreachable_unchecked;
use std::marker::PhantomData;

pub trait FlowingProcess: Sized {
  type ProcessBeforeProduces: ParamList;
  type Produces: ParamList;

  fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
  ) -> impl Future<Output = IntermediateRunResult<Self::Produces>>;

  fn run(
    &self,
    process_before_produces: Self::ProcessBeforeProduces,
  ) -> impl Future<Output = IntermediateRunResult<Self::Produces>>;

  // Param value overlap is prevented by making reading them cumbersome https://github.com/lloydmeta/frunk/issues/187
  // Well you can work around this `limitation` by providing the indices explicitly.
  // Don't do that.
  fn then<
    LinearConsumes: ParamList,
    LinearProduces: ParamList + Concat<Self::Produces>,
    LinearStep: Linear<LinearConsumes, LinearProduces>,
    ProcessBeforeProducesToLastStepConsumesIndices,
  >(
    self,
    step: LinearStep,
  ) -> impl FlowingProcess<
    ProcessBeforeProduces = Self::Produces,
    Produces = <LinearProduces as Concat<Self::Produces>>::Concatenated,
  >
  where
    Self::Produces: TransformTo<LinearConsumes, ProcessBeforeProducesToLastStepConsumesIndices>,
  {
    LinearFlowingProcess {
      process_before: self,
      last_step: step,
      step_index: WILL_BE_RENUMBERED,
      phantom_data: Default::default(),
    }
  }

  fn split<
    SplitterStepConsumes: ParamList,
    SplitterProducesForFirstCase: ParamList,
    SplitterProducesForOtherCases,
    SplitterStep: Splitter<SplitterStepConsumes, Coproduct<SplitterProducesForFirstCase, SplitterProducesForOtherCases>>,
    ProcessBeforeProducesToSplitterStepConsumesIndices,
  >(
    self,
    step: SplitterStep,
  ) -> impl SplitProcess<
    SplitterProducesForOtherCases,
    ProcessBeforeSplitProduces = Self::Produces,
    SplitterProducesForFirstCase = SplitterProducesForFirstCase,
  >
  where
    Self::Produces: TransformTo<SplitterStepConsumes, ProcessBeforeProducesToSplitterStepConsumesIndices>,
  {
    SplitProcessSplitter::<
      Self,
      SplitterStepConsumes,
      SplitterProducesForFirstCase,
      SplitterProducesForOtherCases,
      SplitterStep,
      ProcessBeforeProducesToSplitterStepConsumesIndices,
    > {
      process_before: self,
      splitter: step,
      split_step_index: WILL_BE_RENUMBERED,
      phantom_data: Default::default(),
    }
  }

  fn end<FinalConsumes: ParamList, FinalStep: Final<FinalConsumes>, ProcessBeforeProducesToLastStepConsumesIndices>(
    self,
    step: FinalStep,
  ) -> impl FinalizedProcess<ProcessBeforeProduces = Self::Produces>
  where
    Self::Produces: TransformTo<FinalConsumes, ProcessBeforeProducesToLastStepConsumesIndices>,
  {
    FlowingFinalizedProcess {
      process_before: self,
      final_step: step,
      phantom_data: Default::default(),
    }
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize;
}

impl<Produces: ParamList, EntryStep: Entry<Value, Produces = Produces>> FlowingProcess for EntryStep {
  type ProcessBeforeProduces = HNil;
  type Produces = EntryStep::Produces;

  async fn continue_run(
    &self,
    previous_run_produced: Value,
    _: PreviousRunYieldedAt,
  ) -> IntermediateRunResult<Self::Produces> {
    let map = match previous_run_produced {
      Value::Map(m) => m,
      _ => return Err(anyhow!("Not a map")),
    };
    let result: Produces = EntryStep::handle(self, map).await?;
    Ok(IntermediateRunOutcome::Continue(result))
  }

  async fn run(&self, _: Self::ProcessBeforeProduces) -> IntermediateRunResult<Self::Produces> {
    unsafe { unreachable_unchecked() }
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    last_used_index
  }
}

pub struct Subprocess<ProcessBeforeProduces> {
  pub phantom_data: PhantomData<ProcessBeforeProduces>,
}

impl<ProcessBeforeProduces: ParamList> FlowingProcess for Subprocess<ProcessBeforeProduces> {
  type ProcessBeforeProduces = ProcessBeforeProduces;
  type Produces = ProcessBeforeProduces;

  async fn continue_run(
    &self,
    previous_run_produced: Value,
    _previous_run_yielded_at: PreviousRunYieldedAt,
  ) -> IntermediateRunResult<Self::Produces> {
    let process_before_produces = ProcessBeforeProduces::deserialize(previous_run_produced)?;
    self.run(process_before_produces).await
  }

  async fn run(&self, process_before_produces: Self::ProcessBeforeProduces) -> IntermediateRunResult<Self::Produces> {
    Ok(IntermediateRunOutcome::Continue(process_before_produces))
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    last_used_index
  }
}

pub fn subprocess<ProcessBeforeProduces: ParamList>() -> Subprocess<ProcessBeforeProduces> {
  Subprocess {
    phantom_data: Default::default(),
  }
}

pub struct LinearFlowingProcess<
  ProcessBefore: FlowingProcess,
  LinearConsumes: ParamList,
  LinearProduces: ParamList,
  LinearStep: Linear<LinearConsumes, LinearProduces>,
  ProcessBeforeProducesToLastStepConsumesIndices,
> {
  pub process_before: ProcessBefore,
  pub last_step: LinearStep,
  pub step_index: usize,
  pub phantom_data: PhantomData<(
    LinearConsumes,
    LinearProduces,
    ProcessBeforeProducesToLastStepConsumesIndices,
  )>,
}

impl<
    ProcessBefore: FlowingProcess,
    LastStepConsumes: ParamList,
    LastStepProduces: ParamList + Concat<ProcessBefore::Produces>,
    LastStep: Linear<LastStepConsumes, LastStepProduces>,
    ProcessBeforeProducesToLastStepConsumesIndices,
  > FlowingProcess
  for LinearFlowingProcess<
    ProcessBefore,
    LastStepConsumes,
    LastStepProduces,
    LastStep,
    ProcessBeforeProducesToLastStepConsumesIndices,
  >
where
  <ProcessBefore as FlowingProcess>::Produces:
    TransformTo<LastStepConsumes, ProcessBeforeProducesToLastStepConsumesIndices>,
{
  type ProcessBeforeProduces = ProcessBefore::Produces;
  type Produces = <LastStepProduces as Concat<ProcessBefore::Produces>>::Concatenated;

  async fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
  ) -> IntermediateRunResult<Self::Produces> {
    if previous_run_yielded_at.0 < self.step_index {
      let process_before_output = self
        .process_before
        .continue_run(previous_run_produced, previous_run_yielded_at)
        .await?;
      match process_before_output {
        IntermediateRunOutcome::Continue(process_before_produces) => self.run(process_before_produces).await,
        IntermediateRunOutcome::Yield(a, b, c) => Ok(IntermediateRunOutcome::Yield(a, b, c)),
        IntermediateRunOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
      }
    } else {
      // fixme deserialize only values required only up to the next interaction
      let process_before_produces = ProcessBefore::Produces::deserialize(previous_run_produced)?;
      self.run(process_before_produces).await
    }
  }

  async fn run(&self, process_before_produces: Self::ProcessBeforeProduces) -> IntermediateRunResult<Self::Produces> {
    let last_step_consumes: LastStepConsumes = process_before_produces.clone().transform();
    let last_step_output = self.last_step.handle(last_step_consumes).await?;
    match last_step_output {
      (Some(msg), last_step_produces) =>
      // Should only pass params required in further part of the process, but I don't know what they are.
      // todo Make all the methods generic over Serializer
      {
        let value = last_step_produces.concat(process_before_produces).serialize()?;
        Ok(IntermediateRunOutcome::Yield(
          msg,
          value,
          CurrentRunYieldedAt(self.step_index),
        ))
      }
      (None, last_step_produces) => Ok(IntermediateRunOutcome::Continue(
        last_step_produces.concat(process_before_produces),
      )),
    }
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    let used_index = self.process_before.enumerate_steps(last_used_index);
    self.step_index = used_index + 1;
    self.step_index
  }
}
