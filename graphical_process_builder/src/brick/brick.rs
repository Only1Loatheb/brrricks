use async_trait::async_trait;
use process::brick_domain::{
  FinalBrickHandler, InputParams, LinearBrickHandler, LinearOutput, Message, ParamId, SplitterBrickHandler,
  SplitterOutput,
};
use process_builder_common::process_domain::{Message, ParamId, SplitIndex};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;

// was in common

trait Param: Serialize + DeserializeOwned {}

#[derive(PartialEq, Debug, Eq, Clone)]
pub struct InputParams(pub HashMap<ParamId, dyn Param>);

#[derive(PartialEq, Debug, Eq, Clone)]
pub struct OutputParams(pub HashMap<ParamId, dyn Param>);

// process breaking requires explicit split with final brick
#[derive(PartialEq, Debug, Eq, Clone)]
pub struct LinearOutput(pub Option<Message>, pub OutputParams);

// process breaking requires explicit split with final brick
#[derive(PartialEq, Debug, Eq, Clone)]
pub struct SplitterOutput(pub SplitIndex, pub OutputParams);

// there are no recoverable errors for now, split the process to handle error.
#[async_trait]
pub trait LinearBrickHandler {
  async fn handle(&self, input: InputParams) -> anyhow::Result<LinearOutput>;
}

#[async_trait]
pub trait SplitterBrickHandler {
  async fn handle(&self, input: InputParams) -> anyhow::Result<SplitterOutput>;
}

#[async_trait]
pub trait FinalBrickHandler {
  async fn handle(&self, input: InputParams) -> anyhow::Result<Message>;
}

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

pub enum InternalFlowingProcess {
  Empty,
  Linear {
    process_before: Box<InternalFlowingProcess>,
    last_step: InternalLinearBrick,
  },
  Split(Box<InternalFlowingSplitProcess>),
}

pub enum InternalFinalizedProcess {
  Flowing {
    process_before: InternalFlowingProcess,
    final_step: InternalFinalBrick,
  },
  Split(Box<InternalFinalizedSplitProcess>),
}

// We can allow one case for to support accept forms and confirm forms
pub enum InternalFinalizedSplitProcess {
  FirstCase {
    process_before: InternalFlowingProcess,
    splitter_brick: InternalSplitterBrick,
    first_cases: InternalFinalizedProcess,
  },
  NextCase {
    split_process_before: Box<InternalFinalizedSplitProcess>,
    next_case: InternalFinalizedProcess,
  },
}

// builder has to take care of having at least 2 cases
pub enum InternalFlowingSplitProcess {
  FirstCase {
    process_before: InternalFlowingProcess,
    splitter_brick: InternalSplitterBrick,
    first_case: InternalFlowingProcess,
  },
  NextCaseFlowing {
    split_process_before: Box<InternalFlowingSplitProcess>,
    next_case: InternalFlowingProcess,
  },
  NextCaseFinalized {
    split_process_before: Box<InternalFlowingSplitProcess>,
    next_case: InternalFinalizedProcess,
  },
  NextCaseFromFinalized {
    split_process_before: Box<InternalFinalizedSplitProcess>,
    next_case: InternalFlowingProcess,
  },
}

pub struct NamedProcess {
  pub path: String,
  pub process: InternalFinalizedProcess,
}

// the above was in common

#[derive(serde::Deserialize, serde::Serialize)]
pub enum Brick {
  LinearBrick {
    uses: Vec<ParamId>,
    produces: Vec<ParamId>,
    handler: Box<Linear>,
  },
  SplitterBrick {
    uses: Vec<ParamId>,
    produces: Vec<Vec<ParamId>>,
    handler: Box<Splitter>,
  },
  FinalBrick {
    uses: Vec<ParamId>,
    handler: Box<Final>,
  },
}

impl Default for Brick {
  fn default() -> Self {
    Brick::LinearBrick {
      uses: vec![],
      produces: vec![],
      handler: Box::new(Linear),
    }
  }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Linear;

#[async_trait]
impl LinearBrickHandler for Linear {
  async fn handle(&self, input: InputParams) -> anyhow::Result<LinearOutput> {
    todo!()
  }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Splitter;

#[async_trait]
impl SplitterBrickHandler for Splitter {
  async fn handle(&self, input: InputParams) -> anyhow::Result<SplitterOutput> {
    todo!()
  }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Final;

#[async_trait]
impl FinalBrickHandler for Final {
  async fn handle(&self, input: InputParams) -> anyhow::Result<Message> {
    todo!()
  }
}
