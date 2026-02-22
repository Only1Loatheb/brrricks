use crate::builder::finalized_process::FinalizedProcess;
use crate::builder::{ParamUID, PreviousRunYieldedAt, RunResult, SessionContext};
use crate::step::FailedInputValidationAttempts;

pub struct RunnableProcess<UnderlyingProcess: FinalizedProcess> {
  finalized_process: UnderlyingProcess, // shouldn't be public
  name: &'static str,
  version: u32,
}

impl<UnderlyingProcess: FinalizedProcess> RunnableProcess<UnderlyingProcess> {
  pub fn new(mut finalized_process: UnderlyingProcess, name: &'static str, version: u32) -> Self {
    finalized_process
      .enumerate_steps(std::num::NonZero::<u32>::MIN)
      .unwrap();
    Self {
      finalized_process,
      name,
      version,
    }
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

  pub fn get_name(&self) -> &'static str {
    self.name
  }

  pub fn get_version(&self) -> u32 {
    self.version
  }
}
