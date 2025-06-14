use crate::builder::finalized_process::FinalizedProcess;
use crate::builder::flowing_process::FlowingProcess;
use crate::builder::IntermediateRunOutcome::{Continue, Finish, Yield};
use crate::builder::{PreviousRunYieldedAt, RunOutcome, RunResult};
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

  fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
  ) -> impl Future<Output = RunResult>;

  fn run(&self, process_before_produces: Self::ProcessBeforeProduces) -> impl Future<Output = RunResult>;

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

  async fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
  ) -> RunResult {
    if previous_run_yielded_at.0 < self.split_step_index {
      let process_before_output = self
        .process_before
        .continue_run(previous_run_produced, previous_run_yielded_at)
        .await?;
      match process_before_output {
        Continue(process_before_produces) => self.run(process_before_produces).await,
        Yield(a, b, c) => Ok(RunOutcome::Yield(a, b, c)),
        Finish(a) => Ok(RunOutcome::Finish(a)),
      }
    } else {
      let process_before_produces: ProcessBefore::Produces =
        ProcessBefore::Produces::deserialize(previous_run_produced)?;
      self.run(process_before_produces).await
    }
  }

  async fn run(&self, process_before_produces: Self::ProcessBeforeProduces) -> RunResult {
    let splitter_step_consumes: SplitterStepConsumes = process_before_produces.clone().transform();
    let splitter_step_output: Coproduct<SplitProducesForThisCase, SplitProducesForOtherCases> =
      self.splitter.handle(splitter_step_consumes).await?;
    match splitter_step_output {
      Coproduct::Inl(splitter_step_produces) => {
        // we could get rid of params unused here, but well
        let first_case_consumes: FirstCase::ProcessBeforeProduces =
          splitter_step_produces.concat(process_before_produces).transform();
        self.first_case.run(first_case_consumes).await
      }
      Coproduct::Inr(_) => {
        todo!()
      }
    }
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    let used_index = self.process_before.enumerate_steps(last_used_index);
    self.split_step_index = used_index + 1;
    self.split_step_index
  }
}

// maybe the case_step_index overlaps with FinalizedProcess or maybe it allows for a skip
pub struct NextCaseOfFinalizedSplitProcess<ProcessBefore: FinalizedSplitProcess, NextCase: FinalizedProcess> {
  pub split_process_before: ProcessBefore,
  pub case_step_index: usize,
  pub next_case: NextCase,
}

impl<ProcessBefore: FinalizedSplitProcess, NextCase: FinalizedProcess> FinalizedSplitProcess
  for NextCaseOfFinalizedSplitProcess<ProcessBefore, NextCase>
{
  type ProcessBeforeProduces = ProcessBefore::ProcessBeforeProduces;

  async fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
  ) -> RunResult {
    //Self::SplitterOutput
    todo!()
  }

  async fn run(&self, process_before_produces: Self::ProcessBeforeProduces) -> RunResult {
    todo!()
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    let used_index = self.split_process_before.enumerate_steps(last_used_index);
    self.case_step_index = used_index + 1;
    self.case_step_index
  }
}
