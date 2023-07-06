pub mod brick {
  use std::iter::Map;

  pub struct ParamSerializationError;

  pub struct ParamDeserializationError;

  pub trait Param {
    type T;

    fn name(&self) -> String;

    fn serialize(&self, t: Self::T) -> Result<String, ParamSerializationError>;

    fn deserialize(&self, serialized: &str) -> Result<Self::T, ParamDeserializationError>;
  }

  // consider https://github.com/rust-phf/rust-phf for SplitterBrick
  pub enum Brick<AppParams, Split: Param>
  {
    LinearBrick {
      name: String,
      consumes: Vec<Box<dyn Param<T=AppParams>>>,
      produces: Vec<Box<dyn Param<T=AppParams>>>,
    },
    FinalBrick {
      name: String,
      consumes: Vec<Box<dyn Param<T=AppParams>>>,
    },
    SplitterBrick {
      name: String,
      consumes: Vec<Box<dyn Param<T=AppParams>>>,
      produces: Map<Split, Vec<Box<dyn Param<T=AppParams>>>>,
    },
  }
}