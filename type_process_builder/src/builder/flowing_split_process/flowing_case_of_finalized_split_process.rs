use crate::builder::subprocess::{Subprocess, subprocess};
use crate::builder::{
  FinalizedCaseOfFlowingSplitProcess, FinalizedProcess, FinalizedSplitProcess, FlowingCaseOfFlowingSplitProcess,
  FlowingProcess, FlowingSplitProcess, IntermediateFinalizedSplitOutcome, IntermediateFlowingSplitOutcome,
  IntermediateFlowingSplitResult, IntermediateRunOutcome, IntermediateRunResult, ParamList, ParamUID,
  PreviousRunYieldedAt, SessionContext, StepIndex, WILL_BE_RENUMBERED,
};
use crate::param_list::concat::Concat;
use crate::step::FailedInputValidationAttempts;
use frunk_core::coproduct::{CNil, Coproduct};
use std::marker::PhantomData;

pub struct FlowingCaseOfFinalizedSplitProcess<
  ThisTag: Send + Sync,
  SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  SplitterProducesForOtherCases: Send + Sync,
  ProcessBefore: FinalizedSplitProcess<Coproduct<(ThisTag, SplitterProducesForThisCase), SplitterProducesForOtherCases>>,
  ThisCase: FlowingProcess<ProcessBeforeProduces=
  <SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated
  >,
>
{
  pub split_process_before: ProcessBefore,
  pub case_index: StepIndex,
  pub this_case: ThisCase,
  pub phantom_data: PhantomData<(
    ThisTag,
    SplitterProducesForThisCase,
    SplitterProducesForOtherCases,
  )>,
}

impl<
  ThisTag: Send + Sync,
  NextTag: Send + Sync,
  SplitterProducesForOtherCases: Send + Sync,
  ProcessBefore: FinalizedSplitProcess<
    Coproduct<(ThisTag, SplitterProducesForThisCase), Coproduct<(NextTag, SplitterProducesForNextCase), SplitterProducesForOtherCases>>,
  >,
  SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  SplitterProducesForNextCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  ThisCase: FlowingProcess<
    ProcessBeforeProduces=<SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
  >,
