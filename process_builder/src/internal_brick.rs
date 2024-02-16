use crate::brick::{FinalBrick, FinalBrickHandler, LinearBrick, LinearBrickHandler, ParamId, SplitterBrick, SplitterBrickHandler};

// #[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]

pub(crate) struct InternalLinearBrick {
  pub name: &'static str,
  pub consumes: Vec<ParamId>,
  pub produces: Vec<ParamId>,
  pub handler: Box<dyn LinearBrickHandler>,
}

impl InternalLinearBrick {
  pub(crate) fn new(data: LinearBrick) -> InternalLinearBrick {
    InternalLinearBrick {
      name: data.name,
      consumes: data.consumes,
      produces: data.produces,
      handler: data.handler,
    }
  }
}

  // consider https://github.com/rust-phf/rust-phf for SplitIndex
pub(crate) struct InternalSplitterBrick {
  pub name: &'static str,
  pub consumes: Vec<ParamId>,
  pub produces: Vec<Vec<ParamId>>,
  pub handler: Box<dyn SplitterBrickHandler>,
}

impl InternalSplitterBrick {
  pub(crate) fn new(data: SplitterBrick) -> InternalSplitterBrick {
    InternalSplitterBrick {
      name: data.name,
      consumes: data.consumes,
      produces: data.produces_and_accomplishes.into_iter().map(|(_, params)| params).collect(),
      handler: data.handler,
    }
  }
}

pub(crate) struct InternalFinalBrick {
  pub name: &'static str,
  pub consumes: Vec<ParamId>,
  pub handler: Box<dyn FinalBrickHandler>,
}

impl InternalFinalBrick {
  pub(crate) fn new(data: FinalBrick) -> InternalFinalBrick {
    InternalFinalBrick {
      name: data.name,
      consumes: data.consumes,
      handler: data.handler,
    }
  }
}
