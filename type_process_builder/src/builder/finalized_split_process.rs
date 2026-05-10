pub mod first_case_of_finalized_split_process;
pub mod next_case_of_finalized_split_process;

use crate::builder::{IntermediateFinalizedSplitResult, ParamUID, PreviousRunYieldedAt, SessionContext, StepIndex};
use crate::param_list::ParamList;
use crate::param_list::concat::Concat;
use crate::step::{FailedInputValidationAttempts, ProcessMessages};
use frunk_core::coproduct::Coproduct;
use std::future::Future;

pub trait FinalizedSplitProcess<SplitterProducesForOtherCases>: Sized + Send + Sync {
  // Please specify all associated types at the impl FinalizedSplitProcess side for inference to work.
  type ProcessBeforeSplitProduces: ParamList;
  type SplitterProducesForThisCase: ParamList + Concat<Self::ProcessBeforeSplitProduces>;
  type SplitterTagForThisCase: Send + Sync;
  type SubprocessConsumes: ParamList;
  type Messages: ProcessMessages;

  fn resume_run(
    &self,
    previous_run_produced: SessionContext,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
    failed_input_validation_attempts: FailedInputValidationAttempts,
  ) -> impl Future<
    Output = IntermediateFinalizedSplitResult<
      Self::ProcessBeforeSplitProduces,
      SplitterProducesForOtherCases,
      Self::Messages,
    >,
  > + Send;

  fn continue_run(
    &self,
    process_before_split_produced: Self::ProcessBeforeSplitProduces,
    splitter_produces_for_this_case_or_other_cases_consumes: Coproduct<
      Self::SplitterProducesForThisCase,
      SplitterProducesForOtherCases,
    >,
  ) -> impl Future<
    Output = IntermediateFinalizedSplitResult<
      Self::ProcessBeforeSplitProduces,
      SplitterProducesForOtherCases,
      Self::Messages,
    >,
  > + Send;

  fn run_split_subprocess(
    &self,
    subprocess_consumes: Self::SubprocessConsumes,
  ) -> impl Future<
    Output = IntermediateFinalizedSplitResult<
      Self::ProcessBeforeSplitProduces,
      SplitterProducesForOtherCases,
      Self::Messages,
    >,
  > + Send;

  fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex;

  fn all_param_uids(&self, acc: &mut Vec<ParamUID>);
}