>
FlowingCaseOfFinalizedSplitProcess<
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
      case_index: WILL_BE_RENUMBERED,
      this_case: create_case(subprocess::<
        <SplitterProducesForNextCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
      >()),
      phantom_data: Default::default(),
    }
  }

  pub fn case_via<
    NextCase: FlowingProcess<ProcessBeforeProduces=<SplitterProducesForNextCase as
    Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated>,
    Indices: Sync,
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
      case_index: WILL_BE_RENUMBERED,
      this_case: create_case(subprocess::<
        <SplitterProducesForNextCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
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
  ThisCase: FlowingProcess<ProcessBeforeProduces=<SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated>,
> FlowingSplitProcess<SplitterProducesForOtherCases>
for FlowingCaseOfFinalizedSplitProcess<
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
  type SubprocessConsumes = ProcessBefore::SubprocessConsumes;

  async fn resume_run(
    &self,
    previous_run_produced: SessionContext,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
    failed_input_validation_attempts: FailedInputValidationAttempts,
  ) -> IntermediateFlowingSplitResult<Self::ProcessBeforeSplitProduces, SplitterProducesForOtherCases, Self::EveryFlowingCaseProduces> {
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
        IntermediateFinalizedSplitOutcome::Yield(a, b, c) => Ok(IntermediateFlowingSplitOutcome::Yield(a, b, c)),
        IntermediateFinalizedSplitOutcome::Finish(a) => Ok(IntermediateFlowingSplitOutcome::Finish(a)),
        IntermediateFinalizedSplitOutcome::RetryUserInput(a) => Ok(IntermediateFlowingSplitOutcome::RetryUserInput(a)),
      }
    } else {
      match self.this_case.resume_run(
        previous_run_produced,
        previous_run_yielded_at,
        user_input,
        failed_input_validation_attempts,
      ).await? {
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
  ) -> IntermediateFlowingSplitResult<Self::ProcessBeforeSplitProduces, SplitterProducesForOtherCases, Self::EveryFlowingCaseProduces> {
    match splitter_produces_for_this_case_or_other_cases_consumes {
      Coproduct::Inl(splitter_produces_for_this_case) => {
        let this_case_consumes = splitter_produces_for_this_case.concat(process_before_split_produced);
        match self.this_case.run_subprocess(this_case_consumes).await? {
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

  async fn run_split_subprocess(&self, subprocess_consumes: Self::SubprocessConsumes,) -> IntermediateFlowingSplitResult<Self::ProcessBeforeSplitProduces, SplitterProducesForOtherCases, Self::EveryFlowingCaseProduces> {
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
      IntermediateFinalizedSplitOutcome::Yield(a, b, c) => Ok(IntermediateFlowingSplitOutcome::Yield(a, b, c)),
      IntermediateFinalizedSplitOutcome::Finish(a) => Ok(IntermediateFlowingSplitOutcome::Finish(a)),
      IntermediateFinalizedSplitOutcome::RetryUserInput(a) => Ok(IntermediateFlowingSplitOutcome::RetryUserInput(a)),
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

/// last case
impl<
  ThisTag: Send + Sync,
  SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  ProcessBefore: FinalizedSplitProcess<Coproduct<(ThisTag, SplitterProducesForThisCase), CNil>>,
  ThisCase: FlowingProcess<ProcessBeforeProduces=
  <SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
  >,
> FlowingProcess
for FlowingCaseOfFinalizedSplitProcess<
  ThisTag,
  SplitterProducesForThisCase,
  CNil,
  ProcessBefore,
  ThisCase,
>
{
  type ProcessBeforeProduces = ProcessBefore::ProcessBeforeSplitProduces;
  type Produces = ThisCase::Produces;
  type SubprocessConsumes = ProcessBefore::SubprocessConsumes;

  async fn resume_run(
    &self,
    previous_run_produced: SessionContext,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
    failed_input_validation_attempts: FailedInputValidationAttempts,
  ) -> IntermediateRunResult<Self::Produces> {
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
          Coproduct::Inl((_pd, produces_to_this_case)) => {
            let this_case_consumes = produces_to_this_case.concat(process_before_split_produced);
            match self.this_case.run_subprocess(this_case_consumes).await? {
              IntermediateRunOutcome::Continue(this_case_produced) => Ok(IntermediateRunOutcome::Continue(this_case_produced)),
              IntermediateRunOutcome::Yield(a, b, c) => Ok(IntermediateRunOutcome::Yield(a, b, c)),
              IntermediateRunOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
              IntermediateRunOutcome::RetryUserInput(a) => Ok(IntermediateRunOutcome::RetryUserInput(a)),
            }
          }
          Coproduct::Inr(c_nil) => match c_nil {},
        },
        IntermediateFinalizedSplitOutcome::Yield(a, b, c) => Ok(IntermediateRunOutcome::Yield(a, b, c)),
        IntermediateFinalizedSplitOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
        IntermediateFinalizedSplitOutcome::RetryUserInput(a) => Ok(IntermediateRunOutcome::RetryUserInput(a)),
      }
    } else {
      match self.this_case.resume_run(
        previous_run_produced,
        previous_run_yielded_at,
        user_input,
        failed_input_validation_attempts,
      ).await? {
        IntermediateRunOutcome::Continue(a) => Ok(IntermediateRunOutcome::Continue(a)),
        IntermediateRunOutcome::Yield(a, b, c) => Ok(IntermediateRunOutcome::Yield(a, b, c)),
        IntermediateRunOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
        IntermediateRunOutcome::RetryUserInput(a) => Ok(IntermediateRunOutcome::RetryUserInput(a)),
      }
    }
  }

  /// The process execution will call [crate::builder::SplitProcess::continue_run] instead of this one.
  /// I implemented it like this to avoid:
  /// ```ignore
  /// let _ = EntryA
  ///   .show_split(SplitA, |subprocess|
  ///     subprocess
  ///       .case_via(Case1, |x| x)
  ///       .case_via(Case2, |x| x.show(FormA))
  ///   )
  ///   .end(FinalA);
  /// ```
  /// and use the builder like this instead:
  /// ```ignore
  /// let _ = EntryA
  ///   .show_split(SplitA)
  ///   .case_via(Case1, |x| x)
  ///   .case_via(Case2, |x| x.show(FormA))
  ///   .end(FinalA);
  /// ```
  async fn continue_run(&self, _process_before_produces: Self::ProcessBeforeProduces) -> IntermediateRunResult<Self::Produces> {
    unreachable!("continue_run from last case is unreachable. The process is always continued from SplitProcess")
  }

  async fn run_subprocess(&self, subprocess_consumes: Self::SubprocessConsumes,) -> IntermediateRunResult<Self::Produces> {
        let process_before_output = self.split_process_before.run_split_subprocess(subprocess_consumes).await?;
    match process_before_output {
      IntermediateFinalizedSplitOutcome::GoToCase {
        process_before_split_produced,
        splitter_produces_to_other_cases,
      } => match splitter_produces_to_other_cases {
        Coproduct::Inl((_pd, produces_to_this_case)) => {
          let this_case_consumes = produces_to_this_case.concat(process_before_split_produced);
          match self.this_case.run_subprocess(this_case_consumes).await? {
            IntermediateRunOutcome::Continue(this_case_produced) => Ok(IntermediateRunOutcome::Continue(this_case_produced)),
            IntermediateRunOutcome::Yield(a, b, c) => Ok(IntermediateRunOutcome::Yield(a, b, c)),
            IntermediateRunOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
            IntermediateRunOutcome::RetryUserInput(a) => Ok(IntermediateRunOutcome::RetryUserInput(a)),
          }
        }
        Coproduct::Inr(c_nil) => match c_nil {},
      },
      IntermediateFinalizedSplitOutcome::Yield(a, b, c) => Ok(IntermediateRunOutcome::Yield(a, b, c)),
      IntermediateFinalizedSplitOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
      IntermediateFinalizedSplitOutcome::RetryUserInput(a) => Ok(IntermediateRunOutcome::RetryUserInput(a)),
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
