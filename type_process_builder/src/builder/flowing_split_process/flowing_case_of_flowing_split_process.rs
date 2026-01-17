use crate::builder::{FlowingProcess, FlowingSplitProcess, ParamList};
use crate::hlist_concat::Concat;
// use crate::hlist_intersect::Intersect;
use frunk_core::coproduct::Coproduct;
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
