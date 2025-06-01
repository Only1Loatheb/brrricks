use crate::builder::finalized_process::FinalizedProcess;
use crate::builder::{PreviousRunYieldedAt, RunResult};
use std::io;

pub struct RunnableProcess<FINALIZED_PROCESS: FinalizedProcess> {
  finalized_process: FINALIZED_PROCESS, // shouldn't be public
}

impl<FINALIZED_PROCESS: FinalizedProcess> RunnableProcess<FINALIZED_PROCESS> {
  pub fn new(mut finalized_process: FINALIZED_PROCESS) -> Self {
    finalized_process.enumerate_steps(0);
    Self { finalized_process }
  }

  pub fn run(&self, previous_run_produced: impl io::Read, previous_run_yielded: PreviousRunYieldedAt) -> RunResult {
    self.finalized_process.continue_run(previous_run_produced, previous_run_yielded)
  }
}
