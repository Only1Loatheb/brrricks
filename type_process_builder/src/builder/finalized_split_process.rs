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

pub trait FinalizedSplitProcess<SplitterProducesForOtherCases>: Sized {
  type ProcessBeforeSplitProduces: ParamList;
  type SplitterProducesForThisCase: ParamList + Concat<Self::ProcessBeforeSplitProduces>;

  fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
  ) -> impl Future<Output = IntermediateSplitResult<Self::ProcessBeforeSplitProduces, SplitterProducesForOtherCases>>;

  fn run(
    &self,
    process_before_split_produces: Self::ProcessBeforeSplitProduces,
    this_case_or_other_cases_input: Coproduct<Self::SplitterProducesForThisCase, SplitterProducesForOtherCases>,
  ) -> impl Future<Output = IntermediateSplitResult<Self::ProcessBeforeSplitProduces, SplitterProducesForOtherCases>>;

  // fn case<
  //   PassedForThisCase: ParamList + Concat<Self::ProcessBeforeSplitProduces>,
  //   PassesToOtherCases,
  //   ThisCaseConsumes: ParamList,
  //   ThisCase: FinalizedProcess<ProcessBeforeProduces = ThisCaseConsumes>,
  //   SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
  // >(
  //   self,
  //   this_case: ThisCase,
  // ) -> impl FinalizedSplitProcess<
  //   ProcessBeforeSplitProduces = Self::ProcessBeforeSplitProduces,
  //   SplitterProducesForThisCase = Self::SplitterProducesForThisCase,
  //   SplitterProducesForOtherCases = SplitterProducesForOtherCases,
  // >
  // where
  //   Self::SplitterProducesForThisCase: Concat<Self::ProcessBeforeSplitProduces>,
  //   <Self::SplitterProducesForThisCase as Concat<Self::ProcessBeforeSplitProduces>>::Concatenated:
  //     TransformTo<ThisCase::ProcessBeforeProduces, SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices>,
  // {
  //   NextCaseOfFinalizedSplitProcess::<
  //     Self,
  //     PassedForThisCase,
  //     PassesToOtherCases,
  //     ThisCaseConsumes,
  //     ThisCase,
  //     SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
  //   > {
  //     split_process_before: self,
  //     this_case,
  //     phantom_data: Default::default(),
  //   }
  // }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize;
}

pub struct FirstCaseOfFinalizedSplitProcess<
  ProcessBefore: SplitProcess,
  PassedForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  PassesToOtherCases,
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
    PassesToOtherCases,
    ThisCaseConsumes: ParamList,
    ThisCase: FinalizedProcess<ProcessBeforeProduces = ThisCaseConsumes>,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
  > FinalizedSplitProcess<PassesToOtherCases>
  for FirstCaseOfFinalizedSplitProcess<
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

  async fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
  ) -> IntermediateSplitResult<Self::ProcessBeforeSplitProduces, PassesToOtherCases> {
    let process_before_output = self
      .split_process_before
      .continue_run(previous_run_produced, previous_run_yielded_at)
      .await?;
    match process_before_output {
      IntermediateSplitOutcome::Continue {
        process_before_split_produced,
        passes_to_other_ceses: this_case_produced,
      } => self.run(process_before_split_produced, this_case_produced).await,
      IntermediateSplitOutcome::Yield(a, b, c) => Ok(IntermediateSplitOutcome::Yield(a, b, c)),
      IntermediateSplitOutcome::Finish(a) => Ok(IntermediateSplitOutcome::Finish(a)),
    }
  }

  async fn run(
    &self,
    process_before_split_produces: Self::ProcessBeforeSplitProduces,
    this_case_or_other_cases_input: Coproduct<Self::SplitterProducesForThisCase, PassesToOtherCases>,
  ) -> IntermediateSplitResult<Self::ProcessBeforeSplitProduces, PassesToOtherCases> {
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
        passes_to_other_ceses: other_cases_input,
      }),
    }
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    self.split_process_before.enumerate_steps(last_used_index)
  }
}

pub struct NextCaseOfFinalizedSplitProcess<
  PassesToOtherCases,
  PassedForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  ProcessBefore: FinalizedSplitProcess<PassesToOtherCases>,
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
    PassesToOtherCases,
    PassedForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
    ProcessBefore: FinalizedSplitProcess<PassesToOtherCases>,
    ThisCaseConsumes: ParamList,
    ThisCase: FinalizedProcess<ProcessBeforeProduces = ThisCaseConsumes>,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
  > FinalizedSplitProcess<PassesToOtherCases>
  for NextCaseOfFinalizedSplitProcess<
    PassesToOtherCases,
    PassedForThisCase,
    ProcessBefore,
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

  async fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
  ) -> IntermediateSplitResult<Self::ProcessBeforeSplitProduces, PassesToOtherCases> {
    let process_before_output = self
      .split_process_before
      .continue_run(previous_run_produced, previous_run_yielded_at)
      .await?;
    match process_before_output {
      IntermediateSplitOutcome::Continue {
        process_before_split_produced,
        passes_to_other_ceses: passes_to_other_cases,
      } => self.run(process_before_split_produced, passes_to_other_cases).await,
      IntermediateSplitOutcome::Yield(a, b, c) => Ok(IntermediateSplitOutcome::Yield(a, b, c)),
      IntermediateSplitOutcome::Finish(a) => Ok(IntermediateSplitOutcome::Finish(a)),
    }
  }

  async fn run(
    &self,
    process_before_split_produces: Self::ProcessBeforeSplitProduces,
    this_case_or_other_cases_input: Coproduct<Self::SplitterProducesForThisCase, PassesToOtherCases>,
  ) -> IntermediateSplitResult<Self::ProcessBeforeSplitProduces, PassesToOtherCases> {
    match this_case_or_other_cases_input {
      Coproduct::Inl(this_case_input) => {
        let this_case_consumes: ThisCase::ProcessBeforeProduces =
          this_case_input.concat(process_before_split_produces).transform();
        match self.this_case.run(this_case_consumes).await? {
          RunOutcome::Yield(a, b, c) => Ok(IntermediateSplitOutcome::Yield(a, b, c)),
          RunOutcome::Finish(a) => Ok(IntermediateSplitOutcome::Finish(a)),
        }
      }
      Coproduct::Inr(other_cases_input) => Ok(IntermediateSplitOutcome::Continue {
        process_before_split_produced: process_before_split_produces,
        passes_to_other_ceses: other_cases_input,
      }),
    }
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    self.split_process_before.enumerate_steps(last_used_index)
  }
}
