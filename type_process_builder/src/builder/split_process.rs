use crate::builder::first_case_of_flowing_split_process::FirstCaseOfFlowingSplitProcess;
use crate::builder::{
  subprocess, FinalizedProcess, FirstCaseOfFinalizedSplitProcess, FlowingProcess, IntermediateFinalizedSplitOutcome,
  IntermediateFinalizedSplitResult, IntermediateRunOutcome, ParamList, PreviousRunYieldedAt, Subprocess,
};
use crate::hlist_concat::Concat;
use crate::hlist_transform_to::TransformTo;
use crate::step::step::Splitter;
use crate::type_eq::TypeEq;
use frunk_core::coproduct::Coproduct;
use serde_value::Value;
use std::future::Future;
use std::marker::PhantomData;

/// We enforce at least one cases in the split.
/// We could remove an unnecessary option of implementing a linear proces with a series of splits with single case,
/// but sometimes the need arises to have a select form with one option that is different from input form.
/// We at least remove an illegal state of unfinalized finalized split process.
pub trait SplitProcess<SplitterProducesForOtherCases>: Sized {
  type ProcessBeforeSplitProduces: ParamList;
  type SplitterProducesForFirstCase: ParamList;
  type SplitterTagForFirstCase;

  fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
  ) -> impl Future<
    Output = IntermediateFinalizedSplitResult<
      Self::ProcessBeforeSplitProduces,
      Coproduct<Self::SplitterProducesForFirstCase, SplitterProducesForOtherCases>,
    >,
  >;

  fn run(
    &self,
    process_before_split_produced: Self::ProcessBeforeSplitProduces,
  ) -> impl Future<
    Output = IntermediateFinalizedSplitResult<
      Self::ProcessBeforeSplitProduces,
      Coproduct<Self::SplitterProducesForFirstCase, SplitterProducesForOtherCases>,
    >,
  >;

  fn case<AssumedTag, ThisCase: FinalizedProcess>(
    self,
    create_case: impl FnOnce(
      Subprocess<<Self::SplitterProducesForFirstCase as Concat<Self::ProcessBeforeSplitProduces>>::Concatenated>,
    ) -> ThisCase,
  ) -> FirstCaseOfFinalizedSplitProcess<
    Self::SplitterTagForFirstCase,
    Self::SplitterProducesForFirstCase,
    SplitterProducesForOtherCases,
    Self,
    ThisCase,
  >
  where
    (AssumedTag, Self::SplitterTagForFirstCase): TypeEq,
    Self::SplitterProducesForFirstCase: Concat<Self::ProcessBeforeSplitProduces>,
  {
    FirstCaseOfFinalizedSplitProcess {
      split_process_before: self,
      this_case: create_case(subprocess::<
        <Self::SplitterProducesForFirstCase as Concat<Self::ProcessBeforeSplitProduces>>::Concatenated,
      >()),
      phantom_data: Default::default(),
    }
  }

  fn case_flowing<
    AssumedTag,
    EveryFlowingCaseProduces: ParamList,
    ThisCase: FlowingProcess<ProcessBeforeProduces=<Self::SplitterProducesForFirstCase as
    Concat<Self::ProcessBeforeSplitProduces>>::Concatenated>,
    ThisCaseProducesTransformToEveryFlowingCaseProducesIndices,
  >(
    self,
    create_case: impl FnOnce(
      Subprocess<<Self::SplitterProducesForFirstCase as Concat<Self::ProcessBeforeSplitProduces>>::Concatenated>,
    ) -> ThisCase,
  ) -> FirstCaseOfFlowingSplitProcess<
    Self::SplitterTagForFirstCase,
    Self::SplitterProducesForFirstCase,
    SplitterProducesForOtherCases,
    Self,
    EveryFlowingCaseProduces,
    ThisCase,
    ThisCaseProducesTransformToEveryFlowingCaseProducesIndices,
  >
  where
    (AssumedTag, Self::SplitterTagForFirstCase): TypeEq,
    Self::SplitterProducesForFirstCase: Concat<Self::ProcessBeforeSplitProduces>,
  {
    FirstCaseOfFlowingSplitProcess {
      split_process_before: self,
      this_case: create_case(subprocess::<
        <Self::SplitterProducesForFirstCase as Concat<Self::ProcessBeforeSplitProduces>>::Concatenated,
      >()),
      phantom_data: Default::default(),
    }
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize;
}

