pub mod split_process_form_splitter;
pub mod split_process_splitter;

use crate::builder::first_case_of_flowing_split_process::FirstCaseOfFlowingSplitProcess;
use crate::builder::subprocess::{Subprocess, subprocess};
use crate::builder::{
  FinalizedProcess, FirstCaseOfFinalizedSplitProcess, FlowingProcess, IntermediateFinalizedSplitResult, ParamList,
  PreviousRunYieldedAt, SessionContext, StepIndex, WILL_BE_RENUMBERED,
};
use crate::param_list::concat::Concat;
use crate::step::FailedInputValidationAttempts;
use frunk_core::coproduct::Coproduct;
use std::future::Future;

/// We enforce at least one cases in the split.
/// We could remove an unnecessary option of implementing a linear proces with a series of splits with single case,
/// but sometimes the need arises to have a select form with one option that is different from input form.
/// We at least remove an illegal state of unfinalized finalized split process.
pub trait SplitProcess<SplitterProducesForOtherCases>: Sized + Sync {
  type ProcessBeforeSplitProduces: ParamList;
  type SplitterProducesForFirstCase: ParamList + Concat<Self::ProcessBeforeSplitProduces>;
  type SplitterTagForFirstCase;

  fn resume_run(
    &self,
    previous_run_produced: SessionContext,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
    failed_input_validation_attempts: FailedInputValidationAttempts,
  ) -> impl Future<
    Output = IntermediateFinalizedSplitResult<
      Self::ProcessBeforeSplitProduces,
      Coproduct<Self::SplitterProducesForFirstCase, SplitterProducesForOtherCases>,
    >,
  >;

  fn continue_run(
    &self,
    process_before_split_produced: Self::ProcessBeforeSplitProduces,
  ) -> impl Future<
    Output = IntermediateFinalizedSplitResult<
      Self::ProcessBeforeSplitProduces,
      Coproduct<Self::SplitterProducesForFirstCase, SplitterProducesForOtherCases>,
    >,
  >;

  fn case<ThisCase: FinalizedProcess>(
    self,
    _assumed_tag: Self::SplitterTagForFirstCase,
    create_case: impl FnOnce(
      Subprocess<<Self::SplitterProducesForFirstCase as Concat<Self::ProcessBeforeSplitProduces>>::Concatenated>,
    ) -> ThisCase,
  ) -> FirstCaseOfFinalizedSplitProcess<
    Self::SplitterTagForFirstCase,
    Self::SplitterProducesForFirstCase,
    SplitterProducesForOtherCases,
    Self,
    ThisCase,
  >
  where
    Self::SplitterProducesForFirstCase: Concat<Self::ProcessBeforeSplitProduces>,
  {
    FirstCaseOfFinalizedSplitProcess {
      split_process_before: self,
      this_case: create_case(subprocess::<
        <Self::SplitterProducesForFirstCase as Concat<Self::ProcessBeforeSplitProduces>>::Concatenated,
      >()),
      first_step_in_case_index: WILL_BE_RENUMBERED,
      phantom_data: Default::default(),
    }
  }

  fn case_via<
    ThisCase: FlowingProcess<ProcessBeforeProduces=<Self::SplitterProducesForFirstCase as Concat<Self::ProcessBeforeSplitProduces>>::Concatenated>,
  >(
    self,
    _assumed_tag: Self::SplitterTagForFirstCase,
    create_case: impl FnOnce(
      Subprocess<<Self::SplitterProducesForFirstCase as Concat<Self::ProcessBeforeSplitProduces>>::Concatenated>,
    ) -> ThisCase,
  ) -> FirstCaseOfFlowingSplitProcess<
    Self::SplitterTagForFirstCase,
    Self::SplitterProducesForFirstCase,
    SplitterProducesForOtherCases,
    Self,
    ThisCase,
  >
  where
    Self::SplitterProducesForFirstCase: Concat<Self::ProcessBeforeSplitProduces>,
  {
    FirstCaseOfFlowingSplitProcess {
      split_process_before: self,
      this_case: create_case(subprocess::<
        <Self::SplitterProducesForFirstCase as Concat<Self::ProcessBeforeSplitProduces>>::Concatenated,
      >()),
      first_step_in_case_index: WILL_BE_RENUMBERED,
      phantom_data: Default::default(),
    }
  }

  fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex;
}
