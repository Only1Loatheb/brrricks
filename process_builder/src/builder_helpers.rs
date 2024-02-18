use typenum::*;

use crate::brick::{FinalBrick, LinearBrick};
use crate::builder::*;
use crate::internal_brick::{InternalFinalBrick, InternalLinearBrick};
use crate::internal_process::{InternalFinalizedProcess, InternalFlowingProcess};

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
  BRICK_FORBIDS: Unsigned,
  BRICK_PRODUCES: Unsigned,
  BRICK_ACCOMPLISHES: Unsigned,
>(
  brick: LinearBrick<EMPTY, EMPTY, BRICK_FORBIDS, BRICK_PRODUCES, BRICK_ACCOMPLISHES>,
) -> FlowingProcess<EMPTY, EMPTY, BRICK_FORBIDS, BRICK_PRODUCES, BRICK_ACCOMPLISHES> {
  FlowingProcess {
    process: InternalFlowingProcess::Linear(
      InternalLinearBrick::new(brick),
      Box::new(InternalFlowingProcess::Empty),
    ),
    consumes: Default::default(),
    requires: Default::default(),
    forbids: Default::default(),
    produces: Default::default(),
    accomplishes: Default::default(),
  }
}

pub fn finnish<
  BRICK_FORBIDS: Unsigned,
  BRICK_ACCOMPLISHES: Unsigned,
>(
  brick: FinalBrick<EMPTY, EMPTY, BRICK_FORBIDS, BRICK_ACCOMPLISHES>,
) -> FinalizedProcess<EMPTY, EMPTY, BRICK_FORBIDS, EMPTY, BRICK_ACCOMPLISHES> {
  FinalizedProcess {
    process: InternalFinalizedProcess::One(InternalFinalBrick::new(brick)),
    consumes: Default::default(),
    requires: Default::default(),
    forbids: Default::default(),
    produces: Default::default(),
    accomplishes: Default::default(),
  }
}
