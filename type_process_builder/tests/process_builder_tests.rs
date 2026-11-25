#![allow(clippy::unused_async_trait_impl)]
mod unit;
use serde::{Deserialize, Serialize};

#[allow(non_upper_case_globals)]
const ExtractMsisdnOperatorAndShortcodeString: Entry<HList![EntryParam], Messages> = entry();
use std::marker::PhantomData;
use type_process_builder::builder::*;
use type_process_builder::frunk::to_ref::ToRef;
use type_process_builder::step::{
  Entry, Final, Form, FormSplitter, FormWithContext, InputValidation, Operation, OperationOutcome, ProcessMessages,
  Splitter,
};
use type_process_builder::{CNil, Coprod, HList, HNil, hlist};
use typenum::*;

#[derive(Deserialize, Serialize)]
struct Msisdn(pub u64);

#[allow(clippy::upper_case_acronyms)]
#[derive(Deserialize, Serialize)]
enum Operator {
  MTN,
}

#[derive(Deserialize, Serialize)]
struct ShortcodeString(String);

use type_process_builder::impl_param_value;

#[derive(Deserialize, Serialize)]
struct EntryParam(Msisdn, Operator, ShortcodeString);

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
struct Split1Param;

#[derive(Deserialize, Serialize)]
struct Split2Param;

#[derive(Deserialize, Serialize)]
struct CommonSplitParam;

#[derive(Deserialize, Serialize)]
struct Case1Param;

#[derive(Deserialize, Serialize)]
struct Case2Param;

#[derive(Deserialize, Serialize)]
struct CommonCaseParam;

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
struct CaseOptionParam(pub u8);

#[derive(Deserialize, Serialize)]
struct InnerSplit1Param;

#[derive(Deserialize, Serialize)]
struct InnerSplit2Param;

impl_param_value! {
  EntryParam => U0,
  Split1Param => U1,
  Split2Param => U2,
  CommonSplitParam => U3,
  Case1Param => U4,
  Case2Param => U5,
  CommonCaseParam => U6,
  CaseOptionParam => U7,
  InnerSplit1Param => U8,
  InnerSplit2Param => U9,
}

struct Messages;
impl ProcessMessages for Messages {
  type FormMessage = String;
  type FinalMessage = String;
}

struct ProduceCaseParam1;
impl Operation for ProduceCaseParam1 {
  type Error = CNil;
  type Consumes = HNil;
  type Produces = HList![Case1Param, CommonCaseParam];
  type FinalMessage = String;

  async fn handle(
    &self,
    _consumes: <Self::Consumes as ToRef<'_>>::Ref,
  ) -> Result<OperationOutcome<Self::Produces, Self::FinalMessage>, Self::Error> {
    Ok(OperationOutcome::Successful(hlist!(Case1Param, CommonCaseParam)))
  }
}

struct ProduceCaseParam2;
impl Operation for ProduceCaseParam2 {
  type Error = CNil;
  type Consumes = HNil;
  type Produces = HList![Case2Param, CommonCaseParam];
  type FinalMessage = String;

  async fn handle(
    &self,
    _consumes: <Self::Consumes as ToRef<'_>>::Ref,
  ) -> Result<OperationOutcome<Self::Produces, Self::FinalMessage>, Self::Error> {
    Ok(OperationOutcome::Successful(hlist!(Case2Param, CommonCaseParam)))
  }
}

struct ProduceOnlyCase2Param;
impl Operation for ProduceOnlyCase2Param {
  type Error = CNil;
  type Consumes = HNil;
  type Produces = HList![Case2Param];
  type FinalMessage = String;

  async fn handle(
    &self,
    _consumes: <Self::Consumes as ToRef<'_>>::Ref,
  ) -> Result<OperationOutcome<Self::Produces, Self::FinalMessage>, Self::Error> {
    Ok(OperationOutcome::Successful(hlist!(Case2Param)))
  }
}

struct ProduceOnlyCase1Param;
impl Operation for ProduceOnlyCase1Param {
  type Error = CNil;
  type Consumes = HNil;
  type Produces = HList![Case1Param];
  type FinalMessage = String;

  async fn handle(
    &self,
    _consumes: <Self::Consumes as ToRef<'_>>::Ref,
  ) -> Result<OperationOutcome<Self::Produces, Self::FinalMessage>, Self::Error> {
    Ok(OperationOutcome::Successful(hlist!(Case1Param)))
  }
}

pub struct Case1;
pub struct Case2;
pub struct Case3;
pub struct Case4;

struct SplitByThreeCaseOption;
impl Splitter for SplitByThreeCaseOption {
  type Error = CNil;
  type Consumes = HList![CaseOptionParam];
  type Produces = Coprod![
    (Case1, HList![Split1Param, CommonSplitParam]),
    (Case2, HList![Split2Param, CommonSplitParam]),
    (Case3, HList![CommonSplitParam])
  ];
  async fn handle(&self, consumes: <Self::Consumes as ToRef<'_>>::Ref) -> Result<Self::Produces, Self::Error> {
    Ok(match consumes.head.0 {
      1 => Self::Produces::inject((Case1, hlist!(Split1Param, CommonSplitParam))),
      2 => Self::Produces::inject((Case2, hlist!(Split2Param, CommonSplitParam))),
      _ => Self::Produces::inject((Case3, hlist!(CommonSplitParam))),
    })
  }
}

struct SplitByTwoCaseOption;
impl Splitter for SplitByTwoCaseOption {
  type Error = CNil;
  type Consumes = HList![CaseOptionParam];
  type Produces =
    Coprod![(Case1, HList![Split1Param, CommonSplitParam]), (Case2, HList![Split2Param, CommonSplitParam])];

  async fn handle(&self, consumes: <Self::Consumes as ToRef<'_>>::Ref) -> Result<Self::Produces, Self::Error> {
    Ok(match consumes.head.0 {
      1 => Self::Produces::inject((Case1, hlist!(Split1Param, CommonSplitParam))),
      _ => Self::Produces::inject((Case2, hlist!(Split2Param, CommonSplitParam))),
    })
  }
}

pub struct InnerCase0;
pub struct InnerCase1;
struct InnerSelectCase<CaseIndex: Bit>(PhantomData<CaseIndex>);
impl Splitter for InnerSelectCase<B0> {
  type Error = CNil;
  type Consumes = HNil;
  type Produces = Coprod![(InnerCase0, HNil), (InnerCase1, HNil)];

  async fn handle(&self, _consumes: <Self::Consumes as ToRef<'_>>::Ref) -> Result<Self::Produces, Self::Error> {
    Ok(Self::Produces::inject((InnerCase0, HNil)))
  }
}

impl Splitter for InnerSelectCase<B1> {
  type Error = CNil;
  type Consumes = HNil;
  type Produces = Coprod![(InnerCase0, HNil), (InnerCase1, HNil)];

  async fn handle(&self, _consumes: <Self::Consumes as ToRef<'_>>::Ref) -> Result<Self::Produces, Self::Error> {
    Ok(Self::Produces::inject((InnerCase1, HNil)))
  }
}

struct SplitByFourCaseOption;
impl Splitter for SplitByFourCaseOption {
  type Error = CNil;
  type Consumes = HList![CaseOptionParam];
  type Produces = Coprod![
    (Case1, HList![Split1Param, CommonSplitParam]),
    (Case2, HList![Split2Param, CommonSplitParam]),
    (Case3, HList![Split2Param, CommonSplitParam]),
    (Case4, HList![Split2Param, CommonSplitParam]),
  ];

  async fn handle(&self, consumes: <Self::Consumes as ToRef<'_>>::Ref) -> Result<Self::Produces, Self::Error> {
    Ok(match consumes.head.0 {
      1 => Self::Produces::inject((Case1, hlist!(Split1Param, CommonSplitParam))),
      2 => Self::Produces::inject((Case2, hlist!(Split2Param, CommonSplitParam))),
      3 => Self::Produces::inject((Case3, hlist!(Split2Param, CommonSplitParam))),
      _ => Self::Produces::inject((Case4, hlist!(Split2Param, CommonSplitParam))),
    })
  }
}

struct NoOpOperation;
impl Operation for NoOpOperation {
  type Error = CNil;
  type Consumes = HNil;
  type Produces = HNil;
  type FinalMessage = String;

  async fn handle<'a>(
    &self,
    _consumes: Self::Consumes,
  ) -> Result<OperationOutcome<Self::Produces, Self::FinalMessage>, Self::Error> {
    Ok(OperationOutcome::Successful(HNil))
  }
}

struct FinishEarlyOperation;
impl Operation for FinishEarlyOperation {
  type Error = CNil;
  type Consumes = HNil;
  type Produces = HNil;
  type FinalMessage = String;

  async fn handle(
    &self,
    _consumes: <Self::Consumes as ToRef<'_>>::Ref,
  ) -> Result<OperationOutcome<Self::Produces, Self::FinalMessage>, Self::Error> {
    Ok(OperationOutcome::Finish("Operation finished".into()))
  }
}

#[derive(Serialize, Deserialize)]
struct EmptyFormContext;

struct FinishEarlyForm;
impl Form for FinishEarlyForm {
  type Error = CNil;
  type CreateFormConsumes = HNil;
  type ValidateInputConsumes = HNil;
  type Produces = HNil;
  type Context = EmptyFormContext;
  type Messages = Messages;

  async fn create_form(
    &self,
    _consumes: <Self::CreateFormConsumes as ToRef<'_>>::Ref,
    _back_token: Option<BackToken>,
  ) -> Result<FormWithContext<String, Self::Context>, Self::Error> {
    Ok(FormWithContext("Finish early form".into(), EmptyFormContext))
  }

  async fn handle_input(
    &self,
    _consumes: <Self::ValidateInputConsumes as ToRef<'_>>::Ref,
    _user_input: String,
    _form_context: Self::Context,
    _back_token: Option<BackToken>,
  ) -> Result<InputValidation<Self::Produces, Messages, Self::Context>, Self::Error> {
    Ok(InputValidation::Finish("Form finished".into()))
  }
}

struct SayGoodByAndConsumeCommonParams;
impl Final for SayGoodByAndConsumeCommonParams {
  type Error = CNil;
  type Consumes = HList![EntryParam, CommonSplitParam, CommonCaseParam];
  type FinalMessage = String;

  async fn handle(&self, _consumes: Self::Consumes) -> Result<String, Self::Error> {
    Ok("Good bye".into())
  }
}

struct CommonCaseParam1Form;
impl Form for CommonCaseParam1Form {
  type Error = CNil;
  type CreateFormConsumes = HList![Split1Param];
  type ValidateInputConsumes = HList![Split1Param, CommonSplitParam];
  type Produces = HList![CommonCaseParam];
  type Context = EmptyFormContext;
  type Messages = Messages;

  async fn create_form(
    &self,
    _consumes: <Self::CreateFormConsumes as ToRef<'_>>::Ref,
    _back_token: Option<BackToken>,
  ) -> Result<FormWithContext<String, Self::Context>, Self::Error> {
    Ok(FormWithContext("Enter a number".into(), EmptyFormContext))
  }

