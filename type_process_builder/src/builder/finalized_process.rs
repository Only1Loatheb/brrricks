use crate::builder::finalized_split_process::FinalizedSplitProcess;
use crate::builder::flowing_process::FlowingProcess;
use crate::builder::{IntermediateRunResult, PreviousRunYieldedAt, ProcessBuilder, RunResult};
use crate::step::param_list::ParamList;
use crate::step::step::Final;
use frunk_core::hlist::HNil;
use std::io;
use std::marker::PhantomData;

pub trait FinalizedProcess: ProcessBuilder {
  async fn continue_run(
    &self,
    previous_run_produced: impl io::Read,
    previous_run_yielded: PreviousRunYieldedAt,
  ) -> RunResult; // fixme create result type for finalised process, or undo changes
}

pub struct FlowingFinalizedProcess<PROCESS_BEFORE: FlowingProcess, FINAL_CONSUMES: ParamList, FINAL_STEP: Final<FINAL_CONSUMES>> {
  pub process_before: PROCESS_BEFORE,
  pub final_step: FINAL_STEP,
  pub phantom_data: PhantomData<FINAL_CONSUMES>,
}

impl<PROCESS_BEFORE: FlowingProcess, FINAL_CONSUMES: ParamList, FINAL_STEP: Final<FINAL_CONSUMES>> ProcessBuilder
  for FlowingFinalizedProcess<PROCESS_BEFORE, FINAL_CONSUMES, FINAL_STEP>
{
  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    // most likely not worth to assign an index to final steps, but maybe test
    self.process_before.enumerate_steps(last_used_index)
  }
}

impl<PROCESS_BEFORE: FlowingProcess, FINAL_CONSUMES: ParamList, FINAL_STEP: Final<FINAL_CONSUMES>> FinalizedProcess
  for FlowingFinalizedProcess<PROCESS_BEFORE, FINAL_CONSUMES, FINAL_STEP>
{
  async fn continue_run(
    &self,
    previous_run_produced: impl io::Read,
    previous_run_yielded: PreviousRunYieldedAt,
  ) -> RunResult {
    todo!()
  }
}

pub struct SplitFinalizedProcess<FINALIZED_SPLIT_PROCESS: FinalizedSplitProcess> {
  process: FINALIZED_SPLIT_PROCESS, // maybe box?
}

impl<FINALIZED_SPLIT_PROCESS: FinalizedSplitProcess> ProcessBuilder for SplitFinalizedProcess<FINALIZED_SPLIT_PROCESS> {
  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    // most likely not worth to assign an index to final steps, but maybe test
    self.process.enumerate_steps(last_used_index)
  }
}

impl<FINALIZED_SPLIT_PROCESS: FinalizedSplitProcess> FinalizedProcess for SplitFinalizedProcess<FINALIZED_SPLIT_PROCESS> {
  async fn continue_run(
    &self,
    previous_run_produced: impl io::Read,
    previous_run_yielded: PreviousRunYieldedAt,
  ) -> RunResult {
    todo!()
  }
}
