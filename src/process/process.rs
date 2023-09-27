pub mod process {
  use std::collections::HashMap;
  use crate::bricks::brick::brick::{FinalBrick, LinearBrick, Param, SplitterBrick};
  use crate::process::process::process::sealed::{Finalized, Following, Started};

  // think about brick <Error>
  pub struct OneBrickProcess<'a> {
    path: String,
    brick: &'a dyn FinalBrick,
  }
  pub struct StartedLinearProcess<'a> {
    path: String,
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
    path: String,
    brick: &'a dyn SplitterBrick<SplitParam>,
    cases: HashMap<SplitParam, dyn Following>,
  }

  pub struct OneSplitProcess<'a, SplitParam: Param> {
    path: String,
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

  impl<'a> Finalized for OneBrickProcess {}

  impl<'a> Finalized for FinalizedProcess {}

  impl<'a, SplitParam: Param> Finalized for OneSplitProcess<SplitParam> {}

  impl<'a, SplitParam: Param> Finalized for FinalizedSplitProcess<SplitParam> {}

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

    fn split(&self, brick: &dyn SplitterBrick<SplitParam>, cases: HashMap<SplitParam, Following<dyn Param>>) -> Following<'a, dyn Param> {
      Following::SplitProcess {
        brick,
        cases,
        process_before_brick: self,
      }
    }
  }

  impl Started for StartedLinearProcess {
    fn path(&self) -> &String {
      &self.path
    }
  }

}