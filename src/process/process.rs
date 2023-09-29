pub mod process {
  use std::collections::HashMap;
  use crate::bricks::brick::brick::{FinalBrick, LinearBrick, Param, SplitterBrick};
  use crate::process::process::process::sealed::{Finalized, Following};

  // think about brick <Error>
  pub struct OneBrickProcess<'a> {
    brick: &'a dyn FinalBrick,
  }

  pub struct StartedLinearProcess<'a> {
    brick: &'a dyn LinearBrick,
  }

  pub struct LinearProcess<'a> {
    brick: &'a dyn LinearBrick,
    process_before_brick: &'a dyn Following,
  }

  pub struct FinalizedProcess<'a> {
    brick: &'a dyn FinalBrick,
    process_before_brick: &'a dyn Following,
  }

  pub struct StartedSplitProcess<'a, SplitParam: Param> {
    brick: &'a dyn SplitterBrick<SplitParam>,
    cases: HashMap<SplitParam, dyn Following>,
  }

  pub struct OneSplitProcess<'a, SplitParam: Param> {
    brick: &'a dyn SplitterBrick<SplitParam>,
    cases: HashMap<SplitParam, dyn Finalized>,
  }

  pub struct FinalizedSplitProcess<'a, SplitParam: Param> {
    brick: &'a dyn SplitterBrick<SplitParam>,
    cases: HashMap<SplitParam, dyn Finalized>,
    process_before_brick: &'a dyn Following,
  }

  pub struct SplitProcess<'a, SplitParam: Param> {
    brick: &'a dyn SplitterBrick<SplitParam>,
    cases: HashMap<SplitParam, dyn Following>,
    process_before_brick: &'a dyn Following,
  }

  pub struct Closed<'a> {
    pub path: &'a str,
    pub process: &'a dyn Finalized,
  }

  mod sealed {
    use std::collections::HashMap;
    use crate::bricks::brick::brick::{FinalBrick, LinearBrick, Param, SplitterBrick};
    use crate::process::process::process::*;

    pub trait Finalized {
      fn close(&self, path: &str) -> Closed;
    }

    pub trait Following {
      fn and_then(&self, brick: &dyn LinearBrick) -> LinearProcess;
      fn finnish(&self, brick: &dyn FinalBrick) -> FinalizedProcess;
      fn split<SplitParam: Param>(
        &self,
        brick: &dyn SplitterBrick<SplitParam>,
        cases: HashMap<SplitParam, dyn Following>,
      ) -> dyn Following;
    }
  }

  impl<'a> Finalized for OneBrickProcess<'a> {
    fn close(&self, path: &str) -> Closed {
      Closed { path, process: self }
    }
  }

  impl<'a> Finalized for FinalizedProcess<'a> {
    fn close(&self, path: &str) -> Closed {
      Closed { path, process: self }
    }
  }

  impl<'a, SplitParam: Param> Finalized for OneSplitProcess<SplitParam> {
    fn close(&self, path: &str) -> Closed {
        Closed { path, process: self }
    }
  }

  impl<'a, SplitParam: Param> Finalized for FinalizedSplitProcess<SplitParam> {
    fn close(&self, path: &str) -> Closed {
      Closed { path, process: self }
    }
  }

  // todo:
  impl<SplitParam: Param> Following for StartedSplitProcess<SplitParam> {
    fn and_then(&self, brick: &dyn LinearBrick) -> LinearProcess {
      LinearProcess {
        brick,
        process_before_brick: self,
      }
    }

    fn finnish(&self, brick: &dyn FinalBrick) -> &dyn Finalized {
      todo!()
    }

    fn split<NextSplit: Param>(&self, brick: &dyn SplitterBrick<NextSplit>, cases: HashMap<NextSplit, dyn Following>) -> Box<dyn Following> {
      Following::SplitProcess {
        brick,
        cases,
        process_before_brick: self,
      }
    }
  }
}