
use crate::brick::{FinalBrickHandler, LinearBrickHandler, SplitterBrickHandler};

// accept different types in builder (with additional type params) and do the checking, and build with non-generic types

pub fn empty_process() -> FlowingProcess { FlowingProcess::NoOp }

pub fn process(brick: Box<dyn LinearBrickHandler>) -> FlowingProcess {
  FlowingProcess::Linear {
    0: FlowingLinearProcess {
      brick,
      process_before_brick: Box::new(empty_process()),
    },
  }
}

pub fn finnish(brick: Box<dyn FinalBrickHandler>) -> FinalizedProcess {
  FinalizedProcess::Linear {
    0: FinalizedLinearProcess {
      brick,
      process_before_brick: Box::new(empty_process()),
    },
  }
}

pub struct FlowingLinearProcess {
  pub(crate) brick: Box<dyn LinearBrickHandler>,
  pub(crate) process_before_brick: Box<FlowingProcess>,
}

impl FlowingLinearProcess
{
  pub fn finnish(self, brick: Box<dyn FinalBrickHandler>) -> FinalizedProcess {
    FinalizedProcess::Linear {
      0: FinalizedLinearProcess {
        brick,
        process_before_brick: Box::new(FlowingProcess::Linear(self)),
      },
    }
  }
}

pub struct FlowingSplitProcess {
  pub(crate) brick: Box<dyn SplitterBrickHandler>,
  pub(crate) cases: Vec<FlowingProcess>,
  pub(crate) process_before_brick: Box<FlowingProcess>,
}

impl FlowingSplitProcess {
  pub fn finnish(
    self,
    brick: Box<dyn FinalBrickHandler>,
  ) -> FinalizedProcess {
    FinalizedProcess::Linear {
      0: FinalizedLinearProcess {
        brick,
        process_before_brick: Box::new(FlowingProcess::Split(self)),
      },
    }
  }
}

pub enum FlowingProcess {
  NoOp,
  Linear(FlowingLinearProcess),
  Split(FlowingSplitProcess),
}

pub struct FinalizedLinearProcess {
  pub(crate) brick: Box<dyn FinalBrickHandler>,
  pub(crate) process_before_brick: Box<FlowingProcess>,
}

pub struct FinalizedSplitProcess {
  pub(crate) brick: Box<dyn SplitterBrickHandler>,
  pub(crate) cases: Vec<FinalizedProcess>,
  pub(crate) process_before_brick: Box<FlowingProcess>,
}

pub enum FinalizedProcess {
  Linear(FinalizedLinearProcess),
  Split(FinalizedSplitProcess),
}

impl FlowingProcess {
  pub fn finnish(self, brick: Box<dyn FinalBrickHandler>) -> FinalizedProcess {
    match self {
      FlowingProcess::NoOp => finnish(brick),
      FlowingProcess::Linear(process) => process.finnish(brick),
      FlowingProcess::Split(process) => process.finnish(brick),
    }
  }

  pub fn and_then(self, brick: Box<dyn LinearBrickHandler>) -> FlowingProcess {
    FlowingProcess::Linear {
      0: FlowingLinearProcess {
        brick,
        process_before_brick: Box::new(self),
      },
    }
  }

  pub fn split(
    self,
    brick: Box<dyn SplitterBrickHandler>,
    cases: Vec<FlowingProcess>,
  ) -> FlowingProcess {
    FlowingProcess::Split {
      0: FlowingSplitProcess {
        brick,
        cases,
        process_before_brick: Box::new(self),
      },
    }
  }

  pub fn split_finalized(
    self,
    brick: Box<dyn SplitterBrickHandler>,
    cases: Vec<FinalizedProcess>,
  ) -> FinalizedProcess {
    FinalizedProcess::Split {
      0: FinalizedSplitProcess {
        brick,
        cases,
        process_before_brick: Box::new(self),
      },
    }
  }
}

pub struct NamedProcess {
  pub(crate) path: &'static str,
  pub(crate) process: FinalizedProcess,
}

impl FinalizedProcess {
  pub fn close(self, path: &'static str) -> NamedProcess {
    NamedProcess {
      path,
      process: self,
    }
  }
}
