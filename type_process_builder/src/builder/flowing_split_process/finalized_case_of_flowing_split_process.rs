use crate::builder::{
  subprocess, FinalizedProcess, FlowingCaseOfFlowingSplitProcess, FlowingProcess, FlowingSplitProcess,
  IntermediateFlowingSplitOutcome, IntermediateFlowingSplitResult, IntermediateRunOutcome, IntermediateRunResult,
  ParamList, PreviousRunYieldedAt, RunOutcome, Subprocess,
};
use crate::hlist_concat::Concat;
use frunk_core::coproduct::{CNil, Coproduct};
use serde_value::Value;
use std::marker::PhantomData;

pub struct FinalizedCaseOfFlowingSplitProcess<
  ThisTag,
  SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  SplitterProducesForOtherCases,
  ProcessBefore: FlowingSplitProcess<Coproduct<(ThisTag, SplitterProducesForThisCase), SplitterProducesForOtherCases>>,
  ThisCase: FinalizedProcess<ProcessBeforeProduces=<SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated>,
> {
  pub split_process_before: ProcessBefore,
  pub this_case: ThisCase,
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
  ProcessBefore: FlowingSplitProcess<
    Coproduct<(ThisTag, SplitterProducesForThisCase), Coproduct<(NextTag, SplitterProducesForNextCase), SplitterProducesForOtherCases>>,
  >,
  SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  SplitterProducesForNextCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  ThisCase: FinalizedProcess<
    ProcessBeforeProduces=<SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
  >,
>
FinalizedCaseOfFlowingSplitProcess<
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
      phantom_data: Default::default(),
    }
  }
}

impl<
  ThisTag,
  SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  SplitterProducesForOtherCases,
  ProcessBefore: FlowingSplitProcess<Coproduct<(ThisTag, SplitterProducesForThisCase), SplitterProducesForOtherCases>>,
  ThisCase: FinalizedProcess<ProcessBeforeProduces=<SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated>,
  > FlowingSplitProcess<SplitterProducesForOtherCases>
  for FinalizedCaseOfFlowingSplitProcess<
ThisTag,
SplitterProducesForThisCase,
SplitterProducesForOtherCases,
ProcessBefore,
ThisCase,
  >
{
  type ProcessBeforeSplitProduces = ProcessBefore::ProcessBeforeSplitProduces;
  type SplitterProducesForThisCase = SplitterProducesForThisCase;
  type EveryFlowingCaseProduces = ProcessBefore::EveryFlowingCaseProduces;

  async fn resume_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
  ) -> IntermediateFlowingSplitResult<Self::ProcessBeforeSplitProduces, SplitterProducesForOtherCases, Self::EveryFlowingCaseProduces> {
    todo!()
    // let process_before_output = self
    //   .split_process_before
    //   .resume_run(previous_run_produced, previous_run_yielded_at, user_input)
    //   .await?;
    // match process_before_output {
    //   IntermediateFlowingSplitOutcome::Continue {
    //     process_before_split_produced,
    //     splitter_produces_to_other_cases: this_case_produced,
    //   } => {
    //     let produced = match this_case_produced {
    //       Coproduct::Inl((_pd, params)) => Coproduct::Inl(params),
    //       Coproduct::Inr(inr_value) => Coproduct::Inr(inr_value),
    //     };
    //     self.continue_run(process_before_split_produced, produced).await
    //   }
    //   IntermediateFlowingSplitOutcome::Yield(a, b, c) => Ok(IntermediateFlowingSplitOutcome::Yield(a, b, c)),
    //   IntermediateFlowingSplitOutcome::Finish(a) => Ok(IntermediateFlowingSplitOutcome::Finish(a)),
    // }
  }

  async fn continue_run(
    &self,
    process_before_split_produced: Self::ProcessBeforeSplitProduces,
    splitter_produces_for_this_case_or_other_cases_consumes: Coproduct<
      Self::SplitterProducesForThisCase,
      SplitterProducesForOtherCases,
    >,
  ) -> IntermediateFlowingSplitResult<Self::ProcessBeforeSplitProduces, SplitterProducesForOtherCases, Self::EveryFlowingCaseProduces> {
    todo!()
    // match splitter_produces_for_this_case_or_other_cases_consumes {
    //   Coproduct::Inl(this_case_consumes) => {
    //     let next_case_consumes: ThisCase::ProcessBeforeProduces =
    //       this_case_consumes.concat(process_before_split_produced).transform();
    //     match self.this_case.continue_run(next_case_consumes).await? {
    //       RunOutcome::Yield(a, b, c) => Ok(IntermediateFlowingSplitOutcome::Yield(a, b, c)),
    //       RunOutcome::Finish(a) => Ok(IntermediateFlowingSplitOutcome::Finish(a)),
    //     }
    //   }
    //   Coproduct::Inr(other_cases_consumes) => Ok(IntermediateFlowingSplitOutcome::Continue {
    //     process_before_split_produced: process_before_split_produced,
    //     splitter_produces_to_other_cases: other_cases_consumes,
    //   }),
    // }
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    self.split_process_before.enumerate_steps(last_used_index)
  }
}

/// the last case
impl<
  ThisTag,
  SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  ProcessBefore: FlowingSplitProcess<Coproduct<(ThisTag, SplitterProducesForThisCase), CNil>>,
  ThisCase: FinalizedProcess<ProcessBeforeProduces=<SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated>,
> FlowingProcess
for FinalizedCaseOfFlowingSplitProcess<ThisTag, SplitterProducesForThisCase, CNil, ProcessBefore, ThisCase>
where
// ProcessBefore::SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
// <SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
{
  type ProcessBeforeProduces = <SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated;
  type Produces = ProcessBefore::EveryFlowingCaseProduces;

  async fn resume_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
  ) -> IntermediateRunResult<Self::Produces> {
    let process_before_output = self
      .split_process_before
      .resume_run(previous_run_produced, previous_run_yielded_at, user_input)
      .await?;
    match process_before_output {
      IntermediateFlowingSplitOutcome::GoToCase {
        process_before_split_produced,
        splitter_produces_to_other_cases,
      } => match splitter_produces_to_other_cases {
        Coproduct::Inl((_pd, produces_to_this_case)) => {
          let this_case_consumes = produces_to_this_case.concat(process_before_split_produced);
          match self.this_case.continue_run(this_case_consumes).await? {
            RunOutcome::Yield(a, b, c) => Ok(IntermediateRunOutcome::Yield(a, b, c)),
            RunOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
          }
        }
        Coproduct::Inr(c_nil) => match c_nil {},
      },
      IntermediateFlowingSplitOutcome::Yield(a, b, c) => Ok(IntermediateRunOutcome::Yield(a, b, c)),
      IntermediateFlowingSplitOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
      IntermediateFlowingSplitOutcome::Continue { flowing_case_produced } =>
        Ok(IntermediateRunOutcome::Continue(flowing_case_produced)),
    }
  }

  async fn continue_run(&self, this_case_consumes: Self::ProcessBeforeProduces) -> IntermediateRunResult<Self::Produces> {
    match self.this_case.continue_run(this_case_consumes).await? {
      RunOutcome::Yield(a, b, c) => Ok(IntermediateRunOutcome::Yield(a, b, c)),
      RunOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
    }
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    self.split_process_before.enumerate_steps(last_used_index)
  }
}
