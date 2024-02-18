#![recursion_limit = "256"]

use typenum::*;

pub mod brick;
pub mod builder;
pub(crate) mod internal_brick;
pub(crate) mod split_index;
pub(crate) mod internal_process;
mod builder_helpers;

#[cfg_attr(doc, aquamarine::aquamarine)]
/// ```mermaid
/// stateDiagram-v2
///     [*] --> FinalizedProcess: FinalBrick
///     FinalizedProcess --> [*]
///     [*] --> FlowingProcess: LinearBrick
///     FlowingProcess --> FlowingProcess: LinearBrick
///     FlowingProcess --> FinalizedProcess: FinalBrick
///     [*] --> FinalizedSplitProcess: SplitBrick
///     FlowingProcess --> FinalizedSplitProcess: SplitBrick
///     state finalized_split_cases_final <<choice>>
///     FinalizedSplitProcess --> finalized_split_cases_final: FinalizedProcess
///     finalized_split_cases_final --> FinalizedSplitProcess: some cases left
///     finalized_split_cases_final --> FinalizedProcess: all cases handled
///     state finalized_split_cases_linear <<choice>>
///     FinalizedSplitProcess --> finalized_split_cases_linear: FlowingProcess
///     finalized_split_cases_linear --> FlowingSplitProcess: some cases left
///     finalized_split_cases_linear --> FlowingProcess: all cases handled
///     state flowing_split_cases <<choice>>
///     FlowingSplitProcess --> flowing_split_cases: FinalizedProcess
///     FlowingSplitProcess --> flowing_split_cases: FlowingProcess
///     flowing_split_cases --> FlowingSplitProcess: some cases left
///     flowing_split_cases --> FlowingProcess: all cases handled
/// ```
pub mod process_builder {}

type MyParam = U1;
type HisParam = U32;

type Consumes = op!(MyParam | HisParam);

assert_type_eq!(Consumes, U33);

// pub mod process_builder {
//     use async_trait::async_trait;
//     use std::collections::HashMap;
//
//     use serde::{Deserialize, Serialize};
//     use serde_json::Value;
//
//     use crate::brick::{
//         FinalBrick, FinalBrickHandler, LinearBrick, LinearBrickHandler, ParamId, SplitIndex,
//         SplitterBrick, SplitterBrickHandler,
//     };
//     use crate::builder::{empty_process, finnish, process, NamedProcess};
//
//     #[derive(Serialize, Deserialize)]
//     struct AParam;
//
//     #[derive(Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
//     pub enum SplitP {
//         Bar,
//         Foo,
//     }
//
//     struct Linear;
//
//     #[async_trait]
//     impl LinearBrickHandler for Linear {
//         fn data(&self) -> LinearBrick {
//             LinearBrick {
//                 name: "Linear",
//                 consumes: vec![],
//                 requires_prior_completion: vec![],
//                 forbids_prior_completion: vec![],
//                 produces: vec![],
//                 accomplishes: vec![],
//             }
//         }
//
//         async fn handle(
//             &self,
//             input: HashMap<ParamId, Value>,
//         ) -> anyhow::Result<HashMap<ParamId, Value>> {
//             todo!()
//         }
//     }
//
//     struct Splitter;
//
//     #[async_trait]
//     impl SplitterBrickHandler for Splitter {
//         fn data(&self) -> SplitterBrick {
//             SplitterBrick {
//                 name: "Splitter",
//                 consumes: vec![],
//                 requires_prior_completion: vec![],
//                 forbids_prior_completion: vec![],
//                 produces_and_accomplishes: vec![],
//             }
//         }
//
//         async fn handle(
//             &self,
//             input: HashMap<ParamId, Value>,
//         ) -> anyhow::Result<(SplitIndex, HashMap<ParamId, Value>)> {
//             todo!()
//         }
//     }
//
//     struct Final;
//
//     #[async_trait]
//     impl FinalBrickHandler for Final {
//         fn data(&self) -> FinalBrick {
//             FinalBrick {
//                 name: "Final",
//                 consumes: vec![],
//                 requires_prior_completion: vec![],
//                 forbids_prior_completion: vec![],
//             }
//         }
//
//         async fn handle(&self, input: HashMap<ParamId, Value>) -> anyhow::Result<()> {
//             todo!()
//         }
//     }
//
//     // pub const fn
//     pub fn get_simple_process() -> NamedProcess {
//         process(Box::new(Linear))
//             .and_then(Box::new(Linear))
//             .split(
//                 Box::new(Splitter),
//                 vec![empty_process(), process(Box::new(Linear))],
//             )
//             .split_finalized(
//                 Box::new(Splitter),
//                 vec![
//                     finnish(Box::new(Final)),
//                     process(Box::new(Linear)).finnish(Box::new(Final)),
//                 ],
//             )
//             .close("aa")
//     }
// }

#[cfg(test)]
mod tests {
  // use super::*;

  #[test]
  fn it_works() {
    assert_eq!(true, true);
  }
}
