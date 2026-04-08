use crate::builder::flowing_process::FlowingProcess;
use crate::builder::runnable_process::RunnableProcess;
use crate::builder::{
  IntermediateRunOutcome, ParamUID, PreviousRunYieldedAt, RunOutcome, RunResult, SessionContext, StepIndex,
};
use crate::param_list::ParamList;
use crate::param_list::transform::TransformTo;
use crate::step::{FailedInputValidationAttempts, Final};
use std::future::Future;
use std::marker::PhantomData;

pub trait FinalizedProcess: Sized + Sync {
  // Please specify all associated types at the impl FinalizedProcess side for inference to work.
  type ProcessBeforeProduces: ParamList;
  type SubprocessConsumes: ParamList;

  fn resume_run(
    &self,
    previous_run_produced: SessionContext,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
    failed_input_validation_attempts: FailedInputValidationAttempts,
  ) -> impl Future<Output = RunResult> + Send;

  fn continue_run(
    &self,
    process_before_produces: Self::ProcessBeforeProduces,
  ) -> impl Future<Output = RunResult> + Send;

  fn run_subprocess(&self, subprocess_consumes: Self::SubprocessConsumes) -> impl Future<Output = RunResult> + Send;

  fn build(self, name: &'static str, version: u32) -> RunnableProcess<Self> {
    RunnableProcess::new(self, name, version)
  }

  fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex;

  fn all_param_uids(&self, acc: &mut Vec<ParamUID>);
}

pub struct FlowingFinalizedProcess<
  ProcessBefore: FlowingProcess,
  FinalStep: Final,
  ProcessBeforeProducesTransformToFinalConsumesIndices,
> {
  pub process_before: ProcessBefore,
  pub final_step: FinalStep,
  pub phantom_data: PhantomData<ProcessBeforeProducesTransformToFinalConsumesIndices>,
}

impl<ProcessBefore: FlowingProcess, FinalStep: Final, ProcessBeforeProducesTransformToFinalConsumesIndices: Sync>
  FinalizedProcess
  for FlowingFinalizedProcess<ProcessBefore, FinalStep, ProcessBeforeProducesTransformToFinalConsumesIndices>
where
  ProcessBefore::Produces: TransformTo<FinalStep::Consumes, ProcessBeforeProducesTransformToFinalConsumesIndices>,
{
  type ProcessBeforeProduces = ProcessBefore::Produces;
  type SubprocessConsumes = ProcessBefore::SubprocessConsumes;

  async fn resume_run(
    // check where to resume when copying and pasting to finalized proces with finalized process instead of last case
    &self,
    previous_run_produced: SessionContext,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
    failed_input_validation_attempts: FailedInputValidationAttempts,
  ) -> RunResult {
    let outcome = self
      .process_before
      .resume_run(previous_run_produced, previous_run_yielded_at, user_input, failed_input_validation_attempts)
      .await?;
    match outcome {
      IntermediateRunOutcome::Continue(val) => self.continue_run(val).await,
      IntermediateRunOutcome::Yield(a, b, c) => Ok(RunOutcome::Yield(a, b, c)),
      IntermediateRunOutcome::Finish(a) => Ok(RunOutcome::Finish(a)),
      IntermediateRunOutcome::RetryUserInput(a) => Ok(RunOutcome::RetryUserInput(a)),
    }
  }

  async fn continue_run(&self, process_before_produces: Self::ProcessBeforeProduces) -> RunResult {
    Ok(RunOutcome::Finish(self.final_step.handle(process_before_produces.transform()).await?))
  }

  async fn run_subprocess(&self, subprocess_consumes: Self::SubprocessConsumes) -> RunResult {
    let outcome = self.process_before.run_subprocess(subprocess_consumes).await?;
    match outcome {
      IntermediateRunOutcome::Continue(val) => self.continue_run(val).await,
      IntermediateRunOutcome::Yield(a, b, c) => Ok(RunOutcome::Yield(a, b, c)),
      IntermediateRunOutcome::Finish(a) => Ok(RunOutcome::Finish(a)),
      IntermediateRunOutcome::RetryUserInput(a) => Ok(RunOutcome::RetryUserInput(a)),
    }
  }

  fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex {
    // most likely not worth to assign an index to final steps, but maybe test
    self.process_before.enumerate_steps(last_used_index)
  }

  fn all_param_uids(&self, acc: &mut Vec<ParamUID>) {
    self.process_before.all_param_uids(acc);
  }
}
