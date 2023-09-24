pub mod process {
  use std::collections::HashMap;
  use crate::bricks::brick::brick::{FinalBrick, LinearBrick, Param, SplitterBrick};
  use crate::process::process::process::sealed::{Finalized, Following, Started};

  mod sealed {
    use std::collections::HashMap;
    use crate::bricks::brick::brick::{FinalBrick, LinearBrick, Param, SplitterBrick};

    pub trait Finalized {}

    pub trait Following {
      fn and_then(&self, brick: &dyn LinearBrick) -> &dyn Following;
      fn finnish(&self, brick: &dyn FinalBrick) -> &dyn Finalized;
      fn split<SplitParam: Param>(
        &self,
        brick: &dyn SplitterBrick<SplitParam>,
        cases: HashMap<SplitParam, dyn Following>,
      ) -> dyn Following;
    }

    pub trait Started: Following {
      fn path(&self) -> &String;
    }
  }

  // think about brick <Error>
  pub struct StartedLinearProcess<'a> {
    path: String,
    brick: &'a dyn LinearBrick,
  }

  pub struct StartedSplitProcess<'a, SplitParam: Param> {
    path: String,
    brick: &'a dyn SplitterBrick<SplitParam>,
    cases: HashMap<SplitParam, dyn Following>,
  }

  pub struct OneBrickProcess<'a> {
    path: String,
    brick: &'a dyn FinalBrick,
  }

  pub struct LinearProcess<'a> {
    brick: &'a dyn LinearBrick,
    process_before_brick: Box<dyn Following>,
  }

  pub struct SplitProcess<'a, SplitParam: Param> {
    brick: &'a dyn SplitterBrick<SplitParam>,
    cases: HashMap<SplitParam, dyn Following>,
    process_before_brick: Box<dyn Following>,
  }

  pub struct FinalizedProcess<'a> {
    brick: &'a dyn FinalBrick,
    process_before_brick: Box<dyn Following>,
  }

  // todo:
  impl<SplitParam: Param> Following for StartedSplitProcess<SplitParam> {}

  impl<SplitParam: Param> Started for StartedSplitProcess<SplitParam> {
    fn path(&self) -> &String {
      &self.path
    }
  }

  impl Following for StartedLinearProcess {
    fn and_then(&self, brick: &dyn LinearBrick) -> &dyn Following {
      todo!()
    }

    fn finnish(&self, brick: &dyn FinalBrick) -> &dyn Finalized {
      todo!()
    }

    fn split<SplitParam: Param>(&self, brick: &dyn SplitterBrick<SplitParam>, cases: HashMap<SplitParam, dyn Following>) -> dyn Following {
      todo!()
    }
  }

  impl Started for StartedLinearProcess {
    fn path(&self) -> &String {
      &self.path
    }
  }

  impl<'a, SplitParam: Param> Following<'a, SplitParam> {
    pub fn new(path: String, brick: &dyn LinearBrick) -> Following<'a, SplitParam> {
      Following::StartedProcess {
        path,
        brick,
      }
    }

    pub fn and_then(&self, brick: &dyn LinearBrick) -> Process<'a, SplitParam> {
      Process::LinearProcess {
        brick,
        process_before_brick: self,
      }
    }

    pub fn split(&self, brick: &dyn SplitterBrick<SplitParam>, cases: HashMap<SplitParam, Following<dyn Param>>) -> Following<'a, dyn Param> {
      Following::SplitProcess {
        brick,
        cases,
        process_before_brick: self,
      }
    }
  }
}