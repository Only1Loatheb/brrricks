use typenum::*;

use crate::brick::*;

// #[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]

pub(crate) struct InternalLinearBrick {
  pub name: &'static str,
  pub consumes: Vec<ParamId>,
  pub produces: Vec<ParamId>,
  pub handler: Box<dyn LinearBrickHandler>,
}

impl InternalLinearBrick {
  pub(crate) fn new<
    CONSUMES: ParamBitSet,
    REQUIRES: Unsigned,
    FORBIDS: Unsigned,
    PRODUCES: ParamBitSet,
    ACCOMPLISHES: Unsigned,
  >(
    brick: LinearBrick<CONSUMES, REQUIRES, FORBIDS, PRODUCES, ACCOMPLISHES>,
  ) -> InternalLinearBrick {
    InternalLinearBrick {
      name: brick.name,
      consumes: CONSUMES::get().0,
      produces: PRODUCES::get().0,
      handler: brick.handler,
    }
  }
}

// consider https://github.com/rust-phf/rust-phf for SplitIndex
pub(crate) struct InternalSplitterBrick {
  pub name: &'static str,
  pub consumes: Vec<ParamId>,
  // pub produces: Vec<Vec<ParamId>>,
  pub handler: Box<dyn SplitterBrickHandler>,
}

impl InternalSplitterBrick {
  pub(crate) fn new<
    SPLITS: ParamBitSet,
    CONSUMES: ParamBitSet,
    REQUIRES: Unsigned,
    FORBIDS: Unsigned,
  >(brick: SplitterBrick<SPLITS, CONSUMES, REQUIRES, FORBIDS>) -> InternalSplitterBrick {
    InternalSplitterBrick {
      name: brick.name,
      consumes: CONSUMES::get().0,
      // produces: brick
      //     .produces_and_accomplishes
      //     .into_iter()
      //     .map(|(_, params)| params)
      //     .collect(),
      handler: brick.handler,
    }
  }
}

pub(crate) struct InternalFinalBrick {
  pub name: &'static str,
  pub consumes: Vec<ParamId>,
  pub handler: Box<dyn FinalBrickHandler>,
}

impl InternalFinalBrick {
  pub(crate) fn new<
    CONSUMES: ParamBitSet,
    REQUIRES: Unsigned,
    FORBIDS: Unsigned,
    ACCOMPLISHES: Unsigned,
  >(
    brick: FinalBrick<CONSUMES, REQUIRES, FORBIDS, ACCOMPLISHES>,
  ) -> InternalFinalBrick {
    InternalFinalBrick {
      name: brick.name,
      consumes: CONSUMES::get().0,
      handler: brick.handler,
    }
  }
}
