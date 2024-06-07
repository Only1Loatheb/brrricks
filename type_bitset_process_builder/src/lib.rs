#![recursion_limit = "512"]

pub mod brick;
pub mod builder;
pub(crate) mod split_index;
mod builder_helpers;

pub mod process_builder {
  use std::marker::PhantomData;
  use async_trait::async_trait;
  use typenum::*;
  use process::brick_domain::*;
  use process::internal_process::NamedProcess;

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
  impl TypeSplitterBrickHandler<U3> for Splitter {
    async fn handle(&self, input: InputParams) -> anyhow::Result<TypeSplitterOutput<U3>> {
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
  pub fn get_simple_process() {
    let entry = LinearBrick {
      name: "Entry".to_string(),
      consumes: PhantomData::<EMPTY>,
      requires_prior_completion: PhantomData::<EMPTY>,
      forbids_prior_completion: PhantomData::<EMPTY>,
      produces: PhantomData::<Msisdn>,
      accomplishes: PhantomData::<SessionCharge>,
      handler: Box::new(Linear),
    };
    let linear = LinearBrick {
      name: "Linear".to_string(),
      consumes: PhantomData::<EMPTY>,
      requires_prior_completion: PhantomData::<EMPTY>,
      forbids_prior_completion: PhantomData::<EMPTY>,
      produces: PhantomData::<EMPTY>,
      accomplishes: PhantomData::<EMPTY>,
      handler: Box::new(Linear),
    };
    let linear2 = LinearBrick {
      name: "Linear".to_string(),
      consumes: PhantomData::<EMPTY>,
      requires_prior_completion: PhantomData::<EMPTY>,
      forbids_prior_completion: PhantomData::<EMPTY>,
      produces: PhantomData::<EMPTY>,
      accomplishes: PhantomData::<EMPTY>,
      handler: Box::new(Linear),
    };
    let linear3 = LinearBrick {
      name: "Linear".to_string(),
      consumes: PhantomData::<EMPTY>,
      requires_prior_completion: PhantomData::<EMPTY>,
      forbids_prior_completion: PhantomData::<EMPTY>,
      produces: PhantomData::<EMPTY>,
      accomplishes: PhantomData::<EMPTY>,
      handler: Box::new(Linear),
    };
    let splitter = SplitterBrick {
      name: "Splitter".to_string(),
      consumes: PhantomData::<EMPTY>,
      requires_prior_completion: PhantomData::<EMPTY>,
      forbids_prior_completion: PhantomData::<EMPTY>,
      produces_and_accomplishes: PhantomData::<TArr<(EMPTY, EMPTY), TArr<(EMPTY, EMPTY), TArr<(EMPTY, EMPTY), ATerm>>>>,
      handler: Box::new(Splitter),
    };
    let last = FinalBrick {
      name: "Final".to_string(),
      consumes: PhantomData::<Msisdn>,
      requires_prior_completion: PhantomData::<SessionCharge>,
      forbids_prior_completion: PhantomData::<BoEventSent>,
      accomplishes: PhantomData::<EMPTY>,
      handler: Box::new(Final),
    };
      process(entry)
        .split(splitter, process(linear))
        .next_case(process(linear2))
        .last_case(process(linear3))
        .finnish(last)
        .close("my_process_name".to_string());
    ()
  }
}

#[cfg(test)]
mod tests {
  // use super::*;


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
