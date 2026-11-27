pub mod split_process_form_splitter;
pub mod split_process_splitter;

use crate::builder::case_entry::{CaseEntry, case_entry};
use crate::builder::{
  FinalizedProcess, FirstCaseOfFinalizedSplitProcess, FirstCaseOfFlowingSplitProcess, FlowingProcess,
  WILL_BE_RENUMBERED,
};
use crate::param_list::concat::Concat;

pub(crate) mod sealed {
  use crate::builder::{
    IntermediateFinalizedSplitResult, MaybeFormContext, ParamList, PreviousRunYieldedAt, SessionContext, StepIndex,
  };
  use crate::error_list::ErrorCoproduct;
  use crate::frunk::coproduct::Coproduct;
  use crate::param_list::concat::Concat;
  use crate::step::{BackToken, ProcessMessages};
  use std::future::Future;

  pub trait SplitProcess<SplitterProducesForOtherCases: Send + Sync>: Sized + Send + Sync {
    // Please specify all associated types at the impl SplitProcess side for inference to work.
    type EntryConsumes: ParamList;
    type ProcessBeforeSplitProduces: ParamList;
    type SplitterProducesForFirstCase: ParamList
      + Concat<Self::ProcessBeforeSplitProduces>
      + Concat<Self::ProcessBeforeSplitEverProduced>;
    type SplitterTagForFirstCase: Send + Sync;
    type Messages: ProcessMessages;
    type ProcessBeforeSplitEverProduced: ParamList;
    type EverProduced: ParamList;
    type Errors: ErrorCoproduct;

    fn resume_run(
      &self,
      previous_run_produced: SessionContext,
      previous_run_yielded_at: PreviousRunYieldedAt,
      user_input: String,
      form_context: MaybeFormContext,
      back_token: Option<BackToken>,
    ) -> impl Future<
      Output = IntermediateFinalizedSplitResult<
        Self::ProcessBeforeSplitProduces,
        Coproduct<Self::SplitterProducesForFirstCase, SplitterProducesForOtherCases>,
        Self::Messages,
        Self::Errors,
      >,
    > + Send;

    fn continue_run(
      &self,
      process_before_split_produced: Self::ProcessBeforeSplitProduces,
      back_token: Option<BackToken>,
    ) -> impl Future<
      Output = IntermediateFinalizedSplitResult<
        Self::ProcessBeforeSplitProduces,
        Coproduct<Self::SplitterProducesForFirstCase, SplitterProducesForOtherCases>,
        Self::Messages,
        Self::Errors,
      >,
    > + Send;

    fn run_from_entry(
      &self,
      entry_consumes: Self::EntryConsumes,
      back_token: Option<BackToken>,
    ) -> impl Future<
      Output = IntermediateFinalizedSplitResult<
        Self::ProcessBeforeSplitProduces,
        Coproduct<Self::SplitterProducesForFirstCase, SplitterProducesForOtherCases>,
        Self::Messages,
        Self::Errors,
      >,
    > + Send;

    fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex;
  }
}

/// A process state immediately after `.split(...)` or `.show_split(...)`, awaiting the first branch.
/// Splits require at least two cases (enforced by [`crate::step::SplitterOutput`]); the first case
/// transitions into a split-case state and subsequent cases complete the split once all variants are handled.
pub trait SplitProcess<SplitterProducesForOtherCases: Send + Sync>:
  sealed::SplitProcess<SplitterProducesForOtherCases>
{
  fn case_end<
    ThisCase: FinalizedProcess<
        EntryConsumes = <Self::SplitterProducesForFirstCase as Concat<Self::ProcessBeforeSplitProduces>>::Concatenated,
        Messages = Self::Messages,
      >,
    ErrorUnionIndices: Sync + Send,
  >(
    self,
    _assumed_tag: Self::SplitterTagForFirstCase,
    create_case: impl FnOnce(
      CaseEntry<
        <Self::SplitterProducesForFirstCase as Concat<Self::ProcessBeforeSplitProduces>>::Concatenated,
        <Self::SplitterProducesForFirstCase as Concat<Self::ProcessBeforeSplitEverProduced>>::Concatenated,
        Self::Messages,
      >,
    ) -> ThisCase,
  ) -> FirstCaseOfFinalizedSplitProcess<
    Self::SplitterTagForFirstCase,
    Self::SplitterProducesForFirstCase,
    SplitterProducesForOtherCases,
    Self,
    ThisCase,
    ErrorUnionIndices,
  >
  where
    Self::SplitterProducesForFirstCase: Concat<Self::ProcessBeforeSplitProduces>,
  {
    FirstCaseOfFinalizedSplitProcess {
      split_process_before: self,
      case_index: WILL_BE_RENUMBERED,
      this_case: create_case(case_entry::<
        <Self::SplitterProducesForFirstCase as Concat<Self::ProcessBeforeSplitProduces>>::Concatenated,
        <Self::SplitterProducesForFirstCase as Concat<Self::ProcessBeforeSplitEverProduced>>::Concatenated,
        Self::Messages,
      >()),
      phantom_data: Default::default(),
    }
  }

  fn case_via<
    ThisCase: FlowingProcess<
        EntryConsumes = <Self::SplitterProducesForFirstCase as Concat<Self::ProcessBeforeSplitProduces>>::Concatenated,
        Messages = Self::Messages,
      >,
    ErrorUnionIndices: Sync + Send,
  >(
    self,
    _assumed_tag: Self::SplitterTagForFirstCase,
    create_case: impl FnOnce(
      CaseEntry<
        <Self::SplitterProducesForFirstCase as Concat<Self::ProcessBeforeSplitProduces>>::Concatenated,
        <Self::SplitterProducesForFirstCase as Concat<Self::ProcessBeforeSplitEverProduced>>::Concatenated,
        Self::Messages,
      >,
    ) -> ThisCase,
  ) -> FirstCaseOfFlowingSplitProcess<
    Self::SplitterTagForFirstCase,
    Self::SplitterProducesForFirstCase,
    SplitterProducesForOtherCases,
    Self,
    ThisCase,
    ErrorUnionIndices,
  >
  where
    Self::SplitterProducesForFirstCase: Concat<Self::ProcessBeforeSplitProduces>,
  {
    FirstCaseOfFlowingSplitProcess {
      split_process_before: self,
      case_index: WILL_BE_RENUMBERED,
      this_case: create_case(case_entry::<
        <Self::SplitterProducesForFirstCase as Concat<Self::ProcessBeforeSplitProduces>>::Concatenated,
        <Self::SplitterProducesForFirstCase as Concat<Self::ProcessBeforeSplitEverProduced>>::Concatenated,
        Self::Messages,
      >()),
      phantom_data: Default::default(),
    }
  }
}

impl<SplitterProducesForOtherCases: Send + Sync, T: sealed::SplitProcess<SplitterProducesForOtherCases>>
  SplitProcess<SplitterProducesForOtherCases> for T
{
}
