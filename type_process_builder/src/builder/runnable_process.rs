use crate::builder::finalized_process::FinalizedProcess;
use crate::builder::{CurrentRunYieldedAt, IntermediateRunResult, PreviousRunYieldedAt};
use process_builder_common::process_domain::Message;
use serde_json::Value;
use std::io;

pub enum RunOutcome {
  Yield(Message, Value, CurrentRunYieldedAt),
  Finish(Message),
}

type RunResult = anyhow::Result<RunOutcome>;

pub struct RunnableProcess<FINALIZED_PROCESS: FinalizedProcess> {
  finalized_process: FINALIZED_PROCESS, // shouldn't be public
}

impl<FINALIZED_PROCESS: FinalizedProcess> RunnableProcess<FINALIZED_PROCESS> {
  pub fn new(mut finalized_process: FINALIZED_PROCESS) -> Self {
    finalized_process.enumerate_steps(0);
    Self { finalized_process }
  }

  pub fn run(&self, previous_run_produced: impl io::Read, previous_run_yielded: PreviousRunYieldedAt) -> RunResult {
    self.finalized_process.continue_run(consumes, previous_run_yielded)
  }
}
