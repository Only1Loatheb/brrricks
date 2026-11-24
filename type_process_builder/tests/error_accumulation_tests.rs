#![allow(clippy::unused_async_trait_impl)]
use std::error::Error;
use type_process_builder::builder::{
  FinalizedProcess, FlowingProcess, PreviousRunYieldedAt, ProcessError, RunOutcome, SplitProcess, StepIndex,
};
use type_process_builder::step::{
  BackToken, Final, Form, FormSplitter, FormWithContext, InputValidation, Operation, OperationOutcome, ProcessMessages,
  Splitter, entry,
};
use type_process_builder::{CNil, Coprod, Coproduct, HNil, ToRef, impl_step_error};
use typenum::{U0, U1, U2};

#[derive(Debug, PartialEq, Eq)]
struct Messages;
impl ProcessMessages for Messages {
  type FormMessage = String;
  type FinalMessage = String;
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("db error: {0}")]
struct DbError(&'static str);

#[derive(Debug, thiserror::Error)]
#[error("network error")]
struct NetworkError(#[source] std::io::Error);

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("validation error")]
struct ValidationError;

impl_step_error! {
  DbError => U0,
  NetworkError => U1,
  ValidationError => U2,
}

struct FailOpA(bool);
impl Operation for FailOpA {
  type Consumes = HNil;
  type Produces = HNil;
  type FinalMessage = String;
  type Error = Coprod![DbError];

  async fn handle(&self, _: <Self::Consumes as ToRef<'_>>::Ref) -> Result<OperationOutcome<HNil, String>, Self::Error> {
    if self.0 { Err(Coproduct::inject(DbError("op_a"))) } else { Ok(OperationOutcome::Successful(HNil)) }
  }
}

struct FailOpB(bool);
impl Operation for FailOpB {
  type Consumes = HNil;
  type Produces = HNil;
  type FinalMessage = String;
  // Intentionally repeats DbError (deduplicated via CoprodUnion) and adds NetworkError
  type Error = Coprod![DbError, NetworkError];

  async fn handle(&self, _: <Self::Consumes as ToRef<'_>>::Ref) -> Result<OperationOutcome<HNil, String>, Self::Error> {
    if self.0 {
      Err(Coproduct::inject(NetworkError(std::io::Error::other("timeout"))))
    } else {
      Ok(OperationOutcome::Successful(HNil))
    }
  }
}

struct FailForm {
  fail_create: bool,
  fail_input: bool,
}
impl Form for FailForm {
  type CreateFormConsumes = HNil;
  type ValidateInputConsumes = HNil;
  type Produces = HNil;
  type Context = ();
  type Messages = Messages;
  type Error = Coprod![ValidationError, DbError];

  async fn create_form(
    &self,
    _: <Self::CreateFormConsumes as ToRef<'_>>::Ref,
    _: Option<BackToken>,
  ) -> Result<FormWithContext<String, ()>, Self::Error> {
    if self.fail_create {
      Err(Coproduct::inject(DbError("form_create")))
    } else {
      Ok(FormWithContext("prompt".into(), ()))
    }
  }

  async fn handle_input(
    &self,
    _: <Self::ValidateInputConsumes as ToRef<'_>>::Ref,
    _: String,
    (): (),
    _: Option<BackToken>,
  ) -> Result<InputValidation<HNil, Messages, ()>, Self::Error> {
    if self.fail_input { Err(Coproduct::inject(ValidationError)) } else { Ok(InputValidation::Successful(HNil)) }
  }
}

struct Case1;
struct Case2;

struct FailSplitter {
  fail: bool,
  pick_case1: bool,
}
impl Splitter for FailSplitter {
  type Consumes = HNil;
  type Produces = Coprod![(Case1, HNil), (Case2, HNil)];
  type Error = Coprod![DbError];

  async fn handle(&self, _: <Self::Consumes as ToRef<'_>>::Ref) -> Result<Self::Produces, Self::Error> {
    if self.fail {
      Err(Coproduct::inject(DbError("splitter")))
    } else if self.pick_case1 {
      Ok(Self::Produces::inject((Case1, HNil)))
    } else {
      Ok(Self::Produces::inject((Case2, HNil)))
    }
  }
}

struct FailFormSplitter {
  fail_create: bool,
  fail_input: bool,
  pick_case1: bool,
}
impl FormSplitter for FailFormSplitter {
  type CreateFormConsumes = HNil;
  type ValidateInputConsumes = HNil;
  type Produces = Coprod![(Case1, HNil), (Case2, HNil)];
  type Context = ();
  type Messages = Messages;
  type Error = Coprod![ValidationError];

  async fn create_form(
    &self,
    _: <Self::CreateFormConsumes as ToRef<'_>>::Ref,
    _: Option<BackToken>,
  ) -> Result<FormWithContext<String, ()>, Self::Error> {
    if self.fail_create {
      Err(Coproduct::inject(ValidationError))
    } else {
      Ok(FormWithContext("split_prompt".into(), ()))
    }
  }

  async fn handle_input(
    &self,
    _: <Self::ValidateInputConsumes as ToRef<'_>>::Ref,
    _: String,
    (): (),
    _: Option<BackToken>,
  ) -> Result<InputValidation<Self::Produces, Messages, ()>, Self::Error> {
    if self.fail_input {
      Err(Coproduct::inject(ValidationError))
    } else if self.pick_case1 {
      Ok(InputValidation::Successful(Self::Produces::inject((Case1, HNil))))
    } else {
      Ok(InputValidation::Successful(Self::Produces::inject((Case2, HNil))))
    }
  }
}

struct FailFinal(bool);
impl Final for FailFinal {
  type Consumes = HNil;
  type FinalMessage = String;
  type Error = Coprod![DbError];

  async fn handle(&self, _: HNil) -> Result<String, Self::Error> {
    if self.0 { Err(Coproduct::inject(DbError("final"))) } else { Ok("done".into()) }
  }
}

fn empty_ctx() -> Vec<u8> {
  postcard::to_allocvec(&HNil).unwrap()
}

#[tokio::test]
async fn test_linear_error_accumulation_and_deduplication() {
  // FailOpA -> [DbError]
  // FailOpB -> [DbError, NetworkError] => deduplicated union is [NetworkError, DbError]
  // FailFinal -> [DbError]             => still [NetworkError, DbError]
  let process = entry::<HNil, Messages>().then(FailOpA(true)).then(FailOpB(false)).end(FailFinal(false)).build("p", 0);

  let err =
    process.resume_run(empty_ctx(), PreviousRunYieldedAt(StepIndex::MIN), String::new(), None, None).await.unwrap_err();
  // Targeted check via downcast_ref (type-checked against ErrorCoproduct)
  assert_eq!(err.downcast_ref::<DbError>(), Some(&DbError("op_a")));
  assert!(!err.is::<NetworkError>());
  assert_eq!(format!("{err}"), "db error: op_a");
  assert!(err.source().is_none());

  // Exhaustive handling via peel() + extract()
  let db = match err.peel() {
    Ok(NetworkError(_)) => panic!("unexpected NetworkError"),
    Err(rem) => match rem.peel() {
      Ok(db) => db,
      Err(rem) => match rem.peel() {
        Ok(ser) => panic!("unexpected SerializationError: {ser}"),
        Err(rem) => {
          let idx = rem.extract();
          panic!("unexpected InvalidStepIndexError: {idx}");
        },
      },
    },
  };
  assert_eq!(db, DbError("op_a"));

  // Trigger NetworkError (and check Display, Debug, Error::source(), and downcast)
  let process_b =
    entry::<HNil, Messages>().then(FailOpA(false)).then(FailOpB(true)).end(FailFinal(false)).build("p", 0);
  let err_b = process_b
    .resume_run(empty_ctx(), PreviousRunYieldedAt(StepIndex::MIN), String::new(), None, None)
    .await
    .unwrap_err();
  assert!(err_b.to_string().contains("network error"));
  assert!(format!("{err_b:?}").contains("network error"));
  assert!(err_b.is::<NetworkError>());
  assert!(err_b.source().is_some());
  let err_b = err_b.downcast::<DbError>().unwrap_err();
  let net_err = err_b.downcast::<NetworkError>().expect("expected NetworkError");
  assert!(net_err.source().is_some());

  // Trigger FailFinal error
  let process_f =
    entry::<HNil, Messages>().then(FailOpA(false)).then(FailOpB(false)).end(FailFinal(true)).build("p", 0);
  let err_f = process_f
    .resume_run(empty_ctx(), PreviousRunYieldedAt(StepIndex::MIN), String::new(), None, None)
    .await
    .unwrap_err();
  assert!(err_f.to_string().contains("db error: final"));
  assert_eq!(err_f.downcast_ref::<DbError>(), Some(&DbError("final")));
}

#[tokio::test]
async fn test_form_and_form_splitter_errors() {
  // Form create_form error
  let p_create = entry::<HNil, Messages>()
    .show(FailForm { fail_create: true, fail_input: false })
    .end(FailFinal(false))
    .build("p", 0);
  let err = p_create
    .resume_run(empty_ctx(), PreviousRunYieldedAt(StepIndex::MIN), String::new(), None, None)
    .await
    .unwrap_err();
  assert!(err.to_string().contains("db error: form_create"));

  // Form handle_input error
  let p_input = entry::<HNil, Messages>()
    .show(FailForm { fail_create: false, fail_input: true })
    .end(FailFinal(false))
    .build("p", 0);
  let RunOutcome::Yield(_, ctx, yielded_at, form_ctx) =
    p_input.resume_run(empty_ctx(), PreviousRunYieldedAt(StepIndex::MIN), String::new(), None, None).await.unwrap()
  else {
    panic!("expected yield");
  };
  let err =
    p_input.resume_run(ctx, PreviousRunYieldedAt(yielded_at.0), "hi".into(), Some(form_ctx), None).await.unwrap_err();
  assert!(err.to_string().contains("validation error"));

  // FormSplitter create_form error
  let p_fs_create = entry::<HNil, Messages>()
    .show_split(FailFormSplitter { fail_create: true, fail_input: false, pick_case1: true })
    .case_end(Case1, |x| x.end(FailFinal(false)))
    .case_end(Case2, |x| x.end(FailFinal(false)))
    .build("p", 0);
  let err = p_fs_create
    .resume_run(empty_ctx(), PreviousRunYieldedAt(StepIndex::MIN), String::new(), None, None)
    .await
    .unwrap_err();
  assert!(err.to_string().contains("validation error"));

  // FormSplitter handle_input error
  let p_fs_input = entry::<HNil, Messages>()
    .show_split(FailFormSplitter { fail_create: false, fail_input: true, pick_case1: true })
    .case_end(Case1, |x| x.end(FailFinal(false)))
    .case_end(Case2, |x| x.end(FailFinal(false)))
    .build("p", 0);
  let RunOutcome::Yield(_, ctx, yielded_at, form_ctx) =
    p_fs_input.resume_run(empty_ctx(), PreviousRunYieldedAt(StepIndex::MIN), String::new(), None, None).await.unwrap()
  else {
    panic!("expected yield");
  };
  let err =
    p_fs_input.resume_run(ctx, PreviousRunYieldedAt(yielded_at.0), "1".into(), Some(form_ctx), None).await.unwrap_err();
  assert!(err.to_string().contains("validation error"));
}

#[tokio::test]
async fn test_split_branch_error_accumulation() {
  // Error from splitter itself
  let p_split_err = entry::<HNil, Messages>()
    .split(FailSplitter { fail: true, pick_case1: true })
    .case_via(Case1, |x| x.then(FailOpB(false)))
    .case_via(Case2, |x| x.show(FailForm { fail_create: false, fail_input: false }))
    .end(FailFinal(false))
    .build("p", 0);
  let err = p_split_err
    .resume_run(empty_ctx(), PreviousRunYieldedAt(StepIndex::MIN), String::new(), None, None)
    .await
    .unwrap_err();
  assert!(err.to_string().contains("db error: splitter"));

  // Error from Case1 (flowing split)
  let p_case1_err = entry::<HNil, Messages>()
    .split(FailSplitter { fail: false, pick_case1: true })
    .case_via(Case1, |x| x.then(FailOpB(true)))
    .case_via(Case2, |x| x.show(FailForm { fail_create: false, fail_input: false }))
    .end(FailFinal(false))
    .build("p", 0);
  let err = p_case1_err
    .resume_run(empty_ctx(), PreviousRunYieldedAt(StepIndex::MIN), String::new(), None, None)
    .await
    .unwrap_err();
  assert!(err.to_string().contains("network error"));

  // Error from Case2 (flowing split)
  let p_case2_err = entry::<HNil, Messages>()
    .split(FailSplitter { fail: false, pick_case1: false })
    .case_via(Case1, |x| x.then(FailOpB(false)))
    .case_via(Case2, |x| x.show(FailForm { fail_create: true, fail_input: false }))
    .end(FailFinal(false))
    .build("p", 0);
  let err = p_case2_err
    .resume_run(empty_ctx(), PreviousRunYieldedAt(StepIndex::MIN), String::new(), None, None)
    .await
    .unwrap_err();
  assert!(err.to_string().contains("db error: form_create"));

  // Error from finalized split Case1 and Case2
  let p_fin_case1 = entry::<HNil, Messages>()
    .split(FailSplitter { fail: false, pick_case1: true })
    .case_end(Case1, |x| x.then(FailOpA(true)).end(FailFinal(false)))
    .case_end(Case2, |x| x.then(FailOpB(true)).end(FailFinal(false)))
    .build("p", 0);
  let err = p_fin_case1
    .resume_run(empty_ctx(), PreviousRunYieldedAt(StepIndex::MIN), String::new(), None, None)
    .await
    .unwrap_err();
  assert!(err.to_string().contains("db error: op_a"));

  let p_fin_case2 = entry::<HNil, Messages>()
    .split(FailSplitter { fail: false, pick_case1: false })
    .case_end(Case1, |x| x.then(FailOpA(true)).end(FailFinal(false)))
    .case_end(Case2, |x| x.then(FailOpB(true)).end(FailFinal(false)))
    .build("p", 0);
  let err = p_fin_case2
    .resume_run(empty_ctx(), PreviousRunYieldedAt(StepIndex::MIN), String::new(), None, None)
    .await
    .unwrap_err();
  assert!(err.to_string().contains("network error"));
}

#[tokio::test]
async fn test_infallible_process_has_library_errors() {
  struct InfallibleFinal;
  impl Final for InfallibleFinal {
    type Consumes = HNil;
    type FinalMessage = String;
    type Error = CNil;

    async fn handle(&self, _: HNil) -> Result<String, CNil> {
      Ok("ok".into())
    }
  }

  let process = entry::<HNil, Messages>().end(InfallibleFinal).build("infallible", 0);
  let res: Result<RunOutcome<Messages>, ProcessError<type_process_builder::builder::LibraryErrors>> =
    process.resume_run(empty_ctx(), PreviousRunYieldedAt(StepIndex::MIN), String::new(), None, None).await;
  assert_eq!(res.unwrap(), RunOutcome::Finish("ok".into()));

  let idx_err =
    process.resume_run(empty_ctx(), PreviousRunYieldedAt(999), String::new(), None, None).await.unwrap_err();
  assert!(idx_err.to_string().contains("invalid step index: 999"));
  let idx = idx_err.peel().unwrap_err().extract();
  assert_eq!(idx.0, 999);

  let ser_err = type_process_builder::builder::SerializationError(postcard::Error::DeserializeUnexpectedEnd);
  assert!(ser_err.to_string().contains("failed to serialize or deserialize process state"));
  assert!(ser_err.source().is_some());
}

#[cfg(all(test, target_pointer_width = "64"))]
mod size_asserts {
  use super::ProcessError;
  use crate::{Coprod, impl_step_error};
  use typenum::*;

  #[derive(Debug, thiserror::Error)]
  #[error("network error")]
  struct NetworkError(#[source] std::io::Error);

  #[derive(Debug, thiserror::Error)]
  #[error("other error")]
  struct OtherError(#[source] std::io::Error);

  #[derive(Debug, thiserror::Error)]
  #[error("another error")]
  struct AnotherError(#[source] std::io::Error);

  #[derive(Debug, thiserror::Error)]
  #[error("huge error")]
  #[allow(dead_code)]
  struct HugeError([u8; 1024]);

  impl_step_error! {
    NetworkError =>  U0,
    OtherError =>  U1,
    AnotherError =>  U2,
    HugeError => U3,
  }

  const _: () = assert!(size_of::<ProcessError<Coprod!(NetworkError, OtherError, AnotherError, HugeError)>>() == 8);
  const _: () = assert!(size_of::<ProcessError<crate::CNil>>() == 8);
}
