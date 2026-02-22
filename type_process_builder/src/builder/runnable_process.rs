use std::collections::HashSet;
use crate::builder::finalized_process::FinalizedProcess;
use crate::builder::{ParamUID, PreviousRunYieldedAt, RunResult, SessionContext};
use crate::step::FailedInputValidationAttempts;

pub struct RunnableProcess<UnderlyingProcess: FinalizedProcess> {
  finalized_process: UnderlyingProcess, // shouldn't be public
  name: &'static str,
}

impl<UnderlyingProcess: FinalizedProcess> RunnableProcess<UnderlyingProcess> {
  pub fn new(mut finalized_process: UnderlyingProcess, name: &'static str,) -> Self {
    finalized_process.enumerate_steps(0);
    Self { finalized_process, name}
  }

  pub async fn resume_run(
    &self,
    previous_run_produced: SessionContext,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
    failed_input_validation_attempts: FailedInputValidationAttempts,
  ) -> RunResult {
    self
      .finalized_process
      .resume_run(
        previous_run_produced,
        previous_run_yielded_at,
        user_input,
        failed_input_validation_attempts,
      )
      .await
  }

  pub fn all_columns(&self) -> Vec<ParamUID> {
    Vec::new()
  }
}
