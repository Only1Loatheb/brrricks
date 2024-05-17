struct RunnableBrickIndex(usize);

use process::brick_domain::*;
use process::internal_brick::*;
use process::internal_process::{InternalFinalizedProcess, InternalFlowingProcess, InternalFlowingSplitProcess, NamedProcess};
use std::collections::HashMap;
use crate::runnable_process::RunnableBrick::*;

struct NextBrickTransition {
  // params_to_delete: Vec<ParamId>,
  next_brick: RunnableBrickIndex,
}

enum RunnableBrick {
  RunnableLinearBrick(InternalLinearBrick, NextBrickTransition),
  RunnableSplitterBrick(InternalSplitterBrick, HashMap<SplitIndex, NextBrickTransition>),
  RunnableFinalBrick(InternalFinalBrick),
}

struct RunnableProcess {
  path: String,
  runnable_bricks: Vec<RunnableBrick>, // starts from last
}

impl RunnableProcess {
  pub fn new(named_process: NamedProcess) -> RunnableProcess {
    let mut process = RunnableProcess {
      path: named_process.path,
      runnable_bricks: vec![],
    };
    match named_process.process {
      InternalFinalizedProcess::Flowing(finalBrick, rest) => {
        let idx = process.runnable_bricks.len();
        process.runnable_bricks.push(RunnableFinalBrick(finalBrick));
        continue_flowing(idx, rest, process)
      }
      InternalFinalizedProcess::Split(_) => { process }
    }
  }
}

fn continue_flowing(idx: usize, rest: InternalFlowingProcess, mut process: RunnableProcess) -> RunnableProcess {
  match rest {
    InternalFlowingProcess::Empty => {
      process
    }
    InternalFlowingProcess::Linear(linear_brick, rest) => {
      let idx = process.runnable_bricks.len();
      process.runnable_bricks.push(RunnableLinearBrick(linear_brick, NextBrickTransition { next_brick: RunnableBrickIndex(idx) }));
      process
    }
    InternalFlowingProcess::Split(FlowingSplitProcess) => {
      match *FlowingSplitProcess {
        InternalFlowingSplitProcess::FirstCase { .. } => {}
        InternalFlowingSplitProcess::NextCase { .. } => {}
        InternalFlowingSplitProcess::NextCaseFlowing { .. } => {}
        InternalFlowingSplitProcess::NextCaseFinalized { .. } => {}
      }
      process
    }
  }
}

//
// debug_assert!(next_brick_index < size_of_brick_array)
// unsafe {
//   if next_brick_index >= size_of_brick_array {
//    std::hint::unreachable_unchecked()
//    }
// }
//
//
