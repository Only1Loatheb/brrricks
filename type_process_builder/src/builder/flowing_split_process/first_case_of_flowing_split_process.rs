use crate::builder::{
  subprocess, FinalizedCaseOfFlowingSplitProcess, FinalizedProcess, FlowingCaseOfFlowingSplitProcess, FlowingProcess,
  FlowingSplitProcess, IntermediateFinalizedSplitOutcome, IntermediateFlowingSplitOutcome,
  IntermediateFlowingSplitResult, IntermediateRunOutcome, ParamList, PreviousRunYieldedAt, SplitProcess, Subprocess,
};
use crate::hlist_concat::Concat;
use frunk_core::coproduct::Coproduct;
use serde_value::Value;
use std::marker::PhantomData;

pub struct FirstCaseOfFlowingSplitProcess<
  ThisTag,
  SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  SplitterPassesToOtherCases,
  ProcessBefore: SplitProcess<SplitterPassesToOtherCases>,
  ThisCase: FlowingProcess<ProcessBeforeProduces=<SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated>,
> {
  pub split_process_before: ProcessBefore,
  pub this_case: ThisCase,
  pub phantom_data: PhantomData<(
    ThisTag,
    SplitterProducesForThisCase,
    SplitterPassesToOtherCases,
  )>,
}

impl<
  ThisTag,
  NextTag,
  SplitterPassesToOtherCases,
  ProcessBefore: SplitProcess<
    Coproduct<(NextTag, SplitterProducesForNextCase), SplitterPassesToOtherCases>,
    SplitterProducesForFirstCase=SplitterProducesForThisCase,
    SplitterTagForFirstCase=ThisTag,
  >,
  SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  SplitterProducesForNextCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  ThisCaseProduces: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  ThisCase: FlowingProcess<
    ProcessBeforeProduces=<SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
    Produces=ThisCaseProduces,
  >,
>
FirstCaseOfFlowingSplitProcess<
  ThisTag,
  SplitterProducesForThisCase,
  Coproduct<(NextTag, SplitterProducesForNextCase), SplitterPassesToOtherCases>,
  ProcessBefore,
  ThisCase,
>
{
  pub fn case_end<
    NextCase: FinalizedProcess<ProcessBeforeProduces=<SplitterProducesForNextCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated>,
  >(
    self,
    _assumed_tag: NextTag,
    create_case: impl FnOnce(Subprocess<<SplitterProducesForNextCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated>) -> NextCase,
  ) -> FinalizedCaseOfFlowingSplitProcess<
    NextTag,
    SplitterProducesForNextCase,
    SplitterPassesToOtherCases,
    Self,
    NextCase,
  >
  {
    FinalizedCaseOfFlowingSplitProcess {
      split_process_before: self,
      this_case: create_case(subprocess::<
        <SplitterProducesForNextCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
      >()),
      phantom_data: Default::default(),
    }
  }

  pub fn case_via<
    NextCase: FlowingProcess<ProcessBeforeProduces=<SplitterProducesForNextCase as
    Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated>,
    Indices,
  >(
    self,
    _assumed_tag: NextTag,
    create_case: impl FnOnce(Subprocess<<SplitterProducesForNextCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated>) -> NextCase,
  ) -> FlowingCaseOfFlowingSplitProcess<
    NextTag,
    SplitterProducesForNextCase,
    SplitterPassesToOtherCases,
    Self,
    NextCase,
    Indices,
  >
  {
    FlowingCaseOfFlowingSplitProcess {
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
  SplitterPassesToOtherCases,
  ProcessBefore: SplitProcess<SplitterPassesToOtherCases, SplitterProducesForFirstCase=SplitterProducesForThisCase>,
  ThisCaseProduces: ParamList,
  ThisCase: FlowingProcess<
    ProcessBeforeProduces=<SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
    Produces=ThisCaseProduces,
  >,
> FlowingSplitProcess<SplitterPassesToOtherCases>
for FirstCaseOfFlowingSplitProcess<
  ThisTag,
  SplitterProducesForThisCase,
  SplitterPassesToOtherCases,
  ProcessBefore,
  ThisCase,
>
{
  type EveryFlowingCaseProduces = ThisCase::Produces;
  type ProcessBeforeSplitProduces = ProcessBefore::ProcessBeforeSplitProduces;
  type SplitterProducesForThisCase = SplitterProducesForThisCase;

  async fn resume_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
  ) -> IntermediateFlowingSplitResult<
    ProcessBefore::ProcessBeforeSplitProduces,
    SplitterPassesToOtherCases,
    ThisCase::Produces,
  > {
    let process_before_output = self
      .split_process_before
      .resume_run(previous_run_produced, previous_run_yielded_at, user_input)
      .await?;
    match process_before_output {
      IntermediateFinalizedSplitOutcome::GoToCase {
        process_before_split_produced,
        splitter_passes_to_other_cases: this_case_produced,
      } => self.continue_run(process_before_split_produced, this_case_produced).await,
      IntermediateFinalizedSplitOutcome::Yield(a, b, c) => Ok(IntermediateFlowingSplitOutcome::Yield(a, b, c)),
      IntermediateFinalizedSplitOutcome::Finish(a) => Ok(IntermediateFlowingSplitOutcome::Finish(a)),
    }
  }

  async fn continue_run(
    &self,
    process_before_split_produced: Self::ProcessBeforeSplitProduces,
    splitter_produces_for_this_case_or_other_cases_consumes: Coproduct<
      Self::SplitterProducesForThisCase,
      SplitterPassesToOtherCases,
    >,
  ) -> IntermediateFlowingSplitResult<
    ProcessBefore::ProcessBeforeSplitProduces,
    SplitterPassesToOtherCases,
    ThisCase::Produces,
  > {
    match splitter_produces_for_this_case_or_other_cases_consumes {
      Coproduct::Inl(splitter_produces_for_this_case) => {
        let this_case_consumes = splitter_produces_for_this_case.concat(process_before_split_produced);
        match self.this_case.continue_run(this_case_consumes).await? {
          IntermediateRunOutcome::Continue(flowing_case_produced) => Ok(IntermediateFlowingSplitOutcome::Continue {
            flowing_case_produced,
          }),
          IntermediateRunOutcome::Yield(a, b, c) => Ok(IntermediateFlowingSplitOutcome::Yield(a, b, c)),
          IntermediateRunOutcome::Finish(a) => Ok(IntermediateFlowingSplitOutcome::Finish(a)),
        }
      }
      Coproduct::Inr(splitter_passes_to_other_cases) => Ok(IntermediateFlowingSplitOutcome::GoToCase {
        process_before_split_produced,
        splitter_passes_to_other_cases,
      }),
    }
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    self.split_process_before.enumerate_steps(last_used_index)
  }
}
