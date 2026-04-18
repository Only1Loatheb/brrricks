use crate::builder::subprocess::{Subprocess, subprocess};
use crate::builder::{
  FinalizedProcess, FinalizedSplitProcess, FlowingCaseOfFinalizedSplitProcess, FlowingProcess,
  IntermediateFinalizedSplitOutcome, IntermediateFinalizedSplitResult, ParamList, ParamUID, PreviousRunYieldedAt,
  RunOutcome, RunResult, SessionContext, StepIndex, WILL_BE_RENUMBERED,
};
use crate::param_list::concat::Concat;
use crate::step::FailedInputValidationAttempts;
use frunk_core::coproduct::{CNil, Coproduct};
use std::marker::PhantomData;

pub struct NextCaseOfFinalizedSplitProcess<
  ThisTag: Send + Sync,
  SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  SplitterProducesForOtherCases: Send + Sync,
  ProcessBefore: FinalizedSplitProcess<Coproduct<(ThisTag, SplitterProducesForThisCase), SplitterProducesForOtherCases>>,
  ThisCase: FinalizedProcess<SubprocessConsumes=<SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated>,
> {
  pub split_process_before: ProcessBefore,
  pub case_index: StepIndex,
  pub this_case: ThisCase,
  pub phantom_data: PhantomData<(ThisTag, SplitterProducesForThisCase, SplitterProducesForOtherCases)>,
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
  SplitterProducesForNextCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  ThisCase: FinalizedProcess<
    SubprocessConsumes=<SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
    Messages=ProcessBefore::Messages,
  >,
>
NextCaseOfFinalizedSplitProcess<
  ThisTag,
  SplitterProducesForThisCase,
  Coproduct<(NextTag, SplitterProducesForNextCase), SplitterProducesForOtherCases>,
  ProcessBefore,
  ThisCase,
>
{
  pub fn case_end<NextCase: FinalizedProcess<
    SubprocessConsumes=<SplitterProducesForNextCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
  >>(
    self,
    _assumed_tag: NextTag,
    create_case: impl FnOnce(
      Subprocess<
        <SplitterProducesForNextCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
        ProcessBefore::Messages,
      >,
    ) -> NextCase,
  ) -> NextCaseOfFinalizedSplitProcess<NextTag, SplitterProducesForNextCase, SplitterProducesForOtherCases, Self, NextCase>
  {
    NextCaseOfFinalizedSplitProcess {
      split_process_before: self,
      case_index: WILL_BE_RENUMBERED,
      this_case: create_case(subprocess::<
        <SplitterProducesForNextCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
        ProcessBefore::Messages,
      >()),
      phantom_data: Default::default(),
    }
  }

  pub fn case_via<
    NextCase: FlowingProcess<
      SubprocessConsumes=<SplitterProducesForNextCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated
    >,
  >(
    self,
    _assumed_tag: NextTag,
    create_case: impl FnOnce(Subprocess<
      <SplitterProducesForNextCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
      ProcessBefore::Messages,
    >) -> NextCase,
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
      case_index: WILL_BE_RENUMBERED,
      this_case: create_case(subprocess::<
        <SplitterProducesForNextCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
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
    SubprocessConsumes=<SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
    Messages=ProcessBefore::Messages,
  >,
> FinalizedSplitProcess<SplitterProducesForOtherCases>
for NextCaseOfFinalizedSplitProcess<
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
  type SubprocessConsumes = ProcessBefore::SubprocessConsumes;
  type Messages = ProcessBefore::Messages;

  async fn resume_run(
    &self,
    previous_run_produced: SessionContext,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
    failed_input_validation_attempts: FailedInputValidationAttempts,
  ) -> IntermediateFinalizedSplitResult<Self::ProcessBeforeSplitProduces, SplitterProducesForOtherCases, Self::Messages> {
    if previous_run_yielded_at.0 < self.case_index {
      let process_before_output = self
        .split_process_before
        .resume_run(previous_run_produced, previous_run_yielded_at, user_input, failed_input_validation_attempts)
        .await?;
      match process_before_output {
        IntermediateFinalizedSplitOutcome::GoToCase {
          process_before_split_produced,
          splitter_produces_to_other_cases,
        } => {
          let produced = match splitter_produces_to_other_cases {
            Coproduct::Inl((_pd, params)) => Coproduct::Inl(params),
            Coproduct::Inr(inr_value) => Coproduct::Inr(inr_value),
          };
          self.continue_run(process_before_split_produced, produced).await
        }
        IntermediateFinalizedSplitOutcome::Yield(a, b, c) => Ok(IntermediateFinalizedSplitOutcome::Yield(a, b, c)),
        IntermediateFinalizedSplitOutcome::Finish(a) => Ok(IntermediateFinalizedSplitOutcome::Finish(a)),
        IntermediateFinalizedSplitOutcome::RetryUserInput(a) => Ok(IntermediateFinalizedSplitOutcome::RetryUserInput(a)),
      }
    } else {
      match self.this_case.resume_run(
        previous_run_produced, previous_run_yielded_at, user_input, failed_input_validation_attempts,
      ).await? {
        RunOutcome::Yield(a, b, c) => Ok(IntermediateFinalizedSplitOutcome::Yield(a, b, c)),
        RunOutcome::Finish(a) => Ok(IntermediateFinalizedSplitOutcome::Finish(a)),
        RunOutcome::RetryUserInput(a) => Ok(IntermediateFinalizedSplitOutcome::RetryUserInput(a)),
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
  ) -> IntermediateFinalizedSplitResult<Self::ProcessBeforeSplitProduces, SplitterProducesForOtherCases, Self::Messages> {
    match splitter_produces_for_this_case_or_other_cases_consumes {
      Coproduct::Inl(splitter_produces_for_this_case) => {
        let this_case_consumes = splitter_produces_for_this_case.concat(process_before_split_produced);
        match self.this_case.run_subprocess(this_case_consumes).await? {
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

  async fn run_split_subprocess(&self, subprocess_consumes: Self::SubprocessConsumes) ->
  IntermediateFinalizedSplitResult<Self::ProcessBeforeSplitProduces, SplitterProducesForOtherCases, Self::Messages> {
    let process_before_output = self.split_process_before.run_split_subprocess(subprocess_consumes).await?;
    match process_before_output {
      IntermediateFinalizedSplitOutcome::GoToCase {
        process_before_split_produced,
        splitter_produces_to_other_cases,
      } => {
        let produced = match splitter_produces_to_other_cases {
          Coproduct::Inl((_pd, params)) => Coproduct::Inl(params),
          Coproduct::Inr(inr_value) => Coproduct::Inr(inr_value),
        };
        self.continue_run(process_before_split_produced, produced).await
      }
      IntermediateFinalizedSplitOutcome::Yield(a, b, c) => Ok(IntermediateFinalizedSplitOutcome::Yield(a, b, c)),
      IntermediateFinalizedSplitOutcome::Finish(a) => Ok(IntermediateFinalizedSplitOutcome::Finish(a)),
      IntermediateFinalizedSplitOutcome::RetryUserInput(a) => Ok(IntermediateFinalizedSplitOutcome::RetryUserInput(a)),
    }
  }

  fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex {
    let used_index = self.split_process_before.enumerate_steps(last_used_index);
    self.case_index = used_index + 1;
    self.this_case.enumerate_steps(self.case_index)
  }

  fn all_param_uids(&self, acc: &mut Vec<ParamUID>) {
    self.split_process_before.all_param_uids(acc);
    SplitterProducesForThisCase::all_param_uids(acc);
    self.this_case.all_param_uids(acc);
  }
}

/// the last case
impl<
  ThisTag: Send + Sync,
  SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  ProcessBefore: FinalizedSplitProcess<Coproduct<(ThisTag, SplitterProducesForThisCase), CNil>>,
  ThisCase: FinalizedProcess<
    SubprocessConsumes=<SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
    Messages=ProcessBefore::Messages,
  >,
> FinalizedProcess
for NextCaseOfFinalizedSplitProcess<ThisTag, SplitterProducesForThisCase, CNil, ProcessBefore, ThisCase>
where
  ProcessBefore::SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
{
  type ProcessBeforeProduces = <SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated;
  type SubprocessConsumes = ProcessBefore::SubprocessConsumes;
  type Messages = ProcessBefore::Messages;

  async fn resume_run(
    &self,
    previous_run_produced: SessionContext,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
    failed_input_validation_attempts: FailedInputValidationAttempts,
  ) -> RunResult<Self::Messages> {
    if previous_run_yielded_at.0 < self.case_index {
      let process_before_output = self
        .split_process_before
        .resume_run(previous_run_produced, previous_run_yielded_at, user_input, failed_input_validation_attempts)
        .await?;
      match process_before_output {
        IntermediateFinalizedSplitOutcome::GoToCase {
          process_before_split_produced,
          splitter_produces_to_other_cases,
        } => match splitter_produces_to_other_cases {
          Coproduct::Inl((_pd, splitter_produces_for_this_case)) => {
            let this_case_consumes = splitter_produces_for_this_case.concat(process_before_split_produced);
            self.this_case.run_subprocess(this_case_consumes).await
          }
          Coproduct::Inr(c_nil) => match c_nil {},
        },
        IntermediateFinalizedSplitOutcome::Yield(a, b, c) => Ok(RunOutcome::Yield(a, b, c)),
        IntermediateFinalizedSplitOutcome::Finish(a) => Ok(RunOutcome::Finish(a)),
        IntermediateFinalizedSplitOutcome::RetryUserInput(a) => Ok(RunOutcome::RetryUserInput(a)),
      }
    } else {
      match self.this_case.resume_run(
        previous_run_produced, previous_run_yielded_at, user_input, failed_input_validation_attempts,
      ).await? {
        RunOutcome::Yield(a, b, c) => Ok(RunOutcome::Yield(a, b, c)),
        RunOutcome::Finish(a) => Ok(RunOutcome::Finish(a)),
        RunOutcome::RetryUserInput(a) => Ok(RunOutcome::RetryUserInput(a)),
      }
    }
  }

  async fn continue_run(&self, this_case_consumes: Self::ProcessBeforeProduces) -> RunResult<Self::Messages> {
    self.this_case.run_subprocess(this_case_consumes).await
  }

  async fn run_subprocess(&self, subprocess_consumes: Self::SubprocessConsumes) -> RunResult<Self::Messages> {
    let process_before_output = self.run_split_subprocess(subprocess_consumes).await?;
    match process_before_output {
      IntermediateFinalizedSplitOutcome::GoToCase {
        process_before_split_produced: _, splitter_produces_to_other_cases
      } => { match splitter_produces_to_other_cases {} }
      IntermediateFinalizedSplitOutcome::Yield(a, b, c) => Ok(RunOutcome::Yield(a, b, c)),
      IntermediateFinalizedSplitOutcome::Finish(a) => Ok(RunOutcome::Finish(a)),
      IntermediateFinalizedSplitOutcome::RetryUserInput(a) => Ok(RunOutcome::RetryUserInput(a)),
    }
  }

  fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex {
    let used_index = self.split_process_before.enumerate_steps(last_used_index);
    self.case_index = used_index + 1;
    self.this_case.enumerate_steps(self.case_index)
  }

  fn all_param_uids(&self, acc: &mut Vec<ParamUID>) {
    self.split_process_before.all_param_uids(acc);
    SplitterProducesForThisCase::all_param_uids(acc);
    self.this_case.all_param_uids(acc);
  }
}
