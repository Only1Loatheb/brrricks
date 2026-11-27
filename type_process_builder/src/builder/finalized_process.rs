pub mod final_finalized_process;
pub mod subprocess_finalized_process;

use crate::builder::StepIndex;
use crate::builder::runnable_process::RunnableProcess;

pub(crate) mod sealed {
  use crate::builder::{PreviousRunYieldedAt, RunResult, StepIndex};
  use crate::error_coproduct::ErrorCoproduct;
  use crate::param_list::ParamList;
  use crate::step::{BackToken, ProcessMessages};
  use std::future::Future;

  pub trait FinalizedProcess: Sized + Send + Sync {
    // Please specify all associated types at the impl FinalizedProcess side for inference to work.
    type EntryConsumes: ParamList;
    type ProcessBeforeProduces: ParamList;
    type Messages: ProcessMessages;
    type EverProduced: ParamList;
    type Errors: ErrorCoproduct;

    fn resume_run(
      &self,
      previous_run_produced: &[u8],
      previous_run_yielded_at: PreviousRunYieldedAt,
      user_input: String,
      form_context: Option<&[u8]>,
      back_token: Option<BackToken>,
    ) -> impl Future<Output = RunResult<Self::Messages, Self::Errors>> + Send;

    fn continue_run(
      &self,
      process_before_produces: Self::ProcessBeforeProduces,
      back_token: Option<BackToken>,
    ) -> impl Future<Output = RunResult<Self::Messages, Self::Errors>> + Send;

    fn run_from_entry(
      &self,
      entry_consumes: Self::EntryConsumes,
      back_token: Option<BackToken>,
    ) -> impl Future<Output = RunResult<Self::Messages, Self::Errors>> + Send;

    fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex;
  }
}

pub trait FinalizedProcess: sealed::FinalizedProcess {
  fn build(mut self, name: &'static str, version: u32) -> RunnableProcess<Self> {
    self.enumerate_steps(StepIndex::MIN);
    RunnableProcess { finalized_process: self, name, version }
  }
}

impl<T: sealed::FinalizedProcess> FinalizedProcess for T {}
