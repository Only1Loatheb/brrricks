//@require-annotations-for-level: ERROR
use std::fmt;
use type_process_builder::builder::*;
use type_process_builder::frunk::to_ref::ToRef;
use type_process_builder::step::{Operation, OperationOutcome, ProcessMessages};
use type_process_builder::{Coprod, HNil, impl_step_error};
use typenum::*;

#[derive(Debug)]
struct ErrorA;

impl fmt::Display for ErrorA {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str("error A")
  }
}

impl std::error::Error for ErrorA {}

#[derive(Debug)]
struct ErrorB;

impl fmt::Display for ErrorB {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str("error B")
  }
}

impl std::error::Error for ErrorB {}

// Separate invocations accidentally using the same UID `U0` for two distinct error types
impl_step_error!(ErrorA => U0);
impl_step_error!(ErrorB => U0);

struct Messages;
impl ProcessMessages for Messages {
  type FormMessage = String;
  type FinalMessage = String;
}

struct Step1;
impl Operation for Step1 {
  type Consumes = HNil;
  type Produces = HNil;
  type FinalMessage = String;
  type Error = Coprod!(ErrorA);

  async fn handle(
    &self,
    _consumes: <Self::Consumes as ToRef<'_>>::Ref,
  ) -> Result<OperationOutcome<Self::Produces, Self::FinalMessage>, Self::Error> {
    Ok(OperationOutcome::Successful(HNil))
  }
}

struct Step2;
impl Operation for Step2 {
  type Consumes = HNil;
  type Produces = HNil;
  type FinalMessage = String;
  type Error = Coprod!(ErrorB);

  async fn handle(
    &self,
    _consumes: <Self::Consumes as ToRef<'_>>::Ref,
  ) -> Result<OperationOutcome<Self::Produces, Self::FinalMessage>, Self::Error> {
    Ok(OperationOutcome::Successful(HNil))
  }
}

fn main() {
  let _ = entry::<HNil, Messages>().then(Step1).then(Step2);
  //~^ ERROR: cannot inject `ErrorB` into error coproduct
  //~| ERROR: cannot inject `ErrorB` into error coproduct
  //~| NOTE: If merging step errors across steps, ensure `ErrorB` does not share the same `StepError::UID` with a different error type
}
