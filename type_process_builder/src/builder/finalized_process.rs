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
  ProcessBefore: FlowingProcess,
  FinalConsumes: ParamList,
  FinalStep: Final<FinalConsumes>,
> {
  pub process_before: ProcessBefore,
  pub final_step: FinalStep,
  pub phantom_data: PhantomData<FinalConsumes>,
}

impl<ProcessBefore: FlowingProcess, FinalConsumes: ParamList, FinalStep: Final<FinalConsumes>> FinalizedProcess
  for FlowingFinalizedProcess<ProcessBefore, FinalConsumes, FinalStep>
{
  async fn continue_run(&self, previous_run_produced: Value, previous_run_yielded: PreviousRunYieldedAt) -> RunResult {
    todo!()
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    // most likely not worth to assign an index to final steps, but maybe test
    self.process_before.enumerate_steps(last_used_index)
  }
}

pub struct SplitFinalizedProcess<FinalizedExhaustiveSplit: FinalizedSplitProcess> {
  process: FinalizedExhaustiveSplit, // maybe box?
}

impl<FinalizedExhaustiveSplit: FinalizedSplitProcess> FinalizedProcess
  for SplitFinalizedProcess<FinalizedExhaustiveSplit>
{
  async fn continue_run(&self, previous_run_produced: Value, previous_run_yielded: PreviousRunYieldedAt) -> RunResult {
    todo!()
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    // most likely not worth to assign an index to final steps, but maybe test
    self.process.enumerate_steps(last_used_index)
  }
}
