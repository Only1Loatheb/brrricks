use serde::{Deserialize, Deserializer, Serialize, Serializer};

use brick::{FinalBrick, LinearBrick, SplitterBrick};

mod brick;
mod builder;
mod split_index;

pub mod process_builder {
  use std::collections::HashMap;
  use std::future::Future;

  use serde::{Deserialize, Serialize};
  use serde_json::Value;

  use crate::brick::{FinalBrick, FinalBrickData, LinearBrick, LinearBrickData, ParamId, SplitterBrick, SplitterBrickData};
  use crate::builder::{empty_process, finnish, NamedProcess, process};

  #[derive(Serialize, Deserialize)]
  struct AParam;

  #[derive(Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
  pub enum SplitP {
    Bar,
    Foo,
  }

  struct Linear;

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

    fn handle(&self, input: HashMap<ParamId, Value>) -> impl Future<Output=anyhow::Result<HashMap<ParamId, Value>>> {
      todo!()
    }
  }

  struct Splitter;

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

    fn handle(&self, input: HashMap<ParamId, Value>) -> impl Future<Output=anyhow::Result<(crate::brick::SplitIndex, HashMap<ParamId, Value>)>> {
      todo!()
    }
  }

  struct Final;

  impl FinalBrick for Final {
    fn data(&self) -> FinalBrickData {
      FinalBrickData {
        name: "Final",
        consumes: vec![],
        requires_prior_completion: vec![],
        forbids_prior_completion: vec![],
      }
    }

    fn handle(&self, input: HashMap<ParamId, Value>) -> impl Future<Output=anyhow::Result<()>> {
      todo!()
    }
  }

  // pub const fn
  pub fn get_simple_process() -> NamedProcess {
    process(Linear.into())
      .and_then(Linear.into())
      .split(
        Splitter.into(),
        vec![empty_process(), process(Linear.into())],
      )
      .split_finalized(
        Splitter.into(),
        vec![finnish(Final.into()), process(Linear.into()).finnish(Final.into())])
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