  async fn handle_input(
    &self,
    consumes: <Self::ValidateInputConsumes as ToRef<'_>>::Ref,
    _user_input: String,
    _form_context: Self::Context,
    _back_token: Option<BackToken>,
  ) -> Result<InputValidation<Self::Produces, Messages, Self::Context>, Self::Error> {
    let (_split1, _common_split): (&Split1Param, &CommonSplitParam) = (consumes.head, consumes.tail.head);
    Ok(InputValidation::Successful(hlist![CommonCaseParam]))
  }
}

struct CommonCaseParam2Form;
impl Form for CommonCaseParam2Form {
  type Error = CNil;
  type CreateFormConsumes = HList![Split2Param];
  type ValidateInputConsumes = HNil;
  type Produces = HList![CommonCaseParam];
  type Context = EmptyFormContext;
  type Messages = Messages;

  async fn create_form(
    &self,
    _consumes: <Self::CreateFormConsumes as ToRef<'_>>::Ref,
    _back_token: Option<BackToken>,
  ) -> Result<FormWithContext<String, Self::Context>, Self::Error> {
    Ok(FormWithContext("Enter a number".into(), EmptyFormContext))
  }

  async fn handle_input(
    &self,
    _consumes: <Self::ValidateInputConsumes as ToRef<'_>>::Ref,
    _user_input: String,
    _form_context: Self::Context,
    _back_token: Option<BackToken>,
  ) -> Result<InputValidation<Self::Produces, Messages, Self::Context>, Self::Error> {
    Ok(InputValidation::Successful(hlist![CommonCaseParam]))
  }
}

struct NoOpForm;
impl Form for NoOpForm {
  type Error = CNil;
  type CreateFormConsumes = HNil;
  type ValidateInputConsumes = HNil;
  type Produces = HNil;
  type Context = EmptyFormContext;
  type Messages = Messages;

  async fn create_form(
    &self,
    _consumes: <Self::CreateFormConsumes as ToRef<'_>>::Ref,
    _back_token: Option<BackToken>,
  ) -> Result<FormWithContext<String, Self::Context>, Self::Error> {
    Ok(FormWithContext("Straight to trash".into(), EmptyFormContext))
  }

  async fn handle_input(
    &self,
    _consumes: <Self::ValidateInputConsumes as ToRef<'_>>::Ref,
    _user_input: String,
    _form_context: Self::Context,
    _back_token: Option<BackToken>,
  ) -> Result<InputValidation<Self::Produces, Messages, Self::Context>, Self::Error> {
    Ok(InputValidation::Successful(HNil))
  }
}

struct FinishAfterInput;
impl Form for FinishAfterInput {
  type Error = CNil;
  type CreateFormConsumes = HNil;
  type ValidateInputConsumes = HNil;
  type Produces = HNil;
  type Context = EmptyFormContext;
  type Messages = Messages;

  async fn create_form(
    &self,
    _consumes: <Self::CreateFormConsumes as ToRef<'_>>::Ref,
    _back_token: Option<BackToken>,
  ) -> Result<FormWithContext<String, Self::Context>, Self::Error> {
    Ok(FormWithContext("Last number in the process".into(), EmptyFormContext))
  }

  async fn handle_input(
    &self,
    _consumes: <Self::ValidateInputConsumes as ToRef<'_>>::Ref,
    _user_input: String,
    _form_context: Self::Context,
    _back_token: Option<BackToken>,
  ) -> Result<InputValidation<Self::Produces, Messages, Self::Context>, Self::Error> {
    Ok(InputValidation::Finish("Always finish".into()))
  }
}

struct OneInputRetryForm;
impl Form for OneInputRetryForm {
  type Error = CNil;
  type CreateFormConsumes = HNil;
  type ValidateInputConsumes = HNil;
  type Produces = HList![CommonCaseParam];
  type Context = u16;
  type Messages = Messages;

  async fn create_form(
    &self,
    _consumes: <Self::CreateFormConsumes as ToRef<'_>>::Ref,
    _back_token: Option<BackToken>,
  ) -> Result<FormWithContext<String, Self::Context>, Self::Error> {
    Ok(FormWithContext("This will be discarded".into(), 0))
  }

  async fn handle_input(
    &self,
    _consumes: <Self::ValidateInputConsumes as ToRef<'_>>::Ref,
    user_input: String,
    failed: Self::Context,
    back_token: Option<BackToken>,
  ) -> Result<InputValidation<Self::Produces, Messages, Self::Context>, Self::Error> {
    if user_input == "0"
      && let Some(token) = back_token
    {
      return Ok(InputValidation::Back(token));
    }
    match failed {
      0 => Ok(InputValidation::Retry("This will be accepted".into(), failed + 1)),
      _ => Ok(InputValidation::Successful(hlist![CommonCaseParam])),
    }
  }
}

struct ChooseCaseForm;
impl Form for ChooseCaseForm {
  type Error = CNil;
  type CreateFormConsumes = HNil;
  type ValidateInputConsumes = HNil;
  type Produces = HList![CaseOptionParam];
  type Context = EmptyFormContext;
  type Messages = Messages;

  async fn create_form(
    &self,
    _consumes: <Self::CreateFormConsumes as ToRef<'_>>::Ref,
    _back_token: Option<BackToken>,
  ) -> Result<FormWithContext<String, Self::Context>, Self::Error> {
    Ok(FormWithContext("Choose a case".into(), EmptyFormContext))
  }

  async fn handle_input(
    &self,
    _consumes: <Self::ValidateInputConsumes as ToRef<'_>>::Ref,
    user_input: String,
    _form_context: Self::Context,
    _back_token: Option<BackToken>,
  ) -> Result<InputValidation<Self::Produces, Messages, Self::Context>, Self::Error> {
    let option = user_input.parse::<u8>().unwrap_or(1);
    Ok(InputValidation::Successful(hlist!(CaseOptionParam(option))))
  }
}

struct FinalNoConsumes;
impl Final for FinalNoConsumes {
  type Error = CNil;
  type Consumes = HNil;
  type FinalMessage = String;

  async fn handle(&self, _consumes: Self::Consumes) -> Result<String, Self::Error> {
    Ok("Empty good bye".into())
  }
}

struct FinalConsumeCase2Param;
impl Final for FinalConsumeCase2Param {
  type Error = CNil;
  type Consumes = HList![Case2Param];
  type FinalMessage = String;

  async fn handle(&self, _consumes: Self::Consumes) -> Result<String, Self::Error> {
    Ok("I ate Case2Param".into())
  }
}

struct FinishProcessOperation;
impl Operation for FinishProcessOperation {
  type Error = CNil;
  type Consumes = HNil;
  type Produces = HNil;
  type FinalMessage = String;

  async fn handle(
    &self,
    _consumes: <Self::Consumes as ToRef<'_>>::Ref,
  ) -> Result<OperationOutcome<Self::Produces, Self::FinalMessage>, Self::Error> {
    Ok(OperationOutcome::Finish("Operation finished".into()))
  }
}

struct TestFormSplitter;
impl FormSplitter for TestFormSplitter {
  type Error = CNil;
  type CreateFormConsumes = HNil;
  type ValidateInputConsumes = HNil;
  type Produces = Coprod![(Case1, HList![Split1Param]), (Case2, HList![Split2Param])];
  type Context = u16;
  type Messages = Messages;

  async fn create_form(
    &self,
    _consumes: <Self::CreateFormConsumes as ToRef<'_>>::Ref,
    _back_token: Option<BackToken>,
  ) -> Result<FormWithContext<String, Self::Context>, Self::Error> {
    Ok(FormWithContext("choose case".into(), 0))
  }

  async fn handle_input(
    &self,
    _consumes: <Self::ValidateInputConsumes as ToRef<'_>>::Ref,
    user_input: String,
    failed: Self::Context,
    back_token: Option<BackToken>,
  ) -> Result<InputValidation<Self::Produces, Messages, Self::Context>, Self::Error> {
    match (user_input.as_str(), failed, back_token) {
      ("0", _, Some(token)) => Ok(InputValidation::Back(token)),
      ("retry", 0, _) => Ok(InputValidation::Retry("retry again".into(), failed + 1)),
      ("finish", _, _) => Ok(InputValidation::Finish("finished early".into())),
      ("1", _, _) => Ok(InputValidation::Successful(Self::Produces::inject((Case1, hlist![Split1Param])))),
      _ => Ok(InputValidation::Successful(Self::Produces::inject((Case2, hlist![Split2Param])))),
    }
  }
}

struct InnerFormSplitter;
impl FormSplitter for InnerFormSplitter {
  type Error = CNil;
  type CreateFormConsumes = HNil;
  type ValidateInputConsumes = HNil;
  type Produces = Coprod![(Case1, HList![InnerSplit1Param]), (Case2, HList![InnerSplit2Param])];
  type Context = i16;
  type Messages = Messages;

  async fn create_form(
    &self,
    _consumes: <Self::CreateFormConsumes as ToRef<'_>>::Ref,
    _back_token: Option<BackToken>,
  ) -> Result<FormWithContext<String, Self::Context>, Self::Error> {
    Ok(FormWithContext("choose case".into(), 0))
  }

  async fn handle_input(
    &self,
    _consumes: <Self::ValidateInputConsumes as ToRef<'_>>::Ref,
    user_input: String,
    failed: Self::Context,
    _back_token: Option<BackToken>,
  ) -> Result<InputValidation<Self::Produces, Messages, Self::Context>, Self::Error> {
    match (user_input.as_str(), failed) {
      ("retry", 0) => Ok(InputValidation::Retry("retry again".into(), failed + 1)),
      ("finish", _) => Ok(InputValidation::Finish("finished early".into())),
      ("1", _) => Ok(InputValidation::Successful(Self::Produces::inject((Case1, hlist![InnerSplit1Param])))),
      _ => Ok(InputValidation::Successful(Self::Produces::inject((Case2, hlist![InnerSplit2Param])))),
    }
  }
}

#[tokio::test]
async fn test_end() {
  let process = ExtractMsisdnOperatorAndShortcodeString.end(FinalNoConsumes).build("", 0);
  let messages = vec!["*123#", "Empty good bye"];
  test_process_messages(&process, messages).await;
}

#[tokio::test]
async fn test_split() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByTwoCaseOption)
    .case_via(Case1, |x| x.then(ProduceCaseParam1))
    .case_via(Case2, |x| x.then(ProduceCaseParam2))
    .end(SayGoodByAndConsumeCommonParams)
    .build("", 0);
  test_process_messages(&process, vec!["*123#", "Choose a case", "1", "Good bye"]).await;
  test_process_messages(&process, vec!["*123#", "Choose a case", "2", "Good bye"]).await;
}

async fn test_return_error_on_param_missing_from_init_value(
  process: &RunnableProcess<impl FinalizedProcess<Messages = Messages, Errors: ContainsError<SerializationError>>>,
  messages: Vec<&str>,
) {
  let mut init_value = session_init_value();
  init_value.pop();
  let run_outcome = process
    .resume_run(init_value, PreviousRunYieldedAt(StepIndex::MIN), messages[0].into(), None, None::<BackToken>)
    .await;
  assert!(run_outcome.is_err_and(|x| {
    x.downcast_ref::<SerializationError>().is_some_and(|e| matches!(e.0, postcard::Error::DeserializeUnexpectedEnd))
  }));
}

