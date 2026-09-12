#![allow(clippy::unused_async_trait_impl)]
mod unit;
use serde::{Deserialize, Serialize};
use type_process_builder::builder::*;
use type_process_builder::frunk::to_ref::ToRef;
use type_process_builder::step::*;
use type_process_builder::{Coprod, HList, HNil, hlist, impl_param_value};
use typenum::*;
use unit::test_process_messages;

#[derive(Deserialize, Serialize)]
struct EntryParam(pub u64);

#[derive(Deserialize, Serialize)]
struct CaseOptionParam(pub u8);

#[derive(Deserialize, Serialize)]
struct Branch1Param(pub u64);

#[derive(Deserialize, Serialize)]
struct SharedParam(pub u64);

impl_param_value! {
  EntryParam => U0,
  CaseOptionParam => U1,
  Branch1Param => U2,
  SharedParam => U3,
}

struct Messages;
impl ProcessMessages for Messages {
  type FormMessage = String;
  type FinalMessage = String;
}

struct ChooseCaseForm;
impl Form for ChooseCaseForm {
  type CreateFormConsumes = HNil;
  type ValidateInputConsumes = HNil;
  type Produces = HList![CaseOptionParam];
  type Context = ();
  type Messages = Messages;

  async fn create_form(
    &self,
    _consumes: <Self::CreateFormConsumes as ToRef<'_>>::Ref,
    _back_token: Option<BackToken>,
  ) -> anyhow::Result<FormWithContext<String, ()>> {
    Ok(FormWithContext("Choose a case".into(), ()))
  }

  async fn handle_input(
    &self,
    _consumes: <Self::ValidateInputConsumes as ToRef<'_>>::Ref,
    input: String,
    _context: (),
    _back_token: Option<BackToken>,
  ) -> anyhow::Result<InputValidation<Self::Produces, Messages, ()>> {
    let option = input.parse::<u8>().unwrap_or(1);
    Ok(InputValidation::Successful(hlist!(CaseOptionParam(option))))
  }
}

pub struct Case1;
pub struct Case2;

struct SelectCase;
impl Splitter for SelectCase {
  type Consumes = HList![CaseOptionParam];
  type Produces = Coprod![(Case1, HNil), (Case2, HNil)];

  async fn handle(&self, consumes: <Self::Consumes as ToRef<'_>>::Ref) -> anyhow::Result<Self::Produces> {
    Ok(match consumes.head.0 {
      1 => Self::Produces::inject((Case1, HNil)),
      _ => Self::Produces::inject((Case2, HNil)),
    })
  }
}

struct ProduceBranch1Data;
impl Operation for ProduceBranch1Data {
  type Consumes = HNil;
  type Produces = HList![Branch1Param, SharedParam];
  type FinalMessage = String;

  async fn handle(
    &self,
    _consumes: <Self::Consumes as ToRef<'_>>::Ref,
  ) -> anyhow::Result<OperationOutcome<Self::Produces, Self::FinalMessage>> {
    Ok(OperationOutcome::Successful(hlist!(Branch1Param(0x1111), SharedParam(0x9999))))
  }
}

struct ProduceBranch2Data;
impl Operation for ProduceBranch2Data {
  type Consumes = HNil;
  type Produces = HList![SharedParam];
  type FinalMessage = String;

  async fn handle(
    &self,
    _consumes: <Self::Consumes as ToRef<'_>>::Ref,
  ) -> anyhow::Result<OperationOutcome<Self::Produces, Self::FinalMessage>> {
    Ok(OperationOutcome::Successful(hlist!(SharedParam(0x9999))))
  }
}

struct Branch1Form;
impl Form for Branch1Form {
  type CreateFormConsumes = HList![Branch1Param];
  type ValidateInputConsumes = HNil;
  type Produces = HNil;
  type Context = ();
  type Messages = Messages;

  async fn create_form(
    &self,
    consumes: <Self::CreateFormConsumes as ToRef<'_>>::Ref,
    _back_token: Option<BackToken>,
  ) -> anyhow::Result<FormWithContext<String, ()>> {
    Ok(FormWithContext(format!("Branch 1: {:#X}", consumes.head.0), ()))
  }

  async fn handle_input(
    &self,
    _consumes: <Self::ValidateInputConsumes as ToRef<'_>>::Ref,
    input: String,
    _context: (),
    back_token: Option<BackToken>,
  ) -> anyhow::Result<InputValidation<HNil, Messages, ()>> {
    if input == "0"
      && let Some(t) = back_token
    {
      Ok(InputValidation::Back(t))
    } else {
      Ok(InputValidation::Successful(HNil))
    }
  }
}

struct PostMergeForm;
impl Form for PostMergeForm {
  type CreateFormConsumes = HList![SharedParam];
  type ValidateInputConsumes = HNil;
  type Produces = HNil;
  type Context = ();
  type Messages = Messages;

  async fn create_form(
    &self,
    consumes: <Self::CreateFormConsumes as ToRef<'_>>::Ref,
    _back_token: Option<BackToken>,
  ) -> anyhow::Result<FormWithContext<String, ()>> {
    Ok(FormWithContext(format!("Post merge: {:#X}", consumes.head.0), ()))
  }

  async fn handle_input(
    &self,
    _consumes: <Self::ValidateInputConsumes as ToRef<'_>>::Ref,
    input: String,
    _context: (),
    back_token: Option<BackToken>,
  ) -> anyhow::Result<InputValidation<HNil, Messages, ()>> {
    if input == "0"
      && let Some(t) = back_token
    {
      Ok(InputValidation::Back(t))
    } else {
      Ok(InputValidation::Successful(HNil))
    }
  }
}

struct FinalStep;
impl Final for FinalStep {
  type Consumes = HNil;
  type FinalMessage = String;

  async fn handle(&self, _consumes: Self::Consumes) -> anyhow::Result<String> {
    Ok("Done".into())
  }
}

#[tokio::test]
async fn test_back_into_branch_after_implicit_merge_purge() {
  let process = entry::<HList![EntryParam], Messages>()
    .show(ChooseCaseForm)
    .split(SelectCase)
    .case_via(Case1, |x| x.then(ProduceBranch1Data).show(Branch1Form))
    .case_via(Case2, |x| x.then(ProduceBranch2Data))
    .show(PostMergeForm)
    .end(FinalStep)
    .build("", 0);

  test_process_messages(
    &process,
    session_init_value(),
    vec![
      "",
      "Choose a case",
      "1",
      "Branch 1: 0x1111",
      "next",
      "Post merge: 0x9999",
      "0", // Back at PostMergeForm -> navigates back to Branch1Form with restored branch session_context
      "Branch 1: 0x1111",
    ],
  )
  .await;
}

fn session_init_value() -> SessionContext {
  hlist!(EntryParam(123)).serialize_param_list().unwrap()
}
