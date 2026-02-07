use crate::builder::flowing_process::FlowingProcess;
use crate::builder::runnable_process::RunnableProcess;
use crate::builder::{IntermediateRunOutcome, PreviousRunYieldedAt, RunOutcome, RunResult};
use crate::param_list::ParamList;
use crate::param_list::transform::TransformTo;
use crate::step::{FailedInputValidationAttempts, Final};
use serde_value::Value;
use std::future::Future;
use std::marker::PhantomData;

pub trait FinalizedProcess: Sized {
  type ProcessBeforeProduces: ParamList;

  fn resume_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
    failed_input_validation_attempts: FailedInputValidationAttempts,
  ) -> impl Future<Output = RunResult>;

  fn continue_run(&self, process_before_produces: Self::ProcessBeforeProduces) -> impl Future<Output = RunResult>;

  fn build(self) -> RunnableProcess<Self> {
    RunnableProcess::new(self)
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize;
}

pub struct FlowingFinalizedProcess<
  ProcessBefore: FlowingProcess,
  FinalConsumes: ParamList,
  FinalStep: Final<Consumes = FinalConsumes>,
  ProcessBeforeProducesTransformToFinalConsumesIndices,
> {
  pub process_before: ProcessBefore,
  pub final_step: FinalStep,
  pub phantom_data: PhantomData<ProcessBeforeProducesTransformToFinalConsumesIndices>,
}

impl<
  ProcessBefore: FlowingProcess,
  FinalConsumes: ParamList,
  FinalStep: Final<Consumes = FinalConsumes>,
  ProcessBeforeProducesTransformToFinalConsumesIndices,
> FinalizedProcess
  for FlowingFinalizedProcess<
    ProcessBefore,
    FinalConsumes,
    FinalStep,
    ProcessBeforeProducesTransformToFinalConsumesIndices,
  >
where
  ProcessBefore::Produces: TransformTo<FinalConsumes, ProcessBeforeProducesTransformToFinalConsumesIndices>,
{
  type ProcessBeforeProduces = ProcessBefore::Produces;

  async fn resume_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
    failed_input_validation_attempts: FailedInputValidationAttempts,
  ) -> RunResult {
    let outcome = self
      .process_before
      .resume_run(
        previous_run_produced,
        previous_run_yielded_at,
        user_input,
        failed_input_validation_attempts,
      )
      .await?;
    match outcome {
      IntermediateRunOutcome::Continue(val) => self.continue_run(val).await,
      IntermediateRunOutcome::Yield(a, b, c) => Ok(RunOutcome::Yield(a, b, c)),
      IntermediateRunOutcome::Finish(a) => Ok(RunOutcome::Finish(a)),
      IntermediateRunOutcome::RetryUserInput(a) => Ok(RunOutcome::RetryUserInput(a)),
    }
  }

  async fn continue_run(&self, process_before_produces: Self::ProcessBeforeProduces) -> RunResult {
    Ok(RunOutcome::Finish(
      self.final_step.handle(process_before_produces.transform()).await?,
    ))
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    // most likely not worth to assign an index to final steps, but maybe test
    self.process_before.enumerate_steps(last_used_index)
  }
}