async fn test_return_error_on_param_missing_from_context(
  process: &RunnableProcess<impl FinalizedProcess<Messages = Messages, Errors: ContainsError<SerializationError>>>,
  messages: Vec<&str>,
) {
  let mut messages_index = 0;
  let run_outcome = process
    .resume_run(
      session_init_value(),
      PreviousRunYieldedAt(StepIndex::MIN),
      messages[messages_index].into(),
      None,
      None::<BackToken>,
    )
    .await
    .expect("Test failed");
  messages_index += 1;
  match run_outcome {
    RunOutcome::Yield(_msg, mut value, yielded_at, context) => {
      value.pop();
      let run_outcome = process
        .resume_run(
          value,
          PreviousRunYieldedAt(yielded_at.0),
          messages[messages_index].into(),
          Some(context),
          None::<BackToken>,
        )
        .await;
      assert!(run_outcome.is_err_and(|x| {
        x.downcast_ref::<SerializationError>().is_some_and(|e| matches!(e.0, postcard::Error::DeserializeUnexpectedEnd))
      }));
    },
    RunOutcome::RetryUserInput(msg, _context) => {
      assert_eq!(msg, messages[messages_index]);
    },
    RunOutcome::Finish(msg) => {
      assert_eq!(msg, messages[messages_index]);
    },
    RunOutcome::Back => panic!("Unexpected back"),
  }
}

#[tokio::test]
async fn test_end_emitted_in_form_step() {
  let process = ExtractMsisdnOperatorAndShortcodeString.show(FinishAfterInput).end(FinalNoConsumes).build("", 0);
  test_process_messages(&process, vec!["*123#", "Last number in the process", "10", "Always finish"]).await;
  test_return_error_on_param_missing_from_init_value(&process, vec!["*123#"]).await;
  test_return_error_on_param_missing_from_context(&process, vec!["*123#", "Last number in the process"]).await;
}

#[tokio::test]
async fn test_retry_emitted_in_form_step() {
  let process = ExtractMsisdnOperatorAndShortcodeString.show(OneInputRetryForm).end(FinalNoConsumes).build("", 0);
  let messages = vec!["*123#", "This will be discarded", "10", "This will be accepted", "20", "Empty good bye"];
  test_process_messages(&process, messages).await;
}

#[tokio::test]
async fn test_resume_in_split() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByTwoCaseOption)
    .case_via(Case1, |x| x.show(CommonCaseParam1Form))
    .case_via(Case2, |x| x.then(ProduceCaseParam2))
    .end(SayGoodByAndConsumeCommonParams)
    .build("", 0);
  let messages = vec!["*123#", "Choose a case", "1", "Enter a number", "a number", "Good bye"];
  test_process_messages(&process, messages).await;
}

#[tokio::test]
async fn test_end_first_case_of_finalized_split_process() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByTwoCaseOption)
    .case_end(Case1, |x| x.end(FinalNoConsumes))
    .case_end(Case2, |x| x.end(FinalNoConsumes))
    .build("", 0);
  let messages = vec!["*123#", "Choose a case", "1", "Empty good bye"];
  test_process_messages(&process, messages).await;
}

#[tokio::test]
async fn test_yield_first_case_of_finalized_split_process() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByTwoCaseOption)
    .case_end(Case1, |x| x.show(NoOpForm).end(FinalNoConsumes))
    .case_end(Case2, |x| x.end(FinalNoConsumes))
    .build("", 0);
  let messages = vec!["*123#", "Choose a case", "1", "Straight to trash", "10", "Empty good bye"];
  test_process_messages(&process, messages).await;
}

#[tokio::test]
async fn test_yield_first_case_of_finalized_split_process_split() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByTwoCaseOption)
    .case_end(Case1, |x| x.end(FinalNoConsumes))
    .case_end(Case2, |x| {
      x.split(InnerSelectCase(PhantomData::<B1>))
        .case_end(InnerCase0, |x| x.end(FinalNoConsumes))
        .case_end(InnerCase1, |x| x.end(FinalNoConsumes))
    })
    .build("", 0);
  let messages = vec!["*123#", "Choose a case", "1", "Empty good bye"];
  test_process_messages(&process, messages).await;
  let messages2 = vec!["*123#", "Choose a case", "2", "Empty good bye"];
  test_process_messages(&process, messages2).await;
}

#[tokio::test]
async fn test_flowing_case_of_finalized_split_process() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByTwoCaseOption)
    .case_end(Case1, |x| x.end(FinalNoConsumes))
    .case_via(Case2, |x| x.then(ProduceCaseParam2))
    .end(FinalConsumeCase2Param)
    .build("", 0);
  let messages = vec!["*123#", "Choose a case", "2", "I ate Case2Param"];
  test_process_messages(&process, messages).await;
}

#[tokio::test]
async fn test_nested_split_with_choose_case_form() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByThreeCaseOption)
    .case_via(Case1, |x| x.show(OneInputRetryForm))
    .case_end(Case2, |x| x.end(FinalNoConsumes))
    .case_via(Case3, |x| {
      x.split(InnerSelectCase(PhantomData::<B1>))
        .case_end(InnerCase0, |x| x.end(FinalNoConsumes))
        .case_via(InnerCase1, |x| x.show(NoOpForm))
        .then(ProduceCaseParam2)
    })
    .end(SayGoodByAndConsumeCommonParams)
    .build("", 0);

  test_process_messages(
    &process,
    vec!["*123#", "Choose a case", "1", "This will be discarded", "10", "This will be accepted", "20", "Good bye"],
  )
  .await;
  test_process_messages(&process, vec!["*123#", "Choose a case", "2", "Empty good bye"]).await;
  test_process_messages(&process, vec!["*123#", "Choose a case", "3", "Straight to trash", "20", "Good bye"]).await;
}

#[tokio::test]
async fn test_split_process_form_splitter() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show_split(TestFormSplitter)
    .case_via(Case1, |x| x.show(NoOpForm))
    .case_end(Case2, |x| x.end(FinalNoConsumes))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(&process, vec!["*123#", "choose case", "1", "Straight to trash", "10", "Empty good bye"]).await;
  test_process_messages(
    &process,
    vec!["*123#", "choose case", "retry", "retry again", "1", "Straight to trash", "10", "Empty good bye"],
  )
  .await;
  test_process_messages(&process, vec!["*123#", "choose case", "finish", "finished early"]).await;
  test_process_messages(&process, vec!["*123#", "choose case", "2", "Empty good bye"]).await;
}

#[tokio::test]
async fn test_form_splitter_passthrough_with_multi_step_process_before() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(OneInputRetryForm)
    .show(FinishAfterInput)
    .show_split(TestFormSplitter)
    .case_end(Case1, |x| x.end(FinalNoConsumes))
    .case_end(Case2, |x| x.end(FinalNoConsumes))
    .build("", 0);

  let messages = vec![
    "*123#",
    "This will be discarded",
    "10",
    "This will be accepted",
    "20",
    "Last number in the process",
    "30",
    "Always finish",
  ];
  test_process_messages(&process, messages).await;
}

#[tokio::test]
async fn test_nested_split_retry_and_finish() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByTwoCaseOption)
    .case_end(Case1, |x| x.end(FinalNoConsumes))
    .case_end(Case2, |x| {
      x.show_split(InnerFormSplitter)
        .case_end(Case1, |x| x.show(OneInputRetryForm).end(FinalNoConsumes))
        .case_end(Case2, |x| x.end(FinalNoConsumes))
    })
    .build("", 0);

  // Test retry inside nested split
  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "2",
      "choose case",
      "1",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Empty good bye",
    ],
  )
  .await;

  // Test finish inside nested split
  test_process_messages(&process, vec!["*123#", "Choose a case", "2", "choose case", "finish", "finished early"]).await;
}

#[tokio::test]
async fn test_three_case_split_variant_1() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByThreeCaseOption)
    .case_via(Case1, |x| x.then(ProduceCaseParam1))
    .case_via(Case2, |x| x.then(ProduceCaseParam2))
    .case_end(Case3, |x| x.end(FinalNoConsumes))
    .end(SayGoodByAndConsumeCommonParams)
    .build("", 0);

  test_process_messages(&process, vec!["*123#", "Choose a case", "1", "Good bye"]).await;
  test_process_messages(&process, vec!["*123#", "Choose a case", "2", "Good bye"]).await;
}

#[tokio::test]
async fn test_flowing_split_yield_in_middle_case() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByThreeCaseOption)
    .case_via(Case1, |x| x.then(ProduceCaseParam1))
    .case_via(Case2, |x| x.show(CommonCaseParam2Form).then(ProduceOnlyCase2Param))
    .case_end(Case3, |x| x.end(FinalNoConsumes))
    .end(SayGoodByAndConsumeCommonParams)
    .build("", 0);

  let messages = vec!["*123#", "Choose a case", "2", "Enter a number", "10", "Good bye"];
  test_process_messages(&process, messages).await;
}

#[tokio::test]
async fn test_split_finish_in_case() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByTwoCaseOption)
    .case_via(Case1, |x| x.then(FinishProcessOperation))
    .case_via(Case2, |x| x.then(ProduceCaseParam2))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(&process, vec!["*123#", "Choose a case", "1", "Operation finished"]).await;
}

#[tokio::test]
async fn test_split_retry_in_case() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByTwoCaseOption)
    .case_via(Case1, |x| x.show(OneInputRetryForm).then(ProduceOnlyCase1Param))
    .case_via(Case2, |x| x.then(ProduceCaseParam2))
    .end(FinalNoConsumes)
    .build("", 0);

  let messages = vec![
    "*123#",
    "Choose a case",
    "1",
    "This will be discarded",
    "10",
    "This will be accepted",
    "20",
    "Empty good bye",
  ];
  test_process_messages(&process, messages).await;
}

#[tokio::test]
async fn test_nested_split_run_subprocess() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByTwoCaseOption)
    .case_via(Case1, |x| {
      x.split(InnerSelectCase(PhantomData::<B1>))
        .case_end(InnerCase0, |x| x.end(FinalNoConsumes))
        .case_via(InnerCase1, |x| x.then(ProduceCaseParam2))
    })
    .case_via(Case2, |x| x.then(ProduceCaseParam2))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(&process, vec!["*123#", "Choose a case", "1", "Empty good bye"]).await;
}

#[tokio::test]
async fn test_nested_split_in_middle() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByTwoCaseOption)
    .case_via(Case1, |x| {
      x.show(NoOpForm)
        .split(InnerSelectCase(PhantomData::<B1>))
        .case_end(InnerCase0, |x| x.end(FinalNoConsumes))
        .case_via(InnerCase1, |x| x.show(NoOpForm))
        .then(ProduceOnlyCase1Param)
    })
    .case_via(Case2, |x| x.then(ProduceOnlyCase1Param))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(
    &process,
    vec!["*123#", "Choose a case", "1", "Straight to trash", "10", "Straight to trash", "20", "Empty good bye"],
  )
  .await;
}

