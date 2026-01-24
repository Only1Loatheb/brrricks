use crate::builder::{
  subprocess, FinalizedProcess, FinalizedSplitProcess, IntermediateFinalizedSplitOutcome,
  IntermediateFinalizedSplitResult, NextCaseOfFinalizedSplitProcess, ParamList, PreviousRunYieldedAt, RunOutcome,
  SplitProcess, Subprocess,
};
use crate::hlist_concat::Concat;
use crate::type_eq::TypeEq;
use frunk_core::coproduct::Coproduct;
use serde_value::Value;
use std::marker::PhantomData;

pub struct FirstCaseOfFinalizedSplitProcess<
  ThisTag,
  SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  SplitterPassesToOtherCases,
  ProcessBefore: SplitProcess<SplitterPassesToOtherCases>,
  ThisCase: FinalizedProcess,
> {
  pub split_process_before: ProcessBefore,
  pub this_case: ThisCase,
  pub phantom_data: PhantomData<(ThisTag, SplitterProducesForThisCase, SplitterPassesToOtherCases)>,
}

impl<
    ThisTag,
    NextTag,
    SplitterPassesToOtherCases,
    ProcessBefore: SplitProcess<
      Coproduct<(NextTag, PassesToNextCase), SplitterPassesToOtherCases>,
      SplitterProducesForFirstCase = SplitterProducesForThisCase,
      SplitterTagForFirstCase = ThisTag,
    >,
    SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
    PassesToNextCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
    ThisCase: FinalizedProcess<ProcessBeforeProduces = <SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated>,
  >
  FirstCaseOfFinalizedSplitProcess<
    ThisTag,
    SplitterProducesForThisCase,
    Coproduct<(NextTag, PassesToNextCase), SplitterPassesToOtherCases>,
    ProcessBefore,
    ThisCase,
  >
{
  pub fn case<
    AssumedTag,
    NextCase: FinalizedProcess<
      ProcessBeforeProduces = <PassesToNextCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
    >,
  >(
    self,
    create_case: impl FnOnce(
      Subprocess<<PassesToNextCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated>,
    ) -> NextCase,
  ) -> NextCaseOfFinalizedSplitProcess<NextTag, PassesToNextCase, SplitterPassesToOtherCases, Self, NextCase>
  where
    (AssumedTag, NextTag): TypeEq,
  {
    NextCaseOfFinalizedSplitProcess {
      split_process_before: self,
      this_case: create_case(subprocess::<
        <PassesToNextCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
      >()),
      phantom_data: Default::default(),
    }
  }
}

impl<
    ThisTag,
    SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
    SplitterPassesToOtherCases,
    ProcessBefore: SplitProcess<SplitterPassesToOtherCases, SplitterProducesForFirstCase = SplitterProducesForThisCase>,
    ThisCase: FinalizedProcess<ProcessBeforeProduces = <SplitterProducesForThisCase as Concat<<ProcessBefore>::ProcessBeforeSplitProduces>>::Concatenated>,
  > FinalizedSplitProcess<SplitterPassesToOtherCases>
  for FirstCaseOfFinalizedSplitProcess<
    ThisTag,
    SplitterProducesForThisCase,
    SplitterPassesToOtherCases,
    ProcessBefore,
    ThisCase,
  >
{
  type ProcessBeforeSplitProduces = ProcessBefore::ProcessBeforeSplitProduces;
  type SplitterProducesForThisCase = SplitterProducesForThisCase;
  type SplitterTagForThisCase = ThisTag;

  async fn resume_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
  ) -> IntermediateFinalizedSplitResult<Self::ProcessBeforeSplitProduces, SplitterPassesToOtherCases> {
    let process_before_output = self
      .split_process_before
      .resume_run(previous_run_produced, previous_run_yielded_at, user_input)
      .await?;
    match process_before_output {
      IntermediateFinalizedSplitOutcome::GoToCase {
        process_before_split_produced,
        splitter_passes_to_other_cases,
      } => {
        self
          .continue_run(process_before_split_produced, splitter_passes_to_other_cases)
          .await
      }
      IntermediateFinalizedSplitOutcome::Yield(a, b, c) => Ok(IntermediateFinalizedSplitOutcome::Yield(a, b, c)),
      IntermediateFinalizedSplitOutcome::Finish(a) => Ok(IntermediateFinalizedSplitOutcome::Finish(a)),
    }
  }

  async fn continue_run(
    &self,
    process_before_split_produced: Self::ProcessBeforeSplitProduces,
    splitter_produces_for_this_case_or_other_cases_consumes: Coproduct<
      Self::SplitterProducesForThisCase,
      SplitterPassesToOtherCases,
    >,
  ) -> IntermediateFinalizedSplitResult<Self::ProcessBeforeSplitProduces, SplitterPassesToOtherCases> {
    match splitter_produces_for_this_case_or_other_cases_consumes {
      Coproduct::Inl(splitter_produces_for_first_case) => {
        let this_case_consumes = splitter_produces_for_first_case.concat(process_before_split_produced);
        match self.this_case.continue_run(this_case_consumes).await? {
          RunOutcome::Yield(a, b, c) => Ok(IntermediateFinalizedSplitOutcome::Yield(a, b, c)),
          RunOutcome::Finish(a) => Ok(IntermediateFinalizedSplitOutcome::Finish(a)),
        }
      }
      Coproduct::Inr(splitter_passes_to_other_cases) => Ok(IntermediateFinalizedSplitOutcome::GoToCase {
        process_before_split_produced,
        splitter_passes_to_other_cases,
      }),
    }
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    self.split_process_before.enumerate_steps(last_used_index)
  }
}
