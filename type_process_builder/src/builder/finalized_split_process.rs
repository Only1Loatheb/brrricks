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
  type PassesToOtherCases: SplitterOutput;

  fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
  ) -> impl Future<Output = IntermediateSplitResult<Self::PassesToOtherCases>>;

  fn run(
    &self,
    process_before_produces: Self::ProcessBeforeProduces,
  ) -> impl Future<Output = IntermediateSplitResult<Self::PassesToOtherCases>>;

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize;
}

// one additional trait could be introduced to make split() and first_case() separate methods, but I am who I am
pub struct FirstCaseOfFinalizedSplitProcess<
  ProcessBefore: FlowingProcess,
  SplitterStepConsumes: ParamList,
  SplitterProduces: SplitterOutput,
  SplitterStep: Splitter<SplitterStepConsumes, SplitterProduces>,
  FirstCase: FinalizedProcess,
  ProcessBeforeProducesToSplitterStepConsumesIndices,
  SplitProducesForThisCaseConcatProcessBeforeProducesToFirstCaseConsumesIndices,
> {
  pub process_before: ProcessBefore,
  pub splitter: SplitterStep,
  pub split_step_index: usize,
  pub first_case: FirstCase,
  pub phantom_data: PhantomData<(
    SplitterStepConsumes,
    SplitterProduces,
    ProcessBeforeProducesToSplitterStepConsumesIndices,
    SplitProducesForThisCaseConcatProcessBeforeProducesToFirstCaseConsumesIndices,
  )>,
}

impl<
    ProcessBefore: FlowingProcess,
    SplitterStepConsumes: ParamList,
    SplitProducesForThisCase: ParamList + Concat<ProcessBefore::Produces>,
    SplitProducesForOtherCases: SplitterOutput,
    SplitterStep: Splitter<SplitterStepConsumes, Coproduct<SplitProducesForThisCase, SplitProducesForOtherCases>>,
    FirstCase: FinalizedProcess,
    ProcessBeforeProducesToSplitterStepConsumesIndices,
    SplitProducesForThisCaseConcatProcessBeforeProducesToFirstCaseConsumesIndices,
  > FinalizedSplitProcess
  for FirstCaseOfFinalizedSplitProcess<
    ProcessBefore,
    SplitterStepConsumes,
    Coproduct<SplitProducesForThisCase, SplitProducesForOtherCases>,
    SplitterStep,
    FirstCase,
    ProcessBeforeProducesToSplitterStepConsumesIndices,
    SplitProducesForThisCaseConcatProcessBeforeProducesToFirstCaseConsumesIndices,
  >
where
  ProcessBefore::Produces: TransformTo<SplitterStepConsumes, ProcessBeforeProducesToSplitterStepConsumesIndices>,
  <SplitProducesForThisCase as Concat<ProcessBefore::Produces>>::Concatenated: TransformTo<
    FirstCase::ProcessBeforeProduces,
    SplitProducesForThisCaseConcatProcessBeforeProducesToFirstCaseConsumesIndices,
  >,
{
  type ProcessBeforeProduces = ProcessBefore::Produces;
  type PassesToOtherCases = SplitProducesForOtherCases;

  async fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
  ) -> IntermediateSplitResult<Self::PassesToOtherCases> {
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
      let process_before_produces: ProcessBefore::Produces =
        ProcessBefore::Produces::deserialize(previous_run_produced)?;
      self.run(process_before_produces).await
    }
  }

  async fn run(
    &self,
    process_before_produces: Self::ProcessBeforeProduces,
  ) -> IntermediateSplitResult<Self::PassesToOtherCases> {
    let splitter_step_consumes: SplitterStepConsumes = process_before_produces.clone().transform();
    let splitter_step_output: Coproduct<SplitProducesForThisCase, SplitProducesForOtherCases> =
      self.splitter.handle(splitter_step_consumes).await?;
    match splitter_step_output {
      Coproduct::Inl(produces_for_first_case) => {
        // we could get rid of params unused here, but well
        let first_case_consumes: FirstCase::ProcessBeforeProduces =
          produces_for_first_case.concat(process_before_produces).transform();
        match self.first_case.run(first_case_consumes).await? {
          RunOutcome::Yield(a, b, c) => Ok(IntermediateSplitOutcome::Yield(a, b, c)),
          RunOutcome::Finish(a) => Ok(IntermediateSplitOutcome::Finish(a)),
        }
      }
      Coproduct::Inr(produces_for_other_cases) => Ok(IntermediateSplitOutcome::Continue(produces_for_other_cases)),
    }
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    let used_index = self.process_before.enumerate_steps(last_used_index);
    self.split_step_index = used_index + 1;
    self.split_step_index
  }
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
  > FinalizedSplitProcess
  for NextCaseOfFinalizedSplitProcess<ProcessBefore, PassedForThisCase, PassesToOtherCases, NextCase>
{
  type ProcessBeforeProduces = ProcessBefore::ProcessBeforeProduces;
  type PassesToOtherCases = PassesToOtherCases;

  async fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
  ) -> IntermediateSplitResult<Self::PassesToOtherCases> {
    todo!()
  }

  async fn run(
    &self,
    process_before_produces: Self::ProcessBeforeProduces,
  ) -> IntermediateSplitResult<Self::PassesToOtherCases> {
    todo!()
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    let used_index = self.split_process_before.enumerate_steps(last_used_index);
    self.case_step_index = used_index + 1;
    self.case_step_index
  }
}
