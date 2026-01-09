use crate::builder::{
  subprocess, FinalizedProcess, FinalizedSplitProcess, IntermediateFinalizedSplitOutcome,
  IntermediateFinalizedSplitResult, ParamList, PreviousRunYieldedAt, RunOutcome, RunResult, Subprocess,
};
use crate::hlist_concat::Concat;
use crate::hlist_transform_to::TransformTo;
use crate::type_eq::TypeEq;
use frunk_core::coproduct::{CNil, Coproduct};
use serde_value::Value;
use std::hint::unreachable_unchecked;
use std::marker::PhantomData;

pub struct NextCaseOfFinalizedSplitProcess<
  ThisTag,
  SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  SplitterPassesToOtherCases,
  ProcessBefore: FinalizedSplitProcess<Coproduct<(ThisTag, SplitterProducesForThisCase), SplitterPassesToOtherCases>>,
  ThisCase: FinalizedProcess,
  SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
> {
  pub split_process_before: ProcessBefore,
  pub this_case: ThisCase,
  pub phantom_data: PhantomData<(
    ThisTag,
    SplitterProducesForThisCase,
    SplitterPassesToOtherCases,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
  )>,
}

impl<
    ThisTag,
    NextTag,
    SplitterPassesToOtherCases,
    ProcessBeforeProcessBefore: FinalizedSplitProcess<
      Coproduct<
        (ThisTag, SplitterProducesForThisCase),
        Coproduct<(NextTag, PassesToNextCase), SplitterPassesToOtherCases>,
      >,
    >,
    SplitterProducesForThisCase: ParamList + Concat<ProcessBeforeProcessBefore::ProcessBeforeSplitProduces>,
    PassesToNextCase: ParamList + Concat<ProcessBeforeProcessBefore::ProcessBeforeSplitProduces>,
    ThisCase: FinalizedProcess,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
  >
  NextCaseOfFinalizedSplitProcess<
    ThisTag,
    SplitterProducesForThisCase,
    Coproduct<(NextTag, PassesToNextCase), SplitterPassesToOtherCases>,
    ProcessBeforeProcessBefore,
    ThisCase,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
  >
where
  <SplitterProducesForThisCase as Concat<ProcessBeforeProcessBefore::ProcessBeforeSplitProduces>>::Concatenated:
    TransformTo<ThisCase::ProcessBeforeProduces, SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices>,
{
  // fixme create_case should accept
  // fixme Subprocess<<Self::SplitterProducesForFirstCase as Concat<Self::ProcessBeforeSplitProduces>>::Concatenated>
  pub fn case<
    AssumedTag,
    NextCase: FinalizedProcess,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndicesA,
  >(
    self,
    create_case: impl FnOnce(Subprocess<ProcessBeforeProcessBefore::ProcessBeforeSplitProduces>) -> NextCase,
  ) -> NextCaseOfFinalizedSplitProcess<
    NextTag,
    PassesToNextCase,
    SplitterPassesToOtherCases,
    Self,
    NextCase,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndicesA,
  >
  where
    (AssumedTag, NextTag): TypeEq,
    <PassesToNextCase as Concat<ProcessBeforeProcessBefore::ProcessBeforeSplitProduces>>::Concatenated:
      TransformTo<NextCase::ProcessBeforeProduces, SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndicesA>,
  {
    NextCaseOfFinalizedSplitProcess {
      split_process_before: self,
      this_case: create_case(subprocess::<ProcessBeforeProcessBefore::ProcessBeforeSplitProduces>()),
      phantom_data: Default::default(),
    }
  }
}

/// the last case
impl<
    ThisTag,
    SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
    ProcessBefore: FinalizedSplitProcess<Coproduct<(ThisTag, SplitterProducesForThisCase), CNil>>,
    ThisCase: FinalizedProcess,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
  > FinalizedProcess
  for NextCaseOfFinalizedSplitProcess<
    ThisTag,
    SplitterProducesForThisCase,
    CNil,
    ProcessBefore,
    ThisCase,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
  >
