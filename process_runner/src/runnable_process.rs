struct RunnableBrickIndex(usize);

use process::internal_brick::*;
use process::internal_process::{InternalFinalizedProcess, InternalFlowingProcess, InternalFlowingSplitProcess, NamedProcess};
use crate::runnable_process::RunnableBrick::*;

struct NextBrickTransition {
  // params_to_delete: Vec<ParamId>,
  next_brick: RunnableBrickIndex,
}

enum RunnableBrick {
  RunnableLinearBrick(InternalLinearBrick, NextBrickTransition),
  RunnableSplitterBrick(InternalSplitterBrick, Vec<NextBrickTransition>),
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
        let idx = process.add_final_brick(finalBrick);
        process.continue_flowing(idx, rest)
      }
      InternalFinalizedProcess::Split(_) => { todo!("handle split at the end of the process") }
    }
  }

  fn continue_flowing(mut process: Self, idx: usize, rest: InternalFlowingProcess) -> RunnableProcess {
    match rest {
      InternalFlowingProcess::Empty => {
        process
      }
      InternalFlowingProcess::Linear(linear_brick, rest) => {
        let idx = process.add_linear_brick(linear_brick, NextBrickTransition { next_brick: RunnableBrickIndex(idx) });
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

  fn add_linear_brick(process: &mut Self, linear_brick: InternalLinearBrick, next: NextBrickTransition) -> usize {
    let this = process.runnable_bricks.len();
    process.runnable_bricks.push(RunnableLinearBrick(linear_brick, next));
    this
  }

  fn add_splitter_brick(
    process: &mut Self,
    splitter_brick: InternalSplitterBrick,
    next_brick_transitions: Vec<NextBrickTransition>,
  ) -> usize {
    let this = process.runnable_bricks.len();
    process.runnable_bricks.push(RunnableSplitterBrick(splitter_brick, next_brick_transitions));
    this
  }

  fn add_final_brick(process: &mut Self, final_brick: InternalFinalBrick) -> usize {
    let this = process.runnable_bricks.len();
    process.runnable_bricks.push(RunnableFinalBrick(final_brick));
    this
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
