use crate::builder::subprocess::{Subprocess, subprocess};
use crate::builder::{
  FinalizedProcess, FinalizedSplitProcess, FlowingCaseOfFinalizedSplitProcess, FlowingProcess,
  IntermediateFinalizedSplitOutcome, IntermediateFinalizedSplitResult, NextCaseOfFinalizedSplitProcess, ParamList,
  PreviousRunYieldedAt, RunOutcome, SplitProcess,
};
use crate::param_list::concat::Concat;
use frunk_core::coproduct::Coproduct;
use serde_value::Value;
use std::marker::PhantomData;

pub struct FirstCaseOfFinalizedSplitProcess<
  ThisTag,
  SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  SplitterProducesForOtherCases,
  ProcessBefore: SplitProcess<SplitterProducesForOtherCases>,
  ThisCase: FinalizedProcess,
> {
  pub split_process_before: ProcessBefore,
  pub this_case: ThisCase,
  pub phantom_data: PhantomData<(ThisTag, SplitterProducesForThisCase, SplitterProducesForOtherCases)>,
}

impl<
  ThisTag,
  NextTag,
  SplitterProducesForOtherCases,
  ProcessBefore: SplitProcess<
    Coproduct<(NextTag, SplitterProducesForNextCase), SplitterProducesForOtherCases>,
    SplitterProducesForFirstCase=SplitterProducesForThisCase,
    SplitterTagForFirstCase=ThisTag,
  >,
  SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  SplitterProducesForNextCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  ThisCase: FinalizedProcess<ProcessBeforeProduces=<SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated>,
>
FirstCaseOfFinalizedSplitProcess<
  ThisTag,
  SplitterProducesForThisCase,
  Coproduct<(NextTag, SplitterProducesForNextCase), SplitterProducesForOtherCases>,
  ProcessBefore,
  ThisCase,
>
{
  pub fn case_end<
    NextCase: FinalizedProcess<
      ProcessBeforeProduces=<SplitterProducesForNextCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
    >,
  >(
    self,
    _assumed_tag: NextTag,
    create_case: impl FnOnce(
      Subprocess<<SplitterProducesForNextCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated>,
    ) -> NextCase,
  ) -> NextCaseOfFinalizedSplitProcess<NextTag, SplitterProducesForNextCase, SplitterProducesForOtherCases, Self, NextCase>
  {
    NextCaseOfFinalizedSplitProcess {
      split_process_before: self,
      this_case: create_case(subprocess::<
        <SplitterProducesForNextCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
      >()),
      phantom_data: Default::default(),
    }
  }

  pub fn case_via<
    NextCase: FlowingProcess<
      ProcessBeforeProduces=<SplitterProducesForNextCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated
    >,
  >(
    self,
    _assumed_tag: NextTag,
    create_case: impl FnOnce(Subprocess<<SplitterProducesForNextCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated>) -> NextCase,
  ) -> FlowingCaseOfFinalizedSplitProcess<
    NextTag,
    SplitterProducesForNextCase,
    SplitterProducesForOtherCases,
    Self,
    NextCase,
  >
  {
    FlowingCaseOfFinalizedSplitProcess {
      split_process_before: self,
      this_case: create_case(subprocess::<
        <SplitterProducesForNextCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
      >()),
      phantom_data: Default::default(),
    }
  }
}

impl<
  ThisTag,
  SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  SplitterProducesForOtherCases,
  ProcessBefore: SplitProcess<SplitterProducesForOtherCases, SplitterProducesForFirstCase=SplitterProducesForThisCase>,
  ThisCase: FinalizedProcess<ProcessBeforeProduces=<SplitterProducesForThisCase as Concat<<ProcessBefore>::ProcessBeforeSplitProduces>>::Concatenated>,
> FinalizedSplitProcess<SplitterProducesForOtherCases>
for FirstCaseOfFinalizedSplitProcess<
  ThisTag,
  SplitterProducesForThisCase,
  SplitterProducesForOtherCases,
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
  ) -> IntermediateFinalizedSplitResult<Self::ProcessBeforeSplitProduces, SplitterProducesForOtherCases> {
    let process_before_output = self
      .split_process_before
      .resume_run(previous_run_produced, previous_run_yielded_at, user_input)
      .await?;
    match process_before_output {
      IntermediateFinalizedSplitOutcome::GoToCase {
        process_before_split_produced,
        splitter_produces_to_other_cases,
      } => {
        self
          .continue_run(process_before_split_produced, splitter_produces_to_other_cases)
          .await
      }
      IntermediateFinalizedSplitOutcome::Yield(a, b, c) => Ok(IntermediateFinalizedSplitOutcome::Yield(a, b, c)),
      IntermediateFinalizedSplitOutcome::Finish(a) => Ok(IntermediateFinalizedSplitOutcome::Finish(a)),
      IntermediateFinalizedSplitOutcome::RetryUserInput(a) => Ok(IntermediateFinalizedSplitOutcome::RetryUserInput(a)),
    }
  }

  async fn continue_run(
    &self,
    process_before_split_produced: Self::ProcessBeforeSplitProduces,
    splitter_produces_for_this_case_or_other_cases_consumes: Coproduct<
      Self::SplitterProducesForThisCase,
      SplitterProducesForOtherCases,
    >,
  ) -> IntermediateFinalizedSplitResult<Self::ProcessBeforeSplitProduces, SplitterProducesForOtherCases> {
    match splitter_produces_for_this_case_or_other_cases_consumes {
      Coproduct::Inl(splitter_produces_for_this_case) => {
        let this_case_consumes = splitter_produces_for_this_case.concat(process_before_split_produced);
        match self.this_case.continue_run(this_case_consumes).await? {
          RunOutcome::Yield(a, b, c) => Ok(IntermediateFinalizedSplitOutcome::Yield(a, b, c)),
          RunOutcome::Finish(a) => Ok(IntermediateFinalizedSplitOutcome::Finish(a)),
          RunOutcome::RetryUserInput(a) => Ok(IntermediateFinalizedSplitOutcome::RetryUserInput(a)),
        }
      }
      Coproduct::Inr(splitter_produces_to_other_cases) => Ok(IntermediateFinalizedSplitOutcome::GoToCase {
        process_before_split_produced,
        splitter_produces_to_other_cases,
      }),
    }
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    self.split_process_before.enumerate_steps(last_used_index)
  }
}
