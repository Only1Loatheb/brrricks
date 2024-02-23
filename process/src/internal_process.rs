use crate::internal_brick::*;

pub struct InternalFinalizedSplitSentinel {
  pub brick: InternalSplitterBrick,
  pub process_case: InternalFinalizedProcess,
  pub process_before: InternalFlowingProcess,
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

pub struct InternalFlowingSplitSentinel {
  pub brick: InternalSplitterBrick,
  pub process_case: InternalFlowingProcess,
  pub process_before: InternalFlowingProcess,
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

pub struct NamedProcess {
  pub path: &'static str,
  pub process: InternalFinalizedProcess,
}
