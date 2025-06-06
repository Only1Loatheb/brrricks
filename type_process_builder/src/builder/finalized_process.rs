use crate::builder::finalized_split_process::FinalizedSplitProcess;
use crate::builder::flowing_process::FlowingProcess;
use crate::builder::runnable_process::RunnableProcess;
use crate::builder::{PreviousRunYieldedAt, RunResult};
use crate::param_list::ParamList;
use crate::step::step::Final;
use serde_value::Value;
use std::future::Future;
use std::marker::PhantomData;

pub trait FinalizedProcess: Sized {
  fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded: PreviousRunYieldedAt,
  ) -> impl Future<Output = RunResult>;

  fn build(self) -> RunnableProcess<Self> {
    RunnableProcess::new(self)
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize;
}

pub struct FlowingFinalizedProcess<
  PROCESS_BEFORE: FlowingProcess,
  FINAL_CONSUMES: ParamList,
  FINAL_STEP: Final<FINAL_CONSUMES>,
> {
  pub process_before: PROCESS_BEFORE,
  pub final_step: FINAL_STEP,
  pub phantom_data: PhantomData<FINAL_CONSUMES>,
}

impl<PROCESS_BEFORE: FlowingProcess, FINAL_CONSUMES: ParamList, FINAL_STEP: Final<FINAL_CONSUMES>> FinalizedProcess
  for FlowingFinalizedProcess<PROCESS_BEFORE, FINAL_CONSUMES, FINAL_STEP>
{
  async fn continue_run(&self, previous_run_produced: Value, previous_run_yielded: PreviousRunYieldedAt) -> RunResult {
    todo!()
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    // most likely not worth to assign an index to final steps, but maybe test
    self.process_before.enumerate_steps(last_used_index)
  }
}

pub struct SplitFinalizedProcess<FINALIZED_SPLIT_PROCESS: FinalizedSplitProcess> {
  process: FINALIZED_SPLIT_PROCESS, // maybe box?
}

impl<FINALIZED_SPLIT_PROCESS: FinalizedSplitProcess> FinalizedProcess
  for SplitFinalizedProcess<FINALIZED_SPLIT_PROCESS>
{
  async fn continue_run(&self, previous_run_produced: Value, previous_run_yielded: PreviousRunYieldedAt) -> RunResult {
    todo!()
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    // most likely not worth to assign an index to final steps, but maybe test
    self.process.enumerate_steps(last_used_index)
  }
}
