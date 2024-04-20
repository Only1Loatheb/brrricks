struct RunnableProcess {
    runnable_linear_brick: Vec<RunnableLinearBrick>,
    runnable_splitter_brick: Vec<RunnableSplitterBrick>,
    runnable_final_brick: Vec<RunnableFinalBrick>,
}

struct RunnableBrickIndex(usize);

use process::brick_domain::*;
use process::internal_brick::*;
use std::collections::HashMap;

struct NextBrickTransition {
    params_to_delete: Vec<ParamId>,
    next_brick: RunnableBrickIndex,
}

struct RunnableLinearBrick {
    internal_linear_brick: InternalLinearBrick,
    next_brick_transition: NextBrickTransition,
}

struct RunnableSplitterBrick {
    internal_splitter_brick: InternalSplitterBrick,
    next_brick_transitions: HashMap<SplitIndex, NextBrickTransition>,
}

struct RunnableFinalBrick {
    internal_final_brick: InternalFinalBrick,
}

///
///
/// debug_assert!(next_brick_index < size_of_brick_array)
/// unsafe {
///   if next_brick_index >= size_of_brick_array {
///    std::hint::unreachable_unchecked()
///    }
/// }
///
///
