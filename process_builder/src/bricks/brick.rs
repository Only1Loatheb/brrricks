pub mod brick {

  // use serde::{Deserialize, Serialize};
  // #[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
  // #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]

  #[derive(Clone)]
  pub struct ParamSerializationError {
    pub value: String,
  }

  #[derive(Clone)]
  pub struct ParamDeserializationError {
    pub value: String,
  }

  pub trait Param {
    fn name() -> &'static str where Self: Sized;
    fn serialize(&self) -> Result<String, ParamSerializationError>;
    fn deserialize(serialized: &str) -> Result<Self, ParamDeserializationError> where Self: Sized;
  }

  #[derive(Clone, Hash, PartialEq, Eq)]
  pub struct SplitIndex {
    pub value: usize,
  }

  pub trait SplitParam: Param {
    fn split_index(&self) -> SplitIndex;
  }

  #[derive(Clone)]
  pub struct BrickData {
    pub name: &'static str,
    pub consumes: Vec<&'static dyn Param>,
    pub not_produced_before: Vec<&'static dyn Param>,
  }

  #[derive(Clone)]
  pub struct LinearBrickData {
    pub base: BrickData,
    pub produces: Vec<&'static dyn Param>,
  }

  pub const MAX_PARAMS_SIZE: usize = 10;

  #[derive(Clone)]// consider https://github.com/rust-phf/rust-phf for SplitterBrick
  pub struct SplitterBrickData {
    pub base: BrickData,
    pub(crate) produces: phf::Map<i32, [Option<dyn Param + 'static>; MAX_PARAMS_SIZE]>,
  }

  impl SplitterBrickData {
    pub fn new(base: BrickData, produces: phf::Map<i32, [Option<dyn Param + 'static>; MAX_PARAMS_SIZE]>) -> SplitterBrickData {
      SplitterBrickData {
        base: base,
        produces: produces
          .into_iter()
          .map(|(key, value)| (key.split_index(), value))
          .collect(),
      }
    }
  }

  // add fn handle()
  pub trait LinearBrick {
    fn data(&self) -> LinearBrickData;
  }

  pub trait SplitterBrick {
    fn data(&self) -> SplitterBrickData;
  }

  pub trait FinalBrick {
    fn data(&self) -> BrickData;
  }
}