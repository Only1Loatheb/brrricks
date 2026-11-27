use crate::builder::IntoProcessError;
use crate::builder::finalized_process;
use crate::builder::finalized_split_process::sealed;
use crate::builder::case_entry::{CaseEntry, case_entry};
use crate::builder::{
  FinalizedProcess, FinalizedSplitProcess, FlowingCaseOfFinalizedSplitProcess, FlowingProcess,
  IntermediateFinalizedSplitOutcome, IntermediateFinalizedSplitResult, MaybeFormContext, ParamList,
  PreviousRunYieldedAt, RunOutcome, RunResult, SessionContext, StepIndex, WILL_BE_RENUMBERED,
};
use crate::error_list::union::CoprodUnion;
use crate::frunk::coproduct::{CNil, Coproduct};
use crate::param_list::concat::Concat;
use crate::param_list::union::Union;
use crate::step::BackToken;
use std::marker::PhantomData;

pub struct NextCaseOfFinalizedSplitProcess<
  ThisTag: Send + Sync,
  SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  SplitterProducesForOtherCases: Send + Sync,
  ProcessBefore: FinalizedSplitProcess<Coproduct<(ThisTag, SplitterProducesForThisCase), SplitterProducesForOtherCases>>,
  ThisCase: FinalizedProcess<EntryConsumes=<SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated>,
  ErrorUnionIndices: Sync + Send,
> {
  pub(crate) split_process_before: ProcessBefore,
  pub(crate) case_index: StepIndex,
  pub(crate) this_case: ThisCase,
  pub(crate) phantom_data: PhantomData<(ThisTag, SplitterProducesForThisCase, SplitterProducesForOtherCases, ErrorUnionIndices)>,
}

impl<
  ThisTag: Send + Sync,
  NextTag: Send + Sync,
  SplitterProducesForOtherCases: Send + Sync,
  ProcessBefore: FinalizedSplitProcess<
    Coproduct<
      (ThisTag, SplitterProducesForThisCase),
      Coproduct<(NextTag, SplitterProducesForNextCase), SplitterProducesForOtherCases>,
    >,
  >,
  SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  SplitterProducesForNextCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces> + Concat<ProcessBefore::ProcessBeforeSplitEverProduced>,
  ThisCase: FinalizedProcess<
    EntryConsumes=<SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
    Messages=ProcessBefore::Messages,
  >,
  ErrorUnionIndices: Sync + Send,