#[tokio::test]
async fn test_three_case_finalized_split_case_1() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByThreeCaseOption)
    .case_end(Case1, |x| x.end(FinalNoConsumes))
    .case_end(Case2, |x| x.end(FinalNoConsumes))
    .case_end(Case3, |x| x.end(FinalNoConsumes))
    .build("", 0);

  test_process_messages(&process, vec!["*123#", "Choose a case", "1", "Empty good bye"]).await;
  test_process_messages(&process, vec!["*123#", "Choose a case", "2", "Empty good bye"]).await;
  test_process_messages(&process, vec!["*123#", "Choose a case", "3", "Empty good bye"]).await;
}

#[tokio::test]
async fn test_three_case_mixed_split() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByThreeCaseOption)
    .case_end(Case1, |x| x.end(FinalNoConsumes))
    .case_end(Case2, |x| x.end(FinalNoConsumes))
    .case_via(Case3, |x| x.then(ProduceOnlyCase1Param))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(&process, vec!["*123#", "Choose a case", "3", "Empty good bye"]).await;
}

struct RetryOnceForm;
impl Form for RetryOnceForm {
  type Error = CNil;
  type CreateFormConsumes = HNil;
  type ValidateInputConsumes = HNil;
  type Produces = HNil;
  type Context = EmptyFormContext;
  type Messages = Messages;

  async fn create_form(
    &self,
    _consumes: <Self::CreateFormConsumes as ToRef<'_>>::Ref,
    _back_token: Option<BackToken>,
  ) -> Result<FormWithContext<String, Self::Context>, Self::Error> {
    Ok(FormWithContext("Fancy a retry?".into(), EmptyFormContext))
  }

  async fn handle_input(
    &self,
    _consumes: <Self::ValidateInputConsumes as ToRef<'_>>::Ref,
    user_input: String,
    _form_context: Self::Context,
    _back_token: Option<BackToken>,
  ) -> Result<InputValidation<Self::Produces, Messages, Self::Context>, Self::Error> {
    if user_input == "retry" {
      Ok(InputValidation::Retry("Try again".into(), EmptyFormContext))
    } else {
      Ok(InputValidation::Successful(HNil))
    }
  }
}

#[tokio::test]
async fn test_three_case_split_with_choose_case_form() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByThreeCaseOption)
    .case_via(Case1, |x| x.show(RetryOnceForm).then(ProduceCaseParam1))
    .case_via(Case2, |x| x.show(NoOpForm).then(ProduceCaseParam2))
    .case_end(Case3, |x| x.end(FinalNoConsumes))
    .end(SayGoodByAndConsumeCommonParams)
    .build("", 0);

  test_process_messages(
    &process,
    vec!["*123#", "Choose a case", "1", "Fancy a retry?", "retry", "Try again", "accept", "Good bye"],
  )
  .await;
  test_process_messages(&process, vec!["*123#", "Choose a case", "2", "Straight to trash", "anything", "Good bye"])
    .await;
  test_process_messages(&process, vec!["*123#", "Choose a case", "3", "Empty good bye"]).await;
}

#[tokio::test]
async fn test_flowing_split_with_choose_case_form() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByFourCaseOption)
    .case_via(Case1, |x| x.show(RetryOnceForm).then(FinishProcessOperation))
    .case_via(Case2, |x| x.then(ProduceCaseParam2))
    .case_via(Case3, |x| x.show(NoOpForm).then(FinishProcessOperation))
    .case_end(Case4, |x| x.end(FinalNoConsumes))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(
    &process,
    vec!["*123#", "Choose a case", "1", "Fancy a retry?", "retry", "Try again", "accept", "Operation finished"],
  )
  .await;
  test_process_messages(&process, vec!["*123#", "Choose a case", "2", "Empty good bye"]).await;
  test_process_messages(
    &process,
    vec!["*123#", "Choose a case", "3", "Straight to trash", "anything", "Operation finished"],
  )
  .await;
  test_process_messages(&process, vec!["*123#", "Choose a case", "4", "Empty good bye"]).await;
}

#[tokio::test]
async fn test_select_finalized_case_after_flowing_case() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByTwoCaseOption)
    .case_via(Case1, |x| x.then(ProduceCaseParam1))
    .case_end(Case2, |x| x.end(FinalNoConsumes))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(&process, vec!["*123#", "Choose a case", "2", "Empty good bye"]).await;
}

#[tokio::test]
async fn test_runnable_process_metadata() {
  let process = ExtractMsisdnOperatorAndShortcodeString.end(FinalNoConsumes).build("test_process", 123);

  assert_eq!(process.name(), "test_process");
  assert_eq!(process.version(), 123);
}

#[tokio::test]
async fn test_operation_finish_early() {
  let process = ExtractMsisdnOperatorAndShortcodeString.then(FinishEarlyOperation).end(FinalNoConsumes).build("", 0);
  test_process_messages(&process, vec!["*123#", "Operation finished"]).await;
}

#[tokio::test]
async fn test_form_finish_early() {
  let process = ExtractMsisdnOperatorAndShortcodeString.show(FinishEarlyForm).end(FinalNoConsumes).build("", 0);

  test_process_messages(&process, vec!["*123#", "Finish early form", "any", "Form finished"]).await;
}

#[tokio::test]
async fn test_four_case_mixed_split_with_choose_case_form() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByFourCaseOption)
    .case_via(Case1, |x| x.then(ProduceCaseParam1))
    .case_end(Case2, |x| x.end(FinalNoConsumes))
    .case_via(Case3, |x| x.then(ProduceCaseParam1))
    .case_end(Case4, |x| x.end(FinalNoConsumes))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(&process, vec!["*123#", "Choose a case", "1", "Empty good bye"]).await;
  test_process_messages(&process, vec!["*123#", "Choose a case", "2", "Empty good bye"]).await;
  test_process_messages(&process, vec!["*123#", "Choose a case", "3", "Empty good bye"]).await;
  test_process_messages(&process, vec!["*123#", "Choose a case", "4", "Empty good bye"]).await;
}

#[tokio::test]
async fn test_select_flowing_case_after_finalized_case_yield() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByTwoCaseOption)
    .case_end(Case1, |x| x.end(FinalNoConsumes))
    .case_via(Case2, |x| x.show(NoOpForm).then(ProduceCaseParam2))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(
    &process,
    vec!["*123#", "Choose a case", "2", "Straight to trash", "anything", "Empty good bye"],
  )
  .await;
}

#[tokio::test]
async fn test_retry_in_case_1_finalized_mixed_split() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByTwoCaseOption)
    .case_end(Case1, |x| x.show(RetryOnceForm).end(FinalNoConsumes))
    .case_via(Case2, |x| x.then(FinishProcessOperation).then(ProduceCaseParam2))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(
    &process,
    vec!["*123#", "Choose a case", "1", "Fancy a retry?", "retry", "Try again", "accept", "Empty good bye"],
  )
  .await;
  test_process_messages(&process, vec!["*123#", "Choose a case", "1", "Fancy a retry?", "anything", "Empty good bye"])
    .await;
  test_process_messages(&process, vec!["*123#", "Choose a case", "2", "Operation finished"]).await;
}

#[tokio::test]
async fn test_nested_split_run_subprocess_new() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByTwoCaseOption)
    .case_end(Case1, |x| {
      x.split(InnerSelectCase(PhantomData::<B1>))
        .case_end(InnerCase0, |x| x.end(FinalNoConsumes))
        .case_end(InnerCase1, |x| x.end(FinalNoConsumes))
    })
    .case_end(Case2, |x| x.end(FinalNoConsumes))
    .build("", 0);

  test_process_messages(&process, vec!["*123#", "Choose a case", "1", "Empty good bye"]).await;
}

#[tokio::test]
async fn test_nested_flowing_split_resume_run_new() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByTwoCaseOption)
    .case_via(Case1, |x| {
      x.split(InnerSelectCase(PhantomData::<B0>)).case_via(InnerCase0, |x| x.show(NoOpForm)).case_via(InnerCase1, |x| x)
    })
    .case_via(Case2, |x| x.then(NoOpOperation))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(&process, vec!["*123#", "Choose a case", "1", "Straight to trash", "any", "Empty good bye"])
    .await;
}

#[tokio::test]
async fn test_nested_finalized_split() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByTwoCaseOption)
    .case_end(Case1, |x| {
      x.split(InnerSelectCase(PhantomData::<B0>))
        .case_end(InnerCase0, |x| x.end(FinalNoConsumes))
        .case_end(InnerCase1, |x| x.end(FinalNoConsumes))
    })
    .case_end(Case2, |x| x.end(FinalNoConsumes))
    .build("", 0);

  let messages = vec!["*123#", "Choose a case", "1", "Empty good bye"];
  test_process_messages(&process, messages).await;
}

#[tokio::test]
async fn test_subprocess_resume() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(NoOpForm)
    .then(NoOpOperation)
    .show(ChooseCaseForm)
    .split(SplitByTwoCaseOption)
    .case_via(Case1, |x| x)
    .case_via(Case2, |x| x.then(NoOpOperation))
    .end(FinalNoConsumes)
    .build("", 0);

  assert_eq!(40, size_of_val(&process));
  let messages = vec!["*123#", "Straight to trash", "trash me babe", "Choose a case", "2", "Empty good bye"];
  test_process_messages(&process, messages).await;
}

#[tokio::test]
async fn test_back_navigation() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByTwoCaseOption)
    .case_via(Case1, |x| x.show(OneInputRetryForm))
    .case_via(Case2, |x| x.then(ProduceCaseParam2))
    .end(SayGoodByAndConsumeCommonParams)
    .build("", 0);

  let messages = vec!["*123#", "Choose a case", "1", "This will be discarded", "0", "Choose a case", "2", "Good bye"];
  test_process_messages(&process, messages).await;
}

#[tokio::test]
async fn test_three_case_mixed_split_all_branches() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByThreeCaseOption)
    .case_end(Case1, |x| x.show(RetryOnceForm).end(FinalNoConsumes))
    .case_via(Case2, |x| x.show(OneInputRetryForm).then(ProduceOnlyCase2Param))
    .case_via(Case3, |x| x.then(ProduceOnlyCase1Param))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(
    &process,
    vec!["*123#", "Choose a case", "1", "Fancy a retry?", "retry", "Try again", "ok", "Empty good bye"],
  )
  .await;
  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "2",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Empty good bye",
    ],
  )
  .await;
  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "2",
      "This will be discarded",
      "0",
      "Choose a case",
      "1",
      "Fancy a retry?",
      "retry",
      "Try again",
      "ok",
      "Empty good bye",
    ],
  )
  .await;
  test_process_messages(&process, vec!["*123#", "Choose a case", "3", "Empty good bye"]).await;
}

#[tokio::test]
async fn test_finalized_case_early_finish_in_mixed_split() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByThreeCaseOption)
    .case_via(Case1, |x| x.then(ProduceCaseParam1))
    .case_end(Case2, |x| x.then(FinishProcessOperation).end(FinalNoConsumes))
    .case_end(Case3, |x| x.show(FinishAfterInput).end(FinalNoConsumes))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(&process, vec!["*123#", "Choose a case", "2", "Operation finished"]).await;
  test_process_messages(
    &process,
    vec!["*123#", "Choose a case", "3", "Last number in the process", "finish", "Always finish"],
  )
  .await;
}

