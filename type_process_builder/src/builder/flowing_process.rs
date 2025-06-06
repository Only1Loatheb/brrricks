use crate::builder::finalized_process::{FinalizedProcess, FlowingFinalizedProcess};
use crate::builder::IntermediateRunOutcome::*;
use crate::builder::{CurrentRunYieldedAt, IntermediateRunResult, PreviousRunYieldedAt, WILL_BE_RENUMBERED};
use crate::hlist_concat::Concat;
use crate::hlist_transformer::TransformTo;
use crate::param_list::ParamList;
use crate::step::step::{Entry, Final, Linear};
use anyhow::anyhow;
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
    previous_run_yielded: PreviousRunYieldedAt,
  ) -> impl Future<Output = IntermediateRunResult<Self::Produces>>;

  fn run(
    &self,
    process_before_produces: Self::ProcessBeforeProduces,
  ) -> impl Future<Output = IntermediateRunResult<Self::Produces>>;

  // LINEAR_PRODUCES and Self::Produces overlap is prevented https://github.com/lloydmeta/frunk/issues/187
  fn then<
    LINEAR_CONSUMES: ParamList,
    LINEAR_PRODUCES: ParamList + Concat<Self::Produces>,
    LINEAR_STEP: Linear<LINEAR_CONSUMES, LINEAR_PRODUCES>,
    PROCESS_BEFORE_PRODUCES_TO_LAST_STEP_CONSUMES_INDICES,
  >(
    self,
    step: LINEAR_STEP,
  ) -> impl FlowingProcess<
    ProcessBeforeProduces = <Self as FlowingProcess>::Produces,
    Produces = <LINEAR_PRODUCES as Concat<Self::Produces>>::Concatenated,
  >
  where
    <Self as FlowingProcess>::Produces:
      TransformTo<LINEAR_CONSUMES, PROCESS_BEFORE_PRODUCES_TO_LAST_STEP_CONSUMES_INDICES>,
  {
    LinearFlowingProcess {
      process_before: self,
      last_step: step,
      step_index: WILL_BE_RENUMBERED,
      phantom_data: Default::default(),
    }
  }

  fn end<
    FINAL_CONSUMES: ParamList,
    FINAL_STEP: Final<FINAL_CONSUMES>,
    PROCESS_BEFORE_PRODUCES_TO_LAST_STEP_CONSUMES_INDICES,
  >(
    self,
    step: FINAL_STEP,
  ) -> impl FinalizedProcess
  where
    <Self as FlowingProcess>::Produces:
      TransformTo<FINAL_CONSUMES, PROCESS_BEFORE_PRODUCES_TO_LAST_STEP_CONSUMES_INDICES>,
  {
    FlowingFinalizedProcess {
      process_before: self,
      final_step: step,
      phantom_data: Default::default(),
    }
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize;
}

impl<PRODUCES: ParamList, ENTRY: Entry<Value, Produces = PRODUCES>> FlowingProcess for ENTRY {
  type ProcessBeforeProduces = HNil;
  type Produces = ENTRY::Produces;

  async fn continue_run(
    &self,
    previous_run_produced: Value,
    _: PreviousRunYieldedAt,
  ) -> IntermediateRunResult<Self::Produces> {
    let map = match previous_run_produced {
      Value::Map(m) => m,
      _ => return Err(anyhow!("Not a map")),
    };
    let result: PRODUCES = ENTRY::handle(self, map).await?;
    Ok(Continue(result))
  }

  async fn run(&self, _: Self::ProcessBeforeProduces) -> IntermediateRunResult<Self::Produces> {
    unsafe { unreachable_unchecked() }
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    last_used_index
  }
}

pub struct LinearFlowingProcess<
  PROCESS_BEFORE: FlowingProcess,
  LINEAR_CONSUMES: ParamList,
  LINEAR_PRODUCES: ParamList,
  LINEAR_STEP: Linear<LINEAR_CONSUMES, LINEAR_PRODUCES>,
  PROCESS_BEFORE_PRODUCES_TO_LAST_STEP_CONSUMES_INDICES,
> {
  pub process_before: PROCESS_BEFORE,
  pub last_step: LINEAR_STEP,
  pub step_index: usize,
  pub phantom_data: PhantomData<(
    LINEAR_CONSUMES,
    LINEAR_PRODUCES,
    PROCESS_BEFORE_PRODUCES_TO_LAST_STEP_CONSUMES_INDICES,
  )>,
}

impl<
    PROCESS_BEFORE: FlowingProcess,
    LAST_STEP_CONSUMES: ParamList,
    LAST_STEP_PRODUCES: ParamList + Concat<PROCESS_BEFORE::Produces>,
    LAST_STEP: Linear<LAST_STEP_CONSUMES, LAST_STEP_PRODUCES>,
    PROCESS_BEFORE_PRODUCES_TO_LAST_STEP_CONSUMES_INDICES,
  > FlowingProcess
  for LinearFlowingProcess<
    PROCESS_BEFORE,
    LAST_STEP_CONSUMES,
    LAST_STEP_PRODUCES,
    LAST_STEP,
    PROCESS_BEFORE_PRODUCES_TO_LAST_STEP_CONSUMES_INDICES,
  >
where
  <PROCESS_BEFORE as FlowingProcess>::Produces:
    TransformTo<LAST_STEP_CONSUMES, PROCESS_BEFORE_PRODUCES_TO_LAST_STEP_CONSUMES_INDICES>,
{
  type ProcessBeforeProduces = PROCESS_BEFORE::Produces;
  type Produces = <LAST_STEP_PRODUCES as Concat<PROCESS_BEFORE::Produces>>::Concatenated;

  async fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded: PreviousRunYieldedAt,
  ) -> IntermediateRunResult<Self::Produces> {
    if previous_run_yielded.0 < self.step_index {
      let process_before_output = self
        .process_before
        .continue_run(previous_run_produced, previous_run_yielded)
        .await?;
      match process_before_output {
        Continue(process_before_produces) => self.run(process_before_produces).await,
        Yield(a, b, c) => Ok(Yield(a, b, c)),
        Finish(a) => Ok(Finish(a)),
      }
    } else {
      // fixme deserialize only values required only up to the next interaction
      let process_before_produces: PROCESS_BEFORE::Produces =
        PROCESS_BEFORE::Produces::deserialize(previous_run_produced)?;
      self.run(process_before_produces).await
    }
  }

  async fn run(&self, process_before_produces: Self::ProcessBeforeProduces) -> IntermediateRunResult<Self::Produces> {
    let last_step_consumes: LAST_STEP_CONSUMES = process_before_produces.clone().transform();
    let last_step_output = self.last_step.handle(last_step_consumes).await?;
    match last_step_output {
      (Some(msg), last_step_produces) =>
      // Should only pass params required in further part of the process, but I don't know what they are.
      // todo Make all the methods generic over Serializer
      {
        Ok(Yield(
          msg,
          last_step_produces.concat(process_before_produces).serialize()?,
          CurrentRunYieldedAt(self.step_index),
        ))
      }
      (None, last_step_produces) => Ok(Continue(last_step_produces.concat(process_before_produces))),
    }
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    let used_index = self.process_before.enumerate_steps(last_used_index);
    self.step_index = used_index + 1;
    self.step_index
  }
}
