use std::collections::HashMap;
use async_trait::async_trait;

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct ParamId(pub usize);

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct ActionId(pub usize);

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(PartialEq, Debug, Eq, Clone, PartialOrd, Ord, Hash)]
pub struct Message(pub String);

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct SplitIndex(pub usize);

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(PartialEq, Debug, Eq, Clone)]
pub struct InputParams(pub HashMap<ParamId, serde_json::value::Value>);

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(PartialEq, Debug, Eq, Clone)]
pub struct OutputParams(pub HashMap<ParamId, serde_json::value::Value>);

// process breaking requires explicit split with final brick
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(PartialEq, Debug, Eq, Clone)]
pub struct LinearOutput(pub Option<Message>, pub OutputParams);

// process breaking requires explicit split with final brick
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(PartialEq, Debug, Eq, Clone)]
pub struct SplitterOutput(pub SplitIndex, pub OutputParams);

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
