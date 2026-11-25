//@require-annotations-for-level: ERROR
use serde::{Deserialize, Serialize};
use type_process_builder::builder::*;
use type_process_builder::frunk::to_ref::ToRef;
use type_process_builder::step::{Operation, OperationOutcome, ProcessMessages};
use type_process_builder::{CNil, HList, HNil, hlist, impl_param_value};
use typenum::*;

#[derive(Deserialize, Serialize)]
struct DuplicateParam;

impl_param_value!(DuplicateParam => U0);

struct Messages;
impl ProcessMessages for Messages {
  type FormMessage = String;
  type FinalMessage = String;
}

struct ProduceDuplicateParam;
impl Operation for ProduceDuplicateParam {
  type Consumes = HNil;
  type Produces = HList![DuplicateParam];
  type FinalMessage = String;
  type Error = CNil;

  async fn handle(
    &self,
    _consumes: <Self::Consumes as ToRef<'_>>::Ref,
  ) -> Result<OperationOutcome<Self::Produces, Self::FinalMessage>, Self::Error> {
    Ok(OperationOutcome::Successful(hlist!(DuplicateParam)))
  }
}

fn main() {
  let _ = entry::<HList![DuplicateParam], Messages>().then(ProduceDuplicateParam);
  //~^ ERROR: cannot overwrite parameter `DuplicateParam`: it was already produced by a previous step in this process flow
  //~| NOTE: The step being added produces parameter `DuplicateParam`, but a parameter with the same UID has already been produced by a preceding step.
  //~| ERROR: type mismatch resolving
}
