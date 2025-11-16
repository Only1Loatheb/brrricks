pub mod finalized_case_of_flowing_split_process;
pub mod first_case_of_flowing_split_process;

use crate::builder::{IntermediateSplitResult, PreviousRunYieldedAt};
use crate::hlist_concat::Concat;
use crate::param_list::ParamList;
use frunk_core::coproduct::Coproduct;
use serde_value::Value;
use std::future::Future;

pub trait FlowingSplitProcess<SplitterProducesForOtherCases>: Sized {
  type EveryFlowingCaseProduces: ParamList;
  type ProcessBeforeSplitProduces: ParamList;
  type SplitterProducesForThisCase: ParamList + Concat<Self::ProcessBeforeSplitProduces>;
  type SplitterTagForThisCase;

  fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
  ) -> impl Future<
    Output = IntermediateSplitResult<
      <Self::EveryFlowingCaseProduces as Concat<Self::ProcessBeforeSplitProduces>>::Concatenated,
      SplitterProducesForOtherCases,
    >,
  >;

  fn run(
    &self,
    process_before_split_produced: Self::ProcessBeforeSplitProduces,
    this_case_or_other_cases_consumes: Coproduct<Self::SplitterProducesForThisCase, SplitterProducesForOtherCases>,
  ) -> impl Future<
    Output = IntermediateSplitResult<
      <Self::EveryFlowingCaseProduces as Concat<Self::ProcessBeforeSplitProduces>>::Concatenated,
      SplitterProducesForOtherCases,
    >,
  >;

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize;
}