#[tokio::test]
async fn test_three_case_split_finalized_flowing_finalized() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByThreeCaseOption)
    .case_end(Case1, |x| x.end(FinalNoConsumes))
    .case_via(Case2, |x| x.show(OneInputRetryForm).then(ProduceOnlyCase2Param))
    .case_end(Case3, |x| x.show(NoOpForm).end(FinalNoConsumes))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(&process, vec!["*123#", "Choose a case", "1", "Empty good bye"]).await;
  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "2",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Empty good bye",
    ],
  )
  .await;
  test_process_messages(&process, vec!["*123#", "Choose a case", "3", "Straight to trash", "20", "Empty good bye"])
    .await;
}

#[tokio::test]
async fn test_all_finalized_split_with_forms_and_early_finish() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByThreeCaseOption)
    .case_end(Case1, |x| x.show(OneInputRetryForm).end(FinalNoConsumes))
    .case_end(Case2, |x| x.then(FinishProcessOperation).end(FinalNoConsumes))
    .case_end(Case3, |x| x.show(RetryOnceForm).end(FinalNoConsumes))
    .build("", 0);

  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "1",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Empty good bye",
    ],
  )
  .await;
  test_process_messages(&process, vec!["*123#", "Choose a case", "2", "Operation finished"]).await;
  test_process_messages(
    &process,
    vec!["*123#", "Choose a case", "1", "This will be discarded", "0", "Choose a case", "2", "Operation finished"],
  )
  .await;
  test_process_messages(
    &process,
    vec!["*123#", "Choose a case", "3", "Fancy a retry?", "retry", "Try again", "ok", "Empty good bye"],
  )
  .await;
}

#[tokio::test]
async fn test_form_splitter_all_outcomes_in_flowing_and_finalized_cases() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show_split(TestFormSplitter)
    .case_via(Case1, |x| x.show(OneInputRetryForm).then(ProduceOnlyCase1Param))
    .case_via(Case2, |x| x.show(FinishAfterInput).then(ProduceOnlyCase2Param))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(
    &process,
    vec!["*123#", "choose case", "1", "This will be discarded", "10", "This will be accepted", "20", "Empty good bye"],
  )
  .await;
  test_process_messages(
    &process,
    vec!["*123#", "choose case", "2", "Last number in the process", "finish", "Always finish"],
  )
  .await;

  let process2 = ExtractMsisdnOperatorAndShortcodeString
    .show_split(TestFormSplitter)
    .case_end(Case1, |x| x.show(OneInputRetryForm).end(FinalNoConsumes))
    .case_via(Case2, |x| x.show(RetryOnceForm).then(ProduceOnlyCase2Param))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(
    &process2,
    vec![
      "*123#",
      "choose case",
      "retry",
      "retry again",
      "1",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Empty good bye",
    ],
  )
  .await;
  test_process_messages(&process2, vec!["*123#", "choose case", "finish", "finished early"]).await;
  test_process_messages(
    &process2,
    vec!["*123#", "choose case", "2", "Fancy a retry?", "retry", "Try again", "ok", "Empty good bye"],
  )
  .await;
}

#[tokio::test]
async fn test_flowing_case_of_flowing_split_all_branches() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByThreeCaseOption)
    .case_via(Case1, |x| x.show(OneInputRetryForm).then(ProduceOnlyCase1Param))
    .case_via(Case2, |x| x.show(RetryOnceForm).then(ProduceOnlyCase2Param))
    .case_via(Case3, |x| x.show(NoOpForm).then(ProduceOnlyCase1Param))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "1",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Empty good bye",
    ],
  )
  .await;
  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "1",
      "This will be discarded",
      "0",
      "Choose a case",
      "3",
      "Straight to trash",
      "10",
      "Empty good bye",
    ],
  )
  .await;
  test_process_messages(
    &process,
    vec!["*123#", "Choose a case", "2", "Fancy a retry?", "retry", "Try again", "ok", "Empty good bye"],
  )
  .await;
  test_process_messages(&process, vec!["*123#", "Choose a case", "3", "Straight to trash", "10", "Empty good bye"])
    .await;
}

#[tokio::test]
async fn test_flowing_case_of_finalized_split_all_branches() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByThreeCaseOption)
    .case_end(Case1, |x| x.show(OneInputRetryForm).end(FinalNoConsumes))
    .case_via(Case2, |x| x.show(RetryOnceForm).then(ProduceOnlyCase2Param))
    .case_via(Case3, |x| x.then(FinishProcessOperation))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(
    &process,
    vec!["*123#", "Choose a case", "1", "This will be discarded", "0", "Choose a case", "3", "Operation finished"],
  )
  .await;
  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "1",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Empty good bye",
    ],
  )
  .await;
  test_process_messages(
    &process,
    vec!["*123#", "Choose a case", "2", "Fancy a retry?", "retry", "Try again", "ok", "Empty good bye"],
  )
  .await;
  test_process_messages(&process, vec!["*123#", "Choose a case", "3", "Operation finished"]).await;
}

#[tokio::test]
async fn test_finalized_case_of_flowing_split_all_branches() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByThreeCaseOption)
    .case_via(Case1, |x| x.show(RetryOnceForm).then(ProduceOnlyCase1Param))
    .case_end(Case2, |x| x.show(OneInputRetryForm).end(FinalNoConsumes))
    .case_end(Case3, |x| x.then(FinishProcessOperation).end(FinalNoConsumes))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(
    &process,
    vec!["*123#", "Choose a case", "1", "Fancy a retry?", "retry", "Try again", "ok", "Empty good bye"],
  )
  .await;
  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "2",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Empty good bye",
    ],
  )
  .await;
  test_process_messages(&process, vec!["*123#", "Choose a case", "3", "Operation finished"]).await;
}

#[tokio::test]
async fn test_first_case_of_finalized_split_all_branches() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByThreeCaseOption)
    .case_end(Case1, |x| x.show(OneInputRetryForm).end(FinalNoConsumes))
    .case_end(Case2, |x| x.then(FinishProcessOperation).end(FinalNoConsumes))
    .case_end(Case3, |x| x.end(FinalNoConsumes))
    .build("", 0);

  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "1",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Empty good bye",
    ],
  )
  .await;
  test_process_messages(&process, vec!["*123#", "Choose a case", "2", "Operation finished"]).await;
  test_process_messages(&process, vec!["*123#", "Choose a case", "3", "Empty good bye"]).await;
}

#[tokio::test]
async fn test_first_case_of_flowing_split_all_branches() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByThreeCaseOption)
    .case_via(Case1, |x| x.show(OneInputRetryForm).then(ProduceOnlyCase1Param))
    .case_via(Case2, |x| x.then(FinishProcessOperation))
    .case_end(Case3, |x| x.end(FinalNoConsumes))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "1",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Empty good bye",
    ],
  )
  .await;
  test_process_messages(&process, vec!["*123#", "Choose a case", "2", "Operation finished"]).await;
}

#[tokio::test]
async fn test_nested_splits_all_combos() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByThreeCaseOption)
    .case_via(Case1, |x| x.then(ProduceOnlyCase1Param))
    .case_via(Case2, |x| {
      x.show(NoOpForm)
        .show_split(InnerFormSplitter)
        .case_via(Case1, |x| x.show(OneInputRetryForm).then(ProduceOnlyCase1Param))
        .case_end(Case2, |x| x.show(RetryOnceForm).end(FinalNoConsumes))
        .then(ProduceOnlyCase2Param)
    })
    .case_end(Case3, |x| x.end(FinalNoConsumes))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "2",
      "Straight to trash",
      "10",
      "choose case",
      "1",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Empty good bye",
    ],
  )
  .await;
  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "2",
      "Straight to trash",
      "10",
      "choose case",
      "2",
      "Fancy a retry?",
      "retry",
      "Try again",
      "ok",
      "Empty good bye",
    ],
  )
  .await;
}

#[tokio::test]
async fn test_four_case_mixed_split_all_branch_outcomes() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByFourCaseOption)
    .case_via(Case1, |x| x.show(OneInputRetryForm).then(ProduceOnlyCase1Param))
    .case_via(Case2, |x| x.show(RetryOnceForm).then(ProduceOnlyCase2Param))
    .case_via(Case3, |x| x.then(FinishProcessOperation))
    .case_end(Case4, |x| x.show(FinishAfterInput).end(FinalNoConsumes))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "1",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Empty good bye",
    ],
  )
  .await;
  test_process_messages(
    &process,
    vec!["*123#", "Choose a case", "2", "Fancy a retry?", "retry", "Try again", "ok", "Empty good bye"],
  )
  .await;
  test_process_messages(&process, vec!["*123#", "Choose a case", "3", "Operation finished"]).await;
  test_process_messages(
    &process,
    vec!["*123#", "Choose a case", "1", "This will be discarded", "0", "Choose a case", "3", "Operation finished"],
  )
  .await;
  test_process_messages(
    &process,
    vec!["*123#", "Choose a case", "4", "Last number in the process", "finish", "Always finish"],
  )
  .await;
}

#[tokio::test]
async fn test_four_case_finalized_first_all_branch_outcomes() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByFourCaseOption)
    .case_end(Case1, |x| x.show(OneInputRetryForm).end(FinalNoConsumes))
    .case_via(Case2, |x| x.show(RetryOnceForm).then(ProduceOnlyCase2Param))
    .case_end(Case3, |x| x.then(FinishProcessOperation).end(FinalNoConsumes))
    .case_via(Case4, |x| x.show(FinishAfterInput).then(ProduceOnlyCase1Param))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "1",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Empty good bye",
    ],
  )
  .await;
  test_process_messages(
    &process,
    vec!["*123#", "Choose a case", "2", "Fancy a retry?", "retry", "Try again", "ok", "Empty good bye"],
  )
  .await;
  test_process_messages(&process, vec!["*123#", "Choose a case", "3", "Operation finished"]).await;
  test_process_messages(
    &process,
    vec!["*123#", "Choose a case", "4", "Last number in the process", "finish", "Always finish"],
  )
  .await;
}

#[tokio::test]
async fn test_all_finalized_four_case_split() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByFourCaseOption)
    .case_end(Case1, |x| x.show(OneInputRetryForm).end(FinalNoConsumes))
    .case_end(Case2, |x| x.show(RetryOnceForm).end(FinalNoConsumes))
    .case_end(Case3, |x| x.then(FinishProcessOperation).end(FinalNoConsumes))
    .case_end(Case4, |x| x.show(FinishAfterInput).end(FinalNoConsumes))
    .build("", 0);

  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "1",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Empty good bye",
    ],
  )
  .await;
  test_process_messages(
    &process,
    vec!["*123#", "Choose a case", "2", "Fancy a retry?", "retry", "Try again", "ok", "Empty good bye"],
  )
  .await;
  test_process_messages(&process, vec!["*123#", "Choose a case", "3", "Operation finished"]).await;
  test_process_messages(
    &process,
    vec!["*123#", "Choose a case", "4", "Last number in the process", "finish", "Always finish"],
  )
  .await;
}

