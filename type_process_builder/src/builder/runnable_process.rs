use crate::builder::finalized_process::FinalizedProcess;
use crate::builder::{PreviousRunYieldedAt, RunResult};
use std::io;
use serde_value::Value;

pub struct RunnableProcess<FINALIZED_PROCESS: FinalizedProcess> {
  finalized_process: FINALIZED_PROCESS, // shouldn't be public
}

impl<FINALIZED_PROCESS: FinalizedProcess> RunnableProcess<FINALIZED_PROCESS> {
  pub fn new(mut finalized_process: FINALIZED_PROCESS) -> Self {
    finalized_process.enumerate_steps(0);
    Self { finalized_process }
  }

  pub async fn run(&self, previous_run_produced: Value) -> RunResult {
    self.continue_run(previous_run_produced, PreviousRunYieldedAt(0)).await
  }

  pub async fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded: PreviousRunYieldedAt,
  ) -> RunResult {
    self
      .finalized_process
      .continue_run(previous_run_produced, previous_run_yielded)
      .await
  }
}
