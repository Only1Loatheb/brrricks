pub mod finalized_process;
pub mod finalized_split_process;
pub mod flowing_process;
pub mod flowing_split_process;
pub mod process_error;
pub mod runnable_process;
pub mod split_process;

pub use crate::error_coproduct::contains::CoprodContains;
pub use crate::error_coproduct::union::{CoprodUnion, IfNotKeepError};
pub use crate::error_coproduct::{ContainsError, ErrorCoproduct, PreventDuplicateErrorUidInCoproduct, StepError};
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
pub use process_error::*;
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

  pub type IntermediateRunResult<Produced, Messages, StepErrors> =
    Result<IntermediateRunOutcome<Produced, Messages>, ProcessError<StepErrors>>;

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

  pub type IntermediateFinalizedSplitResult<
    ProcessBeforeSplitProduced,
    SplitterProducesForOtherCases,
    Messages,
    StepErrors,
  > = Result<
    IntermediateFinalizedSplitOutcome<ProcessBeforeSplitProduced, SplitterProducesForOtherCases, Messages>,
    ProcessError<StepErrors>,
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
    StepErrors,
  > = Result<
    IntermediateFlowingSplitOutcome<
      ProcessBeforeSplitProduced,
      SplitterProducesForOtherCases,
      FlowingCaseProduced,
      Messages,
    >,
    ProcessError<StepErrors>,
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

pub type RunResult<Messages, StepErrors> = Result<RunOutcome<Messages>, ProcessError<StepErrors>>;
