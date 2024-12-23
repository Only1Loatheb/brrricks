use async_trait::async_trait;
use process::brick_domain::{
    FinalBrickHandler, InputParams, LinearBrickHandler, LinearOutput, Message, ParamId,
    SplitterBrickHandler, SplitterOutput,
};
use process_builder_common::brick_domain::{Message, ParamId, SplitIndex};
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

// builder has to take care of having at least 2 cases
pub enum InternalFinalizedSplitProcess {
    FirstCase {
        splitter_brick: InternalSplitterBrick,
        first_cases: InternalFinalizedProcess,
        process_before: InternalFlowingProcess,
    },
    NextCase {
        next_case: InternalFinalizedProcess,
        split_process_before: Box<InternalFinalizedSplitProcess>,
    },
}

// builder has to take care of having at least 2 cases
pub enum InternalFlowingSplitProcess {
    FirstCase {
        splitter_brick: InternalSplitterBrick,
        first_case: InternalFlowingProcess,
        process_before: InternalFlowingProcess,
    },
    NextCase {
        next_case: InternalFlowingProcess,
        split_process_before: Box<InternalFinalizedSplitProcess>,
    },
    NextCaseFlowing {
        next_case: InternalFlowingProcess,
        split_process_before: Box<InternalFlowingSplitProcess>,
    },
    NextCaseFinalized {
        next_case: InternalFinalizedProcess,
        split_process_before: Box<InternalFlowingSplitProcess>,
    },
}

pub enum InternalFlowingProcess {
    Empty,
    Linear(InternalLinearBrick, Box<InternalFlowingProcess>),
    Split(Box<InternalFlowingSplitProcess>),
}

pub enum InternalFinalizedProcess {
    Flowing(InternalFinalBrick, InternalFlowingProcess),
    Split(Box<InternalFinalizedSplitProcess>),
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
