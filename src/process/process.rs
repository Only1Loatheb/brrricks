
pub mod process {
  use crate::bricks::brick::brick::Param;
  use crate::bricks::brick::brick::{LinearBrick, SplitterBrick};

  // think about brick <Error>
  pub enum Process<SplitParam: Param> {
    StartedProcess {
      path: String,
      brick: dyn LinearBrick,
    },
    LinearProcess {
      brick: dyn LinearBrick,
      process_before_brick: Process<SplitParam>,
    },
    SplitProcess {
      brick: dyn SplitterBrick<SplitParam>,
      process_before_brick: Process<SplitParam>,
    },
  }

  impl<SplitParam: Param> Process<SplitParam> {
    fn new(path: String, brick: Box<dyn LinearBrick>) -> Process<SplitParam> {
      Process::StartedProcess {
        path,
        brick,
      }
    }

    fn and_then(self, brick: Box<dyn LinearBrick>) -> Process<SplitParam> {
      Process::LinearProcess {
        brick,
        process_before_brick: self,
      }
    }

    fn split(self, brick: Box<dyn SplitterBrick<SplitParam>>) -> Process<SplitParam> {
      Process::SplitProcess {
        brick,
        process_before_brick: self,
      }
    }
  }
}