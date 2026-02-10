use crate::builder::finalized_process::FinalizedProcess;
use crate::builder::{PreviousRunYieldedAt, RunResult, SessionContext};
use crate::step::FailedInputValidationAttempts;

pub struct RunnableProcess<UnderlyingProcess: FinalizedProcess> {
  finalized_process: UnderlyingProcess, // shouldn't be public
}

impl<UnderlyingProcess: FinalizedProcess> RunnableProcess<UnderlyingProcess> {
  pub fn new(mut finalized_process: UnderlyingProcess) -> Self {
    finalized_process.enumerate_steps(0);
    Self { finalized_process }
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
}