>
NextCaseOfFinalizedSplitProcess<
  ThisTag,
  SplitterProducesForThisCase,
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
    create_case: impl FnOnce(
      CaseEntry<
        <SplitterProducesForNextCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
        <SplitterProducesForNextCase as Concat<ProcessBefore::ProcessBeforeSplitEverProduced>>::Concatenated,
        ProcessBefore::Messages,
      >,
    ) -> NextCase,
  ) -> NextCaseOfFinalizedSplitProcess<NextTag, SplitterProducesForNextCase, SplitterProducesForOtherCases, Self, NextCase, NextErrorIndices>
  {
    NextCaseOfFinalizedSplitProcess {
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
  ) -> FlowingCaseOfFinalizedSplitProcess<
    NextTag,
    SplitterProducesForNextCase,
    SplitterProducesForOtherCases,
    Self,
    NextCase,
    NextErrorIndices,
  >
  {
    FlowingCaseOfFinalizedSplitProcess {
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
  SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  SplitterProducesForOtherCases: Send + Sync,
  ProcessBefore: FinalizedSplitProcess<Coproduct<(ThisTag, SplitterProducesForThisCase), SplitterProducesForOtherCases>>,
  ThisCase: FinalizedProcess<
    EntryConsumes=<SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
    Messages=ProcessBefore::Messages,
  >,
  ErrorUnionIndices: Sync + Send,
> sealed::FinalizedSplitProcess<SplitterProducesForOtherCases>
for NextCaseOfFinalizedSplitProcess<
  ThisTag,
  SplitterProducesForThisCase,
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
  type SplitterProducesForThisCase = SplitterProducesForThisCase;
  type SplitterTagForThisCase = ThisTag;
  type Messages = ProcessBefore::Messages;
  type ProcessBeforeSplitEverProduced = ProcessBefore::ProcessBeforeSplitEverProduced;
  type EverProduced = <ThisCase::EverProduced as Union<ProcessBefore::EverProduced>>::Union;
  type Errors = <ThisCase::Errors as CoprodUnion<ProcessBefore::Errors, ErrorUnionIndices>>::Union;

  async fn resume_run(
    &self,
    previous_run_produced: SessionContext,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
    form_context: MaybeFormContext,
    back_token: Option<BackToken>,
  ) -> IntermediateFinalizedSplitResult<Self::ProcessBeforeSplitProduces, SplitterProducesForOtherCases, Self::Messages, Self::Errors> {
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
        } => {
          let produced = match splitter_produces_to_other_cases {
            Coproduct::Inl((_pd, params)) => Coproduct::Inl(params),
            Coproduct::Inr(inr_value) => Coproduct::Inr(inr_value),
          };
          self.continue_run(process_before_split_produced, produced, back_token).await
        }
        IntermediateFinalizedSplitOutcome::Yield(a, b, c, d) => Ok(IntermediateFinalizedSplitOutcome::Yield(a, b, c, d)),
        IntermediateFinalizedSplitOutcome::Finish(a) => Ok(IntermediateFinalizedSplitOutcome::Finish(a)),
        IntermediateFinalizedSplitOutcome::RetryUserInput(a, b) => Ok(IntermediateFinalizedSplitOutcome::RetryUserInput
          (a, b)),
        IntermediateFinalizedSplitOutcome::Back => Ok(IntermediateFinalizedSplitOutcome::Back),
      }
    } else {
      match self.this_case.resume_run(
        previous_run_produced, previous_run_yielded_at, user_input, form_context, back_token,
      ).await.into_process_err()? {
        RunOutcome::Yield(a, b, c, d) => Ok(IntermediateFinalizedSplitOutcome::Yield(a, b, c, d)),
        RunOutcome::Finish(a) => Ok(IntermediateFinalizedSplitOutcome::Finish(a)),
        RunOutcome::RetryUserInput(a, b) => Ok(IntermediateFinalizedSplitOutcome::RetryUserInput(a, b)),
        RunOutcome::Back => Ok(IntermediateFinalizedSplitOutcome::Back),
      }
    }
  }

  async fn continue_run(
    &self,
    process_before_split_produced: Self::ProcessBeforeSplitProduces,
    splitter_produces_for_this_case_or_other_cases_consumes: Coproduct<
      Self::SplitterProducesForThisCase,
      SplitterProducesForOtherCases,
    >,
    back_token: Option<BackToken>,
  ) -> IntermediateFinalizedSplitResult<Self::ProcessBeforeSplitProduces, SplitterProducesForOtherCases, Self::Messages, Self::Errors> {
    match splitter_produces_for_this_case_or_other_cases_consumes {
      Coproduct::Inl(splitter_produces_for_this_case) => {
        let this_case_consumes = splitter_produces_for_this_case.concat(process_before_split_produced);
        match self.this_case.run_from_entry(this_case_consumes, back_token).await.into_process_err()? {
          RunOutcome::Yield(a, b, c, d) => Ok(IntermediateFinalizedSplitOutcome::Yield(a, b, c, d)),
          RunOutcome::Finish(a) => Ok(IntermediateFinalizedSplitOutcome::Finish(a)),
          RunOutcome::RetryUserInput(a, b) => Ok(IntermediateFinalizedSplitOutcome::RetryUserInput(a, b)),
          RunOutcome::Back => Ok(IntermediateFinalizedSplitOutcome::Back),
        }
      }
      Coproduct::Inr(splitter_produces_to_other_cases) => Ok(IntermediateFinalizedSplitOutcome::GoToCase {
        process_before_split_produced,
        splitter_produces_to_other_cases,
      }),
    }
  }

  async fn run_split_from_entry(&self, entry_consumes: Self::EntryConsumes, back_token: Option<BackToken>) ->
  IntermediateFinalizedSplitResult<Self::ProcessBeforeSplitProduces, SplitterProducesForOtherCases, Self::Messages, Self::Errors> {
    let process_before_output = self
      .split_process_before
      .run_split_from_entry(entry_consumes, back_token)
      .await
      .into_process_err()?;
    match process_before_output {
      IntermediateFinalizedSplitOutcome::GoToCase {
        process_before_split_produced,
        splitter_produces_to_other_cases,
      } => {
        let produced = match splitter_produces_to_other_cases {
          Coproduct::Inl((_pd, params)) => Coproduct::Inl(params),
          Coproduct::Inr(inr_value) => Coproduct::Inr(inr_value),
        };
        self.continue_run(process_before_split_produced, produced, back_token).await
      }
      IntermediateFinalizedSplitOutcome::Yield(a, b, c, d) => Ok(IntermediateFinalizedSplitOutcome::Yield(a, b, c, d)),
      IntermediateFinalizedSplitOutcome::Finish(a) => Ok(IntermediateFinalizedSplitOutcome::Finish(a)),
      IntermediateFinalizedSplitOutcome::RetryUserInput(a, b) => Ok(IntermediateFinalizedSplitOutcome::RetryUserInput
        (a, b)),
      IntermediateFinalizedSplitOutcome::Back => Ok(IntermediateFinalizedSplitOutcome::Back),
    }
  }

  fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex {
    let used_index = self.split_process_before.enumerate_steps(last_used_index);
    self.case_index = used_index + 1;
    self.this_case.enumerate_steps(self.case_index)
  }
}

