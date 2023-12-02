pub mod process {
  use std::collections::HashMap;
  use crate::bricks::brick::brick::{FinalBrick, LinearBrick, SplitIndex, SplitParam, SplitterBrick};

  // think about brick <Error>

  const FLOWING_PROCESS_NO_OP: FlowingProcess = FlowingProcess::NoOp;

  pub const fn empty_process() -> FlowingProcess {
    FlowingProcess::NoOp
  }

  pub const fn process(brick: &'static dyn LinearBrick) -> FlowingProcess {
    FlowingProcess::Linear {
      0: FlowingLinearProcess { brick, process_before_brick: &FLOWING_PROCESS_NO_OP },
    }
  }

  pub const fn finnish(brick: &'static dyn FinalBrick) -> FinalizedProcess {
    FinalizedProcess::Linear {
      0: FinalizedLinearProcess { brick, process_before_brick: &FLOWING_PROCESS_NO_OP },
    }
  }

  pub struct FlowingLinearProcess {
    pub(crate) brick: &'static dyn LinearBrick,
    pub(crate) process_before_brick: &'static FlowingProcess,
  }

  impl FlowingLinearProcess {
    pub const fn finnish(self, brick: &'static dyn FinalBrick) -> FinalizedProcess {
      FinalizedProcess::Linear {
        0: FinalizedLinearProcess { brick, process_before_brick: &FlowingProcess::Linear(self) },
      }
    }
  }

  pub struct FlowingSplitProcess {
    pub(crate) brick: &'static dyn SplitterBrick,
    pub(crate) cases: HashMap<SplitIndex, FlowingProcess>,
    // some could be finalized
    pub(crate) process_before_brick: &'static FlowingProcess,
  }

  impl FlowingSplitProcess {
    pub const fn finnish(self, brick: &'static dyn FinalBrick) -> FinalizedProcess {
      FinalizedProcess::Linear {
        0: FinalizedLinearProcess { brick, process_before_brick: &FlowingProcess::Split(self) },
      }
    }
  }

  pub enum FlowingProcess {
    NoOp,
    Linear(FlowingLinearProcess),
    Split(FlowingSplitProcess),
  }

  pub struct FinalizedLinearProcess {
    pub(crate) brick: &'static dyn FinalBrick,
    pub(crate) process_before_brick: &'static FlowingProcess,
  }

  pub struct FinalizedSplitProcess {
    pub(crate) brick: &'static dyn SplitterBrick,
    pub(crate) cases: HashMap<SplitIndex, FinalizedProcess>,
    pub(crate) process_before_brick: &'static FlowingProcess,
  }

  pub enum FinalizedProcess {
    Linear(FinalizedLinearProcess),
    Split(FinalizedSplitProcess),
  }

  impl FlowingProcess {
    pub const fn finnish(self, brick: &'static dyn FinalBrick) -> FinalizedProcess {
      match self {
        FlowingProcess::NoOp => finnish(brick),
        FlowingProcess::Linear(process) => process.finnish(brick),
        FlowingProcess::Split(process) => process.finnish(brick),
      }
    }

    pub const fn and_then(self, brick: &'static dyn LinearBrick) -> FlowingProcess {
      FlowingProcess::Linear {
        0: FlowingLinearProcess { brick, process_before_brick: &self }
      }
    }

    pub const fn split(
      self,
      brick: &'static dyn SplitterBrick,
      cases: HashMap<impl SplitParam, FlowingProcess>,
    ) -> FlowingProcess {
      let a = cases.into_iter()
        .map(|(key, value)| {
          let key_split_index = key.split_index();
          brick.data().produces.get(&key_split_index).unwrap();
          (key_split_index, value)
        }).collect();
      FlowingProcess::Split {
        0: FlowingSplitProcess { brick, cases: a, process_before_brick: &self },
      }
    }

    pub const fn split_finalized(
      self,
      brick: &'static dyn SplitterBrick,
      cases: HashMap<impl SplitParam, FinalizedProcess>,
    ) -> FinalizedProcess {
      let a = cases.into_iter()
        .map(|(key, value)| {
          let key_split_index = key.split_index();
          brick.data().produces.get(&key_split_index).unwrap();
          (key_split_index, value)
        }).collect();
      FinalizedProcess::Split {
        0: FinalizedSplitProcess { brick, cases: a, process_before_brick: &self }
      }
    }
  }

  pub struct NamedProcess {
    pub(crate) path: &'static str,
    pub(crate) process: FinalizedProcess,
  }

  impl FinalizedProcess {
    pub const fn close(self, path: &'static str) -> NamedProcess {
      NamedProcess { path, process: self }
    }
  }
}