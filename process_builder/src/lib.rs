#![recursion_limit = "512"]

pub mod brick;
pub mod builder;
pub(crate) mod internal_brick;
pub(crate) mod split_index;
pub(crate) mod internal_process;
mod builder_helpers;

#[cfg_attr(doc, aquamarine::aquamarine)]
/// It is forbidden to overwrite param value
/// ```mermaid
/// stateDiagram-v2
///     [*] --> FinalizedProcess: FinalBrick
///     FinalizedProcess --> [*]
///     [*] --> FlowingProcess: LinearBrick
///     FlowingProcess --> FlowingProcess: LinearBrick\n or FlowingProcess
///     FlowingProcess --> FinalizedProcess: FinalBrick\n or FinalizedProcess
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
///     FlowingSplitProcess --> flowing_split_cases: FinalizedProcess\n or FlowingProcess
///     flowing_split_cases --> FlowingSplitProcess: some cases left
///     flowing_split_cases --> FlowingProcess: all cases handled
/// ```
pub mod process_builder {
  use std::marker::PhantomData;
  use async_trait::async_trait;
  use typenum::*;

  use crate::brick::*;
  use crate::builder::*;
  use crate::builder_helpers::*;

  // assert_type_eq!(Consumes, U33);
  // assert_type_eq!(op!(U1 << U256), op!(pow(U2, U256)));

  struct Linear;

  #[async_trait]
  impl LinearBrickHandler for Linear {
    async fn handle(&self, input: InputParams) -> anyhow::Result<LinearOutput> {
      todo!()
    }
  }

  struct Splitter;

  #[async_trait]
  impl SplitterBrickHandler for Splitter {
    async fn handle(&self, input: InputParams) -> anyhow::Result<SplitterOutput> {
      todo!()
    }
  }

  struct Final;

  #[async_trait]
  impl FinalBrickHandler for Final {
    async fn handle(&self, input: InputParams) -> anyhow::Result<Message> {
      todo!()
    }
  }

  type Msisdn = U1;
  type Dialed = U2;
  type Both = op!(Msisdn | Dialed);
  type SessionCharge = U1;
  type BoEventSent = U2;
  pub fn get_simple_process() -> NamedProcess {
    let linear = LinearBrick {
      name: "Linear",
      consumes: PhantomData::<EMPTY>,
      requires_prior_completion: PhantomData::<EMPTY>,
      forbids_prior_completion: PhantomData::<EMPTY>,
      produces: PhantomData::<Msisdn>,
      accomplishes: PhantomData::<SessionCharge>,
      handler: Box::new(Linear),
    };
    let last = FinalBrick {
      name: "Final",
      consumes: PhantomData::<Msisdn>,
      requires_prior_completion: PhantomData::<SessionCharge>,
      forbids_prior_completion: PhantomData::<BoEventSent>,
      accomplishes: PhantomData::<EMPTY>,
      handler: Box::new(Final),
    };
    process(linear)
      .finnish(last)
      .close("my_process_name")
  }
}

#[cfg(test)]
mod tests {
  // use super::*;

  //let splitter = SplitterBrick {
  //   name: "Splitter",
  //   splits: PhantomData::<U0>,
  //   consumes: PhantomData::<EMPTY>,
  //   requires_prior_completion: PhantomData::<EMPTY>,
  //   forbids_prior_completion: PhantomData::<EMPTY>,
  //   // produces_and_accomplishes: vec![],
  //   handler: Box::new(Splitter),
  // };

  // process(Box::new(Linear))
  //   .and_then(Box::new(Linear))
  //   .split(
  //     Box::new(Splitter),
  //     vec![empty_process(), process(Box::new(Linear))],
  //   )
  //   .split_finalized(
  //     Box::new(Splitter),
  //     vec![
  //       finnish(Box::new(Final)),
  //       process(Box::new(Linear)).finnish(Box::new(Final)),
  //     ],
  //   )
  //   .close("aa")
  #[test]
  fn it_works() {
    assert_eq!(true, true);
  }
}
