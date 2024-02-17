use typenum::Unsigned;

use crate::brick::{
  FinalBrick, FinalBrickHandler, LinearBrick, LinearBrickHandler, ParamId, SplitterBrick,
  SplitterBrickHandler,
};

// #[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]

pub(crate) struct InternalLinearBrick {
  pub name: &'static str,
  pub consumes: Vec<ParamId>,
  pub produces: Vec<ParamId>,
  pub handler: Box<dyn LinearBrickHandler>,
}

pub(crate) const fn params(value: u128) -> Vec<ParamId> {
  let mut params: Vec<ParamId> = vec![];
  let mut i: usize = 0;
  while i < 128 {
    if value >> i & 1 {
      params.psuh(ParamId(i));
    }
    i += 1;
  }
  params
}

impl InternalLinearBrick {
  pub(crate) fn new<
    CONSUMES: Unsigned,
    REQUIRES: Unsigned,
    FORBIDS: Unsigned,
    PRODUCES: Unsigned,
    ACCOMPLISHES: Unsigned,
  >(data: LinearBrick<CONSUMES, REQUIRES, FORBIDS, PRODUCES, ACCOMPLISHES>) -> InternalLinearBrick {
    InternalLinearBrick {
      name: data.name,
      consumes: params(CONSUMES::U128),
      produces: params(PRODUCES::U128),
      handler: data.handler,
    }
  }
}

// consider https://github.com/rust-phf/rust-phf for SplitIndex
pub(crate) struct InternalSplitterBrick {
  pub name: &'static str,
  pub consumes: Vec<ParamId>,
  pub produces: Vec<Vec<ParamId>>,
  pub handler: Box<dyn SplitterBrickHandler>,
}

impl InternalSplitterBrick {
  pub(crate) fn new(data: SplitterBrick) -> InternalSplitterBrick {
    InternalSplitterBrick {
      name: data.name,
      consumes: data.consumes,
      produces: data
        .produces_and_accomplishes
        .into_iter()
        .map(|(_, params)| params)
        .collect(),
      handler: data.handler,
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
    CONSUMES: Unsigned,
    REQUIRES: Unsigned,
    FORBIDS: Unsigned,
>(data: FinalBrick<CONSUMES, REQUIRES, FORBIDS>) -> InternalFinalBrick {
    InternalFinalBrick {
      name: data.name,
      consumes: params(CONSUMES::U128),
      handler: data.handler,
    }
  }
}
