pub mod finalized_case_of_flowing_split_process;
pub mod first_case_of_flowing_split_process;

use crate::builder::{IntermediateFinalizedSplitResult, PreviousRunYieldedAt};
use crate::hlist_concat::Concat;
use crate::param_list::ParamList;
use crate::step::step::Splitter;
use frunk_core::coproduct::Coproduct;
use serde_value::Value;
use std::future::Future;

///
/// Should we force the user to produce common params before the [Splitter] step?
/// If we allow that the user can produce common params in [Splitter] without defining additional step.
/// The process builder API will be more ergonomic, but the implementation will be more involved.
pub trait FlowingSplitProcess<SplitterProducesForOtherCases>: Sized {
  type EveryFlowingCaseProduces: ParamList + Concat<Self::ProcessBeforeSplitProduces>;
  type ProcessBeforeSplitProduces: ParamList;
  type SplitterProducesForThisCase: ParamList + Concat<Self::ProcessBeforeSplitProduces>;
  type SplitterTagForThisCase;

  fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
  ) -> impl Future<
    Output = IntermediateFinalizedSplitResult<
      <Self::EveryFlowingCaseProduces as Concat<Self::ProcessBeforeSplitProduces>>::Concatenated,
      SplitterProducesForOtherCases,
    >,
  >;

  fn run(
    &self,
    process_before_split_produced: Self::ProcessBeforeSplitProduces,
    splitter_produces_for_this_case_or_other_cases_consumes: Coproduct<
      Self::SplitterProducesForThisCase,
      SplitterProducesForOtherCases,
    >,
  ) -> impl Future<
    Output = IntermediateFinalizedSplitResult<
      <Self::EveryFlowingCaseProduces as Concat<Self::ProcessBeforeSplitProduces>>::Concatenated,
      SplitterProducesForOtherCases,
    >,
  >;

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize;
}