/// Implementation for the last branch in the [`crate::builder::SplitProcess`].
/// The only thing left from [`crate::step::SplitterOutput`] is [`CNil`].
impl<
  ThisTag: Send + Sync,
  SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  ProcessBefore: FinalizedSplitProcess<Coproduct<(ThisTag, SplitterProducesForThisCase), CNil>>,
  ThisCase: FinalizedProcess<
    EntryConsumes=<SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
    Messages=ProcessBefore::Messages,
  >,
  ErrorUnionIndices: Sync + Send,
> finalized_process::sealed::FinalizedProcess
for NextCaseOfFinalizedSplitProcess<ThisTag, SplitterProducesForThisCase, CNil, ProcessBefore, ThisCase, ErrorUnionIndices>
where
  ThisCase::EverProduced: Union<ProcessBefore::EverProduced>,
  <ThisCase::EverProduced as Union<ProcessBefore::EverProduced>>::Union: ParamList,
  ThisCase::Errors: CoprodUnion<ProcessBefore::Errors, ErrorUnionIndices>,
{
  type EntryConsumes = ProcessBefore::EntryConsumes;
  type ProcessBeforeProduces = <SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated;
  type Messages = ProcessBefore::Messages;
  type EverProduced = <ThisCase::EverProduced as Union<ProcessBefore::EverProduced>>::Union;
  type Errors = <ThisCase::Errors as CoprodUnion<ProcessBefore::Errors, ErrorUnionIndices>>::Union;

  async fn resume_run(
    &self,
    previous_run_produced: SessionContext,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
    form_context: MaybeFormContext,
    back_token: Option<BackToken>,
  ) -> RunResult<Self::Messages, Self::Errors> {
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
        } => match splitter_produces_to_other_cases {
          Coproduct::Inl((_pd, splitter_produces_for_this_case)) => {
            let this_case_consumes = splitter_produces_for_this_case.concat(process_before_split_produced);
            self
              .this_case
              .run_from_entry(this_case_consumes, back_token)
              .await
              .into_process_err()
          }
          Coproduct::Inr(c_nil) => match c_nil {},
        },
        IntermediateFinalizedSplitOutcome::Yield(a, b, c, d) => Ok(RunOutcome::Yield(a, b, c, d)),
        IntermediateFinalizedSplitOutcome::Finish(a) => Ok(RunOutcome::Finish(a)),
        IntermediateFinalizedSplitOutcome::RetryUserInput(a, b) => Ok(RunOutcome::RetryUserInput(a, b)),
        IntermediateFinalizedSplitOutcome::Back => Ok(RunOutcome::Back),
      }
    } else {
      match self.this_case.resume_run(
        previous_run_produced, previous_run_yielded_at, user_input, form_context, back_token,
      ).await.into_process_err()? {
        RunOutcome::Yield(a, b, c, d) => Ok(RunOutcome::Yield(a, b, c, d)),
        RunOutcome::Finish(a) => Ok(RunOutcome::Finish(a)),
        RunOutcome::RetryUserInput(a, b) => Ok(RunOutcome::RetryUserInput(a, b)),
        RunOutcome::Back => Ok(RunOutcome::Back),
      }
    }
  }

  async fn continue_run(&self, this_case_consumes: Self::ProcessBeforeProduces, back_token: Option<BackToken>) -> RunResult<Self::Messages, Self::Errors> {
    self
      .this_case
      .run_from_entry(this_case_consumes, back_token)
      .await
      .into_process_err()
  }

  async fn run_from_entry(&self, entry_consumes: Self::EntryConsumes, back_token: Option<BackToken>) -> RunResult<Self::Messages, Self::Errors> {
    let process_before_output = self.run_split_from_entry(entry_consumes, back_token).await?;
    match process_before_output {
      IntermediateFinalizedSplitOutcome::GoToCase {
        process_before_split_produced: _, splitter_produces_to_other_cases
      } => { match splitter_produces_to_other_cases {} }
      IntermediateFinalizedSplitOutcome::Yield(a, b, c, d) => Ok(RunOutcome::Yield(a, b, c, d)),
      IntermediateFinalizedSplitOutcome::Finish(a) => Ok(RunOutcome::Finish(a)),
      IntermediateFinalizedSplitOutcome::RetryUserInput(a, b) => Ok(RunOutcome::RetryUserInput(a, b)),
      IntermediateFinalizedSplitOutcome::Back => Ok(RunOutcome::Back),
    }
  }

  fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex {
    let used_index = self.split_process_before.enumerate_steps(last_used_index);
    self.case_index = used_index + 1;
    self.this_case.enumerate_steps(self.case_index)
  }
}
