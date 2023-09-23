#![feature(generic_const_exprs)]
pub mod brick {
  use std::iter::Map;
  // use serde::{Deserialize, Serialize}; serde(l)

  pub struct ParamSerializationError;

  pub struct ParamDeserializationError;

  pub trait Param {
    fn name() -> String where Self: Sized;
    fn serialize(&self) -> Result<String, ParamSerializationError>;
    fn deserialize(serialized: &str) -> Result<Self, ParamDeserializationError> where Self: Sized;
  }

  // consider https://github.com/rust-phf/rust-phf for SplitterBrick
  pub enum BrickKind<SplitParam: Param>
  {
    LinearBrick {
      name: String,
      consumes: Vec<Box<dyn Param>>,
      produces: Vec<Box<dyn Param>>,
      not_produced_before: Vec<Box<dyn Param>>,
    },
    SplitterBrick {
      name: String,
      consumes: Vec<Box<dyn Param>>,
      produces: Map<SplitParam, Vec<Box<dyn Param>>>,
      not_produced_before: Vec<Box<dyn Param>>,
    },
  }


}