use crate::builder::IntoProcessError;
use crate::builder::case_entry::{CaseEntry, case_entry};
use crate::builder::flowing_split_process::sealed;
use crate::builder::{
  FinalizedCaseOfFlowingSplitProcess, FinalizedProcess, FlowingCaseOfFlowingSplitProcess, FlowingProcess,
  IntermediateFinalizedSplitOutcome, IntermediateFlowingSplitOutcome, IntermediateFlowingSplitResult,
  IntermediateRunOutcome, ParamList, PreviousRunYieldedAt, SplitProcess, StepIndex, WILL_BE_RENUMBERED,
};
use crate::error_coproduct::union::CoprodUnion;
use crate::frunk::coproduct::Coproduct;
use crate::param_list::concat::Concat;
use crate::param_list::union::Union;
use crate::step::BackToken;
use std::marker::PhantomData;

#[must_use]
pub struct FirstCaseOfFlowingSplitProcess<
  ThisTag: Send + Sync,
  SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  SplitterProducesForOtherCases: Send + Sync,
  ProcessBefore: SplitProcess<SplitterProducesForOtherCases>,
  ThisCase: FlowingProcess<
    EntryConsumes = <SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
  >,
  ErrorUnionIndices: Sync + Send,
> {
  pub(crate) split_process_before: ProcessBefore,
  pub(crate) case_index: StepIndex,
  pub(crate) this_case: ThisCase,
  pub(crate) phantom_data:
    PhantomData<(ThisTag, SplitterProducesForThisCase, SplitterProducesForOtherCases, ErrorUnionIndices)>,
}

impl<
  NextTag: Send + Sync,
  SplitterProducesForOtherCases: Send + Sync,
  ProcessBefore: SplitProcess<Coproduct<(NextTag, SplitterProducesForNextCase), SplitterProducesForOtherCases>>,
  SplitterProducesForNextCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces> + Concat<ProcessBefore::ProcessBeforeSplitEverProduced>,
  ThisCase: FlowingProcess<
      EntryConsumes = <ProcessBefore::SplitterProducesForFirstCase as Concat<
        ProcessBefore::ProcessBeforeSplitProduces,
      >>::Concatenated,
      Messages = ProcessBefore::Messages,
    >,
  ErrorUnionIndices: Sync + Send,
>
  FirstCaseOfFlowingSplitProcess<
    ProcessBefore::SplitterTagForFirstCase,
    ProcessBefore::SplitterProducesForFirstCase,
    Coproduct<(NextTag, SplitterProducesForNextCase), SplitterProducesForOtherCases>,
    ProcessBefore,
    ThisCase,
    ErrorUnionIndices,
  >
where
  ThisCase::EverProduced: Union<ProcessBefore::EverProduced>,
  ThisCase::Errors: CoprodUnion<ProcessBefore::Errors, ErrorUnionIndices>,
{
  pub fn case_end<
    NextCase: FinalizedProcess<
      EntryConsumes=<SplitterProducesForNextCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
      Messages = ProcessBefore::Messages,
    >,
    NextErrorIndices: Sync + Send,
  >(
    self,
    _assumed_tag: NextTag,
    create_case: impl FnOnce(CaseEntry<
      <SplitterProducesForNextCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
      <SplitterProducesForNextCase as Concat<ProcessBefore::ProcessBeforeSplitEverProduced>>::Concatenated,
      ProcessBefore::Messages,
    >) -> NextCase,
  ) -> FinalizedCaseOfFlowingSplitProcess<
    NextTag,
    SplitterProducesForNextCase,
    SplitterProducesForOtherCases,
    Self,
    NextCase,
    NextErrorIndices,
  >
{
    FinalizedCaseOfFlowingSplitProcess {
      split_process_before: self,
      case_index: WILL_BE_RENUMBERED,
      this_case: create_case(case_entry::<
        <SplitterProducesForNextCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
        <SplitterProducesForNextCase as Concat<ProcessBefore::ProcessBeforeSplitEverProduced>>::Concatenated,
        ProcessBefore::Messages,
      >()),
      phantom_data: Default::default(),
    }
  }

  pub fn case_via<
    NextCase: FlowingProcess<
      EntryConsumes=
        <SplitterProducesForNextCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
      Messages = ProcessBefore::Messages,
    >,
    Indices: Sync + Send,
    NextErrorIndices: Sync + Send,
  >(
    self,
    _assumed_tag: NextTag,
    create_case: impl FnOnce(CaseEntry<
      <SplitterProducesForNextCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
      <SplitterProducesForNextCase as Concat<ProcessBefore::ProcessBeforeSplitEverProduced>>::Concatenated,
      ProcessBefore::Messages,
    >) -> NextCase,
  ) -> FlowingCaseOfFlowingSplitProcess<
    NextTag,
    SplitterProducesForNextCase,
    SplitterProducesForOtherCases,
    Self,
    NextCase,
    Indices,
    NextErrorIndices,
  >
{
    FlowingCaseOfFlowingSplitProcess {
      split_process_before: self,
      case_index: WILL_BE_RENUMBERED,
      this_case: create_case(case_entry::<
        <SplitterProducesForNextCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
        <SplitterProducesForNextCase as Concat<ProcessBefore::ProcessBeforeSplitEverProduced>>::Concatenated,
        ProcessBefore::Messages,
      >()),
      phantom_data: Default::default(),
    }
  }
}

