use crate::builder::{
  FlowingProcess, IntermediateRunOutcome, IntermediateRunResult, ParamList, PreviousRunYieldedAt, SessionContext,
};
use crate::step::FailedInputValidationAttempts;
use std::marker::PhantomData;

pub struct Subprocess<ProcessBeforeProduces> {
  pub phantom_data: PhantomData<ProcessBeforeProduces>,
}

impl<ProcessBeforeProduces: ParamList> FlowingProcess for Subprocess<ProcessBeforeProduces> {
  type ProcessBeforeProduces = ProcessBeforeProduces;
  type Produces = ProcessBeforeProduces;

  async fn resume_run(
    &self,
    previous_run_produced: SessionContext,
    _previous_run_yielded_at: PreviousRunYieldedAt,
    _user_input: String,
    _failed_input_validation_attempts: FailedInputValidationAttempts,
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
