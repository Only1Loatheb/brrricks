use crate::builder::{
  FlowingProcess, IntermediateRunOutcome, IntermediateRunResult, ParamList, PreviousRunYieldedAt, SessionContext,
};
use crate::param_list::clone_just::CloneJust;
use crate::param_list::concat::Concat;
use crate::step::{FailedInputValidationAttempts, Operation};
use std::marker::PhantomData;

pub struct OperationFlowingProcess<
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
> FlowingProcess
  for OperationFlowingProcess<ProcessBefore, OperationStep, ProcessBeforeProducesToLastStepConsumesIndices>
where
  for<'a> &'a ProcessBefore::Produces: CloneJust<LastStepConsumes, ProcessBeforeProducesToLastStepConsumesIndices>,
{
  type ProcessBeforeProduces = ProcessBefore::Produces;
  type Produces = <LastStepProduces as Concat<ProcessBefore::Produces>>::Concatenated;

  async fn resume_run(
    &self,
    previous_run_produced: SessionContext,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
    failed_input_validation_attempts: FailedInputValidationAttempts,
  ) -> IntermediateRunResult<Self::Produces> {
    if previous_run_yielded_at.0 < self.step_index {
      let process_before_output = self
        .process_before
        .resume_run(
          previous_run_produced,
          previous_run_yielded_at,
          user_input,
          failed_input_validation_attempts,
        )
        .await?;
      match process_before_output {
        IntermediateRunOutcome::Continue(process_before_produces) => self.continue_run(process_before_produces).await,
        IntermediateRunOutcome::Yield(a, b, c) => Ok(IntermediateRunOutcome::Yield(a, b, c)),
        IntermediateRunOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
        IntermediateRunOutcome::RetryUserInput(a) => Ok(IntermediateRunOutcome::RetryUserInput(a)),
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
    let last_step_consumes = process_before_produces.clone_just();
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
