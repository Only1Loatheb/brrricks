pub mod brick {
  use std::iter::Map;

// use serde::{Deserialize, Serialize}; serde(l)

  pub struct ParamSerializationError {
    pub value: String,
  }

  pub struct ParamDeserializationError {
    pub value: String,
  }

  pub trait Param {
    fn name() -> String where Self: Sized;
    fn serialize(&self) -> Result<String, ParamSerializationError>;
    fn deserialize(serialized: &str) -> Result<Self, ParamDeserializationError> where Self: Sized;
  }

  pub struct BrickData {
    pub name: String,
    pub consumes: Vec<Box<dyn Param>>,
    pub not_produced_before: Vec<Box<dyn Param>>,
  }

  pub struct LinearBrickData {
    pub brick_data: BrickData,
    pub produces: Vec<Box<dyn Param>>,
  }

  pub trait LinearBrick {
    fn data(&self) -> LinearBrickData;
  }

  // consider https://github.com/rust-phf/rust-phf for SplitterBrick
  pub struct SplitterBrickData<SplitParam: Param> {
    pub brick_data: BrickData,
    pub produces: Map<SplitParam, Vec<Box<dyn Param>>>,
  }

  pub trait SplitterBrick<SplitParam: Param> {
    fn data(&self) -> SplitterBrickData<SplitParam>;
  }
}