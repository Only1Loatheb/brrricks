use crate::builder::{
  FinalizedProcess, FlowingProcess, FlowingSplitProcess, IntermediateFlowingSplitOutcome, IntermediateRunOutcome,
  IntermediateRunResult, ParamList, PreviousRunYieldedAt, RunOutcome,
};
use crate::hlist_concat::Concat;
use frunk_core::coproduct::{CNil, Coproduct};
use serde_value::Value;
use std::marker::PhantomData;

pub struct FinalizedCaseOfFlowingSplitProcess<
  ThisTag,
  SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  SplitterPassesToOtherCases,
  ProcessBefore: FlowingSplitProcess<Coproduct<(ThisTag, SplitterProducesForThisCase), SplitterPassesToOtherCases>>,
  ThisCase: FinalizedProcess<ProcessBeforeProduces=<SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated>,
> {
  pub split_process_before: ProcessBefore,
  pub this_case: ThisCase,
  pub phantom_data: PhantomData<(
    ThisTag,
    SplitterProducesForThisCase,
    SplitterPassesToOtherCases,
  )>,
}

/// the last case
impl<
  ThisTag,
  SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  ProcessBefore: FlowingSplitProcess<Coproduct<(ThisTag, SplitterProducesForThisCase), CNil>>,
  ThisCase: FinalizedProcess<ProcessBeforeProduces=<SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated>,
> FlowingProcess
for FinalizedCaseOfFlowingSplitProcess<ThisTag, SplitterProducesForThisCase, CNil, ProcessBefore, ThisCase>
where
// ProcessBefore::SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
// <SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
{
  type ProcessBeforeProduces = <SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated;
  type Produces = ProcessBefore::EveryFlowingCaseProduces;

  async fn resume_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
  ) -> IntermediateRunResult<Self::Produces> {
    let process_before_output = self
      .split_process_before
      .resume_run(previous_run_produced, previous_run_yielded_at, user_input)
      .await?;
    match process_before_output {
      IntermediateFlowingSplitOutcome::GoToCase {
        process_before_split_produced,
        splitter_passes_to_other_cases,
      } => match splitter_passes_to_other_cases {
        Coproduct::Inl((_pd, passes_to_this_case)) => {
          let this_case_consumes = passes_to_this_case.concat(process_before_split_produced);
          match self.this_case.continue_run(this_case_consumes).await? {
            RunOutcome::Yield(a, b, c) => Ok(IntermediateRunOutcome::Yield(a, b, c)),
            RunOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
          }
        }
        Coproduct::Inr(c_nil) => match c_nil {},
      },
      IntermediateFlowingSplitOutcome::Yield(a, b, c) => Ok(IntermediateRunOutcome::Yield(a, b, c)),
      IntermediateFlowingSplitOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
      IntermediateFlowingSplitOutcome::Continue { flowing_case_produced } =>
        Ok(IntermediateRunOutcome::Continue(flowing_case_produced)),
    }
  }

  async fn continue_run(&self, this_case_consumes: Self::ProcessBeforeProduces) -> IntermediateRunResult<Self::Produces> {
    match self.this_case.continue_run(this_case_consumes).await? {
      RunOutcome::Yield(a, b, c) => Ok(IntermediateRunOutcome::Yield(a, b, c)),
      RunOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
    }
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    self.split_process_before.enumerate_steps(last_used_index)
  }
}

// impl<
//     ThisTag,
//     NextTag,
//     SplitterPassesToOtherCases,
//     ProcessBeforeProcessBefore: FlowingSplitProcess<
//       Coproduct<
//         (ThisTag, SplitterProducesForThisCase),
//         Coproduct<(NextTag, SplitterProducesForNextCase), SplitterPassesToOtherCases>,
//       >,
//     >,
//     SplitterProducesForThisCase: ParamList + Concat<ProcessBeforeProcessBefore::ProcessBeforeSplitProduces>,
//     SplitterProducesForNextCase: ParamList + Concat<ProcessBeforeProcessBefore::ProcessBeforeSplitProduces>,
//     ThisCase: FinalizedProcess,
//     SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
//   >
//   FinalizedCaseOfFlowingSplitProcess<
//     ThisTag,
//     SplitterProducesForThisCase,
//     Coproduct<(NextTag, SplitterProducesForNextCase), SplitterPassesToOtherCases>,
//     ProcessBeforeProcessBefore,
//     ThisCase,
//     SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
//   >
// where
//   <SplitterProducesForThisCase as Concat<ProcessBeforeProcessBefore::ProcessBeforeSplitProduces>>::Concatenated:
//     TransformTo<ThisCase::ProcessBeforeProduces, SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices>,
// {
//   // fixme create_case should accept
//   // fixme Subprocess<<Self::SplitterProducesForFirstCase as Concat<Self::ProcessBeforeSplitProduces>>::Concatenated>
//   pub fn case<
//     NextCase: FinalizedProcess,
//     SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndicesA,
//   >(
//     self,
//     _assumed_tag: NextTag,
//     create_case: impl FnOnce(Subprocess<ProcessBeforeProcessBefore::ProcessBeforeSplitProduces>) -> NextCase,
//   ) -> FinalizedCaseOfFlowingSplitProcess<
//     NextTag,
//     SplitterProducesForNextCase,
//     SplitterPassesToOtherCases,
//     Self,
//     NextCase,
//     SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndicesA,
//   >
//     <SplitterProducesForNextCase as Concat<ProcessBeforeProcessBefore::ProcessBeforeSplitProduces>>::Concatenated:
//       TransformTo<NextCase::ProcessBeforeProduces, SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndicesA>,
//   {
//     FinalizedCaseOfFlowingSplitProcess {
//       split_process_before: self,
//       this_case: create_case(subprocess::<ProcessBeforeProcessBefore::ProcessBeforeSplitProduces>()),
//       phantom_data: Default::default(),
//     }
//   }
// }

