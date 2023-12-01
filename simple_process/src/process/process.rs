pub mod process {
  use std::collections::HashMap;
  use crate::bricks::brick::brick::{FinalBrick, LinearBrick, SplitIndex, SplitParam, SplitterBrick};

  // think about brick <Error>

  pub fn empty_process() -> FlowingProcess {
    FlowingProcess::NoOp
  }

  pub fn process(brick: Box<dyn LinearBrick>) -> FlowingProcess {
    FlowingProcess::Linear {
      0: FlowingLinearProcess { brick, process_before_brick: FlowingProcess::NoOp },
    }
  }

  pub fn finnish(brick: Box<dyn FinalBrick>) -> FinalizedProcess {
    FinalizedProcess::Linear {
      0: FinalizedLinearProcess { brick, process_before_brick: FlowingProcess::NoOp },
    }
  }

  pub struct FlowingLinearProcess {
    pub(crate) brick: Box<dyn LinearBrick>,
    pub(crate) process_before_brick: FlowingProcess,
  }

  impl FlowingLinearProcess {
    pub fn finnish(self, brick: Box<dyn FinalBrick>) -> FinalizedProcess {
      FinalizedProcess::Linear {
        0: FinalizedLinearProcess { brick, process_before_brick: FlowingProcess::Linear(self) },
      }
    }
  }

  pub struct FlowingSplitProcess {
    pub(crate) brick: Box<dyn SplitterBrick>,
    pub(crate) cases: HashMap<SplitIndex, FlowingProcess>,
    // some could be finalized
    pub(crate) process_before_brick: FlowingProcess,
  }

  impl FlowingSplitProcess {
    pub fn finnish(self, brick: Box<dyn FinalBrick>) -> FinalizedProcess {
      FinalizedProcess::Linear {
        0: FinalizedLinearProcess { brick, process_before_brick: FlowingProcess::Split(self) },
      }
    }
  }

  pub enum FlowingProcess {
    NoOp,
    Linear(FlowingLinearProcess),
    Split(FlowingSplitProcess),
  }

  pub struct FinalizedLinearProcess {
    pub(crate) brick: Box<dyn FinalBrick>,
    pub(crate) process_before_brick: FlowingProcess,
  }

  pub struct FinalizedSplitProcess {
    pub(crate) brick: Box<dyn SplitterBrick>,
    pub(crate) cases: HashMap<SplitIndex, FinalizedProcess>,
    pub(crate) process_before_brick: FlowingProcess,
  }

  pub enum FinalizedProcess {
    Linear(FinalizedLinearProcess),
    Split(FinalizedSplitProcess),
  }

  impl FlowingProcess {
    pub fn finnish(self, brick: Box<dyn FinalBrick>) -> FinalizedProcess {
      match self {
        FlowingProcess::NoOp => finnish(brick),
        FlowingProcess::Linear(process) => process.finnish(brick),
        FlowingProcess::Split(process) => process.finnish(brick),
      }
    }

    pub fn and_then(self, brick: Box<dyn LinearBrick>) -> FlowingProcess {
      FlowingProcess::Linear {
        0: FlowingLinearProcess { brick, process_before_brick: self }
      }
    }

    pub fn split(
      self,
      brick: Box<dyn SplitterBrick>,
      cases: HashMap<impl SplitParam, FlowingProcess>,
    ) -> FlowingProcess {
      let a = cases.into_iter()
        .map(|(key, value)| {
          let key_split_index = key.split_index();
          brick.data().produces.get(&key_split_index).unwrap();
          (key_split_index, value)
        }).collect();
      FlowingProcess::Split {
        0: FlowingSplitProcess { brick, cases: a, process_before_brick: self },
      }
    }

    pub fn split_finalized(
      self,
      brick: Box<dyn SplitterBrick>,
      cases: HashMap<impl SplitParam, FinalizedProcess>,
    ) -> FinalizedProcess {
      let a = cases.into_iter()
        .map(|(key, value)| {
          let key_split_index = key.split_index();
          brick.data().produces.get(&key_split_index).unwrap();
          (key_split_index, value)
        }).collect();
      FinalizedProcess::Split {
        0: FinalizedSplitProcess { brick, cases: a, process_before_brick: self }
      }
    }
  }

  pub struct Named {
    pub(crate) path: &'static str,
    pub(crate) process: FinalizedProcess,
  }

  impl FinalizedProcess {
    pub fn close(self, path: &'static str) -> Named {
      Named { path, process: self }
    }
  }
}