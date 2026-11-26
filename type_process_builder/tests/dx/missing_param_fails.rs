//@require-annotations-for-level: ERROR
use serde::{Deserialize, Serialize};
use type_process_builder::builder::*;
use type_process_builder::frunk::to_ref::ToRef;
use type_process_builder::step::{Operation, OperationOutcome, ProcessMessages};
use type_process_builder::{CNil, HList, HNil, impl_param_value};
use typenum::*;

#[derive(Deserialize, Serialize)]
struct ProducedParam;

#[derive(Deserialize, Serialize)]
struct MissingParam;

impl_param_value!(ProducedParam => U0);
impl_param_value!(MissingParam => U1);

struct Messages;
impl ProcessMessages for Messages {
  type FormMessage = String;
  type FinalMessage = String;
}

struct ConsumeMissingParam;
impl Operation for ConsumeMissingParam {
  type Consumes = HList![MissingParam];
  type Produces = HNil;
  type FinalMessage = String;
  type Error = CNil;

  async fn handle(
    &self,
    _consumes: <Self::Consumes as ToRef<'_>>::Ref,
  ) -> Result<OperationOutcome<Self::Produces, Self::FinalMessage>, Self::Error> {
    Ok(OperationOutcome::Successful(HNil))
  }
}

fn main() {
  let _ = entry::<HList![ProducedParam], Messages>().then(ConsumeMissingParam);
  //~^ ERROR: cannot extract required parameters
  //~| NOTE: A step or sub-process requires parameters that have not been produced by preceding steps in this process flow.
}
