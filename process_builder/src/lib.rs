use brick::{FinalBrick, LinearBrick, SplitterBrick};

pub mod builder;
pub(crate) mod brick;
pub(crate) mod split_index;

pub mod process_builder {
  use std::collections::HashMap;
  use async_trait::async_trait;

  use serde::{Deserialize, Serialize};
  use serde_json::Value;

  use crate::brick::{FinalBrick, FinalBrickData, LinearBrick, LinearBrickData, ParamId, SplitIndex, SplitterBrick, SplitterBrickData};
  use crate::builder::{empty_process, finnish, NamedProcess, process};

  #[derive(Serialize, Deserialize)]
  struct AParam;

  #[derive(Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
  pub enum SplitP {
    Bar,
    Foo,
  }

  struct Linear;

  #[async_trait]
  impl LinearBrick for Linear {
    fn data(&self) -> LinearBrickData {
      LinearBrickData {
        name: "Linear",
        consumes: vec![],
        requires_prior_completion: vec![],
        forbids_prior_completion: vec![],
        produces: vec![],
        accomplishes: vec![],
      }
    }

    async fn handle(&self, input: HashMap<ParamId, Value>) -> anyhow::Result<HashMap<ParamId, Value>> {
      todo!()
    }
  }

  struct Splitter;

  #[async_trait]
  impl SplitterBrick for Splitter {
    fn data(&self) -> SplitterBrickData {
      SplitterBrickData {
        name: "Splitter",
        consumes: vec![],
        requires_prior_completion: vec![],
        forbids_prior_completion: vec![],
        produces_and_accomplishes: vec![],
      }
    }

    async fn handle(&self, input: HashMap<ParamId, Value>) -> anyhow::Result<(SplitIndex, HashMap<ParamId, Value>)> {
      todo!()
    }
  }

  struct Final;

  #[async_trait]
  impl FinalBrick for Final {
    fn data(&self) -> FinalBrickData {
      FinalBrickData {
        name: "Final",
        consumes: vec![],
        requires_prior_completion: vec![],
        forbids_prior_completion: vec![],
      }
    }

    async fn handle(&self, input: HashMap<ParamId, Value>) -> anyhow::Result<()> {
      todo!()
    }
  }

  // pub const fn
  pub fn get_simple_process() -> NamedProcess {
    process(Box::new(Linear))
      .and_then(Box::new(Linear))
      .split(
        Box::new(Splitter),
        vec![empty_process(), process(Box::new(Linear))],
      )
      .split_finalized(
        Box::new(Splitter),
        vec![finnish(Box::new(Final)), process(Box::new(Linear)).finnish(Box::new(Final))])
      .close("aa")
  }
}

#[cfg(test)]
mod tests {
  // use super::*;

  #[test]
  fn it_works() {
    assert_eq!(true, true);
  }
}
