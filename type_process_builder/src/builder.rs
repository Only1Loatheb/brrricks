pub mod finalized_process;
pub mod finalized_split_process;
pub mod flowing_process;
pub mod flowing_split_process;
pub mod runnable_process;
mod split_process;

pub use crate::param_list::*;
pub use crate::step::splitter_output_repr::*;
pub use crate::step::Message;
pub use finalized_process::*;
pub use finalized_split_process::first_case_of_finalized_split_process::*;
pub use finalized_split_process::next_case_of_finalized_split_process::*;
pub use finalized_split_process::*;
pub use flowing_process::*;
pub use flowing_split_process::finalized_case_of_flowing_split_process::*;
pub use flowing_split_process::first_case_of_flowing_split_process::*;
pub use flowing_split_process::*;
pub use runnable_process::*;
use serde_value::Value;
pub use split_process::*;

#[derive(PartialEq, Debug, Eq, Clone, PartialOrd, Ord, Hash)]
pub struct PreviousRunYieldedAt(pub usize);

#[derive(PartialEq, Debug, Eq, Clone, PartialOrd, Ord, Hash)]
pub struct CurrentRunYieldedAt(usize);

pub(crate) const WILL_BE_RENUMBERED: usize = 0;

pub enum IntermediateRunOutcome<Produced: ParamList> {
  Continue(Produced),
  Yield(Message, Value, CurrentRunYieldedAt),
  Finish(Message),
}

pub type IntermediateRunResult<T> = anyhow::Result<IntermediateRunOutcome<T>>;

pub enum IntermediateFinalizedSplitOutcome<ProcessBeforeSplitProduced: ParamList, SplitterPassesToOtherCases> {
  GoToCase {
    process_before_split_produced: ProcessBeforeSplitProduced,
    splitter_passes_to_other_cases: SplitterPassesToOtherCases,
  },
  Yield(Message, Value, CurrentRunYieldedAt),
  Finish(Message),
}

pub type IntermediateFinalizedSplitResult<ProcessBeforeSplitProduced, SplitterPassesToOtherCases> =
  anyhow::Result<IntermediateFinalizedSplitOutcome<ProcessBeforeSplitProduced, SplitterPassesToOtherCases>>;

pub enum RunOutcome {
  Yield(Message, Value, CurrentRunYieldedAt),
  Finish(Message),
}

pub type RunResult = anyhow::Result<RunOutcome>;
