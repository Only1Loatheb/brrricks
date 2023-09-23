use crate::bricks::brick::brick::{BrickKind, Param, ParamDeserializationError, ParamSerializationError};

pub mod bricks;
pub mod process;

struct A;

impl Param for A {
  fn name() -> String {
    "AParamName".to_string()
  }

  fn serialize(&self) -> Result<String, ParamSerializationError> {
    Ok("A".to_string())
  }

  fn deserialize(serialized: &str) -> Result<A, ParamDeserializationError> {
    match serialized {
      "A" => Ok(A),
      _ => Err(ParamDeserializationError),
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
      _ => Err(ParamDeserializationError),
    }
  }
}

struct LBrick;


impl Brick for LBrick {
  type Kind = ();

  fn name() -> String {
    "LBrick".to_string()
  }

  fn consumes() -> Vec<Box<dyn Param>> {
    vec![]
  }

  fn not_produced_before() -> Vec<Box<dyn Param>> {
    vec![]
  }
}

impl LinearBrick for LBrick {
  fn produces() -> Vec<Box<dyn Param>> {
    vec![]
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
    let b: dyn Brick = LBrick;
    assert_eq!(b, 4);
  }
}
