pub mod brick {
  use std::iter::Map;
  use serde::{Deserialize, Serialize};

  pub trait Param: Serialize + Deserialize<'static> {}

  // consider https://github.com/rust-phf/rust-phf for SplitterBrick
  pub enum Brick<Split: Param>
  {
    LinearBrick {
      name: String,
      consumes: Vec<Box<dyn Param>>,
      produces: Vec<Box<dyn Param>>,
    },
    FinalBrick {
      name: String,
      consumes: Vec<Box<dyn Param>>,
    },
    SplitterBrick {
      name: String,
      consumes: Vec<Box<dyn Param>>,
      produces: Map<Split, Vec<Box<dyn Param>>>,
    },
  }
}