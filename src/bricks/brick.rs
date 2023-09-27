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
    pub data: BrickData,
    pub produces: Vec<Box<dyn Param>>,
  }

  // consider https://github.com/rust-phf/rust-phf for SplitterBrick
  pub struct SplitterBrickData<SplitParam: Param> {
    pub data: BrickData,
    pub produces: HashMap<SplitParam, Vec<Box<dyn Param>>>,
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