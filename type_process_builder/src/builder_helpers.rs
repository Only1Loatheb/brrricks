use typenum::*;

use crate::brick::*;
use crate::builder::*;
use process::internal_process::{InternalFinalizedProcess, InternalFlowingProcess};

pub fn empty_process<'same_process>() -> FlowingProcess<'same_process> {
  FlowingProcess {
    process: InternalFlowingProcess::Empty,
    next_param_id: 0,
    same_process_invariant: Default::default(),
  }
}

pub fn process<
  BRICK_USES: ParamBitSet,
  BRICK_REQUIRES: Unsigned,
  BRICK_FORBIDS: Unsigned,
  BRICK_PRODUCES: ParamBitSet,
  BRICK_ACCOMPLISHES: Unsigned,
>(
  brick: LinearBrick<BRICK_USES, BRICK_REQUIRES, BRICK_FORBIDS, BRICK_PRODUCES, BRICK_ACCOMPLISHES>,
) -> FlowingProcess<BRICK_USES, BRICK_REQUIRES, BRICK_FORBIDS, BRICK_PRODUCES, BRICK_ACCOMPLISHES> {
  FlowingProcess {
    process: InternalFlowingProcess::Linear(
      brick.to_internal(),
      Box::new(InternalFlowingProcess::Empty),
    ),
    uses: Default::default(),
    requires: Default::default(),
    forbids: Default::default(),
    produces: Default::default(),
    accomplishes: Default::default(),
  }
}

// split

// split_finnish

pub fn finnish<
  BRICK_USES: ParamBitSet,
  BRICK_REQUIRES: Unsigned,
  BRICK_FORBIDS: Unsigned,
  BRICK_ACCOMPLISHES: Unsigned,
>(
  brick: FinalBrick<BRICK_USES, BRICK_REQUIRES, BRICK_FORBIDS, BRICK_ACCOMPLISHES>,
) -> FinalizedProcess<BRICK_USES, BRICK_REQUIRES, BRICK_FORBIDS, EMPTY, BRICK_ACCOMPLISHES> {
  FinalizedProcess {
    process: InternalFinalizedProcess::Flowing(brick.to_internal(), InternalFlowingProcess::Empty),
    uses: Default::default(),
    requires: Default::default(),
    forbids: Default::default(),
    produces: Default::default(),
    accomplishes: Default::default(),
  }
}
