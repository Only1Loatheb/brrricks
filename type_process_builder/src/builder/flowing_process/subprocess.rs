use crate::builder::{
  FlowingProcess, IntermediateRunOutcome, IntermediateRunResult, ParamList, ParamUID, PreviousRunYieldedAt,
  SessionContext, StepIndex,
};
use crate::step::{FailedInputValidationAttempts, ProcessMessages};
use std::marker::PhantomData;

pub struct Subprocess<ProcessBeforeProduces, Messages> {
  pub phantom_data: PhantomData<(ProcessBeforeProduces, Messages)>,
}

impl<ProcessBeforeProduces: ParamList, Messages: ProcessMessages> FlowingProcess
  for Subprocess<ProcessBeforeProduces, Messages>
{
  type ProcessBeforeProduces = ProcessBeforeProduces;
  type Produces = ProcessBeforeProduces;
  type SubprocessConsumes = ProcessBeforeProduces;
  type Messages = Messages;

  async fn resume_run(
    &self,
    previous_run_produced: SessionContext,
    _previous_run_yielded_at: PreviousRunYieldedAt,
    _user_input: String,
    _failed_input_validation_attempts: FailedInputValidationAttempts,
  ) -> IntermediateRunResult<Self::Produces, Self::Messages> {
    let process_before_produces = ProcessBeforeProduces::deserialize(previous_run_produced)?;
    self.continue_run(process_before_produces).await
  }

  async fn continue_run(
    &self,
    process_before_produces: Self::ProcessBeforeProduces,
  ) -> IntermediateRunResult<Self::Produces, Self::Messages> {
    Ok(IntermediateRunOutcome::Continue(process_before_produces))
  }

  async fn run_subprocess(
    &self,
    subprocess_consumes: Self::SubprocessConsumes,
  ) -> IntermediateRunResult<Self::Produces, Self::Messages> {
    self.continue_run(subprocess_consumes).await
  }

  fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex {
    last_used_index
  }

  fn all_param_uids(&self, _acc: &mut Vec<ParamUID>) {}
}

pub fn subprocess<ProcessBeforeProduces: ParamList, Messages: ProcessMessages>()
-> Subprocess<ProcessBeforeProduces, Messages> {
  Subprocess { phantom_data: Default::default() }
}
