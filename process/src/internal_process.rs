use crate::internal_brick::*;

// builder has to take care of having at least 2 cases
pub enum InternalFinalizedSplitProcess {
  FirstCase {
    brick: InternalSplitterBrick,
    first_cases: InternalFinalizedProcess,
    process_before: InternalFlowingProcess,
  },
  NextCase {
    next_case: InternalFinalizedProcess,
    process_before: Box<InternalFinalizedSplitProcess>,
  },
}

// builder has to take care of having at least 2 cases
pub enum InternalFlowingSplitProcess {
  FirstCase {
    brick: InternalSplitterBrick,
    first_case: InternalFlowingProcess,
    process_before: InternalFlowingProcess,
  },
  NextCase {
    next_case: InternalFlowingProcess,
    process_before: Box<InternalFinalizedSplitProcess>,
  },
  NextCaseFlowing {
    next_case: InternalFlowingProcess,
    process_before: Box<InternalFlowingSplitProcess>,
  },
  NextCaseFinalized {
    next_case: InternalFinalizedProcess,
    process_before: Box<InternalFlowingSplitProcess>,
  },
}

pub enum InternalFlowingProcess {
  Empty,
  Linear(InternalLinearBrick, Box<InternalFlowingProcess>),
  Split(Box<InternalFlowingSplitProcess>),
}

pub enum InternalFinalizedProcess {
  Flowing(InternalFinalBrick, InternalFlowingProcess),
  Split(Box<InternalFinalizedSplitProcess>),
}

pub struct NamedProcess {
  pub path: String,
  pub process: InternalFinalizedProcess,
}
