pub mod process {
  use crate::bricks::brick::brick::{Param};
  use crate::bricks::brick::brick::BrickKind::{LinearBrick, SplitterBrick};

  // think about brick <Error>
  pub enum Process<SplitParam: Param> {
    StartedProcess {
      brick: LinearBrick,
    },
    LinearProcess {
      brick: LinearBrick,
      process_before_brick: Process<SplitParam>,
    },
    SplitProcess {
      brick: SplitterBrick<SplitParam>,
      process_before_brick: Process<SplitParam>,
    },
  }

  impl<SplitParam: Param> Process<SplitParam> {
    fn new() -> Process<SplitParam> {
      Process::StartedProcess {
        brick,
      }
    }

    fn and_then(self, brick: LinearBrick) -> Process<SplitParam> {
      Process::LinearProcess {
        brick,
        process_before_brick: self,
      }
    }

    fn split(self, brick: SplitterBrick) -> Process<SplitParam> {
      Process::SplitProcess {
        brick,
        process_before_brick: self,
      }
    }
  }
}