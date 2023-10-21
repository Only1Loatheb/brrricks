// pub mod process {
//   use std::collections::HashMap;
//   use frunk::{HNil, HCons, hlist};
//   use crate::bricks::brick::brick::{FinalBrick, LinearBrick, Param, SplitterBrick};
//
//   // think about brick <Error>
//
//   pub fn empty_process() -> FlowingProcess<HNil> {
//     FlowingProcess::NoOp
//   }
//
//   pub fn process(brick: &dyn LinearBrick) -> FlowingProcess<HNil> {
//     FlowingProcess::Linear {
//       0: FlowingLinearProcess { brick, process_before_brick: &FlowingProcess::NoOp },
//     }
//   }
//
//   pub fn finnish(brick: &dyn FinalBrick) -> FinalizedProcess<HNil> {
//     FinalizedProcess::Linear {
//       0: FinalizedLinearProcess { brick, process_before_brick: &FlowingProcess::NoOp },
//     }
//   }
//
//   pub struct FlowingLinearProcess<'a, SplitsBefore> {
//     pub brick: &'a dyn LinearBrick,
//     pub process_before_brick: &'a FlowingProcess<'a, SplitsBefore>,
//   }
//
//   impl<SplitsBefore> FlowingLinearProcess<'_, SplitsBefore> {
//     pub fn finnish(self, brick: &dyn FinalBrick) -> FinalizedProcess<SplitsBefore> {
//       FinalizedProcess::Linear {
//         0: FinalizedLinearProcess { brick, process_before_brick: &FlowingProcess::Linear(self) },
//       }
//     }
//   }
//
//   pub struct FlowingSplitProcess<'a, SplitParam: Param, SplitsBefore> {
//     pub brick: &'a dyn SplitterBrick<SplitParam>,
//     pub cases: HashMap<SplitParam, FlowingProcess<'a, ???>>, // some could be finalized
//     pub process_before_brick: &'a FlowingProcess<'a, SplitsBefore>,
//   }
//
//   impl<SplitParam: Param, SplitsBefore> FlowingSplitProcess<'_, SplitParam, SplitsBefore> {
//     pub fn finnish(self, brick: &dyn FinalBrick) -> FinalizedProcess<SplitsBefore> {
//       FinalizedProcess::Linear {
//         0: FinalizedLinearProcess { brick, process_before_brick: &FlowingProcess::Split(self) },
//       }
//     }
//   }
//
//   pub enum FlowingProcess<'a, SplitsBefore> {
//     NoOp,
//     Linear(FlowingLinearProcess<'a, SplitsBefore>),
//     Split(FlowingSplitProcess<'a, SplitsBefore>),
//   }
//
//   pub struct FinalizedLinearProcess<'a, SplitsBefore> {
//     pub brick: &'a dyn FinalBrick,
//     pub process_before_brick: &'a FlowingProcess<'a, SplitsBefore>,
//   }
//
//   pub struct FinalizedSplitProcess<'a, SplitParam: Param, SplitsBefore> {
//     pub brick: &'a dyn SplitterBrick<SplitParam>,
//     pub cases: HashMap<SplitParam, Box<dyn FinalizedProcessAAA>>,
//     pub process_before_brick: &'a FlowingProcess<'a, SplitsBefore>,
//   }
//
//   pub trait FinalizedProcessAAA {
//
//   }
//
//   pub enum FinalizedProcess<'a, SplitsBefore> {
//     Linear(FinalizedLinearProcess<'a, SplitsBefore>),
//     Split(FinalizedSplitProcess<'a, SplitsBefore>),
//   }
//
//   impl<SplitsBefore> FlowingProcess<'_, SplitsBefore> {
//     pub fn finnish(&self, brick: &dyn FinalBrick) -> FinalizedProcess<SplitsBefore> {
//       match self {
//         FlowingProcess::NoOp => finnish(brick),
//         FlowingProcess::Linear(process) => process.finnish(brick),
//         FlowingProcess::Split(process) => todo!()
//       }
//     }
//
//     pub fn and_then(&self, brick: &dyn LinearBrick) -> FlowingProcess<SplitsBefore> {
//       FlowingProcess::Linear {
//         0: FlowingLinearProcess { brick, process_before_brick: self }
//       }
//     }
//
//     pub fn split<SplitParam: Param>(
//       &self,
//       brick: &dyn SplitterBrick<SplitParam>,
//       cases: HashMap<SplitParam, FlowingProcess<SplitsBefore>>,
//     ) -> FlowingProcess<SplitsBefore> {
//       FlowingProcess::Split {
//         0: FlowingSplitProcess { brick, cases, process_before_brick: &self },
//       }
//     }
//
//     pub fn split_finalized<SplitParam: Param>(
//       &self,
//       brick: &dyn SplitterBrick<SplitParam>,
//       cases: HashMap<SplitParam, FinalizedProcess<SplitsBefore>>,
//     ) -> FinalizedProcess<SplitsBefore> {
//       FinalizedProcess::Split {
//         0: FinalizedSplitProcess { brick, cases, process_before_brick: self }
//       }
//     }
//   }
//
//   pub struct Named<'a, SplitsBefore> {
//     pub path: &'a str,
//     pub process: &'a FinalizedProcess<'a, SplitsBefore>,
//   }
//
//   impl<SplitsBefore> FinalizedProcess<'_, SplitsBefore> {
//     pub fn close(&self, path: &str) -> Named<SplitsBefore> {
//       Named { path, process: self }
//     }
//   }
// }