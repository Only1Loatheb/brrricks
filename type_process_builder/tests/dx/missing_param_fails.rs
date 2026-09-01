use serde::{Deserialize, Serialize};
use type_process_builder::builder::*;
use type_process_builder::frunk::to_ref::ToRef;
use type_process_builder::step::{Entry, Operation, OperationOutcome, ProcessMessages};
use type_process_builder::{HList, HNil, hlist, impl_param_value};
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

struct MyEntry;
impl Entry for MyEntry {
  type Produces = HList![ProducedParam];
  type Messages = Messages;

  async fn handle(&self, _consumes: SessionContext, _initial_input: String) -> anyhow::Result<HList![ProducedParam]> {
    Ok(hlist!(ProducedParam))
  }
}

struct ConsumeMissingParam;
impl Operation for ConsumeMissingParam {
  type Consumes = HList![MissingParam];
  type Produces = HNil;
  type FinalMessage = String;

  async fn handle(
    &self,
    _consumes: <Self::Consumes as ToRef<'_>>::Ref,
  ) -> anyhow::Result<OperationOutcome<Self::Produces, Self::FinalMessage>> {
    Ok(OperationOutcome::Successful(HNil))
  }
}

fn main() {
  let _ = MyEntry.then(ConsumeMissingParam);
}
