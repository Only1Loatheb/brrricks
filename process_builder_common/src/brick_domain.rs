use std::collections::HashMap;
use async_trait::async_trait;
use serde::{Serialize, Serializer};
use serde::de::DeserializeOwned;

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct ParamId(pub usize);

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct ActionId(pub usize);

#[derive(PartialEq, Debug, Eq, Clone, PartialOrd, Ord, Hash)]
pub struct Message(pub String);

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct SplitIndex(pub usize);

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