#[tokio::test]
async fn test_all_flowing_four_case_split() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByFourCaseOption)
    .case_via(Case1, |x| x.show(OneInputRetryForm).then(ProduceOnlyCase1Param))
    .case_via(Case2, |x| x.show(RetryOnceForm).then(ProduceOnlyCase2Param))
    .case_via(Case3, |x| x.then(FinishProcessOperation))
    .case_via(Case4, |x| x.show(NoOpForm).then(ProduceOnlyCase1Param))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "1",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Empty good bye",
    ],
  )
  .await;
  test_process_messages(
    &process,
    vec!["*123#", "Choose a case", "2", "Fancy a retry?", "retry", "Try again", "ok", "Empty good bye"],
  )
  .await;
  test_process_messages(&process, vec!["*123#", "Choose a case", "3", "Operation finished"]).await;
  test_process_messages(&process, vec!["*123#", "Choose a case", "4", "Straight to trash", "10", "Empty good bye"])
    .await;
}

#[tokio::test]
async fn test_operations_before_split_cases() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByTwoCaseOption)
    .case_via(Case1, |x| x.then(NoOpOperation).show(OneInputRetryForm).then(ProduceOnlyCase1Param))
    .case_end(Case2, |x| x.then(NoOpOperation).show(RetryOnceForm).end(FinalNoConsumes))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "1",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Empty good bye",
    ],
  )
  .await;
  test_process_messages(
    &process,
    vec!["*123#", "Choose a case", "2", "Fancy a retry?", "retry", "Try again", "ok", "Empty good bye"],
  )
  .await;
}

#[tokio::test]
async fn test_direct_resume_run_step_index_outcomes() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .then(NoOpOperation)
    .show(OneInputRetryForm)
    .end(FinalNoConsumes)
    .build("", 0);

  let session_ctx = session_init_value();

  for invalid_idx in [0, 1, 2, 100] {
    assert!(
      process
        .resume_run(session_ctx.clone(), PreviousRunYieldedAt(invalid_idx), "*123#".into(), None, None::<BackToken>)
        .await
        .is_err()
    );
  }

  let entry_only_process = ExtractMsisdnOperatorAndShortcodeString.end(FinalNoConsumes).build("", 0);
  assert!(
    entry_only_process
      .resume_run(
        session_ctx.clone(),
        PreviousRunYieldedAt(StepIndex::MIN + 1),
        "*123#".into(),
        None,
        None::<BackToken>,
      )
      .await
      .is_err()
  );

  let form_splitter_process = ExtractMsisdnOperatorAndShortcodeString
    .show_split(TestFormSplitter)
    .case_end(Case1, |x| x.end(FinalNoConsumes))
    .case_end(Case2, |x| x.end(FinalNoConsumes))
    .build("", 0);
  assert!(
    form_splitter_process
      .resume_run(
        session_ctx.clone(),
        PreviousRunYieldedAt(StepIndex::MIN + 100),
        "*123#".into(),
        None,
        None::<BackToken>,
      )
      .await
      .is_err()
  );

  let split_process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByTwoCaseOption)
    .case_via(Case1, |x| x.show(NoOpForm))
    .case_via(Case2, |x| x.show(NoOpForm))
    .end(FinalNoConsumes)
    .build("", 0);

  let split_session_ctx = hlist!(
    Split1Param,
    Split2Param,
    CommonSplitParam,
    CaseOptionParam(1),
    EntryParam(Msisdn(2_340_000_000_000), Operator::MTN, ShortcodeString("*123#".to_string()))
  )
  .serialize_param_list()
  .unwrap();

  // Valid form step indices in split_process:
  //   MIN     -> Entry
  //   MIN + 1 -> ChooseCaseForm
  //   MIN + 2 -> Case1 boundary (Subprocess, invalid)
  //   MIN + 3 -> Case1 NoOpForm
  //   MIN + 4 -> Case2 boundary (Subprocess, invalid)
  //   MIN + 5 -> Case2 NoOpForm
  //   >= MIN + 6 -> out of range (invalid)
  for offset in [2, 4, 6, 7, 10] {
    assert!(
      split_process
        .resume_run(
          split_session_ctx.clone(),
          PreviousRunYieldedAt(StepIndex::MIN + offset),
          "*123#".into(),
          None,
          None::<BackToken>,
        )
        .await
        .is_err(),
      "expected error for invalid offset {offset}"
    );
  }
}

#[tokio::test]
async fn test_multiple_forms_and_operations_in_flowing_case_of_flowing_split() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByThreeCaseOption)
    .case_via(Case1, |x| {
      x.show(OneInputRetryForm).show(FinishAfterInput).then(NoOpOperation).then(ProduceOnlyCase1Param)
    })
    .case_via(Case2, |x| x.show(OneInputRetryForm).show(RetryOnceForm).then(NoOpOperation).then(ProduceOnlyCase2Param))
    .case_via(Case3, |x| x.show(NoOpForm).then(ProduceOnlyCase1Param))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "1",
      "This will be discarded",
      "0",
      "Choose a case",
      "3",
      "Straight to trash",
      "10",
      "Empty good bye",
    ],
  )
  .await;
  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "1",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Last number in the process",
      "finish",
      "Always finish",
    ],
  )
  .await;
  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "2",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Fancy a retry?",
      "retry",
      "Try again",
      "ok",
      "Empty good bye",
    ],
  )
  .await;
}

#[tokio::test]
async fn test_multiple_forms_and_operations_in_flowing_case_of_finalized_split() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByThreeCaseOption)
    .case_end(Case1, |x| x.show(OneInputRetryForm).show(FinishAfterInput).end(FinalNoConsumes))
    .case_via(Case2, |x| x.show(OneInputRetryForm).show(RetryOnceForm).then(NoOpOperation).then(ProduceOnlyCase2Param))
    .case_via(Case3, |x| x.then(FinishProcessOperation))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(
    &process,
    vec!["*123#", "Choose a case", "1", "This will be discarded", "0", "Choose a case", "3", "Operation finished"],
  )
  .await;
  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "1",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Last number in the process",
      "finish",
      "Always finish",
    ],
  )
  .await;
  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "2",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Fancy a retry?",
      "retry",
      "Try again",
      "ok",
      "Empty good bye",
    ],
  )
  .await;
}

#[tokio::test]
async fn test_multiple_forms_and_operations_in_finalized_case_of_flowing_split() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByThreeCaseOption)
    .case_via(Case1, |x| x.show(OneInputRetryForm).show(RetryOnceForm).then(ProduceOnlyCase1Param))
    .case_end(Case2, |x| x.show(OneInputRetryForm).show(FinishAfterInput).end(FinalNoConsumes))
    .case_end(Case3, |x| x.show(OneInputRetryForm).show(RetryOnceForm).end(FinalNoConsumes))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "2",
      "This will be discarded",
      "0",
      "Choose a case",
      "3",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Fancy a retry?",
      "retry",
      "Try again",
      "ok",
      "Empty good bye",
    ],
  )
  .await;
  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "2",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Last number in the process",
      "finish",
      "Always finish",
    ],
  )
  .await;
}

#[tokio::test]
async fn test_four_case_split_with_multiple_forms_per_case() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByFourCaseOption)
    .case_via(Case1, |x| x.show(OneInputRetryForm).show(RetryOnceForm).then(ProduceOnlyCase1Param))
    .case_via(Case2, |x| x.show(OneInputRetryForm).show(FinishAfterInput).then(ProduceOnlyCase2Param))
    .case_via(Case3, |x| x.show(OneInputRetryForm).then(FinishProcessOperation))
    .case_end(Case4, |x| x.show(OneInputRetryForm).show(FinishAfterInput).end(FinalNoConsumes))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "1",
      "This will be discarded",
      "0",
      "Choose a case",
      "3",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Operation finished",
    ],
  )
  .await;
  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "2",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Last number in the process",
      "finish",
      "Always finish",
    ],
  )
  .await;
  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "4",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Last number in the process",
      "finish",
      "Always finish",
    ],
  )
  .await;
}

#[tokio::test]
async fn test_nested_inner_select_case_all_flowing_and_mixed() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByTwoCaseOption)
    .case_via(Case1, |x| x.then(ProduceOnlyCase1Param))
    .case_via(Case2, |x| {
      x.show(NoOpForm)
        .split(InnerSelectCase(PhantomData::<B0>))
        .case_via(InnerCase0, |x| x.show(OneInputRetryForm))
        .case_via(InnerCase1, |x| x.show(RetryOnceForm))
    })
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "2",
      "Straight to trash",
      "10",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Empty good bye",
    ],
  )
  .await;
}

#[tokio::test]
async fn test_nested_inner_form_splitter_flowing_and_finalized() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByTwoCaseOption)
    .case_via(Case1, |x| x.then(ProduceOnlyCase1Param))
    .case_via(Case2, |x| {
      x.show(NoOpForm)
        .show_split(InnerFormSplitter)
        .case_end(Case1, |x| x.show(OneInputRetryForm).end(FinalNoConsumes))
        .case_via(Case2, |x| x.show(RetryOnceForm).then(ProduceOnlyCase2Param))
    })
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "2",
      "Straight to trash",
      "10",
      "choose case",
      "1",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Empty good bye",
    ],
  )
  .await;
  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "2",
      "Straight to trash",
      "10",
      "choose case",
      "2",
      "Fancy a retry?",
      "retry",
      "Try again",
      "ok",
      "Empty good bye",
    ],
  )
  .await;
}

#[tokio::test]
async fn test_all_outcome_arms_in_three_case_flowing_split() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByThreeCaseOption)
    .case_via(Case1, |x| x.show(OneInputRetryForm).show(RetryOnceForm).then(ProduceOnlyCase1Param))
    .case_via(Case2, |x| x.show(OneInputRetryForm).show(FinishAfterInput).then(ProduceOnlyCase2Param))
    .case_via(Case3, |x| x.show(OneInputRetryForm).then(FinishProcessOperation))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "1",
      "This will be discarded",
      "0",
      "Choose a case",
      "3",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Operation finished",
    ],
  )
  .await;
  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "1",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Fancy a retry?",
      "retry",
      "Try again",
      "ok",
      "Empty good bye",
    ],
  )
  .await;
  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "2",
      "This will be discarded",
      "0",
      "Choose a case",
      "3",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Operation finished",
    ],
  )
  .await;
  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "2",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Last number in the process",
      "finish",
      "Always finish",
    ],
  )
  .await;
}

#[tokio::test]
async fn test_all_outcome_arms_in_three_case_mixed_split_finalized_first() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByThreeCaseOption)
    .case_end(Case1, |x| x.show(OneInputRetryForm).show(RetryOnceForm).end(FinalNoConsumes))
    .case_via(Case2, |x| x.show(OneInputRetryForm).show(FinishAfterInput).then(ProduceOnlyCase2Param))
    .case_via(Case3, |x| x.show(OneInputRetryForm).then(FinishProcessOperation))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "1",
      "This will be discarded",
      "0",
      "Choose a case",
      "3",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Operation finished",
    ],
  )
  .await;
  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "1",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Fancy a retry?",
      "retry",
      "Try again",
      "ok",
      "Empty good bye",
    ],
  )
  .await;
  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "2",
      "This will be discarded",
      "0",
      "Choose a case",
      "3",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Operation finished",
    ],
  )
  .await;
  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "2",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Last number in the process",
      "finish",
      "Always finish",
    ],
  )
  .await;
}

