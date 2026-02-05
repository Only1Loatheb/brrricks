use crate::builder::finalized_process::{FinalizedProcess, FlowingFinalizedProcess};
use crate::builder::split_process::SplitProcessSplitter;
use crate::builder::*;
use crate::hlist_concat::Concat;
use crate::hlist_transform::{CloneJust, TransformTo};
use crate::param_list::ParamList;
use crate::step::{Entry, Final, Form, Operation, Splitter};
use anyhow::anyhow;
use frunk_core::coproduct::Coproduct;
use frunk_core::hlist::HNil;
use serde_value::Value;
use std::future::Future;
use std::hint::unreachable_unchecked;
use std::marker::PhantomData;

/// Param value overlap is prevented by making reading them cumbersome https://github.com/lloydmeta/frunk/issues/187
/// Well you can work around this `limitation` by providing the indices explicitly
/// or replaceing [Concat] with [crate::hlist_intersect::Intersect] in the implementation.
/// Don't do that. The params should be immutable to avoid the need to overwrite them with every session context save.
pub trait FlowingProcess: Sized {
  type ProcessBeforeProduces: ParamList;
  type Produces: ParamList;
  // add a dependent type for split process to pass values produced by the splitter step to this specific branch.

  fn resume_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
  ) -> impl Future<Output = IntermediateRunResult<Self::Produces>>;

  fn continue_run(
    &self,
    process_before_produces: Self::ProcessBeforeProduces,
  ) -> impl Future<Output = IntermediateRunResult<Self::Produces>>;

  fn then<
    LinearConsumes: ParamList,
    LinearProduces: ParamList + Concat<Self::Produces>,
    LinearStep: Operation<Consumes = LinearConsumes, Produces = LinearProduces>,
    ProcessBeforeProducesToLastStepConsumesIndices,
  >(
    self,
    step: LinearStep,
  ) -> impl FlowingProcess<
    ProcessBeforeProduces = Self::Produces,
    Produces = <LinearProduces as Concat<Self::Produces>>::Concatenated,
  >
  where
    for<'a> &'a Self::Produces: CloneJust<LinearConsumes, ProcessBeforeProducesToLastStepConsumesIndices>,
  {
    LinearFlowingProcess {
      process_before: self,
      last_step: step,
      step_index: WILL_BE_RENUMBERED,
      phantom_data: Default::default(),
    }
  }

  fn show<
    FormConsumes: ParamList,
    FormProduces: ParamList + Concat<Self::Produces>,
    FormStep: Form<Consumes=FormConsumes, Produces=FormProduces>,
    ProcessBeforeProducesToLastStepConsumesIndices,
  >(
    self,
    step: FormStep,
  ) -> impl FlowingProcess<
    ProcessBeforeProduces = Self::Produces,
    Produces = <FormProduces as Concat<Self::Produces>>::Concatenated,
  >
  where
    for<'a> &'a Self::Produces: CloneJust<FormConsumes, ProcessBeforeProducesToLastStepConsumesIndices>,
  {
    FormFlowingProcess {
      process_before: self,
      last_step: step,
      step_index: WILL_BE_RENUMBERED,
      phantom_data: Default::default(),
    }
  }

  fn split<
    Tag,
    SplitterStepConsumes: ParamList,
    SplitterProducesForFirstCase: ParamList + Concat<Self::Produces>,
    SplitterProducesForOtherCases,
    SplitterStep: Splitter<
        Consumes = SplitterStepConsumes,
        Produces = Coproduct<(Tag, SplitterProducesForFirstCase), SplitterProducesForOtherCases>,
      >,
    ProcessBeforeProducesToSplitterStepConsumesIndices,
  >(
    self,
    step: SplitterStep,
  ) -> impl SplitProcess<
    SplitterProducesForOtherCases,
    ProcessBeforeSplitProduces = Self::Produces,
    SplitterProducesForFirstCase = SplitterProducesForFirstCase,
    SplitterTagForFirstCase = Tag,
  >
  where
    for<'a> &'a Self::Produces: CloneJust<SplitterStepConsumes, ProcessBeforeProducesToSplitterStepConsumesIndices>,
  {
    SplitProcessSplitter::<
      Tag,
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

  fn end<
    FinalConsumes: ParamList,
    FinalStep: Final<Consumes = FinalConsumes>,
    ProcessBeforeProducesToLastStepConsumesIndices,
  >(
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

  async fn resume_run(
    &self,
    previous_run_produced: Value,
    _: PreviousRunYieldedAt,
    user_input: String,
  ) -> IntermediateRunResult<Self::Produces> {
    let map = match previous_run_produced {
      Value::Map(m) => m,
      _ => return Err(anyhow!("Not a map")),
    };
    let result: Produces = EntryStep::handle(self, map, user_input).await?;
    Ok(IntermediateRunOutcome::Continue(result))
  }

  async fn continue_run(&self, _: Self::ProcessBeforeProduces) -> IntermediateRunResult<Self::Produces> {
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

  async fn resume_run(
    &self,
    previous_run_produced: Value,
    _previous_run_yielded_at: PreviousRunYieldedAt,
    _user_input: String,
  ) -> IntermediateRunResult<Self::Produces> {
    let process_before_produces = ProcessBeforeProduces::deserialize(previous_run_produced)?;
    self.continue_run(process_before_produces).await
  }

  async fn continue_run(
    &self,
    process_before_produces: Self::ProcessBeforeProduces,
  ) -> IntermediateRunResult<Self::Produces> {
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
  OperationStep: Operation,
  ProcessBeforeProducesToLastStepConsumesIndices,
> {
  pub process_before: ProcessBefore,
  pub last_step: OperationStep,
  pub step_index: usize,
  pub phantom_data: PhantomData<ProcessBeforeProducesToLastStepConsumesIndices>,
}

impl<
  ProcessBefore: FlowingProcess,
  LastStepConsumes: ParamList,
  LastStepProduces: ParamList + Concat<ProcessBefore::Produces>,
  OperationStep: Operation<Consumes = LastStepConsumes, Produces = LastStepProduces>,
  ProcessBeforeProducesToLastStepConsumesIndices,
> FlowingProcess for LinearFlowingProcess<ProcessBefore, OperationStep, ProcessBeforeProducesToLastStepConsumesIndices>
where
  for<'a> &'a ProcessBefore::Produces: CloneJust<LastStepConsumes, ProcessBeforeProducesToLastStepConsumesIndices>,
{
  type ProcessBeforeProduces = ProcessBefore::Produces;
  type Produces = <LastStepProduces as Concat<ProcessBefore::Produces>>::Concatenated;

  async fn resume_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
  ) -> IntermediateRunResult<Self::Produces> {
    if previous_run_yielded_at.0 < self.step_index {
      let process_before_output = self
        .process_before
        .resume_run(previous_run_produced, previous_run_yielded_at, user_input)
        .await?;
      match process_before_output {
        IntermediateRunOutcome::Continue(process_before_produces) => self.continue_run(process_before_produces).await,
        IntermediateRunOutcome::Yield(a, b, c) => Ok(IntermediateRunOutcome::Yield(a, b, c)),
        IntermediateRunOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
      }
    } else {
      let process_before_produces = ProcessBefore::Produces::deserialize(previous_run_produced)?;
      self.continue_run(process_before_produces).await
    }
  }

  async fn continue_run(
    &self,
    process_before_produces: Self::ProcessBeforeProduces,
  ) -> IntermediateRunResult<Self::Produces> {
    let last_step_consumes: LastStepConsumes = process_before_produces.clone_just();
    let last_step_output = self.last_step.handle(last_step_consumes).await?;
    Ok(IntermediateRunOutcome::Continue(
      last_step_output.concat(process_before_produces),
    ))
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    let used_index = self.process_before.enumerate_steps(last_used_index);
    self.step_index = used_index + 1;
    self.step_index
  }
}

pub struct FormFlowingProcess<
  ProcessBefore: FlowingProcess,
  LinearStep: Form,
  ProcessBeforeProducesToLastStepConsumesIndices,
> {
  pub process_before: ProcessBefore,
  pub last_step: LinearStep,
  pub step_index: usize,
  pub phantom_data: PhantomData<ProcessBeforeProducesToLastStepConsumesIndices>,
}

impl<
  ProcessBefore: FlowingProcess,
  LastStepConsumes: ParamList,
  LastStepProduces: ParamList + Concat<ProcessBefore::Produces>,
  FormStep: Form<Consumes = LastStepConsumes, Produces = LastStepProduces>,
  ProcessBeforeProducesToLastStepConsumesIndices,
> FlowingProcess for FormFlowingProcess<ProcessBefore, FormStep, ProcessBeforeProducesToLastStepConsumesIndices>
where
  for<'a> &'a <ProcessBefore as FlowingProcess>::Produces:
    CloneJust<LastStepConsumes, ProcessBeforeProducesToLastStepConsumesIndices>,
{
  type ProcessBeforeProduces = ProcessBefore::Produces;
  type Produces = <LastStepProduces as Concat<ProcessBefore::Produces>>::Concatenated;

  async fn resume_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
  ) -> IntermediateRunResult<Self::Produces> {
    if previous_run_yielded_at.0 < self.step_index {
      let process_before_output = self
        .process_before
        .resume_run(previous_run_produced, previous_run_yielded_at, user_input)
        .await?;
      match process_before_output {
        IntermediateRunOutcome::Continue(process_before_produces) => self.continue_run(process_before_produces).await,
        IntermediateRunOutcome::Yield(a, b, c) => Ok(IntermediateRunOutcome::Yield(a, b, c)),
        IntermediateRunOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
      }
    } else {
      // fixme deserialize only values required only up to the next interaction
      let process_before_produces = ProcessBefore::Produces::deserialize(previous_run_produced)?;
      let last_step_consumes: LastStepConsumes = process_before_produces.clone_just();
      Ok(IntermediateRunOutcome::Continue(
        self
          .last_step
          .handle_input(last_step_consumes, user_input)
          .await?
          .concat(process_before_produces),
      ))
    }
  }

  async fn continue_run(
    &self,
    process_before_produces: Self::ProcessBeforeProduces,
  ) -> IntermediateRunResult<Self::Produces> {
    let last_step_consumes: LastStepConsumes = process_before_produces.clone_just();
    Ok(IntermediateRunOutcome::Yield(
      self.last_step.show_form(last_step_consumes).await?,
      process_before_produces.serialize()?,
      CurrentRunYieldedAt(self.step_index),
    ))
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    let used_index = self.process_before.enumerate_steps(last_used_index);
    self.step_index = used_index + 1;
    self.step_index
  }
}
