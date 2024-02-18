use typenum::*;

use crate::brick::*;

// #[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]

pub(crate) struct InternalLinearBrick {
  pub name: &'static str,
  pub consumes: Vec<ParamId>,
  pub produces: Vec<ParamId>,
  pub handler: Box<dyn LinearBrickHandler>,
}

fn get_params(value: Vec<bool>) -> Vec<ParamId> {
  let mut params = vec![];
  for (index, is_present) in value.iter().rev().enumerate() {
    if *is_present {
      params.push(ParamId(index))
    }
  }
  params
}

impl InternalLinearBrick {
  pub(crate) fn new<
    CONSUMES: TypeLevelSet,
    REQUIRES: TypeLevelSet,
    FORBIDS: TypeLevelSet,
    PRODUCES: TypeLevelSet,
    ACCOMPLISHES: TypeLevelSet,
  >(
    brick: LinearBrick<CONSUMES, REQUIRES, FORBIDS, PRODUCES, ACCOMPLISHES>,
  ) -> InternalLinearBrick {
    InternalLinearBrick {
      name: brick.name,
      consumes: get_params(CONSUMES::get()),
      produces: get_params(PRODUCES::get()),
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
    SPLITS: TypeLevelSet,
    CONSUMES: TypeLevelSet,
    REQUIRES: TypeLevelSet,
    FORBIDS: TypeLevelSet,
  >(brick: SplitterBrick<SPLITS, CONSUMES, REQUIRES, FORBIDS>) -> InternalSplitterBrick {
    InternalSplitterBrick {
      name: brick.name,
      consumes: get_params(CONSUMES::get()),
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
    CONSUMES: TypeLevelSet,
    REQUIRES: TypeLevelSet,
    FORBIDS: TypeLevelSet,
    ACCOMPLISHES: TypeLevelSet,
  >(
    brick: FinalBrick<CONSUMES, REQUIRES, FORBIDS, ACCOMPLISHES>,
  ) -> InternalFinalBrick {
    InternalFinalBrick {
      name: brick.name,
      consumes: get_params(CONSUMES::get()),
      handler: brick.handler,
    }
  }
}
