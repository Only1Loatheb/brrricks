use std::collections::HashMap;

use async_trait::async_trait;

// #[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]

#[derive(Clone)]
pub struct ParamId(pub usize);

#[derive(Clone)]
pub struct ActionId(pub usize);

#[derive(Clone)]
pub struct SplitIndex(pub usize);

#[derive(Clone)]
pub struct LinearBrickData {
  pub name: &'static str,
  pub consumes: Vec<ParamId>,
  pub requires_prior_completion: Vec<ActionId>,
  pub forbids_prior_completion: Vec<ActionId>,
  pub produces: Vec<ParamId>,
  pub accomplishes: Vec<ActionId>,
}

#[derive(Clone)]
pub struct SplitterBrickData {
  pub name: &'static str,
  pub consumes: Vec<ParamId>,
  pub requires_prior_completion: Vec<ActionId>,
  pub forbids_prior_completion: Vec<ActionId>,
  pub produces_and_accomplishes: Vec<(Vec<ActionId>, Vec<ParamId>)>, // consider https://github.com/rust-phf/rust-phf for SplitIndex
}

#[derive(Clone)]
pub struct FinalBrickData {
  pub name: &'static str,
  pub consumes: Vec<ParamId>,
  pub requires_prior_completion: Vec<ActionId>,
  pub forbids_prior_completion: Vec<ActionId>,
}

#[async_trait]
pub trait LinearBrick {
  fn data(&self) -> LinearBrickData;
  async fn handle(&self, input: HashMap<ParamId, serde_json::value::Value>)
                  -> anyhow::Result<HashMap<ParamId, serde_json::value::Value>>;
}

#[async_trait]
pub trait SplitterBrick {
  fn data(&self) -> SplitterBrickData;
  async fn handle(&self, input: HashMap<ParamId, serde_json::value::Value>)
                  -> anyhow::Result<(SplitIndex, HashMap<ParamId, serde_json::value::Value>)>;
}

#[async_trait]
pub trait FinalBrick {
  fn data(&self) -> FinalBrickData;
  async fn handle(&self, input: HashMap<ParamId, serde_json::value::Value>)
                  -> anyhow::Result<()>;
}