// impl<
//     ThisTag,
//     SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
//     SplitterProducesForOtherCases,
//     ProcessBefore: FlowingSplitProcess<Coproduct<(ThisTag, SplitterProducesForThisCase), SplitterProducesForOtherCases>>,
//     ThisCase: FinalizedProcess,
//     SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
//   > FlowingSplitProcess<SplitterProducesForOtherCases>
//   for FinalizedCaseOfFlowingSplitProcess<
//     ThisTag,
//     SplitterProducesForThisCase,
//     SplitterProducesForOtherCases,
//     ProcessBefore,
//     ThisCase,
//     SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
//   >
// where
//   <SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated:
//     TransformTo<ThisCase::ProcessBeforeProduces, SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices>,
// {
//   type ProcessBeforeSplitProduces = ProcessBefore::ProcessBeforeSplitProduces;
//   type SplitterProducesForThisCase = SplitterProducesForThisCase;
//   type SplitterTagForThisCase = ThisTag;
//
//   async fn resume_run(
//     &self,
//     previous_run_produced: Value,
//     previous_run_yielded_at: PreviousRunYieldedAt,
//     user_input: String,
//   ) -> IntermediateFinalizedSplitResult<Self::ProcessBeforeSplitProduces, SplitterProducesForOtherCases> {
//     let process_before_output = self
//       .split_process_before
//       .resume_run(previous_run_produced, previous_run_yielded_at, user_input)
//       .await?;
//     match process_before_output {
//       IntermediateFinalizedSplitOutcome::Continue {
//         process_before_split_produced,
//         splitter_passes_to_other_cases: this_case_produced,
//       } => {
//         let produced = match this_case_produced {
//           Coproduct::Inl((_pd, params)) => Coproduct::Inl(params),
//           Coproduct::Inr(inr_value) => Coproduct::Inr(inr_value),
//         };
//         self.continue_run(process_before_split_produced, produced).await
//       }
//       IntermediateFinalizedSplitOutcome::Yield(a, b, c) => Ok(IntermediateFinalizedSplitOutcome::Yield(a, b, c)),
//       IntermediateFinalizedSplitOutcome::Finish(a) => Ok(IntermediateFinalizedSplitOutcome::Finish(a)),
//     }
//   }
//
//   async fn continue_run(
//     &self,
//     process_before_split_produced: Self::ProcessBeforeSplitProduces,
//     splitter_produces_for_this_case_or_other_cases_consumes: Coproduct<Self::SplitterProducesForThisCase, SplitterProducesForOtherCases>,
//   ) -> IntermediateFinalizedSplitResult<Self::ProcessBeforeSplitProduces, SplitterProducesForOtherCases> {
//     match splitter_produces_for_this_case_or_other_cases_consumes {
//       Coproduct::Inl(this_case_consumes) => {
//         let next_case_consumes: ThisCase::ProcessBeforeProduces =
//           this_case_consumes.concat(process_before_split_produced).transform();
//         match self.this_case.continue_run(next_case_consumes).await? {
//           RunOutcome::Yield(a, b, c) => Ok(IntermediateFinalizedSplitOutcome::Yield(a, b, c)),
//           RunOutcome::Finish(a) => Ok(IntermediateFinalizedSplitOutcome::Finish(a)),
//         }
//       }
//       Coproduct::Inr(other_cases_consumes) => Ok(IntermediateFinalizedSplitOutcome::Continue {
//         process_before_split_produced: process_before_split_produced,
//         splitter_passes_to_other_cases: other_cases_consumes,
//       }),
//     }
//   }
//
//   fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
//     self.split_process_before.enumerate_steps(last_used_index)
//   }
// }
