use crate::builder::{
  subprocess, FinalizedProcess, FlowingProcess, FlowingSplitProcess, IntermediateFinalizedSplitOutcome,
  IntermediateFlowingSplitOutcome, IntermediateFlowingSplitResult, IntermediateRunOutcome,
  NextCaseOfFlowingSplitProcess, ParamList, PreviousRunYieldedAt, SplitProcess, Subprocess,
};
use crate::hlist_concat::Concat;
use crate::hlist_transform_to::TransformTo;
use crate::type_eq::TypeEq;
use frunk_core::coproduct::Coproduct;
use serde_value::Value;
use std::marker::PhantomData;

pub struct FirstCaseOfFlowingSplitProcess<
  ThisTag,
  SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  SplitterPassesToOtherCases,
  ProcessBefore: SplitProcess<SplitterPassesToOtherCases>,
  EveryFlowingCaseProduces: ParamList,
  ThisCase: FlowingProcess,
  SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
  Ix,
  ThisCaseProducesTransformToEveryFlowingCaseProducesIndices,
> {
  pub split_process_before: ProcessBefore,
  pub this_case: ThisCase,
  pub phantom_data: PhantomData<(
    ThisTag,
    SplitterProducesForThisCase,
    SplitterPassesToOtherCases,
    EveryFlowingCaseProduces,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
    Ix,
    ThisCaseProducesTransformToEveryFlowingCaseProducesIndices,
  )>,
}

impl<
    ThisTag,
    NextTag,
    SplitterPassesToOtherCases,
    ProcessBefore: SplitProcess<
      Coproduct<(NextTag, SplitterProducesForNextCase), SplitterPassesToOtherCases>,
      SplitterProducesForFirstCase = SplitterProducesForThisCase,
      SplitterTagForFirstCase = ThisTag,
    >,
    SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
    SplitterProducesForNextCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
    EveryFlowingCaseProduces: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
    ThisCase: FlowingProcess,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
    Ix,
    ThisCaseProducesTransformToEveryFlowingCaseProducesIndices,
  >
  FirstCaseOfFlowingSplitProcess<
    ThisTag,
    SplitterProducesForThisCase,
    Coproduct<(NextTag, SplitterProducesForNextCase), SplitterPassesToOtherCases>,
    ProcessBefore,
    EveryFlowingCaseProduces,
    ThisCase,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
    Ix,
    ThisCaseProducesTransformToEveryFlowingCaseProducesIndices,
  >
where
  <SplitterProducesForThisCase as Concat<<ProcessBefore>::ProcessBeforeSplitProduces>>::Concatenated:
    TransformTo<ThisCase::ProcessBeforeProduces, SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices>,
{
  pub fn case<
    AssumedTag,
    NextCase: FinalizedProcess,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndicesA,
  >(
    self,
    create_case: impl FnOnce(Subprocess<ProcessBefore::ProcessBeforeSplitProduces>) -> NextCase,
  ) -> NextCaseOfFlowingSplitProcess<
    NextTag,
    SplitterProducesForNextCase,
    SplitterPassesToOtherCases,
    Self,
    EveryFlowingCaseProduces,
    NextCase,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndicesA,
  >
  where
    (AssumedTag, NextTag): TypeEq,
    <SplitterProducesForNextCase as Concat<<ProcessBefore>::ProcessBeforeSplitProduces>>::Concatenated:
      TransformTo<NextCase::ProcessBeforeProduces, SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndicesA>,
    ThisCase::Produces:
      TransformTo<EveryFlowingCaseProduces, ThisCaseProducesTransformToEveryFlowingCaseProducesIndices>,
  {
    NextCaseOfFlowingSplitProcess {
      split_process_before: self,
      this_case: create_case(subprocess::<ProcessBefore::ProcessBeforeSplitProduces>()),
      phantom_data: Default::default(),
    }
  }
}

impl<
    ThisTag,
    SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
    SplitterPassesToOtherCases,
    ProcessBefore: SplitProcess<SplitterPassesToOtherCases, SplitterProducesForFirstCase = SplitterProducesForThisCase>,
    EveryFlowingCaseProduces: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
    ThisCase: FlowingProcess,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
    Ix,
    ThisCaseProducesTransformToEveryFlowingCaseProducesIndices,
  > FlowingSplitProcess<SplitterPassesToOtherCases>
  for FirstCaseOfFlowingSplitProcess<
    ThisTag,
    SplitterProducesForThisCase,
    SplitterPassesToOtherCases,
    ProcessBefore,
    EveryFlowingCaseProduces,
    ThisCase,
    SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
    Ix,
    ThisCaseProducesTransformToEveryFlowingCaseProducesIndices,
  >
