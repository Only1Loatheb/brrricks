// use std::collections::HashMap;
// use crate::bricks::brick::brick::{BrickData, FinalBrick, LinearBrick, LinearBrickData, Param, ParamDeserializationError, ParamSerializationError, SplitterBrick, SplitterBrickData};
// use crate::process::process::process::*;
//
// pub mod bricks;
// pub mod process;
//
// struct AParam;
//
// impl Param for AParam {
//   fn name() -> String {
//     "AParamName".to_string()
//   }
//
//   fn serialize(&self) -> Result<String, ParamSerializationError> {
//     Ok("A".to_string())
//   }
//
//   fn deserialize(serialized: &str) -> Result<AParam, ParamDeserializationError> {
//     match serialized {
//       "A" => Ok(AParam),
//       _ => Err(ParamDeserializationError { value: "ParamDeserializationError".to_string() }),
//     }
//   }
// }
//
// pub enum SplitP {
//   Bar,
//   Foo,
// }
//
// impl Param for SplitP {
//   fn name() -> String {
//     "BParamName".to_string()
//   }
//
//   fn serialize(&self) -> Result<String, ParamSerializationError> {
//     Ok("B".to_string())
//   }
//
//   fn deserialize(serialized: &str) -> Result<SplitP, ParamDeserializationError> {
//     match serialized {
//       "B" => Ok(SplitP::Bar),
//       _ => Err(ParamDeserializationError { value: "ParamDeserializationError".to_string() }),
//     }
//   }
// }
//
// struct Linear;
//
// impl LinearBrick for Linear {
//   fn data(&self) -> LinearBrickData {
//     LinearBrickData {
//       base: BrickData {
//         name: "Linear",
//         consumes: vec![],
//         not_produced_before: vec![],
//       },
//       produces: vec![],
//     }
//   }
// }
//
// struct Splitter;
//
// impl SplitterBrick<SplitP> for Splitter {
//   fn data(&self) -> SplitterBrickData<SplitP> {
//     SplitterBrickData {
//       base: BrickData {
//         name: "Splitter",
//         consumes: vec![],
//         not_produced_before: vec![],
//       },
//       produces: HashMap::from([
//         (SplitP::Bar, vec![]),
//         (SplitP::Foo, vec![]),
//       ]),
//     }
//   }
// }
//
// struct Final;
//
// impl FinalBrick for Final {
//   fn data(&self) -> BrickData {
//     BrickData {
//       name: "Final",
//       consumes: vec![],
//       not_produced_before: vec![],
//     }
//   }
// }
//
// pub fn get_type_encoded_process() {
//   let a = process(&Linear)
//     .and_then(&Linear)
//     .split(
//       &Splitter,
//       HashMap::from([
//         (SplitP::Bar, empty_process()),
//         (SplitP::Foo, process(&Linear)),
//       ]),
//     )
//     .split_finalized(
//       &Splitter,
//       HashMap::from([
//         (SplitP::Bar, finnish(&Final)),
//         (SplitP::Foo, process(&Linear).finnish(&Final)),
//       ]),
//     );
//   dbg!("{}", a);
// }
//
// use frunk::{HCons, hlist, HNil};
// #[cfg(test)]
// mod tests {
//   use std::ops::Index;
//   use frunk::{HCons, HNil};
//   use frunk::hlist::Plucker;
//   use frunk::labelled::chars::T;
//   use super::*;
//
//
//   #[test]
//   fn it_works() {
//     let list: HCons<i32, HCons<&str, HCons<bool, HCons<f32, HNil>>>> = hlist![1, "hello", true, 42f32];
//     let (b, list): (bool, _) = list.pluck();
//     assert!(b);
//   }
// }
