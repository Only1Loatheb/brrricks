use crate::brick_domain::*;

/// The workhorse of the brick architecture
pub struct InternalLinearBrick {
  pub name: String,
  pub deletes: Vec<ParamId>,
  pub handler: Box<dyn LinearBrickHandler>,
}

/// I think we can safely repeat the common deletes, because most of the params will be deleted before this brick
pub struct InternalSplitterBrick {
  pub name: String,
  pub case_specific_deletes: Vec<Vec<ParamId>>,
  pub handler: Box<dyn SplitterBrickHandler>,
}

/// All params should be deleted after running this brick.
/// There is no point in specifying them individually.
pub struct InternalFinalBrick {
  pub name: String,
  pub handler: Box<dyn FinalBrickHandler>,
}

// There should be a redirect brick, but its implementation is left as an exercise for the reader.
