pub mod finalized_process;
pub mod finalized_split_process;
pub mod flowing_process;
pub mod flowing_split_process;
pub mod runnable_process;
pub mod split_process;

pub use crate::param_list::*;
use crate::step::ProcessMessages;
pub use crate::step::{BackToken, Entry, entry};
pub use finalized_process::*;
pub(crate) use finalized_split_process::FinalizedSplitProcess;
pub use finalized_split_process::first_case_of_finalized_split_process::*;
pub use finalized_split_process::next_case_of_finalized_split_process::*;
pub use flowing_process::*;
pub(crate) use flowing_split_process::FlowingSplitProcess;
pub use flowing_split_process::finalized_case_of_flowing_split_process::*;
pub use flowing_split_process::first_case_of_flowing_split_process::*;
pub use flowing_split_process::flowing_case_of_finalized_split_process::*;
pub use flowing_split_process::flowing_case_of_flowing_split_process::*;
pub use runnable_process::*;
pub use split_process::*;

pub type StepIndex = i32;

#[derive(PartialEq, Debug, Eq, Clone, PartialOrd, Ord, Hash)]
pub struct PreviousRunYieldedAt(pub StepIndex);

#[derive(PartialEq, Debug, Eq, Clone, PartialOrd, Ord, Hash)]
pub struct CurrentRunYieldedAt(pub StepIndex);

pub(crate) const WILL_BE_RENUMBERED: StepIndex = StepIndex::MAX;

pub type FormContext = Vec<u8>;
pub type MaybeFormContext = Option<FormContext>;

#[derive(Debug, thiserror::Error)]
pub enum ProcessError {
  /// Failed to serialize or deserialize a [`SessionContext`] or [`FormContext`].
  #[error("failed to serialize or deserialize process state: {0}")]
  Serialization(#[from] postcard::Error),
  /// The supplied [`PreviousRunYieldedAt`] does not match any valid resume point in the process.
  #[error("invalid step index: {0}")]
  InvalidStepIndex(StepIndex),
  /// A user-implemented step returned an error.
  #[error("step execution failed: {0}")]
  Step(#[from] anyhow::Error),
}

pub(crate) use sealed::*;

pub(crate) mod sealed {
  use super::{CurrentRunYieldedAt, FormContext, ParamList, ProcessError, ProcessMessages, SessionContext};

  #[derive(Debug, PartialEq, Eq)]
  pub enum IntermediateRunOutcome<Produced: ParamList, Messages: ProcessMessages> {
    Continue(Produced),
    Yield(Messages::FormMessage, SessionContext, CurrentRunYieldedAt, FormContext),
    Finish(Messages::FinalMessage),
    RetryUserInput(Messages::FormMessage, FormContext),
    Back,
  }

  pub type IntermediateRunResult<Produced, Messages> = Result<IntermediateRunOutcome<Produced, Messages>, ProcessError>;

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
    Yield(Messages::FormMessage, SessionContext, CurrentRunYieldedAt, FormContext),
    Finish(Messages::FinalMessage),
    RetryUserInput(Messages::FormMessage, FormContext),
    Back,
  }

  pub type IntermediateFinalizedSplitResult<ProcessBeforeSplitProduced, SplitterProducesForOtherCases, Messages> =
    Result<
      IntermediateFinalizedSplitOutcome<ProcessBeforeSplitProduced, SplitterProducesForOtherCases, Messages>,
      ProcessError,
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
    Yield(Messages::FormMessage, SessionContext, CurrentRunYieldedAt, FormContext),
    Finish(Messages::FinalMessage),
    RetryUserInput(Messages::FormMessage, FormContext),
    Back,
  }

  pub type IntermediateFlowingSplitResult<
    ProcessBeforeSplitProduced,
    SplitterProducesForOtherCases,
    FlowingCaseProduced,
    Messages,
  > = Result<
    IntermediateFlowingSplitOutcome<
      ProcessBeforeSplitProduced,
      SplitterProducesForOtherCases,
      FlowingCaseProduced,
      Messages,
    >,
    ProcessError,
  >;
}

/// Outcome of a single [`RunnableProcess::resume_run`] call.
#[derive(Debug, PartialEq, Eq)]
pub enum RunOutcome<Messages: ProcessMessages> {
  /// Process paused at a form step; persist the returned context/step and present the form message.
  Yield(Messages::FormMessage, SessionContext, CurrentRunYieldedAt, FormContext),
  /// Process completed with a final message.
  Finish(Messages::FinalMessage),
  /// User input was rejected by the current form step; re-prompt with the updated [`FormContext`].
  RetryUserInput(Messages::FormMessage, FormContext),
  /// User requested navigating back to the previous form step.
  Back,
}

pub type RunResult<Messages> = Result<RunOutcome<Messages>, ProcessError>;