pub struct SplitProcessSplitter<
  Tag,
  ProcessBefore: FlowingProcess,
  SplitterStepConsumes: ParamList,
  SplitterProducesForFirstCase: ParamList,
  SplitterProducesForOtherCases,
  SplitterStep: Splitter<SplitterStepConsumes, Coproduct<(Tag, SplitterProducesForFirstCase), SplitterProducesForOtherCases>>,
  ProcessBeforeProducesToSplitterStepConsumesIndices,
> {
  pub process_before: ProcessBefore,
  pub splitter: SplitterStep,
  pub split_step_index: usize,
  pub phantom_data: PhantomData<(
    Tag,
    SplitterStepConsumes,
    SplitterProducesForFirstCase,
    SplitterProducesForOtherCases,
    ProcessBeforeProducesToSplitterStepConsumesIndices,
  )>,
}

impl<
    Tag,
    ProcessBefore: FlowingProcess,
    SplitterStepConsumes: ParamList,
    SplitterProducesForFirstCase: ParamList,
    SplitterProducesForOtherCases,
    SplitterStep: Splitter<SplitterStepConsumes, Coproduct<(Tag, SplitterProducesForFirstCase), SplitterProducesForOtherCases>>,
    ProcessBeforeProducesToSplitterStepConsumesIndices,
  > SplitProcess<SplitterProducesForOtherCases>
  for SplitProcessSplitter<
    Tag,
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
  type SplitterTagForFirstCase = Tag;

  async fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
  ) -> IntermediateFinalizedSplitResult<
    Self::ProcessBeforeSplitProduces,
    Coproduct<Self::SplitterProducesForFirstCase, SplitterProducesForOtherCases>,
  > {
    if previous_run_yielded_at.0 < self.split_step_index {
      let process_before_output = self
        .process_before
        .continue_run(previous_run_produced, previous_run_yielded_at, user_input)
        .await?;
      match process_before_output {
        IntermediateRunOutcome::Continue(process_before_split_produced) => {
          self.run(process_before_split_produced).await
        }
        IntermediateRunOutcome::Yield(a, b, c) => Ok(IntermediateFinalizedSplitOutcome::Yield(a, b, c)),
        IntermediateRunOutcome::Finish(a) => Ok(IntermediateFinalizedSplitOutcome::Finish(a)),
      }
    } else {
      let process_before_split_produced = ProcessBefore::Produces::deserialize(previous_run_produced)?;
      self.run(process_before_split_produced).await
    }
  }

  async fn run(
    &self,
    process_before_split_produced: Self::ProcessBeforeSplitProduces,
  ) -> IntermediateFinalizedSplitResult<
    Self::ProcessBeforeSplitProduces,
    Coproduct<Self::SplitterProducesForFirstCase, SplitterProducesForOtherCases>,
  > {
    let splitter_step_consumes: SplitterStepConsumes = process_before_split_produced.clone().transform();
    let splitter_passes_to_other_cases = match self.splitter.handle(splitter_step_consumes).await? {
      Coproduct::Inl(a) => Coproduct::Inl(a.1),
      Coproduct::Inr(b) => Coproduct::Inr(b),
    };
    Ok(IntermediateFinalizedSplitOutcome::GoToCase {
      process_before_split_produced,
      splitter_passes_to_other_cases,
    })
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    let used_index = self.process_before.enumerate_steps(last_used_index);
    self.split_step_index = used_index + 1;
    self.split_step_index
  }
}
