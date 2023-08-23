pub mod process {
  use crate::bricks::brick::brick::{Param};
  use crate::bricks::brick::brick::Brick::{LinearBrick, SplitterBrick};

  // think about brick <Error>
  pub enum Process<SplitParam: Param> {
    ContinuationProcess {
      brick: LinearBrick,
      process_before_brick: Process<SplitParam>
    },
    SplitProcess {
      brick: SplitterBrick<SplitParam>,
      process_before_brick: Process<SplitParam>
    },
  }

  impl<SplitParam: Param> Process<SplitParam> {
    fn and_then(self, brick: LinearBrick) -> Process<SplitParam> {
      Process::ContinuationProcess {
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