#[tokio::test]
async fn test_process_before_split_outcomes_flowing_split() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(NoOpForm)
    .show(OneInputRetryForm)
    .show(NoOpForm)
    .show(ChooseCaseForm)
    .split(SplitByTwoCaseOption)
    .case_via(Case1, |x| x.then(ProduceOnlyCase1Param))
    .case_via(Case2, |x| x.then(ProduceOnlyCase2Param))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(
    &process,
    vec![
      "*123#",
      "Straight to trash",
      "10",
      "This will be discarded",
      "0",
      "Straight to trash",
      "10",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Straight to trash",
      "30",
      "Choose a case",
      "1",
      "Empty good bye",
    ],
  )
  .await;
}

#[tokio::test]
async fn test_process_before_split_outcomes_finalized_split() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(NoOpForm)
    .show(OneInputRetryForm)
    .show(NoOpForm)
    .show(ChooseCaseForm)
    .split(SplitByTwoCaseOption)
    .case_end(Case1, |x| x.end(FinalNoConsumes))
    .case_end(Case2, |x| x.end(FinalNoConsumes))
    .build("", 0);

  test_process_messages(
    &process,
    vec![
      "*123#",
      "Straight to trash",
      "10",
      "This will be discarded",
      "0",
      "Straight to trash",
      "10",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Straight to trash",
      "30",
      "Choose a case",
      "1",
      "Empty good bye",
    ],
  )
  .await;
}

#[tokio::test]
async fn test_process_before_split_outcomes_mixed_split() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(NoOpForm)
    .show(OneInputRetryForm)
    .show(NoOpForm)
    .show(ChooseCaseForm)
    .split(SplitByTwoCaseOption)
    .case_end(Case1, |x| x.end(FinalNoConsumes))
    .case_via(Case2, |x| x.then(ProduceOnlyCase2Param))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(
    &process,
    vec![
      "*123#",
      "Straight to trash",
      "10",
      "This will be discarded",
      "0",
      "Straight to trash",
      "10",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Straight to trash",
      "30",
      "Choose a case",
      "2",
      "Empty good bye",
    ],
  )
  .await;
}

#[tokio::test]
async fn test_nested_splits_all_subprocess_arm_outcomes() {
  let process1 = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByTwoCaseOption)
    .case_via(Case1, |x| x.then(ProduceOnlyCase1Param))
    .case_via(Case2, |x| {
      x.show(NoOpForm)
        .show_split(InnerFormSplitter)
        .case_end(Case1, |x| x.show(OneInputRetryForm).end(FinalNoConsumes))
        .case_via(Case2, |x| x.show(OneInputRetryForm).then(FinishProcessOperation))
    })
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(
    &process1,
    vec![
      "*123#",
      "Choose a case",
      "2",
      "Straight to trash",
      "10",
      "choose case",
      "2",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Operation finished",
    ],
  )
  .await;

  let process2 = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByTwoCaseOption)
    .case_via(Case1, |x| x.then(ProduceOnlyCase1Param))
    .case_via(Case2, |x| {
      x.show(NoOpForm)
        .show_split(InnerFormSplitter)
        .case_via(Case1, |x| x.show(OneInputRetryForm).then(ProduceOnlyCase1Param))
        .case_end(Case2, |x| x.show(OneInputRetryForm).end(FinalNoConsumes))
    })
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(
    &process2,
    vec![
      "*123#",
      "Choose a case",
      "2",
      "Straight to trash",
      "10",
      "choose case",
      "2",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Empty good bye",
    ],
  )
  .await;

  let process3 = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByTwoCaseOption)
    .case_via(Case1, |x| x.then(ProduceOnlyCase1Param))
    .case_via(Case2, |x| {
      x.show(NoOpForm)
        .show_split(InnerFormSplitter)
        .case_via(Case1, |x| x.show(OneInputRetryForm).then(ProduceOnlyCase1Param))
        .case_via(Case2, |x| x.show(OneInputRetryForm).then(ProduceOnlyCase2Param))
    })
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(
    &process3,
    vec![
      "*123#",
      "Choose a case",
      "2",
      "Straight to trash",
      "10",
      "choose case",
      "1",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Empty good bye",
    ],
  )
  .await;
}

#[tokio::test]
async fn test_flowing_case_last_case_all_outcomes() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByThreeCaseOption)
    .case_via(Case1, |x| x.show(OneInputRetryForm).show(FinishAfterInput).then(ProduceOnlyCase1Param))
    .case_via(Case2, |x| x.show(OneInputRetryForm).show(FinishAfterInput).then(ProduceOnlyCase2Param))
    .case_via(Case3, |x| x.show(OneInputRetryForm).show(FinishAfterInput).then(ProduceOnlyCase1Param))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "3",
      "This will be discarded",
      "0",
      "Choose a case",
      "1",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Last number in the process",
      "finish",
      "Always finish",
    ],
  )
  .await;

  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "3",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Last number in the process",
      "finish",
      "Always finish",
    ],
  )
  .await;
}

#[tokio::test]
async fn test_finalized_case_of_flowing_split_last_case_all_outcomes() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByThreeCaseOption)
    .case_via(Case1, |x| x.show(OneInputRetryForm).show(FinishAfterInput).then(ProduceOnlyCase1Param))
    .case_via(Case2, |x| x.show(OneInputRetryForm).show(FinishAfterInput).then(ProduceOnlyCase2Param))
    .case_end(Case3, |x| x.show(OneInputRetryForm).show(FinishAfterInput).end(FinalNoConsumes))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "3",
      "This will be discarded",
      "0",
      "Choose a case",
      "1",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Last number in the process",
      "finish",
      "Always finish",
    ],
  )
  .await;

  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "3",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Last number in the process",
      "finish",
      "Always finish",
    ],
  )
  .await;
}

#[tokio::test]
async fn test_form_splitter_retry_finish_back() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(NoOpForm)
    .show_split(TestFormSplitter)
    .case_via(Case1, |x| x.then(ProduceOnlyCase1Param))
    .case_via(Case2, |x| x.then(ProduceOnlyCase2Param))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(
    &process,
    vec!["*123#", "Straight to trash", "10", "choose case", "retry", "retry again", "1", "Empty good bye"],
  )
  .await;

  test_process_messages(&process, vec!["*123#", "Straight to trash", "10", "choose case", "finish", "finished early"])
    .await;

  test_process_messages(
    &process,
    vec![
      "*123#",
      "Straight to trash",
      "10",
      "choose case",
      "0",
      "Straight to trash",
      "10",
      "choose case",
      "1",
      "Empty good bye",
    ],
  )
  .await;
}

#[tokio::test]
async fn test_finalized_process_subprocess_outcomes() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByTwoCaseOption)
    .case_end(Case1, |x| x.show(NoOpForm).show(OneInputRetryForm).show(FinishAfterInput).end(FinalNoConsumes))
    .case_via(Case2, |x| x.then(ProduceOnlyCase2Param))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "1",
      "Straight to trash",
      "10",
      "This will be discarded",
      "0",
      "Straight to trash",
      "10",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Last number in the process",
      "finish",
      "Always finish",
    ],
  )
  .await;

  test_process_messages(
    &process,
    vec![
      "*123#",
      "Choose a case",
      "1",
      "Straight to trash",
      "10",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "Last number in the process",
      "finish",
      "Always finish",
    ],
  )
  .await;
}

#[tokio::test]
async fn test_flowing_split_subprocess_outcomes_flowing_and_finalized() {
  let process1 = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByTwoCaseOption)
    .case_via(Case1, |x| x.then(ProduceOnlyCase1Param))
    .case_via(Case2, |x| {
      x.show_split(InnerFormSplitter)
        .case_via(Case1, |x| x.show(OneInputRetryForm).show(FinishAfterInput).then(ProduceOnlyCase1Param))
        .case_via(Case2, |x| x.show(OneInputRetryForm).then(FinishProcessOperation))
    })
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(
    &process1,
    vec![
      "*123#",
      "Choose a case",
      "2",
      "choose case",
      "1",
      "This will be discarded",
      "10",
      "This will be accepted",
      "ok",
      "Last number in the process",
      "finish",
      "Always finish",
    ],
  )
  .await;

  test_process_messages(
    &process1,
    vec![
      "*123#",
      "Choose a case",
      "2",
      "choose case",
      "2",
      "This will be discarded",
      "0",
      "choose case",
      "2",
      "This will be discarded",
      "10",
      "This will be accepted",
      "ok",
      "Operation finished",
    ],
  )
  .await;

  let process2 = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByTwoCaseOption)
    .case_via(Case1, |x| x.then(ProduceOnlyCase1Param))
    .case_via(Case2, |x| {
      x.show_split(InnerFormSplitter)
        .case_end(Case1, |x| x.show(OneInputRetryForm).end(FinalNoConsumes))
        .case_via(Case2, |x| x.show(OneInputRetryForm).show(FinishAfterInput).then(ProduceOnlyCase2Param))
    })
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(
    &process2,
    vec![
      "*123#",
      "Choose a case",
      "2",
      "choose case",
      "2",
      "This will be discarded",
      "10",
      "This will be accepted",
      "ok",
      "Last number in the process",
      "finish",
      "Always finish",
    ],
  )
  .await;

  test_process_messages(
    &process2,
    vec![
      "*123#",
      "Choose a case",
      "2",
      "choose case",
      "1",
      "This will be discarded",
      "10",
      "This will be accepted",
      "ok",
      "Empty good bye",
    ],
  )
  .await;
}

struct BackTokenInspectionForm1;
impl Form for BackTokenInspectionForm1 {
  type Error = CNil;
  type CreateFormConsumes = HNil;
  type ValidateInputConsumes = HNil;
  type Produces = HList![Case1Param];
  type Context = EmptyFormContext;
  type Messages = Messages;

  async fn create_form(
    &self,
    _consumes: <Self::CreateFormConsumes as ToRef<'_>>::Ref,
    back_token: Option<BackToken>,
  ) -> Result<FormWithContext<String, Self::Context>, Self::Error> {
    Ok(FormWithContext(format!("Form1 back={}", back_token.is_some()), EmptyFormContext))
  }

  async fn handle_input(
    &self,
    _consumes: <Self::ValidateInputConsumes as ToRef<'_>>::Ref,
    user_input: String,
    _form_context: Self::Context,
    back_token: Option<BackToken>,
  ) -> Result<InputValidation<Self::Produces, Messages, Self::Context>, Self::Error> {
    if user_input == "0" {
      if let Some(token) = back_token {
        return Ok(InputValidation::Back(token));
      }
      return Ok(InputValidation::Retry("Form1 cannot go back".into(), EmptyFormContext));
    }
    Ok(InputValidation::Successful(hlist!(Case1Param)))
  }
}

