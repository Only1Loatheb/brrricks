use crate::internal_brick::{InternalFinalBrick, InternalLinearBrick, InternalSplitterBrick};

pub(crate) struct InternalFinalizedSplitSentinel {
  brick: InternalSplitterBrick,
  process_case: InternalFinalizedProcess,
  process_before: InternalFlowingProcess,
}

pub enum InternalFinalizedSplitProcess {
  SecondCase {
    case_process: InternalFinalizedProcess,
    process_before: InternalFinalizedSplitSentinel,
  },
  NextCase {
    case_process: InternalFinalizedProcess,
    process_before: Box<InternalFinalizedSplitProcess>,
  },
}

pub(crate) struct InternalFlowingSplitSentinel {
  brick: InternalSplitterBrick,
  process_case: InternalFlowingProcess,
  process_before: InternalFlowingProcess,
}

pub enum InternalFlowingSplitProcess {
  SecondCaseFinalizedSentinel {
    case_process: InternalFlowingProcess,
    process_before: InternalFinalizedSplitSentinel,
  },
  SecondCaseFlowing {
    case_process: InternalFlowingProcess,
    process_before: InternalFlowingSplitSentinel,
  },
  SecondCaseFinalized {
    case_process: InternalFinalizedProcess,
    process_before: InternalFlowingSplitSentinel,
  },
  NextCaseFlowing {
    case_process: InternalFlowingProcess,
    process_before: Box<InternalFlowingSplitProcess>,
  },
  NextCaseFinalized {
    case_process: InternalFinalizedProcess,
    process_before: Box<InternalFlowingSplitProcess>,
  },
}

pub enum InternalFlowingProcess {
  Empty,
  Linear(InternalLinearBrick, Box<InternalFlowingProcess>),
  Split(Box<InternalFlowingSplitProcess>),
}

pub enum InternalFinalizedProcess {
  One(InternalFinalBrick),
  Flowing(InternalFinalBrick, InternalFlowingProcess),
  Split(Box<InternalFinalizedSplitProcess>),
}
