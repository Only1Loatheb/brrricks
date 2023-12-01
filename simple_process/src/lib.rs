use std::collections::HashMap;
use std::io::BufRead;
use crate::bricks::brick::brick::{BrickData, FinalBrick, LinearBrick, LinearBrickData, Param, ParamDeserializationError, ParamSerializationError, SplitIndex, SplitParam, SplitterBrick, SplitterBrickData};
use crate::process::process::process::*;

pub mod bricks;
pub mod process;

struct AParam;

impl Param for AParam {
  fn name() -> &'static str where Self: Sized {
    "AParam"
  }

  fn serialize(&self) -> Result<String, ParamSerializationError> {
    Ok("A".to_string())
  }

  fn deserialize(serialized: &str) -> Result<AParam, ParamDeserializationError> {
    match serialized {
      "A" => Ok(AParam),
      _ => Err(ParamDeserializationError { value: "ParamDeserializationError".to_string() }),
    }
  }
}

#[derive(Eq, PartialEq, Hash)]
pub enum SplitP {
  Bar,
  Foo,
}

impl Param for SplitP {

  fn name() -> &'static str where Self: Sized {
    "SplitP"
  }

  fn serialize(&self) -> Result<String, ParamSerializationError> {
    match self {
      SplitP::Bar => Ok("Bar".to_string()),
      SplitP::Foo => Ok("Foo".to_string()),
    }
  }

  fn deserialize(serialized: &str) -> Result<Self, ParamDeserializationError> where Self: Sized {
    match serialized {
      "Bar" => Ok(SplitP::Bar),
      "Foo" => Ok(SplitP::Foo),
      other => Err(ParamDeserializationError {value: other.to_string() })
    }
  }
}

impl SplitParam for SplitP {
  fn split_index(&self) -> SplitIndex {
    match self {
      SplitP::Bar => SplitIndex { value: 0 },
      SplitP::Foo => SplitIndex { value: 1 },
    }
  }
}

struct Linear;

impl LinearBrick for Linear {
  fn data(&self) -> LinearBrickData {
    LinearBrickData {
      base: BrickData {
        name: "Linear",
        consumes: vec![],
        not_produced_before: vec![],
      },
      produces: vec![],
    }
  }
}

struct Splitter;

impl SplitterBrick for Splitter {
  fn data(&self) -> SplitterBrickData {
    SplitterBrickData::new(
      BrickData {
        name: "Splitter",
        consumes: vec![],
        not_produced_before: vec![],
      },
      HashMap::from([
        (SplitP::Bar, vec![]),
        (SplitP::Foo, vec![]),
      ]),
    )
  }
}

struct Final;

impl FinalBrick for Final {
  fn data(&self) -> BrickData {
    BrickData {
      name: "Final",
      consumes: vec![],
      not_produced_before: vec![],
    }
  }
}
// pub const fn
pub fn get_simple_process() -> NamedProcess {
  process(Box::new(Linear))
    .and_then(Box::new(Linear))
    .split(
      Box::new(Splitter),
      HashMap::from([
        (SplitP::Bar, empty_process()),
        (SplitP::Foo, process(Box::new(Linear))),
      ]),
    )
    .split_finalized(
      Box::new(Splitter),
      HashMap::from([
        (SplitP::Bar, finnish(Box::new(Final))),
        (SplitP::Foo, process(Box::new(Linear)).finnish(Box::new(Final))),
      ]),
    )
    .close("aa")

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    assert_eq!(Linear.data().base.name, "Linear".to_string());
  }
}
