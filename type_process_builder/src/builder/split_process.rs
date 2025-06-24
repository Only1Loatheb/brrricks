use crate::builder::{
  subprocess, FinalizedProcess, FinalizedSplitProcess, FirstCaseOfFinalizedSplitProcess, FlowingProcess,
  IntermediateRunOutcome, IntermediateSplitOutcome, IntermediateSplitResult, ParamList, PreviousRunYieldedAt,
  Subprocess,
};
use crate::hlist_concat::Concat;
use crate::hlist_transform_to::TransformTo;
use crate::step::step::Splitter;
use frunk_core::coproduct::Coproduct;
use serde_value::Value;
use std::future::Future;
use std::marker::PhantomData;

pub trait SplitProcess<SplitterProducesForOtherCases>: Sized {
  type ProcessBeforeSplitProduces: ParamList;
  type SplitterProducesForFirstCase: ParamList;

  fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
  ) -> impl Future<
    Output = IntermediateSplitResult<
      Self::ProcessBeforeSplitProduces,
      Coproduct<Self::SplitterProducesForFirstCase, SplitterProducesForOtherCases>,
    >,
  >;

  fn run(
    &self,
    process_before_split_produces: Self::ProcessBeforeSplitProduces,
  ) -> impl Future<
    Output = IntermediateSplitResult<
      Self::ProcessBeforeSplitProduces,
      Coproduct<Self::SplitterProducesForFirstCase, SplitterProducesForOtherCases>,
    >,
  >;

  fn case<ThisCase: FinalizedProcess, SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices>(
    self,
    create_case: impl FnOnce(Subprocess<Self::ProcessBeforeSplitProduces>) -> ThisCase,
  ) -> FirstCaseOfFinalizedSplitProcess<
    Self::SplitterProducesForFirstCase,
    SplitterProducesForOtherCases,
    Self,
    ThisCase,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
  >
  where
    Self::SplitterProducesForFirstCase: Concat<Self::ProcessBeforeSplitProduces>,
    <Self::SplitterProducesForFirstCase as Concat<Self::ProcessBeforeSplitProduces>>::Concatenated:
      TransformTo<ThisCase::ProcessBeforeProduces, SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices>,
  {
    FirstCaseOfFinalizedSplitProcess {
      split_process_before: self,
      this_case: create_case(subprocess::<Self::ProcessBeforeSplitProduces>()),
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
> {
  pub process_before: ProcessBefore,
  pub splitter: SplitterStep,
  pub split_step_index: usize,
  pub phantom_data: PhantomData<(
    SplitterStepConsumes,
    SplitterProducesForFirstCase,
    SplitterProducesForOtherCases,
    ProcessBeforeProducesToSplitterStepConsumesIndices,
  )>,
}

impl<
    ProcessBefore: FlowingProcess,
    SplitterStepConsumes: ParamList,
    SplitterProducesForFirstCase: ParamList,
    SplitterProducesForOtherCases,
    SplitterStep: Splitter<SplitterStepConsumes, Coproduct<SplitterProducesForFirstCase, SplitterProducesForOtherCases>>,
    ProcessBeforeProducesToSplitterStepConsumesIndices,
  > SplitProcess<SplitterProducesForOtherCases>
  for SplitProcessSplitter<
    ProcessBefore,
    SplitterStepConsumes,
    SplitterProducesForFirstCase,
    SplitterProducesForOtherCases,
    SplitterStep,
    ProcessBeforeProducesToSplitterStepConsumesIndices,
  >
where
  ProcessBefore::Produces: TransformTo<SplitterStepConsumes, ProcessBeforeProducesToSplitterStepConsumesIndices>,
{
  type ProcessBeforeSplitProduces = ProcessBefore::Produces;
  type SplitterProducesForFirstCase = SplitterProducesForFirstCase;

  async fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
  ) -> IntermediateSplitResult<
    Self::ProcessBeforeSplitProduces,
    Coproduct<Self::SplitterProducesForFirstCase, SplitterProducesForOtherCases>,
  > {
    if previous_run_yielded_at.0 < self.split_step_index {
      let process_before_output = self
        .process_before
        .continue_run(previous_run_produced, previous_run_yielded_at, user_input)
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
    Coproduct<Self::SplitterProducesForFirstCase, SplitterProducesForOtherCases>,
  > {
    let splitter_step_consumes: SplitterStepConsumes = process_before_split_produces.clone().transform();
    Ok(IntermediateSplitOutcome::Continue {
      process_before_split_produced: process_before_split_produces,
      passes_to_other_ceses: self.splitter.handle(splitter_step_consumes).await?,
    })
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    let used_index = self.process_before.enumerate_steps(last_used_index);
    self.split_step_index = used_index + 1;
    self.split_step_index
  }
}
