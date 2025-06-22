use crate::builder::flowing_process::FlowingProcess;
use crate::builder::runnable_process::RunnableProcess;
use crate::builder::{PreviousRunYieldedAt, RunOutcome, RunResult};
use crate::hlist_transform_to::TransformTo;
use crate::param_list::ParamList;
use crate::step::step::Final;
use serde_value::Value;
use std::future::Future;
use std::marker::PhantomData;

pub trait FinalizedProcess: Sized {
  type ProcessBeforeProduces: ParamList;

  fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
  ) -> impl Future<Output = RunResult>;

  fn run(&self, process_before_produces: Self::ProcessBeforeProduces) -> impl Future<Output = RunResult>;

  fn build(self) -> RunnableProcess<Self> {
    RunnableProcess::new(self)
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize;
}

pub struct FinalStepProcess<
  ProcessBeforeProduces: ParamList + TransformTo<FinalConsumes, ProcessBeforeProducesToFinalConsumesIndices>,
  FinalConsumes: ParamList,
  FinalStep: Final<FinalConsumes>,
  ProcessBeforeProducesToFinalConsumesIndices,
> {
  pub final_step: FinalStep,
  pub phantom_data: PhantomData<(
    ProcessBeforeProduces,
    FinalConsumes,
    ProcessBeforeProducesToFinalConsumesIndices,
  )>,
}

impl<
    ProcessBeforeProduces: ParamList + TransformTo<FinalConsumes, ProcessBeforeProducesToFinalConsumesIndices>,
    FinalConsumes: ParamList,
    FinalStep: Final<FinalConsumes>,
    ProcessBeforeProducesToFinalConsumesIndices,
  > FinalizedProcess
  for FinalStepProcess<ProcessBeforeProduces, FinalConsumes, FinalStep, ProcessBeforeProducesToFinalConsumesIndices>
{
  type ProcessBeforeProduces = ProcessBeforeProduces;

  async fn continue_run(
    &self,
    previous_run_produced: Value,
    _previous_run_yielded_at: PreviousRunYieldedAt,
  ) -> RunResult {
    let process_before_produces = ProcessBeforeProduces::deserialize(previous_run_produced)?;
    self.run(process_before_produces).await
  }

  async fn run(&self, process_before_produces: Self::ProcessBeforeProduces) -> RunResult {
    let final_consumes = process_before_produces.transform();
    Ok(RunOutcome::Finish(self.final_step.handle(final_consumes).await?))
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    last_used_index
  }
}

pub struct FlowingFinalizedProcess<
  ProcessBefore: FlowingProcess,
  FinalConsumes: ParamList,
  FinalStep: Final<FinalConsumes>,
> {
  pub process_before: ProcessBefore,
  pub final_step: FinalStep,
  pub phantom_data: PhantomData<FinalConsumes>,
}

impl<ProcessBefore: FlowingProcess, FinalConsumes: ParamList, FinalStep: Final<FinalConsumes>> FinalizedProcess
  for FlowingFinalizedProcess<ProcessBefore, FinalConsumes, FinalStep>
{
  type ProcessBeforeProduces = ProcessBefore::Produces;

  async fn continue_run(
    &self,
    _previous_run_produced: Value,
    _previous_run_yielded_at: PreviousRunYieldedAt,
  ) -> RunResult {
    todo!()
  }

  async fn run(&self, _process_before_produces: Self::ProcessBeforeProduces) -> RunResult {
    todo!()
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    // most likely not worth to assign an index to final steps, but maybe test
    self.process_before.enumerate_steps(last_used_index)
  }
}
