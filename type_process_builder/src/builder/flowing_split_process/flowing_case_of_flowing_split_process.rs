use crate::builder::subprocess::{Subprocess, subprocess};
use crate::builder::{
  FinalizedCaseOfFlowingSplitProcess, FinalizedProcess, FlowingProcess, FlowingSplitProcess,
  IntermediateFlowingSplitOutcome, IntermediateFlowingSplitResult, IntermediateRunOutcome, IntermediateRunResult,
  ParamList, PreviousRunYieldedAt, SessionContext, StepIndex,
};
use crate::param_list::concat::Concat;
use crate::param_list::intersect::Intersect;
use crate::param_list::transform::TransformTo;
use crate::step::FailedInputValidationAttempts;
use frunk_core::coproduct::{CNil, Coproduct};
use std::marker::PhantomData;

pub struct FlowingCaseOfFlowingSplitProcess<
  ThisTag,
  SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  SplitterProducesForOtherCases,
  ProcessBefore: FlowingSplitProcess<Coproduct<(ThisTag, SplitterProducesForThisCase), SplitterProducesForOtherCases>>,
  ThisCase: FlowingProcess<ProcessBeforeProduces=<SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated>,
  Indices,
>
{
  pub split_process_before: ProcessBefore,
  pub this_case: ThisCase,
  pub phantom_data: PhantomData<(
    ThisTag,
    SplitterProducesForThisCase,
    SplitterProducesForOtherCases,
    Indices,
  )>,
}

impl<
  ThisTag,
  NextTag,
  SplitterProducesForOtherCases,
  ProcessBefore: FlowingSplitProcess<Coproduct<(ThisTag, SplitterProducesForThisCase), Coproduct<(NextTag, SplitterProducesForNextCase), SplitterProducesForOtherCases>>>,
  SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  SplitterProducesForNextCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  ThisCase: FlowingProcess<ProcessBeforeProduces=<SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated>,
  ThisIndices,
>
FlowingCaseOfFlowingSplitProcess<
  ThisTag,
  SplitterProducesForThisCase,
  Coproduct<(NextTag, SplitterProducesForNextCase), SplitterProducesForOtherCases>,
  ProcessBefore,
  ThisCase,
  ThisIndices,
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
  > where
    ProcessBefore::EveryFlowingCaseProduces: Intersect<ThisCase::Produces>,
    <ProcessBefore::EveryFlowingCaseProduces as Intersect<ThisCase::Produces>>::Intersection: ParamList,
    ThisCase::Produces: TransformTo<<ProcessBefore::EveryFlowingCaseProduces as Intersect<ThisCase::Produces>>::Intersection, ThisIndices>,
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
    NextCase: FlowingProcess<ProcessBeforeProduces=<SplitterProducesForNextCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated>,
    NextIndices,
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
    NextIndices,
  > where
    ProcessBefore::EveryFlowingCaseProduces: Intersect<ThisCase::Produces>,
    <ProcessBefore::EveryFlowingCaseProduces as Intersect<ThisCase::Produces>>::Intersection: ParamList,
    ThisCase::Produces: TransformTo<<ProcessBefore ::EveryFlowingCaseProduces as Intersect<ThisCase::Produces>>::Intersection, ThisIndices>
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
  SplitterProducesForOtherCases,
  ProcessBefore: FlowingSplitProcess<Coproduct<(ThisTag, SplitterProducesForThisCase), SplitterProducesForOtherCases>>,
  ThisCase: FlowingProcess<ProcessBeforeProduces=<SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated>,
  Indices,
> FlowingSplitProcess<SplitterProducesForOtherCases>
for FlowingCaseOfFlowingSplitProcess<
  ThisTag,
  SplitterProducesForThisCase,
  SplitterProducesForOtherCases,
  ProcessBefore,
  ThisCase,
  Indices,
