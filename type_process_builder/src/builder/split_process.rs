use crate::builder::{
  FinalizedProcess, FinalizedSplitProcessCase, FlowingProcess, IntermediateRunOutcome, IntermediateSplitOutcome,
  IntermediateSplitResult, NextCaseOfFinalizedSplitProcess, ParamList, PreviousRunYieldedAt, SplitterOutput,
};
use crate::hlist_concat::Concat;
use crate::hlist_transform_to::TransformTo;
use crate::step::step::Splitter;
use frunk_core::coproduct::Coproduct;
use serde_value::Value;
use std::future::Future;
use std::marker::PhantomData;

pub trait SplitProcess: Sized {
  type ProcessBeforeSplitProduces: ParamList;
  type SplitterProducesForFirstCase: ParamList;
  type SplitterProducesForOtherCases;

  fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
  ) -> impl Future<
    Output = IntermediateSplitResult<
      Self::ProcessBeforeSplitProduces,
      Coproduct<Self::SplitterProducesForFirstCase, Self::SplitterProducesForOtherCases>,
    >,
  >;

  fn run(
    &self,
    process_before_split_produces: Self::ProcessBeforeSplitProduces,
  ) -> impl Future<
    Output = IntermediateSplitResult<
      Self::ProcessBeforeSplitProduces,
      Coproduct<Self::SplitterProducesForFirstCase, Self::SplitterProducesForOtherCases>,
    >,
  >;

  fn case<
    PassedForThisCase: ParamList + Concat<<Self as SplitProcess>::ProcessBeforeSplitProduces>,
    PassesToOtherCases: SplitterOutput,
    ThisCase: FinalizedProcess<
      ProcessBeforeProduces = <PassedForThisCase as Concat<Self::ProcessBeforeSplitProduces>>::Concatenated,
    >,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
  >(
    self,
    this_case: ThisCase,
  ) -> impl FinalizedSplitProcessCase<
    ProcessBeforeSplitProduces = Self::ProcessBeforeSplitProduces,
    SplitterProducesForThisCase = PassedForThisCase,
    SplitterProducesForOtherCases = PassesToOtherCases,
  > {
    NextCaseOfFinalizedSplitProcess {
      split_process_before: self,
      this_case,
      phantom_data: Default::default(),
    }
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize;
}

pub struct SplitProcessSplitter<
  ProcessBefore: FlowingProcess,
  SplitterStepConsumes: ParamList,
  SplitterProducesForFirstCase: ParamList,
  SplitterProducesForOtherCases,
  SplitterStep: Splitter<SplitterStepConsumes, Coproduct<SplitterProducesForFirstCase, SplitterProducesForOtherCases>>,
  ProcessBeforeProducesToSplitterStepConsumesIndices,
  SplitProducesForThisCaseConcatProcessBeforeProducesToFirstCaseConsumesIndices,
> {
  pub process_before: ProcessBefore,
  pub splitter: SplitterStep,
  pub split_step_index: usize,
  pub phantom_data: PhantomData<(
    SplitterStepConsumes,
    SplitterProducesForFirstCase,
    SplitterProducesForOtherCases,
    ProcessBeforeProducesToSplitterStepConsumesIndices,
    SplitProducesForThisCaseConcatProcessBeforeProducesToFirstCaseConsumesIndices,
  )>,
}

impl<
    ProcessBefore: FlowingProcess,
    SplitterStepConsumes: ParamList,
    SplitterProducesForFirstCase: ParamList,
    SplitterProducesForOtherCases,
    SplitterStep: Splitter<SplitterStepConsumes, Coproduct<SplitterProducesForFirstCase, SplitterProducesForOtherCases>>,
    ProcessBeforeProducesToSplitterStepConsumesIndices,
    SplitProducesForThisCaseConcatProcessBeforeProducesToFirstCaseConsumesIndices,
  > SplitProcess
  for SplitProcessSplitter<
    ProcessBefore,
    SplitterStepConsumes,
    SplitterProducesForFirstCase,
    SplitterProducesForOtherCases,
    SplitterStep,
    ProcessBeforeProducesToSplitterStepConsumesIndices,
    SplitProducesForThisCaseConcatProcessBeforeProducesToFirstCaseConsumesIndices,
  >
where
  ProcessBefore::Produces: TransformTo<SplitterStepConsumes, ProcessBeforeProducesToSplitterStepConsumesIndices>,
{
  type ProcessBeforeSplitProduces = ProcessBefore::Produces;
  type SplitterProducesForFirstCase = SplitterProducesForFirstCase;
  type SplitterProducesForOtherCases = SplitterProducesForOtherCases;

  async fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
  ) -> IntermediateSplitResult<
    Self::ProcessBeforeSplitProduces,
    Coproduct<Self::SplitterProducesForFirstCase, Self::SplitterProducesForOtherCases>,
  > {
    if previous_run_yielded_at.0 < self.split_step_index {
      let process_before_output = self
        .process_before
        .continue_run(previous_run_produced, previous_run_yielded_at)
        .await?;
      match process_before_output {
        IntermediateRunOutcome::Continue(process_before_split_produces) => {
          self.run(process_before_split_produces).await
        }
        IntermediateRunOutcome::Yield(a, b, c) => Ok(IntermediateSplitOutcome::Yield(a, b, c)),
        IntermediateRunOutcome::Finish(a) => Ok(IntermediateSplitOutcome::Finish(a)),
      }
    } else {
      let process_before_split_produces = ProcessBefore::Produces::deserialize(previous_run_produced)?;
      self.run(process_before_split_produces).await
    }
  }

  async fn run(
    &self,
    process_before_split_produces: Self::ProcessBeforeSplitProduces,
  ) -> IntermediateSplitResult<
    Self::ProcessBeforeSplitProduces,
    Coproduct<Self::SplitterProducesForFirstCase, Self::SplitterProducesForOtherCases>,
  > {
    let splitter_step_consumes: SplitterStepConsumes = process_before_split_produces.clone().transform();
    Ok(IntermediateSplitOutcome::Continue {
      process_before_split_produced: process_before_split_produces,
      this_case_produced: self.splitter.handle(splitter_step_consumes).await?,
    })
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    let used_index = self.process_before.enumerate_steps(last_used_index);
    self.split_step_index = used_index + 1;
    self.split_step_index
  }
}
