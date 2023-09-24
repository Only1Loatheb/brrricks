pub mod process {
  use std::collections::HashMap;
  use std::hash::Hash;
  use crate::bricks::brick::brick::{LinearBrick, Param, SplitterBrick};

  // think about brick <Error>
  pub enum Process<'a, SplitParam: Param> {
    StartedProcess {
      path: String,
      brick: &'a dyn LinearBrick,
    },
    LinearProcess {
      brick: &'a dyn LinearBrick,
      process_before_brick: &'a Process<'a, dyn Param>,
    },
    SplitProcess {
      brick: &'a dyn SplitterBrick<SplitParam>,
      cases: HashMap<SplitParam, Process<'a, dyn Param>>,
      process_before_brick: &'a Process<'a, dyn Param>,
    },
  }

  impl<'a, SplitParam: Param> Process<'a, SplitParam> {
    pub fn new(path: String, brick: &dyn LinearBrick) -> Process<'a, SplitParam> {
      Process::StartedProcess {
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

    pub fn split(&self, brick: &dyn SplitterBrick<SplitParam>, cases: HashMap<SplitParam, Process<dyn Param>>) -> Process<'a, dyn Param> {
      Process::SplitProcess {
        brick,
        cases,
        process_before_brick: self,
      }
    }
  }
}