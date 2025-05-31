pub mod builder;
mod hlist_concat;
mod hlist_transformer;
pub mod step;

pub mod process_builder {}

#[cfg(test)]
mod tests {
  use crate::builder::flowing_process::{EmptyProcess, FlowingProcess};
  use crate::step::step::Linear;
  use crate::step::ParamValue;
  use frunk_core::hlist::HNil;
  use process_builder_common::process_domain::Message;
  use serde::{Deserializer, Serializer};

  #[derive(Clone, serde::Deserialize, serde::Serialize)]
  struct Param1;

  impl ParamValue for Param1 {
    const NAME: &'static str = "Param1";
  }

  struct LinearA;

  impl Linear<HNil, HNil> for LinearA { async fn handle(&self, input: HNil) -> anyhow::Result<(Option<Message>, HNil)> { todo!() }}
  // impl Linear<HNil, HCons<Param1, HNil>> for LinearA { async fn handle(&self, input: HNil) -> anyhow::Result<(Option<Message>, HCons<Param1, HNil>)> {todo!()}}

  struct LinearB;

  impl Linear<HNil, HNil> for LinearB { async fn handle(&self, input: HNil) -> anyhow::Result<(Option<Message>, HNil)> { todo!() } }
  // impl Linear<HCons<Param1, HNil>, HNil> for LinearB { async fn handle(&self, input: HCons<Param1, HNil>) -> anyhow::Result<(Option<Message>, HNil)> { todo!() } }

  #[test]
  fn test_hcons() {
    let empty = EmptyProcess;
    let one_step = empty.then(LinearA);
    let two_steps = one_step.then(LinearB);
    assert_eq!(1, 1);
  }
}

//
// pub mod process_builder {
//   use std::marker::PhantomData;
//   use async_trait::async_trait;
//   use process_builder_common::brick_domain::*;
//   use crate::brick::*;
//   use crate::builder::*;
//   use crate::builder_helpers::*;
//
//   // assert_type_eq!(Uses, U33);
//   // assert_type_eq!(op!(U1 << U256), op!(pow(U2, U256)));
//
//   struct Linear;
//
//   #[async_trait]
//   impl LinearBrickHandler for Linear {
//     async fn handle(&self, input: InputParams) -> anyhow::Result<LinearOutput> {
//       todo!()
//     }
//   }
//
//   struct Splitter;
//
//   #[async_trait]
//   impl TypeSplitterBrickHandler<'same_process, U3, CASE_OTHER> for Splitter {
//     async fn handle(&self, input: InputParams) -> anyhow::Result<TypeSplitterOutput<U3>> {
//       todo!()
//     }
//   }
//
//   struct Final;
//
//   #[async_trait]
//   impl FinalBrickHandler for Final {
//     async fn handle(&self, input: InputParams) -> anyhow::Result<Message> {
//       todo!()
//     }
//   }
//
//   type Msisdn = U1;
//   type Dialed = U2;
//   type Both = op!(Msisdn | Dialed);
//   type SessionCharge = U1;
//   type BoEventSent = U2;
//   pub fn get_simple_process() {
//     let entry = LinearBrick {
//       name: "Entry".to_string(),
//       uses: PhantomData::<EMPTY>,
//       requires_prior_completion: PhantomData::<EMPTY>,
//       forbids_prior_completion: PhantomData::<EMPTY>,
//       produces: PhantomData::<Msisdn>,
//       accomplishes: PhantomData::<SessionCharge>,
//       handler: Box::new(Linear),
//     };
//     let linear = LinearBrick {
//       name: "Linear".to_string(),
//       uses: PhantomData::<EMPTY>,
//       requires_prior_completion: PhantomData::<EMPTY>,
//       forbids_prior_completion: PhantomData::<EMPTY>,
//       produces: PhantomData::<EMPTY>,
//       accomplishes: PhantomData::<EMPTY>,
//       handler: Box::new(Linear),
//     };
//     let linear2 = LinearBrick {
//       name: "Linear".to_string(),
//       uses: PhantomData::<EMPTY>,
//       requires_prior_completion: PhantomData::<EMPTY>,
//       forbids_prior_completion: PhantomData::<EMPTY>,
//       produces: PhantomData::<EMPTY>,
//       accomplishes: PhantomData::<EMPTY>,
//       handler: Box::new(Linear),
//     };
//     let linear3 = LinearBrick {
//       name: "Linear".to_string(),
//       uses: PhantomData::<EMPTY>,
//       requires_prior_completion: PhantomData::<EMPTY>,
//       forbids_prior_completion: PhantomData::<EMPTY>,
//       produces: PhantomData::<EMPTY>,
//       accomplishes: PhantomData::<EMPTY>,
//       handler: Box::new(Linear),
//     };
//     let splitter = SplitterBrick {
//       name: "Splitter".to_string(),
//       uses: PhantomData::<EMPTY>,
//       requires_prior_completion: PhantomData::<EMPTY>,
//       forbids_prior_completion: PhantomData::<EMPTY>,
//       produces_and_accomplishes: PhantomData::<TArr<(EMPTY, EMPTY), TArr<(EMPTY, EMPTY), TArr<(EMPTY, EMPTY), ATerm>>>>,
//       handler: Box::new(Splitter),
//     };
//     let last = FinalBrick {
//       name: "Final".to_string(),
//       uses: PhantomData::<Msisdn>,
//       requires_prior_completion: PhantomData::<SessionCharge>,
//       forbids_prior_completion: PhantomData::<BoEventSent>,
//       accomplishes: PhantomData::<EMPTY>,
//       handler: Box::new(Final),
//     };
//       process(entry)
//         .split(splitter, process(linear))
//         .next_case(process(linear2))
//         .last_case(process(linear3))
//         .finnish(last)
//         .close("my_process_name".to_string());
//     ()
//   }
// }
//
// #[cfg(test)]
// mod tests {
//   // use super::*;
//
//
//   // process(Box::new(Linear))
//   //   .and_then(Box::new(Linear))
//   //   .split(
//   //     Box::new(Splitter),
//   //     vec![empty_process(), process(Box::new(Linear))],
//   //   )
//   //   .split_finalized(
//   //     Box::new(Splitter),
//   //     vec![
//   //       finnish(Box::new(Final)),
//   //       process(Box::new(Linear)).finnish(Box::new(Final)),
//   //     ],
//   //   )
//   //   .close("aa")
//   #[test]
//   fn it_works() {
//     assert_eq!(true, true);
//   }
// }
