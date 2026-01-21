use crate::builder::{
  FlowingProcess, FlowingSplitProcess, IntermediateFlowingSplitOutcome, IntermediateRunOutcome, IntermediateRunResult,
  ParamList, PreviousRunYieldedAt,
};
use crate::hlist_concat::Concat;
use crate::hlist_intersect::Intersect;
use crate::hlist_transform_to::TransformTo;
use frunk_core::coproduct::{CNil, Coproduct};
use serde_value::Value;
use std::marker::PhantomData;

pub struct FlowingCaseOfFlowingSplitProcess<
  ThisTag,
  SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  SplitterPassesToOtherCases,
  ProcessBefore: FlowingSplitProcess<Coproduct<(ThisTag, SplitterProducesForThisCase), SplitterPassesToOtherCases>>,
  ThisCase: FlowingProcess<ProcessBeforeProduces=
  <SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated
  >,
  Indices,
>
{
  pub split_process_before: ProcessBefore,
  pub this_case: ThisCase,
  pub phantom_data: PhantomData<(
    ThisTag,
    SplitterProducesForThisCase,
    SplitterPassesToOtherCases,
    Indices,
  )>,
}

/// last case
impl<
  ThisTag,
  SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  ProcessBefore: FlowingSplitProcess<Coproduct<(ThisTag, SplitterProducesForThisCase), CNil>>,
  ThisCase: FlowingProcess<ProcessBeforeProduces=
  <SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
  >,
  Indices,
> FlowingProcess
for FlowingCaseOfFlowingSplitProcess<
  ThisTag,
  SplitterProducesForThisCase,
  CNil,
  ProcessBefore,
  ThisCase,
  Indices,
>
where
  ProcessBefore::EveryFlowingCaseProduces: Intersect<ThisCase::Produces>,
  <ProcessBefore::EveryFlowingCaseProduces as Intersect<ThisCase::Produces>>::Intersection: ParamList,
  ThisCase::Produces: TransformTo<<ProcessBefore::EveryFlowingCaseProduces as Intersect<ThisCase::Produces>>::Intersection, Indices>,
{
  type ProcessBeforeProduces = ProcessBefore::ProcessBeforeSplitProduces;
  type Produces = <ProcessBefore::EveryFlowingCaseProduces as Intersect<ThisCase::Produces>>::Intersection;

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
      IntermediateFlowingSplitOutcome::GoToCase {
        process_before_split_produced,
        splitter_passes_to_other_cases,
      } => match splitter_passes_to_other_cases {
        Coproduct::Inl((_pd, passes_to_this_case)) => {
          let this_case_consumes = passes_to_this_case.concat(process_before_split_produced.clone());
          match self.this_case.run(this_case_consumes).await? {
            IntermediateRunOutcome::Continue(this_case_produced) =>
              Ok(IntermediateRunOutcome::Continue(this_case_produced.transform())),
            IntermediateRunOutcome::Yield(a, b, c) => Ok(IntermediateRunOutcome::Yield(a, b, c)),
            IntermediateRunOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
          }
        }
        Coproduct::Inr(c_nil) => match c_nil {},
      },
      IntermediateFlowingSplitOutcome::Yield(a, b, c) => Ok(IntermediateRunOutcome::Yield(a, b, c)),
      IntermediateFlowingSplitOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
      IntermediateFlowingSplitOutcome::Continue { flowing_case_produced } =>
        Ok(IntermediateRunOutcome::Continue(flowing_case_produced.intersect())),
    }
  }

  async fn run(&self, _process_before_produces: Self::ProcessBeforeProduces) -> IntermediateRunResult<Self::Produces> {
    todo!()
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    self.split_process_before.enumerate_steps(last_used_index)
  }
}
