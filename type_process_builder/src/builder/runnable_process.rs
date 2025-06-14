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

  pub async fn run(&self, previous_run_produced: Value) -> RunResult {
    self.continue_run(previous_run_produced, PreviousRunYieldedAt(0)).await
  }

  pub async fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
  ) -> RunResult {
    self
      .finalized_process
      .continue_run(previous_run_produced, previous_run_yielded_at)
      .await
  }
}
