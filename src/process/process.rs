pub mod process {
  use std::collections::HashMap;
  use crate::bricks::brick::brick::{FinalBrick, LinearBrick, Param, SplitterBrick};

  // think about brick <Error>

  pub fn empty_process() -> FlowingProcess {
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
    pub brick: &'a dyn LinearBrick,
    pub process_before_brick: &'a FlowingProcess<'a>,
  }

  impl FlowingLinearProcess<'_> {
    pub fn finnish(self, brick: &dyn FinalBrick) -> FinalizedProcess {
      FinalizedProcess::Linear {
        0: FinalizedLinearProcess { brick, process_before_brick: &FlowingProcess::Linear(self) },
      }
    }
  }

  pub struct FlowingSplitProcess<'a, SplitParam: Param> {
    pub brick: &'a dyn SplitterBrick<SplitParam>,
    pub cases: HashMap<SplitParam, &'a FlowingProcess<'a>>, // some could be finalized
    pub process_before_brick: &'a FlowingProcess<'a>,
  }

  impl<SplitParam: Param> FlowingSplitProcess<'_, SplitParam> {
    pub fn finnish(self, brick: &dyn FinalBrick) -> FinalizedProcess {
      FinalizedProcess::Linear {
        0: FinalizedLinearProcess { brick, process_before_brick: &FlowingProcess::Split(self) },
      }
    }
  }

  pub enum FlowingProcess<'a> {
    NoOp,
    Linear(FlowingLinearProcess<'a>),
    Split(FlowingSplitProcess<'a, dyn Param>),
  }

  pub struct FinalizedLinearProcess<'a> {
    pub brick: &'a dyn FinalBrick,
    pub process_before_brick: &'a FlowingProcess<'a>,
  }

  pub struct FinalizedSplitProcess<'a, SplitParam: Param> {
    pub brick: &'a dyn SplitterBrick<SplitParam>,
    pub cases: HashMap<SplitParam, FinalizedProcess<'a>>,
    pub process_before_brick: &'a FlowingProcess<'a>,
  }

  pub enum FinalizedProcess<'a> {
    Linear(FinalizedLinearProcess<'a>),
    Split(FinalizedSplitProcess<'a, dyn Param>),
  }

  impl FlowingProcess<'_> {
    pub fn finnish(&self, brick: &dyn FinalBrick) -> FinalizedProcess {
      match self {
        FlowingProcess::NoOp => finnish(brick),
        FlowingProcess::Linear(process) => process.finnish(brick),
        FlowingProcess::Split(process) => todo!()
      }
    }

    pub fn and_then(&self, brick: &dyn LinearBrick) -> FlowingProcess {
      FlowingProcess::Linear {
        0: FlowingLinearProcess { brick, process_before_brick: self }
      }
    }

    pub fn split<SplitParam: Param>(
      &self,
      brick: &dyn SplitterBrick<SplitParam>,
      cases: HashMap<SplitParam, FlowingProcess>,
    ) -> FlowingProcess {
      FlowingProcess::Split {
        0: FlowingSplitProcess { brick, cases, process_before_brick: &self },
      }
    }

    pub fn split_finalized<SplitParam: Param>(
      &self,
      brick: &dyn SplitterBrick<SplitParam>,
      cases: HashMap<SplitParam, FinalizedProcess>,
    ) -> FinalizedProcess {
      FinalizedProcess::Split {
        0: FinalizedSplitProcess { brick, cases, process_before_brick: self }
      }
    }
  }

  pub struct Named<'a> {
    pub path: &'a str,
    pub process: &'a FinalizedProcess<'a>,
  }

  impl FinalizedProcess<'_> {
    pub fn close(&self, path: &str) -> Named {
      Named { path, process: self }
    }
  }
}