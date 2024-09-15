use crate::brick_domain::*;

// #[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]

pub struct InternalLinearBrick {
  pub name: String,                           // could be useful for metrics
  pub uses: Vec<ParamId>,                     // get rid of these, move them to checking
  pub produces: Vec<ParamId>,                 // get rid of these, move them to checking
  pub handler: Box<dyn LinearBrickHandler>,
}

// consider https://github.com/rust-phf/rust-phf for SplitIndex
pub struct InternalSplitterBrick {
  pub name: String,
  pub uses: Vec<ParamId>,                         // get rid of these, move them to checking
  pub produces: Vec<Vec<ParamId>>,                // get rid of these, move them to checking
  pub handler: Box<dyn SplitterBrickHandler>,
}

pub struct InternalFinalBrick {
  pub name: String,
  pub uses: Vec<ParamId>,                              // get rid of these, move them to checking
  pub handler: Box<dyn FinalBrickHandler>,
}
