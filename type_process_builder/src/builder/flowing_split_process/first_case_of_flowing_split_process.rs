use crate::builder::{
  subprocess, FinalizedProcess, FlowingProcess, FlowingSplitProcess, IntermediateRunOutcome, IntermediateRunResult,
  IntermediateSplitOutcome, IntermediateSplitResult, NextCaseOfFlowingSplitProcess, ParamList, PreviousRunYieldedAt,
  RunOutcome, SplitProcess, Subprocess,
};
use crate::hlist_concat::Concat;
use crate::hlist_transform_to::TransformTo;
use crate::type_eq::TypeEq;
use frunk_core::coproduct::{CNil, Coproduct};
use serde_value::Value;
use std::hint::unreachable_unchecked;
use std::marker::PhantomData;

pub struct FirstCaseOfFlowingSplitProcess<
  ThisTag,
  PassedForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  PassesToOtherCases,
  ProcessBefore: SplitProcess<PassesToOtherCases>,
  EveryFlowingCaseProduces: ParamList,
  ThisCase: FlowingProcess,
  SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
  Ix,
> {
  pub split_process_before: ProcessBefore,
  pub this_case: ThisCase,
  pub phantom_data: PhantomData<(
    ThisTag,
    PassedForThisCase,
    PassesToOtherCases,
    EveryFlowingCaseProduces,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
    Ix,
  )>,
}

impl<
    ThisTag,
    NextTag,
    PassesToOtherCases,
    ProcessBefore: SplitProcess<
      Coproduct<(PhantomData<NextTag>, PassesToNextCase), PassesToOtherCases>,
      SplitterProducesForFirstCase = PassedForThisCase,
      SplitterTagForFirstCase = ThisTag,
    >,
    PassedForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
    PassesToNextCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
    EveryFlowingCaseProduces: ParamList,
    ThisCase: FinalizedProcess,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
  >
  FirstCaseOfFlowingSplitProcess<
    ThisTag,
    PassedForThisCase,
    Coproduct<(PhantomData<NextTag>, PassesToNextCase), PassesToOtherCases>,
    ProcessBefore,
    EveryFlowingCaseProduces,
    ThisCase,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
  >
where
  <PassedForThisCase as Concat<<ProcessBefore>::ProcessBeforeSplitProduces>>::Concatenated:
    TransformTo<ThisCase::ProcessBeforeProduces, SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices>,
{
  pub fn case<
    AssumedTag,
    NextCase: FinalizedProcess,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndicesA,
  >(
    self,
    create_case: impl FnOnce(Subprocess<ProcessBefore::ProcessBeforeSplitProduces>) -> NextCase,
  ) -> NextCaseOfFlowingSplitProcess<
    NextTag,
    PassesToNextCase,
    PassesToOtherCases,
    Self,
    EveryFlowingCaseProduces,
    NextCase,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndicesA,
  >
  where
    (AssumedTag, PhantomData<NextTag>): TypeEq,
    <PassesToNextCase as Concat<<ProcessBefore>::ProcessBeforeSplitProduces>>::Concatenated:
      TransformTo<NextCase::ProcessBeforeProduces, SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndicesA>,
  {
    NextCaseOfFlowingSplitProcess {
      split_process_before: self,
      this_case: create_case(subprocess::<ProcessBefore::ProcessBeforeSplitProduces>()),
      phantom_data: Default::default(),
    }
  }
}

/// Removing this would forbid having just one case in a split
impl<
    ThisTag,
    ProcessBefore: SplitProcess<CNil>,
    EveryFlowingCaseProduces: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
    ThisCase: FlowingProcess,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
    Ix,
  > FlowingProcess
  for FirstCaseOfFlowingSplitProcess<
    ThisTag,
    ProcessBefore::SplitterProducesForFirstCase,
    CNil,
    ProcessBefore,
    EveryFlowingCaseProduces,
    ThisCase,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
    Ix,
  >
