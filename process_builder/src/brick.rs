use std::collections::HashMap;
use std::future::Future;

// #[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]

pub struct ParamId(pub usize);

pub struct ActionId(pub usize);

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

pub trait LinearBrick {
  fn data(&self) -> LinearBrickData;
  fn handle(&self, input: HashMap<ParamId, serde_json::value::Value>)
    -> impl Future<Output=anyhow::Result<HashMap<ParamId, serde_json::value::Value>>>;
}

pub trait SplitterBrick {
  fn data(&self) -> SplitterBrickData;
  fn handle(&self, input: HashMap<ParamId, serde_json::value::Value>)
    -> impl Future<Output=anyhow::Result<(SplitIndex, HashMap<ParamId, serde_json::value::Value>)>>;
}

pub trait FinalBrick {
  fn data(&self) -> FinalBrickData;
  fn handle(&self, input: HashMap<ParamId, serde_json::value::Value>)
    -> impl Future<Output=anyhow::Result<()>>;
}
