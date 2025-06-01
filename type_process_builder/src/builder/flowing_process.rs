use crate::builder::IntermediateRunOutcome::*;
use crate::builder::{
  CurrentRunYieldedAt, IntermediateRunResult, PreviousRunYieldedAt, ProcessBuilder, WILL_BE_RENUMBERED,
};
use crate::hlist_concat::Concat;
use crate::hlist_transformer::TransformTo;
use crate::step::param_list::ParamList;
use crate::step::step::{Linear};
use frunk_core::hlist::HNil;
use serde::de::DeserializeOwned;
use std::io;
use std::marker::PhantomData;

pub trait FlowingProcess: ProcessBuilder {
  type ProcessBeforeProduces: ParamList;
  type Produces: ParamList;

  async fn continue_run(
    &self,
    previous_run_produced: impl io::Read,
    previous_run_yielded: PreviousRunYieldedAt,
  ) -> IntermediateRunResult<Self::Produces>;

  async fn run(&self, process_before_produces: Self::ProcessBeforeProduces) -> IntermediateRunResult<Self::Produces>;

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
    <Self as FlowingProcess>::Produces: TransformTo<LINEAR_CONSUMES, PROCESS_BEFORE_PRODUCES_TO_LAST_STEP_CONSUMES_INDICES>,
  {
    LinearFlowingProcess {
      process_before: self,
      last_step: step,
      step_index: WILL_BE_RENUMBERED,
      phantom_data: Default::default(),
    }
  }
}

pub struct EmptyProcess;

impl ProcessBuilder for EmptyProcess {
  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    last_used_index
  }
}

impl FlowingProcess for EmptyProcess {
  type ProcessBeforeProduces = HNil;
  type Produces = HNil;

  async fn continue_run(
    &self,
    previous_run_produced: impl io::Read,
    previous_run_yielded: PreviousRunYieldedAt,
  ) -> IntermediateRunResult<Self::Produces> {
    Ok(Continue(HNil))
  }

  async fn run(&self, process_before_produces: Self::ProcessBeforeProduces) -> IntermediateRunResult<Self::Produces> {
    Ok(Continue(HNil))
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
  > ProcessBuilder
  for LinearFlowingProcess<
    PROCESS_BEFORE,
    LAST_STEP_CONSUMES,
    LAST_STEP_PRODUCES,
    LAST_STEP,
    PROCESS_BEFORE_PRODUCES_TO_LAST_STEP_CONSUMES_INDICES,
  >
where
  <PROCESS_BEFORE as FlowingProcess>::Produces: TransformTo<LAST_STEP_CONSUMES, PROCESS_BEFORE_PRODUCES_TO_LAST_STEP_CONSUMES_INDICES>,
{
  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    let used_index = self.process_before.enumerate_steps(last_used_index);
    self.step_index = used_index + 1;
    self.step_index
  }
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
  <PROCESS_BEFORE as FlowingProcess>::Produces: TransformTo<LAST_STEP_CONSUMES, PROCESS_BEFORE_PRODUCES_TO_LAST_STEP_CONSUMES_INDICES>,
{
  type ProcessBeforeProduces = PROCESS_BEFORE::Produces;
  type Produces = <LAST_STEP_PRODUCES as Concat<PROCESS_BEFORE::Produces>>::Concatenated;

  async fn continue_run(
    &self,
    previous_run_produced: impl io::Read,
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
      let process_before_produces: PROCESS_BEFORE::Produces = todo!(); // serde_json::from_reader(consumes)?;
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
          last_step_produces
            .concat(process_before_produces)
            .serialize(serde_json::value::Serializer)?, // fixme make it generic over format i.e. json
          CurrentRunYieldedAt(self.step_index),
        ))
      }
      (None, last_step_produces) => Ok(Continue(last_step_produces.concat(process_before_produces))),
    }
  }
}
