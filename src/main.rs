use crate::bricks::brick::brick::{BrickData, LinearBrick, LinearBrickData, Param, ParamDeserializationError, ParamSerializationError};

pub mod bricks;
pub mod process;

struct AParam;

impl Param for AParam {
  fn name() -> String {
    "AParamName".to_string()
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

struct BParam;

impl Param for BParam {
  fn name() -> String {
    "BParamName".to_string()
  }

  fn serialize(&self) -> Result<String, ParamSerializationError> {
    Ok("B".to_string())
  }

  fn deserialize(serialized: &str) -> Result<BParam, ParamDeserializationError> {
    match serialized {
      "B" => Ok(BParam),
      _ => Err(ParamDeserializationError { value: "ParamDeserializationError".to_string() }),
    }
  }
}

struct LBrick;

impl LinearBrick for LBrick {
  fn data(&self) -> LinearBrickData {
    LinearBrickData {
      brick_data: BrickData {
        name: "LBrick".to_string(),
        consumes: vec![],
        not_produced_before: vec![],
      },
      produces: vec![],
    }
  }
}

fn main() {
  println!("Hello, world!");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    assert_eq!(LBrick.data().brick_data.name, "LBrick".to_string());
  }
}
