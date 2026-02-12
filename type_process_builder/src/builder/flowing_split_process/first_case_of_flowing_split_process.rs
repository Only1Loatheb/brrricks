use crate::builder::subprocess::{Subprocess, subprocess};
use crate::builder::{
  FinalizedCaseOfFlowingSplitProcess, FinalizedProcess, FlowingCaseOfFlowingSplitProcess, FlowingProcess,
  FlowingSplitProcess, IntermediateFinalizedSplitOutcome, IntermediateFlowingSplitOutcome,
  IntermediateFlowingSplitResult, IntermediateRunOutcome, ParamList, PreviousRunYieldedAt, SessionContext,
  SplitProcess, StepIndex, WILL_BE_RENUMBERED,
};
use crate::param_list::concat::Concat;
use crate::step::FailedInputValidationAttempts;
use frunk_core::coproduct::Coproduct;
use std::marker::PhantomData;

pub struct FirstCaseOfFlowingSplitProcess<
  ThisTag,
  SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  SplitterProducesForOtherCases,
  ProcessBefore: SplitProcess<SplitterProducesForOtherCases>,
  ThisCase: FlowingProcess<ProcessBeforeProduces=<SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated>,
> {
  pub split_process_before: ProcessBefore,
  pub this_case: ThisCase,
  pub idx: StepIndex,
  pub phantom_data: PhantomData<(
    ThisTag,
    SplitterProducesForThisCase,
    SplitterProducesForOtherCases,
  )>,
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
  ThisCaseProduces: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  ThisCase: FlowingProcess<
    ProcessBeforeProduces=<SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
    Produces=ThisCaseProduces,
  >,
>
FirstCaseOfFlowingSplitProcess<
  ThisTag,
  SplitterProducesForThisCase,
  Coproduct<(NextTag, SplitterProducesForNextCase), SplitterProducesForOtherCases>,
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
    SplitterProducesForOtherCases,
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
    SplitterProducesForOtherCases,
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
      idx: WILL_BE_RENUMBERED,
      phantom_data: Default::default(),
    }
  }
}

impl<
  ThisTag,
  SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  SplitterProducesForOtherCases,
  ProcessBefore: SplitProcess<SplitterProducesForOtherCases, SplitterProducesForFirstCase=SplitterProducesForThisCase>,
  ThisCaseProduces: ParamList,
  ThisCase: FlowingProcess<
    ProcessBeforeProduces=<SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
    Produces=ThisCaseProduces,
  >,
> FlowingSplitProcess<SplitterProducesForOtherCases>
for FirstCaseOfFlowingSplitProcess<
  ThisTag,
  SplitterProducesForThisCase,
  SplitterProducesForOtherCases,
  ProcessBefore,
  ThisCase,
>
{
  type ProcessBeforeSplitProduces = ProcessBefore::ProcessBeforeSplitProduces;
  type SplitterProducesForThisCase = SplitterProducesForThisCase;
  type EveryFlowingCaseProduces = ThisCase::Produces;

  async fn resume_run(
    &self,
    previous_run_produced: SessionContext,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
    failed_input_validation_attempts: FailedInputValidationAttempts,
  ) -> IntermediateFlowingSplitResult<
    ProcessBefore::ProcessBeforeSplitProduces,
    SplitterProducesForOtherCases,
    ThisCase::Produces,
  > {
    if previous_run_yielded_at.0 < self.idx {
      let process_before_output = self
        .split_process_before
        .resume_run(previous_run_produced, previous_run_yielded_at, user_input, failed_input_validation_attempts)
        .await?;
      match process_before_output {
        IntermediateFinalizedSplitOutcome::GoToCase {
          process_before_split_produced,
          splitter_produces_to_other_cases,
        } => self.continue_run(process_before_split_produced, splitter_produces_to_other_cases).await,
        IntermediateFinalizedSplitOutcome::Yield(a, b, c) => Ok(IntermediateFlowingSplitOutcome::Yield(a, b, c)),
        IntermediateFinalizedSplitOutcome::Finish(a) => Ok(IntermediateFlowingSplitOutcome::Finish(a)),
        IntermediateFinalizedSplitOutcome::RetryUserInput(a) => Ok(IntermediateFlowingSplitOutcome::RetryUserInput(a)),
      }
    } else {
      let a = self.this_case.resume_run(previous_run_produced, previous_run_yielded_at, user_input,
                                failed_input_validation_attempts).await?;
      match a {
        IntermediateRunOutcome::Continue(a) => Ok(IntermediateFlowingSplitOutcome::Continue(a)),
        IntermediateRunOutcome::Yield(a, b, c) => Ok(IntermediateFlowingSplitOutcome::Yield(a, b, c)),
        IntermediateRunOutcome::Finish(a) => Ok(IntermediateFlowingSplitOutcome::Finish(a)),
        IntermediateRunOutcome::RetryUserInput(a) => Ok(IntermediateFlowingSplitOutcome::RetryUserInput(a)),
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
  ) -> IntermediateFlowingSplitResult<
    ProcessBefore::ProcessBeforeSplitProduces,
    SplitterProducesForOtherCases,
    ThisCase::Produces,
  > {
    match splitter_produces_for_this_case_or_other_cases_consumes {
      Coproduct::Inl(splitter_produces_for_this_case) => {
        let this_case_consumes = splitter_produces_for_this_case.concat(process_before_split_produced);
        match self.this_case.continue_run(this_case_consumes).await? {
          IntermediateRunOutcome::Continue(a) => Ok(IntermediateFlowingSplitOutcome::Continue(a)),
          IntermediateRunOutcome::Yield(a, b, c) => Ok(IntermediateFlowingSplitOutcome::Yield(a, b, c)),
          IntermediateRunOutcome::Finish(a) => Ok(IntermediateFlowingSplitOutcome::Finish(a)),
          IntermediateRunOutcome::RetryUserInput(a) => Ok(IntermediateFlowingSplitOutcome::RetryUserInput(a)),
        }
      }
      Coproduct::Inr(splitter_produces_to_other_cases) => Ok(IntermediateFlowingSplitOutcome::GoToCase {
        process_before_split_produced,
        splitter_produces_to_other_cases,
      }),
    }
  }

  fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex {
    let used_index = self.split_process_before.enumerate_steps(last_used_index);
    self.idx = used_index + 1;
    self.this_case.enumerate_steps(used_index)
  }
}
