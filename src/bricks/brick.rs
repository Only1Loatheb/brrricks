pub mod brick {
  use std::iter::Map;
  use serde::{Deserialize, Serialize};

  pub trait Param<'de>: Serialize + Deserialize<'de> {}

  // consider https://github.com/rust-phf/rust-phf for SplitterBrick
  pub enum Brick<'de, Split, const ConsumesSize: usize>
    where Split: Sized
  {
    LinearBrick { name: String, consumes: [Box<dyn Param<'de>>; ConsumesSize], brick_type: BrickType },
    SplitterBrick { name: String, consumes: Map<Split, Box<dyn Param<'de>>>, brick_type: BrickType },
  }


  // mix it into brick enum????
  pub enum BrickType {
    FollowingBrick,
    FinalBrick(FinalBrick),
  }

  pub enum FinalBrick {
    ResultBrickType,
    RedirectionBrickType,
  }
}