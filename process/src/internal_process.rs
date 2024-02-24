use crate::internal_brick::*;

pub struct InternalFinalizedSplitSentinel {
  pub brick: InternalSplitterBrick,
  pub first_cases: InternalFinalizedProcess,
  pub process_before: InternalFlowingProcess,
}

pub enum InternalFinalizedSplitProcess {
  SecondCase {
    second_case: InternalFinalizedProcess,
    process_before: InternalFinalizedSplitSentinel,
  },
  NextCase {
    next_case: InternalFinalizedProcess,
    process_before: Box<InternalFinalizedSplitProcess>,
  },
}

pub struct InternalFlowingSplitSentinel {
  pub brick: InternalSplitterBrick,
  pub first_case: InternalFlowingProcess,
  pub process_before: InternalFlowingProcess,
}

pub enum InternalFlowingSplitProcess {
  SecondCaseFinalizedSentinel {
    second_case: InternalFlowingProcess,
    process_before: InternalFinalizedSplitSentinel,
  },
  SecondCaseFlowing {
    second_case: InternalFlowingProcess,
    process_before: InternalFlowingSplitSentinel,
  },
  SecondCaseFinalized {
    second_case: InternalFinalizedProcess,
    process_before: InternalFlowingSplitSentinel,
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
  One(InternalFinalBrick),
  Flowing(InternalFinalBrick, InternalFlowingProcess),
  Split(Box<InternalFinalizedSplitProcess>),
}

pub struct NamedProcess {
  pub path: &'static str,
  pub process: InternalFinalizedProcess,
}
