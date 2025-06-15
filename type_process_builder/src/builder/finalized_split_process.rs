use crate::builder::finalized_process::FinalizedProcess;
use crate::builder::flowing_process::FlowingProcess;
use crate::builder::*;
use crate::hlist_concat::Concat;
use crate::hlist_transformer::TransformTo;
use crate::param_list::ParamList;
use crate::step::splitter_output_repr::SplitterOutput;
use crate::step::step::Splitter;
use frunk_core::coproduct::Coproduct;
use serde_value::Value;
use std::future::Future;
use std::marker::PhantomData;

pub trait FinalizedSplitProcess: Sized {
  type ProcessBeforeProduces: ParamList;
  type SplitterProducesForFirstCase: ParamList;
  type SplitterProducesForOtherCases: SplitterOutput;

  fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
  ) -> impl Future<
    Output = IntermediateSplitResult<
      Coproduct<Self::SplitterProducesForFirstCase, Self::SplitterProducesForOtherCases>,
    >,
  >;

  fn run(
    &self,
    process_before_produces: Self::ProcessBeforeProduces,
  ) -> impl Future<
    Output = IntermediateSplitResult<
      Coproduct<Self::SplitterProducesForFirstCase, Self::SplitterProducesForOtherCases>,
    >,
  >;

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize;
}

pub struct FirstCaseOfFinalizedSplitProcess<
  ProcessBefore: FlowingProcess,
  SplitterStepConsumes: ParamList,
  SplitterProducesForFirstCase: ParamList,
  SplitterProducesForOtherCases: SplitterOutput,
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
    SplitterProducesForOtherCases: SplitterOutput,
    SplitterStep: Splitter<SplitterStepConsumes, Coproduct<SplitterProducesForFirstCase, SplitterProducesForOtherCases>>,
    ProcessBeforeProducesToSplitterStepConsumesIndices,
    SplitProducesForThisCaseConcatProcessBeforeProducesToFirstCaseConsumesIndices,
  > FinalizedSplitProcess
  for FirstCaseOfFinalizedSplitProcess<
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
  type ProcessBeforeProduces = ProcessBefore::Produces;
  type SplitterProducesForFirstCase = SplitterProducesForFirstCase;
  type SplitterProducesForOtherCases = SplitterProducesForOtherCases;

  async fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
  ) -> IntermediateSplitResult<Coproduct<Self::SplitterProducesForFirstCase, Self::SplitterProducesForOtherCases>> {
    if previous_run_yielded_at.0 < self.split_step_index {
      let process_before_output = self
        .process_before
        .continue_run(previous_run_produced, previous_run_yielded_at)
        .await?;
      match process_before_output {
        IntermediateRunOutcome::Continue(process_before_produces) => self.run(process_before_produces).await,
        IntermediateRunOutcome::Yield(a, b, c) => Ok(IntermediateSplitOutcome::Yield(a, b, c)),
        IntermediateRunOutcome::Finish(a) => Ok(IntermediateSplitOutcome::Finish(a)),
      }
    } else {
      let process_before_produces = ProcessBefore::Produces::deserialize(previous_run_produced)?;
      self.run(process_before_produces).await
    }
  }

  async fn run(
    &self,
    process_before_produces: Self::ProcessBeforeProduces,
  ) -> IntermediateSplitResult<Coproduct<Self::SplitterProducesForFirstCase, Self::SplitterProducesForOtherCases>> {
    let splitter_step_consumes: SplitterStepConsumes = process_before_produces.transform();
    Ok(IntermediateSplitOutcome::Continue(
      self.splitter.handle(splitter_step_consumes).await?,
    ))
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    let used_index = self.process_before.enumerate_steps(last_used_index);
    self.split_step_index = used_index + 1;
    self.split_step_index
  }
}

pub trait FinalizedSplitProcessCase: Sized {
  type ProcessBeforeProduces: ParamList;
  type SplitterProducesForThisCase: ParamList + Concat<Self::ProcessBeforeProduces>;
  type SplitterProducesForOtherCases: SplitterOutput;

  fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
  ) -> impl Future<Output = IntermediateSplitResult<Self::SplitterProducesForOtherCases>>;

  fn run(
    &self,
    this_case_or_other_case_input: Coproduct<Self::SplitterProducesForThisCase, Self::SplitterProducesForOtherCases>,
  ) -> impl Future<Output = IntermediateSplitResult<Self::SplitterProducesForOtherCases>>;

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize;
}

// maybe the case_step_index overlaps with FinalizedProcess or maybe it allows for a skip
pub struct NextCaseOfFinalizedSplitProcess<
  ProcessBefore: FinalizedSplitProcess,
  PassedForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeProduces>,
  PassesToOtherCases: SplitterOutput,
  NextCase: FinalizedProcess,
> {
  pub split_process_before: ProcessBefore,
  pub case_step_index: usize,
  pub next_case: NextCase,
  pub phantom_data: PhantomData<(PassedForThisCase, PassesToOtherCases)>,
}

impl<
    ProcessBefore: FinalizedSplitProcess,
    PassedForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeProduces>,
    PassesToOtherCases: SplitterOutput,
    NextCase: FinalizedProcess,
  > FinalizedSplitProcessCase
  for NextCaseOfFinalizedSplitProcess<ProcessBefore, PassedForThisCase, PassesToOtherCases, NextCase>
{
  type ProcessBeforeProduces = ProcessBefore::ProcessBeforeProduces;
  type SplitterProducesForThisCase = PassedForThisCase;
  type SplitterProducesForOtherCases = PassesToOtherCases;

  async fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
  ) -> IntermediateSplitResult<Self::SplitterProducesForOtherCases> {
    if previous_run_yielded_at.0 < self.case_step_index {
      let process_before_output = self
        .split_process_before
        .continue_run(previous_run_produced, previous_run_yielded_at)
        .await?;
      match process_before_output {
        IntermediateSplitOutcome::Continue(process_before_produces) => self.run(process_before_produces).await,
        IntermediateSplitOutcome::Yield(a, b, c) => Ok(IntermediateSplitOutcome::Yield(a, b, c)),
        IntermediateSplitOutcome::Finish(a) => Ok(IntermediateSplitOutcome::Finish(a)),
      }
    } else {
      let process_before_produces =
        <Self::SplitterProducesForThisCase as Concat<Self::ProcessBeforeProduces>>::Concatenated::deserialize(
          previous_run_produced,
        )?;
      self.run(process_before_produces).await
    }
  }

  async fn run(
    &self,
    this_case_or_other_case_input: Coproduct<Self::SplitterProducesForThisCase, Self::SplitterProducesForOtherCases>,
  ) -> IntermediateSplitResult<Self::SplitterProducesForOtherCases> {
    todo!()
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    let used_index = self.split_process_before.enumerate_steps(last_used_index);
    self.case_step_index = used_index + 1;
    self.case_step_index
  }
}
