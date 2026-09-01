use serde::{Deserialize, Serialize};
use type_process_builder::builder::*;
use type_process_builder::frunk::to_ref::ToRef;
use type_process_builder::step::{Entry, Operation, OperationOutcome, ProcessMessages};
use type_process_builder::{HList, HNil, hlist, impl_param_value};
use typenum::*;

#[derive(Deserialize, Serialize)]
struct DuplicateParam;

impl_param_value!(DuplicateParam => U0);

struct Messages;
impl ProcessMessages for Messages {
  type FormMessage = String;
  type FinalMessage = String;
}

struct MyEntry;
impl Entry for MyEntry {
  type Produces = HList![DuplicateParam];
  type Messages = Messages;

  async fn handle(&self, _consumes: SessionContext, _initial_input: String) -> anyhow::Result<HList![DuplicateParam]> {
    Ok(hlist!(DuplicateParam))
  }
}

struct ProduceDuplicateParam;
impl Operation for ProduceDuplicateParam {
  type Consumes = HNil;
  type Produces = HList![DuplicateParam];
  type FinalMessage = String;

  async fn handle(
    &self,
    _consumes: <Self::Consumes as ToRef<'_>>::Ref,
  ) -> anyhow::Result<OperationOutcome<Self::Produces, Self::FinalMessage>> {
    Ok(OperationOutcome::Successful(hlist!(DuplicateParam)))
  }
}

fn main() {
  let _ = MyEntry.then(ProduceDuplicateParam);
}
