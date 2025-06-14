pub mod finalized_process;
pub mod finalized_split_process;
pub mod flowing_process;
pub mod flowing_split_process;
pub mod runnable_process;

use crate::param_list::ParamList;
use crate::step::splitter_output_repr::SplitterOutput;
use crate::step::Message;
use serde_value::Value;

#[derive(PartialEq, Debug, Eq, Clone, PartialOrd, Ord, Hash)]
pub struct PreviousRunYieldedAt(usize);

#[derive(PartialEq, Debug, Eq, Clone, PartialOrd, Ord, Hash)]
pub struct CurrentRunYieldedAt(usize);

pub(crate) const WILL_BE_RENUMBERED: usize = 0;

pub enum IntermediateRunOutcome<T: ParamList> {
  Continue(T),
  Yield(Message, Value, CurrentRunYieldedAt),
  Finish(Message),
}

pub type IntermediateRunResult<T> = anyhow::Result<IntermediateRunOutcome<T>>;

pub enum IntermediateSplitOutcome<T: SplitterOutput> {
  Continue(T),
  Yield(Message, Value, CurrentRunYieldedAt),
  Finish(Message),
}

pub type IntermediateSplitResult<T> = anyhow::Result<IntermediateSplitOutcome<T>>;

pub enum RunOutcome {
  Yield(Message, Value, CurrentRunYieldedAt),
  Finish(Message),
}

pub type RunResult = anyhow::Result<RunOutcome>;
