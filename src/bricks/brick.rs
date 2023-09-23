


pub mod brick {
  use std::iter::Map;
  use crate::bricks::brick::brick::sealed::BrickKind;
  // use serde::{Deserialize, Serialize}; serde(l)

  pub struct ParamSerializationError;

  pub struct ParamDeserializationError;

  pub trait Param {
    fn name() -> String where Self: Sized;
    fn serialize(&self) -> Result<String, ParamSerializationError>;
    fn deserialize(serialized: &str) -> Result<Self, ParamDeserializationError> where Self: Sized;
  }

  mod sealed {
    // https://predr.ag/blog/definitive-guide-to-sealed-traits-in-rust/
    pub trait BrickKind {}
  }

  pub trait LinearBrickKind: BrickKind {}

  pub trait SplitterBrickKind: BrickKind {}

  pub trait Brick {
    type Kind: BrickKind;
    fn name() -> String;
    fn consumes() -> Vec<Box<dyn Param>>;
    fn not_produced_before() -> Vec<Box<dyn Param>>;
  }

  pub trait LinearBrick: Brick {
    type Kind = dyn LinearBrickKind;
    fn produces() -> Vec<Box<dyn Param>>;
  }
  pub trait SplitterBrick<SplitParam: Param>: Brick {
    type Kind = dyn SplitterBrickKind;
    // consider produces() https://github.com/rust-phf/rust-phf for SplitterBrick
    fn produces() -> Map<SplitParam, Vec<Box<dyn Param>>>;
  }
}