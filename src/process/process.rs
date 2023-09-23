pub mod process {
  use crate::bricks::brick::brick::{LinearBrick, Param, SplitterBrick};

  // think about brick <Error>
  pub enum Process<'a, SplitParam: Param> {
    StartedProcess {
      path: String,
      brick: &'a dyn LinearBrick,
    },
    LinearProcess {
      brick: &'a dyn LinearBrick,
      process_before_brick: &'a Process<'a, SplitParam>,
    },
    SplitProcess {
      brick: &'a dyn SplitterBrick<SplitParam>,
      process_before_brick: &'a Process<'a, SplitParam>,
    },
  }

  impl<'a, SplitParam: Param> Process<'a, SplitParam> {
    fn new(path: String, brick: &dyn LinearBrick) -> Process<'a, SplitParam> {
      Process::StartedProcess {
        path,
        brick,
      }
    }

    fn and_then(&self, brick: &dyn LinearBrick) -> Process<'a, SplitParam> {
      Process::LinearProcess {
        brick,
        process_before_brick: self,
      }
    }

    fn split(&self, brick: &dyn SplitterBrick<SplitParam>) -> Process<'a, SplitParam> {
      Process::SplitProcess {
        brick,
        process_before_brick: self,
      }
    }
  }
}