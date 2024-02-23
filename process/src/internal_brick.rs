use crate::brick_domain::*;

// #[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]

pub struct InternalLinearBrick {
  pub name: &'static str,
  pub consumes: Vec<ParamId>,
  pub produces: Vec<ParamId>,
  pub handler: Box<dyn LinearBrickHandler>,
}

// consider https://github.com/rust-phf/rust-phf for SplitIndex
pub struct InternalSplitterBrick {
  pub name: &'static str,
  pub consumes: Vec<ParamId>,
  // pub produces: Vec<Vec<ParamId>>,
  pub handler: Box<dyn SplitterBrickHandler>,
}

pub struct InternalFinalBrick {
  pub name: &'static str,
  pub consumes: Vec<ParamId>,
  pub handler: Box<dyn FinalBrickHandler>,
}