impl<
  ThisTag: Send + Sync,
  SplitterProducesForOtherCases: Send + Sync,
  ProcessBefore: SplitProcess<SplitterProducesForOtherCases>,
  ThisCase: FlowingProcess<
      EntryConsumes = <ProcessBefore::SplitterProducesForFirstCase as Concat<
        ProcessBefore::ProcessBeforeSplitProduces,
      >>::Concatenated,
      Messages = ProcessBefore::Messages,
    >,
  ErrorUnionIndices: Sync + Send,
> sealed::FlowingSplitProcess<SplitterProducesForOtherCases>
  for FirstCaseOfFlowingSplitProcess<
    ThisTag,
    ProcessBefore::SplitterProducesForFirstCase,
    SplitterProducesForOtherCases,
    ProcessBefore,
    ThisCase,
    ErrorUnionIndices,
  >
where
  ThisCase::EverProduced: Union<ProcessBefore::EverProduced>,
  <ThisCase::EverProduced as Union<ProcessBefore::EverProduced>>::Union: ParamList,
  ThisCase::Errors: CoprodUnion<ProcessBefore::Errors, ErrorUnionIndices>,
{
  type EntryConsumes = ProcessBefore::EntryConsumes;
  type ProcessBeforeSplitProduces = ProcessBefore::ProcessBeforeSplitProduces;
  type SplitterProducesForThisCase = ProcessBefore::SplitterProducesForFirstCase;
  type EveryFlowingCaseProduces = ThisCase::Produces;
  type Messages = ProcessBefore::Messages;
  type ProcessBeforeSplitEverProduced = ProcessBefore::ProcessBeforeSplitEverProduced;
  type EverProduced = <ThisCase::EverProduced as Union<ProcessBefore::EverProduced>>::Union;
  type Errors = <ThisCase::Errors as CoprodUnion<ProcessBefore::Errors, ErrorUnionIndices>>::Union;

  async fn resume_run(
    &self,
    previous_run_produced: &[u8],
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: &str,
    form_context: Option<&[u8]>,
    back_token: Option<BackToken>,
  ) -> IntermediateFlowingSplitResult<
    ProcessBefore::ProcessBeforeSplitProduces,
    SplitterProducesForOtherCases,
    ThisCase::Produces,
    Self::Messages,
    Self::Errors,
  > {
    if previous_run_yielded_at.0 < self.case_index {
      let process_before_output = self
        .split_process_before
        .resume_run(previous_run_produced, previous_run_yielded_at, user_input, form_context, back_token)
        .await
        .into_process_err()?;
      match process_before_output {
        IntermediateFinalizedSplitOutcome::GoToCase {
          process_before_split_produced,
          splitter_produces_to_other_cases,
        } => self.continue_run(process_before_split_produced, splitter_produces_to_other_cases, back_token).await,
        IntermediateFinalizedSplitOutcome::Yield(a, b, c, d) => Ok(IntermediateFlowingSplitOutcome::Yield(a, b, c, d)),
        IntermediateFinalizedSplitOutcome::Finish(a) => Ok(IntermediateFlowingSplitOutcome::Finish(a)),
        IntermediateFinalizedSplitOutcome::RetryUserInput(a, b) => {
          Ok(IntermediateFlowingSplitOutcome::RetryUserInput(a, b))
        },
        IntermediateFinalizedSplitOutcome::Back => Ok(IntermediateFlowingSplitOutcome::Back),
      }
    } else {
      match self
        .this_case
        .resume_run(previous_run_produced, previous_run_yielded_at, user_input, form_context, back_token)
        .await
        .into_process_err()?
      {
        IntermediateRunOutcome::Continue(a) => Ok(IntermediateFlowingSplitOutcome::Continue(a)),
        IntermediateRunOutcome::Yield(a, b, c, d) => Ok(IntermediateFlowingSplitOutcome::Yield(a, b, c, d)),
        IntermediateRunOutcome::Finish(a) => Ok(IntermediateFlowingSplitOutcome::Finish(a)),
        IntermediateRunOutcome::RetryUserInput(a, b) => Ok(IntermediateFlowingSplitOutcome::RetryUserInput(a, b)),
        IntermediateRunOutcome::Back => Ok(IntermediateFlowingSplitOutcome::Back),
      }
    }
  }

  async fn run_split_from_entry(
    &self,
    entry_consumes: Self::EntryConsumes,
    back_token: Option<BackToken>,
  ) -> IntermediateFlowingSplitResult<
    Self::ProcessBeforeSplitProduces,
    SplitterProducesForOtherCases,
    Self::EveryFlowingCaseProduces,
    Self::Messages,
    Self::Errors,
  > {
    let process_before_output =
      self.split_process_before.run_from_entry(entry_consumes, back_token).await.into_process_err()?;
    match process_before_output {
      IntermediateFinalizedSplitOutcome::GoToCase {
        process_before_split_produced,
        splitter_produces_to_other_cases,
      } => self.continue_run(process_before_split_produced, splitter_produces_to_other_cases, back_token).await,
      IntermediateFinalizedSplitOutcome::Yield(a, b, c, d) => Ok(IntermediateFlowingSplitOutcome::Yield(a, b, c, d)),
      IntermediateFinalizedSplitOutcome::Finish(a) => Ok(IntermediateFlowingSplitOutcome::Finish(a)),
      IntermediateFinalizedSplitOutcome::RetryUserInput(a, b) => {
        Ok(IntermediateFlowingSplitOutcome::RetryUserInput(a, b))
      },
      IntermediateFinalizedSplitOutcome::Back => Ok(IntermediateFlowingSplitOutcome::Back),
    }
  }

  fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex {
    let used_index = self.split_process_before.enumerate_steps(last_used_index);
    self.case_index = used_index + 1;
    self.this_case.enumerate_steps(self.case_index)
  }
}