where
  <SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated:
    TransformTo<ThisCase::ProcessBeforeProduces, SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices>,
  ThisCase::Produces: TransformTo<EveryFlowingCaseProduces, ThisCaseProducesTransformToEveryFlowingCaseProducesIndices>,
{
  type EveryFlowingCaseProduces = EveryFlowingCaseProduces;
  type ProcessBeforeSplitProduces = ProcessBefore::ProcessBeforeSplitProduces;
  type SplitterProducesForThisCase = SplitterProducesForThisCase;
  type SplitterTagForThisCase = ThisTag;

  async fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
  ) -> IntermediateFlowingSplitResult<
    ProcessBefore::ProcessBeforeSplitProduces,
    SplitterPassesToOtherCases,
    EveryFlowingCaseProduces,
  > {
    let process_before_output = self
      .split_process_before
      .continue_run(previous_run_produced, previous_run_yielded_at, user_input)
      .await?;
    match process_before_output {
      IntermediateFinalizedSplitOutcome::GoToCase {
        process_before_split_produced,
        splitter_passes_to_other_cases: this_case_produced,
      } => self.run(process_before_split_produced, this_case_produced).await,
      IntermediateFinalizedSplitOutcome::Yield(a, b, c) => Ok(IntermediateFlowingSplitOutcome::Yield(a, b, c)),
      IntermediateFinalizedSplitOutcome::Finish(a) => Ok(IntermediateFlowingSplitOutcome::Finish(a)),
    }
  }

  async fn run(
    &self,
    process_before_split_produced: Self::ProcessBeforeSplitProduces,
    splitter_produces_for_this_case_or_other_cases_consumes: Coproduct<
      Self::SplitterProducesForThisCase,
      SplitterPassesToOtherCases,
    >,
  ) -> IntermediateFlowingSplitResult<
    ProcessBefore::ProcessBeforeSplitProduces,
    SplitterPassesToOtherCases,
    EveryFlowingCaseProduces,
  > {
    match splitter_produces_for_this_case_or_other_cases_consumes {
      Coproduct::Inl(splitter_produces_for_this_case) => {
        let next_case_consumes: ThisCase::ProcessBeforeProduces = splitter_produces_for_this_case
          .concat(process_before_split_produced.clone())
          .transform();
        match self.this_case.run(next_case_consumes).await? {
          IntermediateRunOutcome::Continue(produces) => Ok(IntermediateFlowingSplitOutcome::Continue {
            process_before_split_produced,
            flowing_case_produced: produces.transform(),
          }),
          IntermediateRunOutcome::Yield(a, b, c) => Ok(IntermediateFlowingSplitOutcome::Yield(a, b, c)),
          IntermediateRunOutcome::Finish(a) => Ok(IntermediateFlowingSplitOutcome::Finish(a)),
        }
      }
      Coproduct::Inr(splitter_passes_to_other_cases) => Ok(IntermediateFlowingSplitOutcome::GoToCase {
        process_before_split_produced,
        splitter_passes_to_other_cases,
      }),
    }
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    self.split_process_before.enumerate_steps(last_used_index)
  }
}

// /// last case
// /// Removing this would forbid having just one case in a split
// impl<
//     ThisTag,
//     ProcessBefore: SplitProcess<CNil>,
//     EveryFlowingCaseProduces: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
//     ThisCase: FlowingProcess,
//     SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
//     Ix,
//   > FlowingProcess
//   for FirstCaseOfFlowingSplitProcess<
//     ThisTag,
//     ProcessBefore::SplitterProducesForFirstCase,
//     CNil,
//     ProcessBefore,
//     EveryFlowingCaseProduces,
//     ThisCase,
//     SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices,
//     Ix,
//   >
// where
//   ProcessBefore::SplitterProducesForFirstCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
//   <ProcessBefore::SplitterProducesForFirstCase as Concat<<ProcessBefore>::ProcessBeforeSplitProduces>>::Concatenated:
//     TransformTo<ThisCase::ProcessBeforeProduces, SplitterStepProducesWithProcessBeforeProducesToCaseConsumesIndices>,
//   <ThisCase as FlowingProcess>::Produces: TransformTo<EveryFlowingCaseProduces, Ix>,
// {
//   type ProcessBeforeProduces = ProcessBefore::ProcessBeforeSplitProduces;
//   type Produces = <EveryFlowingCaseProduces as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated;
//
//   async fn continue_run(
//     &self,
//     previous_run_produced: Value,
//     previous_run_yielded_at: PreviousRunYieldedAt,
//     user_input: String,
//   ) -> IntermediateRunResult<Self::Produces> {
//     let process_before_output = self
//       .split_process_before
//       .continue_run(previous_run_produced, previous_run_yielded_at, user_input)
//       .await?;
//     match process_before_output {
//       IntermediateFinalizedSplitOutcome::Continue {
//         process_before_split_produced,
//         splitter_passes_to_other_cases,
//       } => match splitter_passes_to_other_cases {
//         Coproduct::Inl(passes_to_this_case) => {
//           let this_case_consumes: <ThisCase as FlowingProcess>::ProcessBeforeProduces = passes_to_this_case
//             .concat(process_before_split_produced.clone())
//             .transform();
//           match self.this_case.run(this_case_consumes).await? {
//             IntermediateRunOutcome::Continue(produced) => {
//               let every_flowing_case_produces: EveryFlowingCaseProduces = produced.transform();
//               Ok(IntermediateRunOutcome::Continue(
//                 every_flowing_case_produces.concat(process_before_split_produced),
//               ))
//             }
//             IntermediateRunOutcome::Yield(a, b, c) => Ok(IntermediateRunOutcome::Yield(a, b, c)),
//             IntermediateRunOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
//           }
//         }
//         Coproduct::Inr(c_nil) => match c_nil {},
//       },
//       IntermediateFinalizedSplitOutcome::Yield(a, b, c) => Ok(IntermediateRunOutcome::Yield(a, b, c)),
//       IntermediateFinalizedSplitOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
//     }
//   }
//
//   async fn run(&self, _process_before_produces: Self::ProcessBeforeProduces) -> IntermediateRunResult<Self::Produces> {
//     unsafe { unreachable_unchecked() } // fixme sadge
//   }
//
//   fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
//     self.split_process_before.enumerate_steps(last_used_index)
//   }
// }
//
