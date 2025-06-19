use crate::builder::finalized_process::FinalizedProcess;
use crate::builder::flowing_process::FlowingProcess;
use crate::builder::*;
use crate::hlist_concat::Concat;
use crate::hlist_transform_to::TransformTo;
use crate::param_list::ParamList;
use crate::step::step::Splitter;
use frunk_core::coproduct::Coproduct;
use serde_value::Value;
use std::future::Future;
use std::marker::PhantomData;

pub trait FinalizedSplitProcess: Sized {
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

  // fn case<
  //   LinearConsumes: ParamList,
  //   LinearProduces: ParamList + Concat<Self::Produces>,
  //   LinearStep: Linear<LinearConsumes, LinearProduces>,
  //   ProcessBeforeProducesToLastStepConsumesIndices,
  // >(
  //   self,
  //   step: LinearStep,
  // ) -> impl FlowingProcess<
  //   ProcessBeforeProduces = <Self as FlowingProcess>::Produces,
  //   Produces = <LinearProduces as Concat<Self::Produces>>::Concatenated,
  // >
  // where
  //   <Self as FlowingProcess>::Produces: TransformTo<LinearConsumes, ProcessBeforeProducesToLastStepConsumesIndices>,
  // {
  //   // aaa
  //   LinearFlowingProcess {
  //     process_before: self,
  //     last_step: step,
  //     step_index: WILL_BE_RENUMBERED,
  //     phantom_data: Default::default(),
  //   }
  // }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize;
}

pub struct SplitProcess<
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
  > FinalizedSplitProcess
  for SplitProcess<
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
  ProcessBefore: FinalizedSplitProcess,
  PassedForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  PassesToOtherCases: SplitterOutput,
  NextCase: FinalizedProcess,
  SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
> {
  pub split_process_before: ProcessBefore,
  pub next_case: NextCase,
  pub phantom_data: PhantomData<(
    PassedForThisCase,
    PassesToOtherCases,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
  )>,
}

impl<
    ProcessBefore: FinalizedSplitProcess<
      SplitterProducesForFirstCase = PassedForThisCase,
      SplitterProducesForOtherCases = PassesToOtherCases,
    >,
    PassedForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
    PassesToOtherCases: SplitterOutput,
    NextCase: FinalizedProcess,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
  > FinalizedSplitProcessCase
  for NextCaseOfFinalizedSplitProcess<
    ProcessBefore,
    PassedForThisCase,
    PassesToOtherCases,
    NextCase,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
  >
where
  <PassedForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated:
    TransformTo<NextCase::ProcessBeforeProduces, SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices>,
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
        let next_case_consumes: NextCase::ProcessBeforeProduces =
          this_case_input.concat(process_before_split_produces).transform();
        match self.next_case.run(next_case_consumes).await? {
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
