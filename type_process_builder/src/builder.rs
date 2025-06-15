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

pub enum IntermediateRunOutcome<Produced: ParamList> {
  Continue(Produced),
  Yield(Message, Value, CurrentRunYieldedAt),
  Finish(Message),
}

pub type IntermediateRunResult<T> = anyhow::Result<IntermediateRunOutcome<T>>;

pub enum IntermediateSplitOutcome<ProcessBeforeSplitProduced: ParamList, ThisCaseProduced: SplitterOutput> {
  Continue {
    process_before_split_produced: ProcessBeforeSplitProduced,
    this_case_produced: ThisCaseProduced,
  },
  Yield(Message, Value, CurrentRunYieldedAt),
  Finish(Message),
}

pub type IntermediateSplitResult<ProcessBeforeSplitProduced: ParamList, ThisCaseProduced: SplitterOutput> =
  anyhow::Result<IntermediateSplitOutcome<ProcessBeforeSplitProduced, ThisCaseProduced>>;

pub enum RunOutcome {
  Yield(Message, Value, CurrentRunYieldedAt),
  Finish(Message),
}

pub type RunResult = anyhow::Result<RunOutcome>;