where
  ProcessBefore::SplitterProducesForFirstCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  <ProcessBefore::SplitterProducesForFirstCase as Concat<<ProcessBefore>::ProcessBeforeSplitProduces>>::Concatenated:
    TransformTo<ThisCase::ProcessBeforeProduces, SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices>,
  <ThisCase as FlowingProcess>::Produces: TransformTo<EveryFlowingCaseProduces, Ix>,
{
  type ProcessBeforeProduces = ProcessBefore::ProcessBeforeSplitProduces;
  type Produces = <EveryFlowingCaseProduces as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated;

  async fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
  ) -> IntermediateRunResult<Self::Produces> {
    let process_before_output = self
      .split_process_before
      .continue_run(previous_run_produced, previous_run_yielded_at, user_input)
      .await?;
    match process_before_output {
      IntermediateSplitOutcome::Continue {
        process_before_split_produced,
        passes_to_other_cases,
      } => match passes_to_other_cases {
        Coproduct::Inl(passes_to_this_case) => {
          let this_case_consumes: <ThisCase as FlowingProcess>::ProcessBeforeProduces = passes_to_this_case
            .concat(process_before_split_produced.clone())
            .transform();
          match self.this_case.run(this_case_consumes).await? {
            IntermediateRunOutcome::Continue(produced) => {
              let every_flowing_case_produces: EveryFlowingCaseProduces = produced.transform();
              Ok(IntermediateRunOutcome::Continue(
                every_flowing_case_produces.concat(process_before_split_produced),
              ))
            }
            IntermediateRunOutcome::Yield(a, b, c) => Ok(IntermediateRunOutcome::Yield(a, b, c)),
            IntermediateRunOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
          }
        }
        Coproduct::Inr(c_nil) => match c_nil {},
      },
      IntermediateSplitOutcome::Yield(a, b, c) => Ok(IntermediateRunOutcome::Yield(a, b, c)),
      IntermediateSplitOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
    }
  }

  async fn run(&self, process_before_produces: Self::ProcessBeforeProduces) -> IntermediateRunResult<Self::Produces> {
    unsafe { unreachable_unchecked() } // fixme sadge
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    self.split_process_before.enumerate_steps(last_used_index)
  }
}

impl<
    ThisTag,
    PassedForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
    PassesToOtherCases,
    ProcessBefore: SplitProcess<PassesToOtherCases, SplitterProducesForFirstCase = PassedForThisCase>,
    EveryFlowingCaseProduces: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
    ThisCase: FinalizedProcess,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
    Ix,
  > FlowingSplitProcess<PassesToOtherCases>
  for FirstCaseOfFlowingSplitProcess<
    ThisTag,
    PassedForThisCase,
    PassesToOtherCases,
    ProcessBefore,
    EveryFlowingCaseProduces,
    ThisCase,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
    Ix,
  >
where
  <PassedForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated:
    TransformTo<ThisCase::ProcessBeforeProduces, SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices>,
{
  type EveryFlowingCaseProduces = EveryFlowingCaseProduces;
  type ProcessBeforeSplitProduces = ProcessBefore::ProcessBeforeSplitProduces;
  type SplitterProducesForThisCase = PassedForThisCase;
  type SplitterTagForThisCase = ThisTag;

  async fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
  ) -> IntermediateSplitResult<
    <EveryFlowingCaseProduces as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
    PassesToOtherCases,
  > {
    let process_before_output = self
      .split_process_before
      .continue_run(previous_run_produced, previous_run_yielded_at, user_input)
      .await?;
    match process_before_output {
      IntermediateSplitOutcome::Continue {
        process_before_split_produced,
        passes_to_other_cases: this_case_produced,
      } => self.run(process_before_split_produced, this_case_produced).await,
      IntermediateSplitOutcome::Yield(a, b, c) => Ok(IntermediateSplitOutcome::Yield(a, b, c)),
      IntermediateSplitOutcome::Finish(a) => Ok(IntermediateSplitOutcome::Finish(a)),
    }
  }

  async fn run(
    &self,
    process_before_split_produced: Self::ProcessBeforeSplitProduces,
    this_case_or_other_cases_consumes: Coproduct<Self::SplitterProducesForThisCase, PassesToOtherCases>,
  ) -> IntermediateSplitResult<
    <EveryFlowingCaseProduces as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
    PassesToOtherCases,
  > {
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
