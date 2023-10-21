pub mod process {
  use std::collections::HashMap;
  use crate::bricks::brick::brick::{FinalBrick, LinearBrick, SplitIndex, SplitParam, SplitterBrick};

  // think about brick <Error>

  pub fn empty_process() -> FlowingProcess<'static> {
    FlowingProcess::NoOp
  }

  pub fn process(brick: &dyn LinearBrick) -> FlowingProcess {
    FlowingProcess::Linear {
      0: FlowingLinearProcess { brick, process_before_brick: &FlowingProcess::NoOp },
    }
  }

  pub fn finnish(brick: &dyn FinalBrick) -> FinalizedProcess {
    FinalizedProcess::Linear {
      0: FinalizedLinearProcess { brick, process_before_brick: &FlowingProcess::NoOp },
    }
  }

  pub struct FlowingLinearProcess<'a> {
    pub(crate) brick: &'a dyn LinearBrick,
    pub(crate) process_before_brick: &'a FlowingProcess<'a>,
  }

  impl<'a> FlowingLinearProcess<'a> {
    pub fn finnish(self, brick: &'a dyn FinalBrick) -> FinalizedProcess<'a> {
      FinalizedProcess::Linear {
        0: FinalizedLinearProcess { brick, process_before_brick: &FlowingProcess::Linear(self) },
      }
    }
  }

  pub struct FlowingSplitProcess<'a> {
    pub(crate) brick: &'a dyn SplitterBrick,
    pub(crate) cases: HashMap<SplitIndex, FlowingProcess<'a>>,
    // some could be finalized
    pub(crate) process_before_brick: &'a FlowingProcess<'a>,
  }

  impl<'a> FlowingSplitProcess<'a> {
    pub fn finnish(self, brick: &'a dyn FinalBrick) -> FinalizedProcess<'a> {
      FinalizedProcess::Linear {
        0: FinalizedLinearProcess { brick, process_before_brick: &FlowingProcess::Split(self) },
      }
    }
  }

  pub enum FlowingProcess<'a> {
    NoOp,
    Linear(FlowingLinearProcess<'a>),
    Split(FlowingSplitProcess<'a>),
  }

  pub struct FinalizedLinearProcess<'a> {
    pub(crate) brick: &'a dyn FinalBrick,
    pub(crate) process_before_brick: &'a FlowingProcess<'a>,
  }

  pub struct FinalizedSplitProcess<'a> {
    pub(crate) brick: &'a dyn SplitterBrick,
    pub(crate) cases: HashMap<SplitIndex, FinalizedProcess<'a>>,
    pub(crate) process_before_brick: &'a FlowingProcess<'a>,
  }

  pub enum FinalizedProcess<'a> {
    Linear(FinalizedLinearProcess<'a>),
    Split(FinalizedSplitProcess<'a>),
  }

  impl FlowingProcess<'_> {
    pub fn finnish(&self, brick: &dyn FinalBrick) -> FinalizedProcess {
      match self {
        FlowingProcess::NoOp => finnish(brick),
        FlowingProcess::Linear(process) => process.finnish(brick),
        FlowingProcess::Split(process) => process.finnish(brick),
      }
    }

    pub fn and_then<'a>(&'a self, brick: &'a dyn LinearBrick) -> FlowingProcess {
      FlowingProcess::Linear {
        0: FlowingLinearProcess { brick, process_before_brick: self }
      }
    }

    pub fn split(
      &self,
      brick: &dyn SplitterBrick,
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

    pub fn split_finalized(
      &self,
      brick: &dyn SplitterBrick,
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

  pub struct Named<'a> {
    pub(crate) path: &'a str,
    pub(crate) process: &'a FinalizedProcess<'a>,
  }

  impl FinalizedProcess<'_> {
    pub fn close(&self, path: &str) -> Named {
      Named { path, process: self }
    }
  }
}