struct BackTokenInspectionForm2;
impl Form for BackTokenInspectionForm2 {
  type Error = CNil;
  type CreateFormConsumes = HNil;
  type ValidateInputConsumes = HNil;
  type Produces = HList![Case2Param];
  type Context = EmptyFormContext;
  type Messages = Messages;

  async fn create_form(
    &self,
    _consumes: <Self::CreateFormConsumes as ToRef<'_>>::Ref,
    back_token: Option<BackToken>,
  ) -> Result<FormWithContext<String, Self::Context>, Self::Error> {
    Ok(FormWithContext(format!("Form2 back={}", back_token.is_some()), EmptyFormContext))
  }

  async fn handle_input(
    &self,
    _consumes: <Self::ValidateInputConsumes as ToRef<'_>>::Ref,
    user_input: String,
    _form_context: Self::Context,
    back_token: Option<BackToken>,
  ) -> Result<InputValidation<Self::Produces, Messages, Self::Context>, Self::Error> {
    if user_input == "0"
      && let Some(token) = back_token
    {
      return Ok(InputValidation::Back(token));
    }
    Ok(InputValidation::Successful(hlist!(Case2Param)))
  }
}

struct BackTokenInspectionForm3;
impl Form for BackTokenInspectionForm3 {
  type Error = CNil;
  type CreateFormConsumes = HNil;
  type ValidateInputConsumes = HNil;
  type Produces = HNil;
  type Context = EmptyFormContext;
  type Messages = Messages;

  async fn create_form(
    &self,
    _consumes: <Self::CreateFormConsumes as ToRef<'_>>::Ref,
    back_token: Option<BackToken>,
  ) -> Result<FormWithContext<String, Self::Context>, Self::Error> {
    Ok(FormWithContext(format!("Form3 back={}", back_token.is_some()), EmptyFormContext))
  }

  async fn handle_input(
    &self,
    _consumes: <Self::ValidateInputConsumes as ToRef<'_>>::Ref,
    user_input: String,
    _form_context: Self::Context,
    back_token: Option<BackToken>,
  ) -> Result<InputValidation<Self::Produces, Messages, Self::Context>, Self::Error> {
    if user_input == "0"
      && let Some(token) = back_token
    {
      return Ok(InputValidation::Back(token));
    }
    Ok(InputValidation::Finish("Done".into()))
  }
}

#[tokio::test]
async fn test_back_token_disabled_at_first_form_step() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(BackTokenInspectionForm1)
    .show(BackTokenInspectionForm2)
    .show(BackTokenInspectionForm3)
    .end(FinalNoConsumes)
    .build("", 0);

  let messages = vec![
    "*123#",
    "Form1 back=false",
    "next",
    "Form2 back=true",
    "next",
    "Form3 back=true",
    "0",
    "Form2 back=true",
    "0",
    "Form1 back=true",
    "0",
    "Form1 back=false",
    "0",
    "Form1 cannot go back",
  ];

  test_process_messages(&process, messages).await;
}

#[tokio::test]
async fn test_flowing_split_case1_ending_with_form_resume() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByThreeCaseOption)
    .case_via(Case1, |x| x.show(OneInputRetryForm))
    .case_via(Case2, |x| x.then(ProduceCaseParam2))
    .case_via(Case3, |x| x.then(ProduceOnlyCase1Param))
    .end(FinalNoConsumes)
    .build("", 0);

  let messages = vec![
    "*123#",
    "Choose a case",
    "1",
    "This will be discarded",
    "10",
    "This will be accepted",
    "20",
    "Empty good bye",
  ];
  test_process_messages(&process, messages).await;
}

fn reusable_retry_and_produce_case2() -> impl FlowingProcess<
  EntryConsumes = HNil,
  SubprocessConsumes = HNil,
  Messages = Messages,
  Produces = HList![Case2Param, CommonCaseParam],
  EverProduced = HList![Case2Param, CommonCaseParam],
  Errors = LibraryErrors,
> {
  entry::<HNil, Messages>().show(OneInputRetryForm).then(ProduceOnlyCase2Param)
}

fn reusable_back_inspection() -> impl FlowingProcess<
  EntryConsumes = HNil,
  SubprocessConsumes = HNil,
  Messages = Messages,
  Produces = HList![Case2Param, Case1Param],
  EverProduced = HList![Case2Param, Case1Param],
  Errors = LibraryErrors,
> {
  entry::<HNil, Messages>().show(BackTokenInspectionForm1).show(BackTokenInspectionForm2)
}

fn reusable_retry_and_end_case2() -> impl FinalizedProcess<
  EntryConsumes = HList![Case2Param],
  SubprocessConsumes = HList![Case2Param],
  Messages = Messages,
  EverProduced = HList![CommonCaseParam, Case2Param],
  Errors = LibraryErrors,
> {
  entry::<HList![Case2Param], Messages>().show(OneInputRetryForm).end(FinalConsumeCase2Param)
}

fn reusable_end_no_consumes() -> impl FinalizedProcess<
  EntryConsumes = HNil,
  SubprocessConsumes = HNil,
  Messages = Messages,
  EverProduced = HNil,
  Errors = LibraryErrors,
> {
  entry::<HNil, Messages>().end(FinalNoConsumes)
}

#[tokio::test]
async fn test_then_via_form_retry_and_param_forwarding() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .then_via(reusable_retry_and_produce_case2())
    .end(FinalConsumeCase2Param)
    .build("", 0);

  let messages = vec!["*123#", "This will be discarded", "10", "This will be accepted", "20", "I ate Case2Param"];
  test_process_messages(&process, messages.clone()).await;

  // Reuse the exact same extracted sub-process in a different parameter context.
  let process_with_extra_param = ExtractMsisdnOperatorAndShortcodeString
    .then(ProduceOnlyCase1Param)
    .then_via(reusable_retry_and_produce_case2())
    .end(FinalConsumeCase2Param)
    .build("", 0);
  test_process_messages(&process_with_extra_param, messages).await;

  // Invalid step index past sub_end_index
  let Err(err) = process.resume_run(session_init_value(), PreviousRunYieldedAt(999), String::new(), None, None).await
  else {
    panic!("expected invalid step index error");
  };
  assert!(err.to_string().contains("invalid step index: 999"));
}

#[tokio::test]
async fn test_then_via_back_navigation_and_in_split() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByThreeCaseOption)
    .case_via(Case1, |x| x.then_via(reusable_back_inspection()))
    .case_via(Case2, |x| x.then(ProduceCaseParam2))
    .case_via(Case3, |x| x.then(ProduceOnlyCase1Param))
    .show(BackTokenInspectionForm3)
    .end(FinalNoConsumes)
    .build("", 0);

  let messages = vec![
    "*123#",
    "Choose a case",
    "1",
    "Form1 back=true",
    "next",
    "Form2 back=true",
    "next",
    "Form3 back=true",
    "0",
    "Form2 back=true",
    "0",
    "Form1 back=true",
    "0",
    "Choose a case",
    "1",
    "Form1 back=true",
    "next",
    "Form2 back=true",
    "next",
    "Form3 back=true",
    "next",
    "Done",
  ];
  test_process_messages(&process, messages).await;
}

#[tokio::test]
async fn test_then_via_early_outcomes() {
  let p_finish_before = ExtractMsisdnOperatorAndShortcodeString
    .then(FinishEarlyOperation)
    .then_via(entry::<HNil, Messages>().show(OneInputRetryForm))
    .end(FinalNoConsumes)
    .build("", 0);
  test_process_messages(&p_finish_before, vec!["*123#", "Operation finished"]).await;

  let p_finish_inside = ExtractMsisdnOperatorAndShortcodeString
    .then_via(entry::<HNil, Messages>().show(FinishAfterInput))
    .show(OneInputRetryForm)
    .end(FinalNoConsumes)
    .build("", 0);
  test_process_messages(&p_finish_inside, vec!["*123#", "Last number in the process", "10", "Always finish"]).await;

  let p_retry_inside = ExtractMsisdnOperatorAndShortcodeString
    .then_via(entry::<HNil, Messages>().show(OneInputRetryForm))
    .end(FinalNoConsumes)
    .build("", 0);
  test_process_messages(
    &p_retry_inside,
    vec!["*123#", "This will be discarded", "10", "This will be accepted", "20", "Empty good bye"],
  )
  .await;
}

#[tokio::test]
async fn test_then_end_form_retry_back_and_finish() {
  // 1. Reusable finalized sub-process with param forwarding and form retry.
  let p_sub = ExtractMsisdnOperatorAndShortcodeString
    .then(ProduceOnlyCase2Param)
    .then_end(reusable_retry_and_end_case2())
    .build("", 0);
  test_process_messages(
    &p_sub,
    vec!["*123#", "This will be discarded", "10", "This will be accepted", "20", "I ate Case2Param"],
  )
  .await;

  // 2. InvalidStepIndex past sub_end_index.
  let Err(err) = p_sub.resume_run(session_init_value(), PreviousRunYieldedAt(999), String::new(), None, None).await
  else {
    panic!("expected InvalidStepIndex error");
  };
  assert!(err.to_string().contains("invalid step index: 999"));

  // 3. Back navigation from inside `then_end` back to a form before `then_end`, and then forward again.
  let p_back = ExtractMsisdnOperatorAndShortcodeString
    .show(BackTokenInspectionForm1)
    .then_end(entry::<HNil, Messages>().show(BackTokenInspectionForm2).end(FinalNoConsumes))
    .build("", 0);
  test_process_messages(
    &p_back,
    vec![
      "*123#",
      "Form1 back=false",
      "next",
      "Form2 back=true",
      "0",
      "Form1 back=true",
      "next",
      "Form2 back=true",
      "next",
      "Empty good bye",
    ],
  )
  .await;

  // 4. Early Finish before `then_end`.
  let p_finish_before = ExtractMsisdnOperatorAndShortcodeString
    .then(FinishEarlyOperation)
    .then_end(reusable_end_no_consumes())
    .build("", 0);
  test_process_messages(&p_finish_before, vec!["*123#", "Operation finished"]).await;

  // 5. Extracted finalized sub-processes reused inside `split` `case_end` with different parameter contexts.
  let p_in_split = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByThreeCaseOption)
    .case_end(Case1, |x| x.then_end(reusable_end_no_consumes()))
    .case_end(Case2, |x| x.then(ProduceOnlyCase2Param).then_end(reusable_retry_and_end_case2()))
    .case_end(Case3, |x| x.then_end(reusable_end_no_consumes()))
    .build("", 0);
  test_process_messages(
    &p_in_split,
    vec![
      "*123#",
      "Choose a case",
      "2",
      "This will be discarded",
      "10",
      "This will be accepted",
      "20",
      "I ate Case2Param",
    ],
  )
  .await;
}

async fn test_process_messages(
  process: &RunnableProcess<impl FinalizedProcess<Messages = Messages>>,
  messages: Vec<&str>,
) {
  unit::test_process_messages(process, session_init_value(), messages).await;
}

fn session_init_value() -> SessionContext {
  hlist!(EntryParam(Msisdn(2_340_000_000_000), Operator::MTN, ShortcodeString("*123#".to_string())))
    .serialize_param_list()
    .unwrap()
}
