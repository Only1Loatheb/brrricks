pub mod brick {
  use std::collections::HashMap;
// use serde::{Deserialize, Serialize}; serde(l)

  pub struct ParamSerializationError {
    pub value: String,
  }

  pub struct ParamDeserializationError {
    pub value: String,
  }

  pub trait Param {
    fn name() -> & 'static str where Self: Sized;
    fn serialize(&self) -> Result<String, ParamSerializationError>;
    fn deserialize(serialized: &str) -> Result<Self, ParamDeserializationError> where Self: Sized;
  }

  pub struct BrickData {
    pub name: & 'static str,
    pub consumes: Vec<& 'static dyn Param>,
    pub not_produced_before: Vec<& 'static dyn Param>,
  }

  pub struct LinearBrickData {
    pub base: BrickData,
    pub produces: Vec<& 'static dyn Param>,
  }

  // consider https://github.com/rust-phf/rust-phf for SplitterBrick
  pub struct SplitterBrickData<SplitParam: Param> {
    pub base: BrickData,
    pub produces: HashMap<SplitParam, Vec<& 'static dyn Param>>,
  }

  // add fn handle()
  pub trait LinearBrick {
    fn data(&self) -> LinearBrickData;
  }

  pub trait SplitterBrick<SplitParam: Param> {
    fn data(&self) -> SplitterBrickData<SplitParam>;
  }

  pub trait FinalBrick {
    fn data(&self) -> BrickData;
  }
}