impl<
  ThisTag: Send + Sync,
  SplitterProducesForOtherCases: Send + Sync,
  ProcessBefore: SplitProcess<SplitterProducesForOtherCases>,
  ThisCase: FlowingProcess<
      EntryConsumes = <ProcessBefore::SplitterProducesForFirstCase as Concat<
        ProcessBefore::ProcessBeforeSplitProduces,
      >>::Concatenated,
      Messages = ProcessBefore::Messages,
    >,
  ErrorUnionIndices: Sync + Send,
>
  FirstCaseOfFlowingSplitProcess<
    ThisTag,
    ProcessBefore::SplitterProducesForFirstCase,
    SplitterProducesForOtherCases,
    ProcessBefore,
    ThisCase,
    ErrorUnionIndices,
  >
where
  ThisCase::EverProduced: Union<ProcessBefore::EverProduced>,
  <ThisCase::EverProduced as Union<ProcessBefore::EverProduced>>::Union: ParamList,
  ThisCase::Errors: CoprodUnion<ProcessBefore::Errors, ErrorUnionIndices>,
{
  async fn continue_run(
    &self,
    process_before_split_produced: ProcessBefore::ProcessBeforeSplitProduces,
    splitter_produces_for_this_case_or_other_cases_consumes: Coproduct<
      ProcessBefore::SplitterProducesForFirstCase,
      SplitterProducesForOtherCases,
    >,
    back_token: Option<BackToken>,
  ) -> IntermediateFlowingSplitResult<
    ProcessBefore::ProcessBeforeSplitProduces,
    SplitterProducesForOtherCases,
    ThisCase::Produces,
    ProcessBefore::Messages,
    <ThisCase::Errors as CoprodUnion<ProcessBefore::Errors, ErrorUnionIndices>>::Union,
  > {
    match splitter_produces_for_this_case_or_other_cases_consumes {
      Coproduct::Inl(splitter_produces_for_this_case) => {
        let this_case_consumes = splitter_produces_for_this_case.concat(process_before_split_produced);
        match self.this_case.run_from_entry(this_case_consumes, back_token).await.into_process_err()? {
          IntermediateRunOutcome::Continue(a) => Ok(IntermediateFlowingSplitOutcome::Continue(a)),
          IntermediateRunOutcome::Yield(a, b, c, d) => Ok(IntermediateFlowingSplitOutcome::Yield(a, b, c, d)),
          IntermediateRunOutcome::Finish(a) => Ok(IntermediateFlowingSplitOutcome::Finish(a)),
          IntermediateRunOutcome::RetryUserInput(a, b) => Ok(IntermediateFlowingSplitOutcome::RetryUserInput(a, b)),
          IntermediateRunOutcome::Back => Ok(IntermediateFlowingSplitOutcome::Back),
        }
      },
      Coproduct::Inr(splitter_produces_to_other_cases) => Ok(IntermediateFlowingSplitOutcome::GoToCase {
        process_before_split_produced,
        splitter_produces_to_other_cases,
      }),
    }
  }
}
