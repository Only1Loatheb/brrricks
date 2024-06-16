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
        let next_brick_idx = RunnableProcess::add_final_brick(&mut process, finalBrick);
        process.add_process(next_brick_idx, rest)
      }
      InternalFinalizedProcess::Split(_) => { todo!("handle split at the end of the process") }
    }
  }

  fn add_process(mut self, next_brick_idx: usize, rest: InternalFlowingProcess) -> RunnableProcess {
    match rest {
      InternalFlowingProcess::Empty => {
        self
      }
      InternalFlowingProcess::Linear(linear_brick, rest) => {
        let this_brick_idx = self.add_linear_brick(
          linear_brick,
          NextBrickTransition { next_brick: RunnableBrickIndex(next_brick_idx) },
        );
        self.add_process(this_brick_idx, *rest)
      }
      InternalFlowingProcess::Split(flowing_split_process) => {
        self.add_split_process(*flowing_split_process, next_brick_idx)
      }
    }
  }

  fn add_split_process(mut self, flowing_split_process: InternalFlowingSplitProcess, after_split_brick_index: usize) ->
  RunnableProcess {
    match flowing_split_process {
      InternalFlowingSplitProcess::FirstCase { splitter_brick, first_case, process_before } => {
        assert!(false, "only one option in split, use linear process instead");
        let mut runnable_process = self.add_process(after_split_brick_index, first_case);
        let len = runnable_process.runnable_bricks.len();
        let next_brick_idx = runnable_process.add_splitter_brick(
          splitter_brick,
          vec![NextBrickTransition { next_brick: RunnableBrickIndex(len) }],
        );
        runnable_process.add_process(next_brick_idx, process_before)
      }
      InternalFlowingSplitProcess::NextCase { next_case, split_process_before } => {
        self
      }
      InternalFlowingSplitProcess::NextCaseFlowing { next_case, split_process_before } => {
        self
      }
      InternalFlowingSplitProcess::NextCaseFinalized { next_case, split_process_before } => {
        self
      }
    }
  }

  fn add_linear_brick(&mut self, linear_brick: InternalLinearBrick, next: NextBrickTransition) -> usize {
    let this_brick_idx = self.runnable_bricks.len();
    self.runnable_bricks.push(RunnableLinearBrick(linear_brick, next));
    this_brick_idx
  }

  fn add_splitter_brick(
    &mut self,
    splitter_brick: InternalSplitterBrick,
    next_brick_transitions: Vec<NextBrickTransition>,
  ) -> usize {
    let this_brick_idx = self.runnable_bricks.len();
    self.runnable_bricks.push(RunnableSplitterBrick(splitter_brick, next_brick_transitions));
    this_brick_idx
  }

  fn add_final_brick(&mut self, final_brick: InternalFinalBrick) -> usize {
    let this_brick_idx = self.runnable_bricks.len();
    self.runnable_bricks.push(RunnableFinalBrick(final_brick));
    this_brick_idx
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
