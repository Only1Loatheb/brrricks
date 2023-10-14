pub mod process {
  use std::collections::HashMap;
  use crate::bricks::brick::brick::{FinalBrick, LinearBrick, Param, SplitterBrick};

  // think about brick <Error>

  fn finnish(brick: &dyn FinalBrick) -> FinalizedProcess {
    FinalizedProcess::FinalizedLinearProcess {
      0: FinalizedLinearProcess { brick, process_before_brick: &FlowingProcess::FlowingNoOpProcess },
    }
  }

  pub struct LinearProcess<'a> {
    pub brick: &'a dyn LinearBrick,
    pub process_before_brick: &'a FlowingProcess<'a>,
  }

  impl LinearProcess<'_> {
    fn finnish(self, brick: &dyn FinalBrick) -> FinalizedProcess {
      FinalizedProcess::FinalizedLinearProcess {
        0: FinalizedLinearProcess { brick, process_before_brick: &FlowingProcess::FlowingLinearProcess(self) },
      }
    }
  }

  pub struct SplitProcess<'a, SplitParam: Param> {
    pub brick: &'a dyn SplitterBrick<SplitParam>,
    pub cases: HashMap<SplitParam, &'a FlowingProcess<'a>>,
    pub process_before_brick: &'a FlowingProcess<'a>,
  }

  impl<SplitParam: Param> SplitProcess<'_, SplitParam> {
    fn finnish(self, brick: &dyn FinalBrick) -> FinalizedProcess {
      FinalizedProcess::FinalizedLinearProcess {
        0: FinalizedLinearProcess { brick, process_before_brick: &FlowingProcess::FlowingSplitProcess(self) },
      }
    }
  }

  pub enum FlowingProcess<'a> {
    FlowingNoOpProcess,
    FlowingLinearProcess(LinearProcess<'a>),
    FlowingSplitProcess(SplitProcess<'a, dyn Param>),
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
    FinalizedLinearProcess(FinalizedLinearProcess<'a>),
    FinalizedSplitProcess(FinalizedSplitProcess<'a, dyn Param>),
  }

  impl FlowingProcess<'_> {
    fn and_then(&self, brick: &dyn LinearBrick) -> LinearProcess {
      LinearProcess { brick, process_before_brick: self }
    }
    // fn split<SplitParam: Param>(
    //   &self,
    //   brick: &dyn SplitterBrick<SplitParam>,
    //   cases: HashMap<SplitParam, dyn Following>,
    // ) -> dyn Following;
  }

  pub struct Named<'a> {
    pub path: &'a str,
    pub process: &'a FinalizedProcess<'a>,
  }

  impl FinalizedProcess<'_> {
    fn close(&self, path: &str) -> Named {
      Named { path, process: self }
    }
  }
}