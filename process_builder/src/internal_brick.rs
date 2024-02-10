use std::collections::HashMap;

use async_trait::async_trait;
use crate::brick::{ActionId, FinalBrickData, LinearBrickData, ParamId, SplitIndex, SplitterBrickData};

// #[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]

#[derive(Clone)]
pub(crate) struct InternalLinearBrickData {
  pub name: &'static str,
  pub consumes: Vec<ParamId>,
  pub produces: Vec<ParamId>,
}

impl InternalLinearBrickData {
  pub(crate) fn new(data: LinearBrickData) -> InternalLinearBrickData {
    InternalLinearBrickData {
      name: data.name, consumes: data.consumes, produces: data.produces
    }
  }
}

#[derive(Clone)]
pub(crate) struct InternalSplitterBrickData {
  pub name: &'static str,
  pub consumes: Vec<ParamId>,
  pub produces: Vec<Vec<ParamId>>, // consider https://github.com/rust-phf/rust-phf for SplitIndex
}

impl InternalSplitterBrickData {
  impl InternalLinearBrickData {
  pub(crate) fn new(data: LinearBrickData) -> InternalLinearBrickData {
    InternalLinearBrickData {
      name: data.name, consumes: data.consumes, produces: data.produces
    }
  }
}
}

#[derive(Clone)]
pub(crate) struct InternalFinalBrickData {
  pub name: &'static str,
  pub consumes: Vec<ParamId>,
}

#[async_trait]
pub(crate) trait LinearBrick {
  fn data(&self) -> LinearBrickData;
  async fn handle(&self, input: HashMap<ParamId, serde_json::value::Value>)
                  -> anyhow::Result<HashMap<ParamId, serde_json::value::Value>>;
}

#[async_trait]
pub(crate) trait SplitterBrick {
  fn data(&self) -> SplitterBrickData;
  async fn handle(&self, input: HashMap<ParamId, serde_json::value::Value>)
                  -> anyhow::Result<(SplitIndex, HashMap<ParamId, serde_json::value::Value>)>;
}

#[async_trait]
pub(crate) trait FinalBrick {
  fn data(&self) -> FinalBrickData;
  async fn handle(&self, input: HashMap<ParamId, serde_json::value::Value>)
                  -> anyhow::Result<()>;
}
