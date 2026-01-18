use crate::builder::{FlowingProcess, FlowingSplitProcess, ParamList};
use crate::hlist_concat::Concat;
use crate::hlist_intersect::Intersect;
use frunk_core::coproduct::{CNil, Coproduct};
use std::marker::PhantomData;

pub struct FlowingCaseOfFlowingSplitProcess<
  ThisTag,
  SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  SplitterPassesToOtherCases,
  ProcessBefore: FlowingSplitProcess<Coproduct<(ThisTag, SplitterProducesForThisCase), SplitterPassesToOtherCases>>,
  ThisCase: FlowingProcess<ProcessBeforeProduces=
  <SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated
  >,
>
// where
//     ThisCase::Produces: Intersect<ProcessBefore::EveryFlowingCaseProduces>,
//     <ThisCase::Produces as Intersect<ProcessBefore::EveryFlowingCaseProduces>>::Intersection: ParamList,
{
  pub split_process_before: ProcessBefore,
  pub this_case: ThisCase,
  pub phantom_data: PhantomData<(
    ThisTag,
    SplitterProducesForThisCase,
    SplitterPassesToOtherCases,
  )>,
}
// todo
/// last case
/// Removing this would forbid having just one case in a split
impl<
  ThisTag,
  SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  ProcessBefore: FlowingSplitProcess<Coproduct<(ThisTag, SplitterProducesForThisCase), CNil>>,
  ThisCase: FlowingProcess<ProcessBeforeProduces=
  <SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated
  >,
> FlowingProcess
for FlowingCaseOfFlowingSplitProcess<
  ThisTag,
  SplitterProducesForThisCase,
  CNil,
  ProcessBefore,
  ThisCase,
>
where
  ThisCase::Produces: Intersect<ProcessBefore::EveryFlowingCaseProduces>,
  <ThisCase::Produces as Intersect<ProcessBefore::EveryFlowingCaseProduces>>::Intersection: ParamList,
{
  type ProcessBeforeProduces = ProcessBefore::ProcessBeforeSplitProduces;
  type Produces = <<ThisCase::Produces as Intersect<ProcessBefore::EveryFlowingCaseProduces>>::Intersection as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated;

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
      IntermediateFinalizedSplitOutcome::Continue {
        process_before_split_produced,
        splitter_passes_to_other_cases,
      } => match splitter_passes_to_other_cases {
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
      IntermediateFinalizedSplitOutcome::Yield(a, b, c) => Ok(IntermediateRunOutcome::Yield(a, b, c)),
      IntermediateFinalizedSplitOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
    }
  }

  async fn run(&self, _process_before_produces: Self::ProcessBeforeProduces) -> IntermediateRunResult<Self::Produces> {
    unsafe { unreachable_unchecked() } // fixme sadge
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    self.split_process_before.enumerate_steps(last_used_index)
  }
}
