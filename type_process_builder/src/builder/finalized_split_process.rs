use crate::builder::finalized_process::FinalizedProcess;
use crate::builder::split_process::SplitProcess;
use crate::builder::*;
use crate::hlist_concat::Concat;
use crate::hlist_transform_to::TransformTo;
use crate::param_list::ParamList;
use frunk_core::coproduct::Coproduct;
use serde_value::Value;
use std::future::Future;
use std::marker::PhantomData;

pub trait FinalizedSplitProcessCase: Sized {
  type ProcessBeforeSplitProduces: ParamList;
  type SplitterProducesForThisCase: ParamList + Concat<Self::ProcessBeforeSplitProduces>;
  type SplitterProducesForOtherCases: SplitterOutput;

  fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
  ) -> impl Future<Output = IntermediateSplitResult<Self::ProcessBeforeSplitProduces, Self::SplitterProducesForOtherCases>>;

  fn run(
    &self,
    process_before_split_produces: Self::ProcessBeforeSplitProduces,
    this_case_or_other_cases_input: Coproduct<Self::SplitterProducesForThisCase, Self::SplitterProducesForOtherCases>,
  ) -> impl Future<Output = IntermediateSplitResult<Self::ProcessBeforeSplitProduces, Self::SplitterProducesForOtherCases>>;

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize;
}

// maybe the case_step_index overlaps with FinalizedProcess or maybe it allows for a skip
pub struct NextCaseOfFinalizedSplitProcess<
  ProcessBefore: SplitProcess,
  PassedForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  PassesToOtherCases: SplitterOutput,
  ThisCaseConsumes: ParamList,
  ThisCase: FinalizedProcess<ProcessBeforeProduces = ThisCaseConsumes>,
  SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
> {
  pub split_process_before: ProcessBefore,
  pub this_case: ThisCase,
  pub phantom_data: PhantomData<(
    PassedForThisCase,
    PassesToOtherCases,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
  )>,
}

impl<
    ProcessBefore: SplitProcess<SplitterProducesForFirstCase = PassedForThisCase, SplitterProducesForOtherCases = PassesToOtherCases>,
    PassedForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
    PassesToOtherCases: SplitterOutput,
    ThisCaseConsumes: ParamList,
    ThisCase: FinalizedProcess<ProcessBeforeProduces = ThisCaseConsumes>,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
  > FinalizedSplitProcessCase
  for NextCaseOfFinalizedSplitProcess<
    ProcessBefore,
    PassedForThisCase,
    PassesToOtherCases,
    ThisCaseConsumes,
    ThisCase,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
  >
where
  <PassedForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated:
    TransformTo<ThisCase::ProcessBeforeProduces, SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices>,
{
  type ProcessBeforeSplitProduces = ProcessBefore::ProcessBeforeSplitProduces;
  type SplitterProducesForThisCase = PassedForThisCase;
  type SplitterProducesForOtherCases = PassesToOtherCases;

  async fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
  ) -> IntermediateSplitResult<Self::ProcessBeforeSplitProduces, Self::SplitterProducesForOtherCases> {
    let process_before_output = self
      .split_process_before
      .continue_run(previous_run_produced, previous_run_yielded_at)
      .await?;
    match process_before_output {
      IntermediateSplitOutcome::Continue {
        process_before_split_produced,
        this_case_produced,
      } => self.run(process_before_split_produced, this_case_produced).await,
      IntermediateSplitOutcome::Yield(a, b, c) => Ok(IntermediateSplitOutcome::Yield(a, b, c)),
      IntermediateSplitOutcome::Finish(a) => Ok(IntermediateSplitOutcome::Finish(a)),
    }
  }

  async fn run(
    &self,
    process_before_split_produces: Self::ProcessBeforeSplitProduces,
    this_case_or_other_cases_input: Coproduct<Self::SplitterProducesForThisCase, Self::SplitterProducesForOtherCases>,
  ) -> IntermediateSplitResult<Self::ProcessBeforeSplitProduces, Self::SplitterProducesForOtherCases> {
    match this_case_or_other_cases_input {
      Coproduct::Inl(this_case_input) => {
        let next_case_consumes: ThisCase::ProcessBeforeProduces =
          this_case_input.concat(process_before_split_produces).transform();
        match self.this_case.run(next_case_consumes).await? {
          RunOutcome::Yield(a, b, c) => Ok(IntermediateSplitOutcome::Yield(a, b, c)),
          RunOutcome::Finish(a) => Ok(IntermediateSplitOutcome::Finish(a)),
        }
      }
      Coproduct::Inr(other_cases_input) => Ok(IntermediateSplitOutcome::Continue {
        process_before_split_produced: process_before_split_produces,
        this_case_produced: other_cases_input,
      }),
    }
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    self.split_process_before.enumerate_steps(last_used_index)
  }
}
