pub mod finalized_process;
pub mod finalized_split_process;
pub mod flowing_process;
pub mod flowing_split_process;
pub mod runnable_process;
pub mod split_process;

pub use crate::param_list::*;
pub use crate::step::Message;
pub use finalized_process::*;
pub use finalized_split_process::first_case_of_finalized_split_process::*;
pub use finalized_split_process::next_case_of_finalized_split_process::*;
pub use finalized_split_process::*;
pub use flowing_process::*;
pub use flowing_split_process::finalized_case_of_flowing_split_process::*;
pub use flowing_split_process::first_case_of_flowing_split_process::*;
pub use flowing_split_process::flowing_case_of_finalized_split_process::*;
pub use flowing_split_process::flowing_case_of_flowing_split_process::*;
pub use flowing_split_process::*;
pub use runnable_process::*;
use serde_value::Value;
pub use split_process::*;

type StepIndex = u32;

#[derive(PartialEq, Debug, Eq, Clone, PartialOrd, Ord, Hash)]
pub struct PreviousRunYieldedAt(pub StepIndex);

#[derive(PartialEq, Debug, Eq, Clone, PartialOrd, Ord, Hash)]
pub struct CurrentRunYieldedAt(pub StepIndex);

pub(crate) const WILL_BE_RENUMBERED: StepIndex = 0;

pub(crate) type SessionContext = Vec<(u32, Value)>;

#[derive(Debug, PartialEq, Eq)]
pub enum IntermediateRunOutcome<Produced: ParamList> {
  Continue(Produced),
  Yield(Message, SessionContext, CurrentRunYieldedAt),
  Finish(Message),
  RetryUserInput(Message),
}

pub type IntermediateRunResult<T> = anyhow::Result<IntermediateRunOutcome<T>>;

#[derive(Debug, PartialEq, Eq)]
pub enum IntermediateFinalizedSplitOutcome<ProcessBeforeSplitProduced: ParamList, SplitterProducesForOtherCases> {
  GoToCase {
    process_before_split_produced: ProcessBeforeSplitProduced,
    splitter_produces_to_other_cases: SplitterProducesForOtherCases,
  },
  Yield(Message, SessionContext, CurrentRunYieldedAt),
  Finish(Message),
  RetryUserInput(Message),
}

pub type IntermediateFinalizedSplitResult<ProcessBeforeSplitProduced, SplitterProducesForOtherCases> =
  anyhow::Result<IntermediateFinalizedSplitOutcome<ProcessBeforeSplitProduced, SplitterProducesForOtherCases>>;

#[derive(Debug, PartialEq, Eq)]
pub enum IntermediateFlowingSplitOutcome<
  ProcessBeforeSplitProduced: ParamList,
  SplitterProducesForOtherCases,
  FlowingCaseProduced: ParamList,
> {
  Continue(FlowingCaseProduced), // includes ProcessBeforeSplitProduced
  GoToCase {
    process_before_split_produced: ProcessBeforeSplitProduced,
    splitter_produces_to_other_cases: SplitterProducesForOtherCases,
  },
  Yield(Message, SessionContext, CurrentRunYieldedAt),
  Finish(Message),
  RetryUserInput(Message),
}

pub type IntermediateFlowingSplitResult<
  ProcessBeforeSplitProduced,
  SplitterProducesForOtherCases,
  FlowingCaseProduced,
> = anyhow::Result<
  IntermediateFlowingSplitOutcome<ProcessBeforeSplitProduced, SplitterProducesForOtherCases, FlowingCaseProduced>,
>;

#[derive(Debug, PartialEq, Eq)]
pub enum RunOutcome {
  Yield(Message, SessionContext, CurrentRunYieldedAt),
  Finish(Message),
  RetryUserInput(Message),
}

pub type RunResult = anyhow::Result<RunOutcome>;
