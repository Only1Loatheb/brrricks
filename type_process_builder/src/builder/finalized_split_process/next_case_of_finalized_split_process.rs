use crate::builder::{
  subprocess, FinalizedProcess, FinalizedSplitProcess, IntermediateSplitOutcome, IntermediateSplitResult, ParamList,
  PreviousRunYieldedAt, RunOutcome, RunResult, Subprocess,
};
use crate::hlist_transform_to::TransformTo;
use crate::type_eq::TypeEq;
use frunk_core::coproduct::{CNil, Coproduct};
use serde_value::Value;
use std::hint::unreachable_unchecked;
use std::marker::PhantomData;

use crate::builder::split_process::SplitProcess;
use crate::builder::*;
use crate::hlist_concat::Concat;

pub struct NextCaseOfFinalizedSplitProcess<
  ThisTag,
  PassedForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  PassesToOtherCases,
  ProcessBefore: FinalizedSplitProcess<Coproduct<(PhantomData<ThisTag>, PassedForThisCase), PassesToOtherCases>>,
  ThisCase: FinalizedProcess,
  SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
> {
  pub split_process_before: ProcessBefore,
  pub this_case: ThisCase,
  pub phantom_data: PhantomData<(
    ThisTag,
    PassedForThisCase,
    PassesToOtherCases,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
  )>,
}

/// the last case
impl<
    ThisTag,
    PassedForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
    ProcessBefore: FinalizedSplitProcess<Coproduct<(PhantomData<ThisTag>, PassedForThisCase), CNil>>,
    ThisCase: FinalizedProcess,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
  > FinalizedProcess
  for NextCaseOfFinalizedSplitProcess<
    ThisTag,
    PassedForThisCase,
    CNil,
    ProcessBefore,
    ThisCase,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
  >
where
  ProcessBefore::SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  <PassedForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated:
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
      IntermediateSplitOutcome::Continue {
        process_before_split_produced,
        passes_to_other_cases,
      } => match passes_to_other_cases {
        Coproduct::Inl((_pd, params_passed_to_other_cases)) => {
          let this_case_consumes: ThisCase::ProcessBeforeProduces = params_passed_to_other_cases
            .concat(process_before_split_produced)
            .transform();
          self.this_case.run(this_case_consumes).await
        }
        Coproduct::Inr(c_nil) => match c_nil {},
      },
      IntermediateSplitOutcome::Yield(a, b, c) => Ok(RunOutcome::Yield(a, b, c)),
      IntermediateSplitOutcome::Finish(a) => Ok(RunOutcome::Finish(a)),
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
    SplitterProducesForOtherCases,
    PassedForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
    ProcessBefore: FinalizedSplitProcess<Coproduct<(PhantomData<ThisTag>, PassedForThisCase), SplitterProducesForOtherCases>>,
    ThisCase: FinalizedProcess,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
  > FinalizedSplitProcess<SplitterProducesForOtherCases>
  for NextCaseOfFinalizedSplitProcess<
    ThisTag,
    PassedForThisCase,
    SplitterProducesForOtherCases,
    ProcessBefore,
    ThisCase,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
  >
where
  <PassedForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated:
    TransformTo<ThisCase::ProcessBeforeProduces, SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices>,
{
  type ProcessBeforeSplitProduces = ProcessBefore::ProcessBeforeSplitProduces;
  type SplitterProducesForThisCase = PassedForThisCase;
  type SplitterTagForThisCase = ThisTag;

  async fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
  ) -> IntermediateSplitResult<Self::ProcessBeforeSplitProduces, SplitterProducesForOtherCases> {
    let process_before_output = self
      .split_process_before
      .continue_run(previous_run_produced, previous_run_yielded_at, user_input)
      .await?;
    match process_before_output {
      IntermediateSplitOutcome::Continue {
        process_before_split_produced,
        passes_to_other_cases: this_case_produced,
      } => {
        let produced = match this_case_produced {
          Coproduct::Inl((_pd, params)) => Coproduct::Inl(params),
          Coproduct::Inr(inr_value) => Coproduct::Inr(inr_value),
        };
        self.run(process_before_split_produced, produced).await
      }
      IntermediateSplitOutcome::Yield(a, b, c) => Ok(IntermediateSplitOutcome::Yield(a, b, c)),
      IntermediateSplitOutcome::Finish(a) => Ok(IntermediateSplitOutcome::Finish(a)),
    }
  }

  async fn run(
    &self,
    process_before_split_produced: Self::ProcessBeforeSplitProduces,
    this_case_or_other_cases_consumes: Coproduct<Self::SplitterProducesForThisCase, SplitterProducesForOtherCases>,
  ) -> IntermediateSplitResult<Self::ProcessBeforeSplitProduces, SplitterProducesForOtherCases> {
    match this_case_or_other_cases_consumes {
      Coproduct::Inl(this_case_consumes) => {
        let next_case_consumes: ThisCase::ProcessBeforeProduces =
          this_case_consumes.concat(process_before_split_produced).transform();
        match self.this_case.run(next_case_consumes).await? {
          RunOutcome::Yield(a, b, c) => Ok(IntermediateSplitOutcome::Yield(a, b, c)),
          RunOutcome::Finish(a) => Ok(IntermediateSplitOutcome::Finish(a)),
        }
      }
      Coproduct::Inr(other_cases_consumes) => Ok(IntermediateSplitOutcome::Continue {
        process_before_split_produced: process_before_split_produced,
        passes_to_other_cases: other_cases_consumes,
      }),
    }
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    self.split_process_before.enumerate_steps(last_used_index)
  }
}

impl<
    Tag,
    NextTag,
    PassesToOtherCases,
    ProcessBeforeProcessBefore: FinalizedSplitProcess<
      Coproduct<
        (PhantomData<Tag>, PassedForThisCase),
        Coproduct<(PhantomData<NextTag>, PassesToNextCase), PassesToOtherCases>,
      >,
    >,
    PassedForThisCase: ParamList + Concat<ProcessBeforeProcessBefore::ProcessBeforeSplitProduces>,
    PassesToNextCase: ParamList + Concat<ProcessBeforeProcessBefore::ProcessBeforeSplitProduces>,
    ThisCase: FinalizedProcess,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
  >
  NextCaseOfFinalizedSplitProcess<
    Tag,
    PassedForThisCase,
    Coproduct<(PhantomData<NextTag>, PassesToNextCase), PassesToOtherCases>,
    ProcessBeforeProcessBefore,
    ThisCase,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
  >
where
  <PassedForThisCase as Concat<ProcessBeforeProcessBefore::ProcessBeforeSplitProduces>>::Concatenated:
    TransformTo<ThisCase::ProcessBeforeProduces, SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices>,
{
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
    PassesToOtherCases,
    Self,
    NextCase,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndicesA,
  >
  where
    (AssumedTag, PhantomData<NextTag>): TypeEq,
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
