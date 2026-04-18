pub mod finalized_process;
pub mod finalized_split_process;
pub mod flowing_process;
pub mod flowing_split_process;
pub mod runnable_process;
pub mod split_process;

pub use crate::param_list::*;
use crate::step::ProcessMessages;
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

pub type StepIndex = i32;

#[derive(PartialEq, Debug, Eq, Clone, PartialOrd, Ord, Hash)]
pub struct PreviousRunYieldedAt(pub StepIndex);

#[derive(PartialEq, Debug, Eq, Clone, PartialOrd, Ord, Hash)]
pub struct CurrentRunYieldedAt(pub StepIndex);

pub(crate) const WILL_BE_RENUMBERED: i32 = i32::MAX;

pub type ParamUID = u32;

pub(crate) type SessionContext = Vec<(ParamUID, Value)>;

#[derive(Debug, PartialEq, Eq)]
pub enum IntermediateRunOutcome<Produced: ParamList, Messages: ProcessMessages> {
  Continue(Produced),
  Yield(Messages::FormMessage, SessionContext, CurrentRunYieldedAt),
  Finish(Messages::FinalMessage),
  RetryUserInput(Messages::FormMessage),
}

pub type IntermediateRunResult<Produced, Messages> = anyhow::Result<IntermediateRunOutcome<Produced, Messages>>;

#[derive(Debug, PartialEq, Eq)]
pub enum IntermediateFinalizedSplitOutcome<
  ProcessBeforeSplitProduced: ParamList,
  SplitterProducesForOtherCases,
  Messages: ProcessMessages,
> {
  GoToCase {
    process_before_split_produced: ProcessBeforeSplitProduced,
    splitter_produces_to_other_cases: SplitterProducesForOtherCases,
  },
  Yield(Messages::FormMessage, SessionContext, CurrentRunYieldedAt),
  Finish(Messages::FinalMessage),
  RetryUserInput(Messages::FormMessage),
}

pub type IntermediateFinalizedSplitResult<ProcessBeforeSplitProduced, SplitterProducesForOtherCases, Messages> =
  anyhow::Result<
    IntermediateFinalizedSplitOutcome<ProcessBeforeSplitProduced, SplitterProducesForOtherCases, Messages>,
  >;

#[derive(Debug, PartialEq, Eq)]
pub enum IntermediateFlowingSplitOutcome<
  ProcessBeforeSplitProduced: ParamList,
  SplitterProducesForOtherCases,
  FlowingCaseProduced: ParamList,
  Messages: ProcessMessages,
> {
  Continue(FlowingCaseProduced), // includes ProcessBeforeSplitProduced
  GoToCase {
    process_before_split_produced: ProcessBeforeSplitProduced,
    splitter_produces_to_other_cases: SplitterProducesForOtherCases,
  },
  Yield(Messages::FormMessage, SessionContext, CurrentRunYieldedAt),
  Finish(Messages::FinalMessage),
  RetryUserInput(Messages::FormMessage),
}

pub type IntermediateFlowingSplitResult<
  ProcessBeforeSplitProduced,
  SplitterProducesForOtherCases,
  FlowingCaseProduced,
  Messages,
> = anyhow::Result<
  IntermediateFlowingSplitOutcome<
    ProcessBeforeSplitProduced,
    SplitterProducesForOtherCases,
    FlowingCaseProduced,
    Messages,
  >,
>;

#[derive(Debug, PartialEq, Eq)]
pub enum RunOutcome<Messages: ProcessMessages> {
  Yield(Messages::FormMessage, SessionContext, CurrentRunYieldedAt),
  Finish(Messages::FinalMessage),
  RetryUserInput(Messages::FormMessage),
}

pub type RunResult<Messages> = anyhow::Result<RunOutcome<Messages>>;