where
  ProcessBefore::SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  <SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated:
    TransformTo<ThisCase::ProcessBeforeProduces, SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices>,
{
  type ProcessBeforeProduces = ProcessBefore::ProcessBeforeSplitProduces;

  async fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
  ) -> RunResult {
    let process_before_output = self
      .split_process_before
      .continue_run(previous_run_produced, previous_run_yielded_at, user_input)
      .await?;
    match process_before_output {
      IntermediateFinalizedSplitOutcome::GoToCase {
        process_before_split_produced,
        splitter_passes_to_other_cases,
      } => match splitter_passes_to_other_cases {
        Coproduct::Inl((_pd, params_passed_to_other_cases)) => {
          let this_case_consumes: ThisCase::ProcessBeforeProduces = params_passed_to_other_cases
            .concat(process_before_split_produced)
            .transform();
          self.this_case.run(this_case_consumes).await
        }
        Coproduct::Inr(c_nil) => match c_nil {},
      },
      IntermediateFinalizedSplitOutcome::Yield(a, b, c) => Ok(RunOutcome::Yield(a, b, c)),
      IntermediateFinalizedSplitOutcome::Finish(a) => Ok(RunOutcome::Finish(a)),
    }
  }

  async fn run(&self, _process_before_produces: Self::ProcessBeforeProduces) -> RunResult {
    // most likely design flow, but I don't think it will happen :)
    unsafe { unreachable_unchecked() }
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    self.split_process_before.enumerate_steps(last_used_index)
  }
}

impl<
    ThisTag,
    SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
    SplitterProducesForOtherCases,
    ProcessBefore: FinalizedSplitProcess<Coproduct<(ThisTag, SplitterProducesForThisCase), SplitterProducesForOtherCases>>,
    ThisCase: FinalizedProcess,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
  > FinalizedSplitProcess<SplitterProducesForOtherCases>
  for NextCaseOfFinalizedSplitProcess<
    ThisTag,
    SplitterProducesForThisCase,
    SplitterProducesForOtherCases,
    ProcessBefore,
    ThisCase,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
  >
where
  <SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated:
    TransformTo<ThisCase::ProcessBeforeProduces, SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices>,
{
  type ProcessBeforeSplitProduces = ProcessBefore::ProcessBeforeSplitProduces;
  type SplitterProducesForThisCase = SplitterProducesForThisCase;
  type SplitterTagForThisCase = ThisTag;

  async fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
  ) -> IntermediateFinalizedSplitResult<Self::ProcessBeforeSplitProduces, SplitterProducesForOtherCases> {
    let process_before_output = self
      .split_process_before
      .continue_run(previous_run_produced, previous_run_yielded_at, user_input)
      .await?;
    match process_before_output {
      IntermediateFinalizedSplitOutcome::GoToCase {
        process_before_split_produced,
        splitter_passes_to_other_cases,
      } => {
        let produced = match splitter_passes_to_other_cases {
          Coproduct::Inl((_pd, params)) => Coproduct::Inl(params),
          Coproduct::Inr(inr_value) => Coproduct::Inr(inr_value),
        };
        self.run(process_before_split_produced, produced).await
      }
      IntermediateFinalizedSplitOutcome::Yield(a, b, c) => Ok(IntermediateFinalizedSplitOutcome::Yield(a, b, c)),
      IntermediateFinalizedSplitOutcome::Finish(a) => Ok(IntermediateFinalizedSplitOutcome::Finish(a)),
    }
  }

  async fn run(
    &self,
    process_before_split_produced: Self::ProcessBeforeSplitProduces,
    splitter_produces_for_this_case_or_other_cases_consumes: Coproduct<
      Self::SplitterProducesForThisCase,
      SplitterProducesForOtherCases,
    >,
  ) -> IntermediateFinalizedSplitResult<Self::ProcessBeforeSplitProduces, SplitterProducesForOtherCases> {
    match splitter_produces_for_this_case_or_other_cases_consumes {
      Coproduct::Inl(splitter_produces_for_first_case) => {
        let this_case_consumes: ThisCase::ProcessBeforeProduces = splitter_produces_for_first_case
          .concat(process_before_split_produced)
          .transform();
        match self.this_case.run(this_case_consumes).await? {
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