>
where
  ProcessBefore::EveryFlowingCaseProduces: Intersect<ThisCase::Produces>,
  <ProcessBefore::EveryFlowingCaseProduces as Intersect<ThisCase::Produces>>::Intersection: ParamList,
  ThisCase::Produces: TransformTo<<ProcessBefore::EveryFlowingCaseProduces as Intersect<ThisCase::Produces>>::Intersection, Indices>,
{
  type ProcessBeforeSplitProduces = ProcessBefore::ProcessBeforeSplitProduces;
  type SplitterProducesForThisCase = SplitterProducesForThisCase;
  type EveryFlowingCaseProduces = <ProcessBefore::EveryFlowingCaseProduces as Intersect<ThisCase::Produces>>::Intersection;

  async fn resume_run(
    &self,
    previous_run_produced: SessionContext,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
    failed_input_validation_attempts: FailedInputValidationAttempts,
  ) -> IntermediateFlowingSplitResult<Self::ProcessBeforeSplitProduces, SplitterProducesForOtherCases, Self::EveryFlowingCaseProduces> {
    let process_before_output = self
      .split_process_before
      .resume_run(previous_run_produced, previous_run_yielded_at, user_input, failed_input_validation_attempts)
      .await?;
    match process_before_output {
      IntermediateFlowingSplitOutcome::Continue(a) => Ok(IntermediateFlowingSplitOutcome::Continue(a.intersect())),
      IntermediateFlowingSplitOutcome::GoToCase {
        process_before_split_produced,
        splitter_produces_to_other_cases,
      } => {
        let produced = match splitter_produces_to_other_cases {
          Coproduct::Inl((_pd, params)) => Coproduct::Inl(params),
          Coproduct::Inr(inr_value) => Coproduct::Inr(inr_value),
        };
        self.continue_run(process_before_split_produced, produced).await
      }
      IntermediateFlowingSplitOutcome::Yield(a, b, c) => Ok(IntermediateFlowingSplitOutcome::Yield(a, b, c)),
      IntermediateFlowingSplitOutcome::Finish(a) => Ok(IntermediateFlowingSplitOutcome::Finish(a)),
      IntermediateFlowingSplitOutcome::RetryUserInput(a) => Ok(IntermediateFlowingSplitOutcome::RetryUserInput(a)),
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
        match self.this_case.continue_run(this_case_consumes).await? {
          IntermediateRunOutcome::Continue(a) => Ok(IntermediateFlowingSplitOutcome::Continue(a.transform())),
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
    self.this_case.enumerate_steps(used_index)
  }
}

/// last case
impl<
  ThisTag,
  SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  ProcessBefore: FlowingSplitProcess<Coproduct<(ThisTag, SplitterProducesForThisCase), CNil>>,
  ThisCase: FlowingProcess<ProcessBeforeProduces=
  <SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
  >,
  Indices,
> FlowingProcess
for FlowingCaseOfFlowingSplitProcess<
  ThisTag,
  SplitterProducesForThisCase,
  CNil,
  ProcessBefore,
  ThisCase,
  Indices,
>
where
  ProcessBefore::EveryFlowingCaseProduces: Intersect<ThisCase::Produces>,
  <ProcessBefore::EveryFlowingCaseProduces as Intersect<ThisCase::Produces>>::Intersection: ParamList,
  ThisCase::Produces: TransformTo<<ProcessBefore::EveryFlowingCaseProduces as Intersect<ThisCase::Produces>>::Intersection, Indices>,
{
  type ProcessBeforeProduces = ProcessBefore::ProcessBeforeSplitProduces;
  type Produces = <ProcessBefore::EveryFlowingCaseProduces as Intersect<ThisCase::Produces>>::Intersection;

  async fn resume_run(
    &self,
    previous_run_produced: SessionContext,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
    failed_input_validation_attempts: FailedInputValidationAttempts,
  ) -> IntermediateRunResult<Self::Produces> {
    let process_before_output = self
      .split_process_before
      .resume_run(previous_run_produced, previous_run_yielded_at, user_input, failed_input_validation_attempts)
      .await?;
    match process_before_output {
      IntermediateFlowingSplitOutcome::Continue(a) => Ok(IntermediateRunOutcome::Continue(a.intersect())),
      IntermediateFlowingSplitOutcome::GoToCase {
        process_before_split_produced,
        splitter_produces_to_other_cases,
      } => match splitter_produces_to_other_cases {
        Coproduct::Inl((_pd, produces_to_this_case)) => {
          let this_case_consumes = produces_to_this_case.concat(process_before_split_produced);
          match self.this_case.continue_run(this_case_consumes).await? {
            IntermediateRunOutcome::Continue(this_case_produced) =>
              Ok(IntermediateRunOutcome::Continue(this_case_produced.transform())),
            IntermediateRunOutcome::Yield(a, b, c) => Ok(IntermediateRunOutcome::Yield(a, b, c)),
            IntermediateRunOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
            IntermediateRunOutcome::RetryUserInput(a) => Ok(IntermediateRunOutcome::RetryUserInput(a)),
          }
        }
        Coproduct::Inr(c_nil) => match c_nil {},
      },
      IntermediateFlowingSplitOutcome::Yield(a, b, c) => Ok(IntermediateRunOutcome::Yield(a, b, c)),
      IntermediateFlowingSplitOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
      IntermediateFlowingSplitOutcome::RetryUserInput(a) => Ok(IntermediateRunOutcome::RetryUserInput(a)),
    }
  }

  async fn continue_run(&self, _process_before_produces: Self::ProcessBeforeProduces) -> IntermediateRunResult<Self::Produces> {
    todo!()
  }

  fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex {
    let used_index = self.split_process_before.enumerate_steps(last_used_index);
    self.this_case.enumerate_steps(used_index)
  }
}
