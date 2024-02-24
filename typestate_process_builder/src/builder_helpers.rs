use typenum::*;

use crate::brick::{ParamBitSet, FinalBrick, LinearBrick};
use crate::builder::*;
use process::internal_process::{InternalFinalizedProcess, InternalFlowingProcess};

pub fn empty_process() -> FlowingProcess<EMPTY, EMPTY, EMPTY, EMPTY, EMPTY> {
  FlowingProcess {
    process: InternalFlowingProcess::Empty,
    consumes: Default::default(),
    requires: Default::default(),
    forbids: Default::default(),
    produces: Default::default(),
    accomplishes: Default::default(),
  }
}

pub fn process<
  BRICK_CONSUMES: ParamBitSet,
  BRICK_REQUIRES: Unsigned,
  BRICK_FORBIDS: Unsigned,
  BRICK_PRODUCES: ParamBitSet,
  BRICK_ACCOMPLISHES: Unsigned,
>(
  brick: LinearBrick<BRICK_CONSUMES, BRICK_REQUIRES, BRICK_FORBIDS, BRICK_PRODUCES, BRICK_ACCOMPLISHES>,
) -> FlowingProcess<BRICK_CONSUMES, BRICK_REQUIRES, BRICK_FORBIDS, BRICK_PRODUCES, BRICK_ACCOMPLISHES> {
  FlowingProcess {
    process: InternalFlowingProcess::Linear(
      brick.to_internal(),
      Box::new(InternalFlowingProcess::Empty),
    ),
    consumes: Default::default(),
    requires: Default::default(),
    forbids: Default::default(),
    produces: Default::default(),
    accomplishes: Default::default(),
  }
}

// split

// split_finnish

pub fn finnish<
  BRICK_CONSUMES: ParamBitSet,
  BRICK_REQUIRES: Unsigned,
  BRICK_FORBIDS: Unsigned,
  BRICK_ACCOMPLISHES: Unsigned,
>(
  brick: FinalBrick<BRICK_CONSUMES, BRICK_REQUIRES, BRICK_FORBIDS, BRICK_ACCOMPLISHES>,
) -> FinalizedProcess<BRICK_CONSUMES, BRICK_REQUIRES, BRICK_FORBIDS, EMPTY, BRICK_ACCOMPLISHES> {
  FinalizedProcess {
    process: InternalFinalizedProcess::One(brick.to_internal()),
    consumes: Default::default(),
    requires: Default::default(),
    forbids: Default::default(),
    produces: Default::default(),
    accomplishes: Default::default(),
  }
}
