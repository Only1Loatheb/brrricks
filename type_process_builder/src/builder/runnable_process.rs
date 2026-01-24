use crate::builder::finalized_process::FinalizedProcess;
use crate::builder::{PreviousRunYieldedAt, RunResult};
use serde_value::Value;

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
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
  ) -> RunResult {
    self
      .finalized_process
      .resume_run(previous_run_produced, previous_run_yielded_at, user_input)
      .await
  }
}
