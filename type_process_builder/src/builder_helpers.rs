use serde::Serializer;
use crate::brick::*;
use crate::builder::*;
use process_builder_common::internal_process::*;

// should be method in this builder
pub fn empty_process<'same_process>() -> FlowingProcess<'same_process> {
  FlowingProcess {
    process: InternalFlowingProcess::Empty,
    next_param_id: 0,
    same_process_invariant: Default::default(),
  }
}

// should be method in this builder
pub fn process<'same_process, CONSUMES: ParamReprList<'same_process>, PRODUCES: ParamReprList<'same_process>>(
  brick: LinearBrick<'same_process, CONSUMES, CONSUMES>,
) -> FlowingProcess<'same_process> {
  FlowingProcess {
    process: InternalFlowingProcess::Linear(
      brick.to_internal(),
      Box::new(InternalFlowingProcess::Empty),
    ),
    next_param_id: 0, // todo
    same_process_invariant: Default::default(),
  }
}

// split

// split_finnish

pub fn finnish<'same_process, CONSUMES: ParamReprList<'same_process>>(
  brick: FinalBrick<'same_process, CONSUMES>,
) -> FinalizedProcess<'same_process> {
  FinalizedProcess {
    process: InternalFinalizedProcess::Flowing(brick.to_internal(), InternalFlowingProcess::Empty),
    next_param_id: 0, // todo
    same_process_invariant: Default::default(),
  }
}
