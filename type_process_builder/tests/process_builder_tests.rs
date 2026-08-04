use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use type_process_builder::builder::*;
use type_process_builder::documentation_diagrams::{SessionState, in_memory_process_runner};
use type_process_builder::frunk::to_ref::ToRef;
use type_process_builder::step::{
  Entry, Final, Form, FormSplitter, FormWithContext, InputValidation, Operation, OperationOutcome, ProcessMessages,
  Splitter,
};
use type_process_builder::{Coprod, HList, HNil, hlist};
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

#[derive(Deserialize, Serialize)]
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

#[derive(Deserialize, Serialize)]
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

struct ExtractMsisdnOperatorAndShortcodeString;
impl Entry for ExtractMsisdnOperatorAndShortcodeString {
  type Produces = HList![EntryParam];
  type Messages = Messages;

  async fn handle(&self, mut consumes: SessionContext, initial_input: String) -> anyhow::Result<HList![EntryParam]> {
    let operator_value = consumes.pop().ok_or_else(|| anyhow!("Admin error or error on frontend."))?.1;
    let msisdn_value = consumes.pop().ok_or_else(|| anyhow!("Admin error or error on frontend."))?.1;
    let msisdn_str: String = postcard::from_bytes(&msisdn_value).map_err(|_| anyhow!("Admin error on frontend."))?;
    let msisdn = msisdn_str.parse::<u64>().map_err(|_| anyhow!("Admin error on frontend."))?;
    Ok(hlist!(EntryParam(
      Msisdn(msisdn),
      postcard::from_bytes(&operator_value).map_err(|_| anyhow!("Admin error or error on frontend."))?,
      ShortcodeString(initial_input)
    )))
  }
}

struct ProduceCaseParam1;
impl Operation for ProduceCaseParam1 {
  type Consumes = HNil;
  type Produces = HList![Case1Param, CommonCaseParam];
  type FinalMessage = String;

  async fn handle(
    &self,
    _consumes: <Self::Consumes as ToRef<'_>>::Ref,
  ) -> anyhow::Result<OperationOutcome<Self::Produces, Self::FinalMessage>> {
    Ok(OperationOutcome::Successful(hlist!(Case1Param, CommonCaseParam)))
  }
}

struct ProduceCaseParam2;
impl Operation for ProduceCaseParam2 {
  type Consumes = HNil;
  type Produces = HList![Case2Param, CommonCaseParam];
  type FinalMessage = String;

  async fn handle(
    &self,
    _consumes: <Self::Consumes as ToRef<'_>>::Ref,
  ) -> anyhow::Result<OperationOutcome<Self::Produces, Self::FinalMessage>> {
    Ok(OperationOutcome::Successful(hlist!(Case2Param, CommonCaseParam)))
  }
}

struct ProduceOnlyCase2Param;
impl Operation for ProduceOnlyCase2Param {
  type Consumes = HNil;
  type Produces = HList![Case2Param];
  type FinalMessage = String;

  async fn handle(
    &self,
    _consumes: <Self::Consumes as ToRef<'_>>::Ref,
  ) -> anyhow::Result<OperationOutcome<Self::Produces, Self::FinalMessage>> {
    Ok(OperationOutcome::Successful(hlist!(Case2Param)))
  }
}

struct ProduceOnlyCase1Param;
impl Operation for ProduceOnlyCase1Param {
  type Consumes = HNil;
  type Produces = HList![Case1Param];
  type FinalMessage = String;

  async fn handle(
    &self,
    _consumes: <Self::Consumes as ToRef<'_>>::Ref,
  ) -> anyhow::Result<OperationOutcome<Self::Produces, Self::FinalMessage>> {
    Ok(OperationOutcome::Successful(hlist!(Case1Param)))
  }
}

pub struct Case1;
pub struct Case2;
pub struct Case3;
pub struct Case4;

struct SplitByThreeCaseOption;
impl Splitter for SplitByThreeCaseOption {
  type Consumes = HList![CaseOptionParam];
  type Produces = Coprod![
    (Case1, HList![Split1Param, CommonSplitParam]),
    (Case2, HList![Split2Param, CommonSplitParam]),
    (Case3, HList![CommonSplitParam])
  ];
  async fn handle(&self, consumes: <Self::Consumes as ToRef<'_>>::Ref) -> anyhow::Result<Self::Produces> {
    Ok(match consumes.head.0 {
      1 => Self::Produces::inject((Case1, hlist!(Split1Param, CommonSplitParam))),
      2 => Self::Produces::inject((Case2, hlist!(Split2Param, CommonSplitParam))),
      _ => Self::Produces::inject((Case3, hlist!(CommonSplitParam))),
    })
  }
}

struct SplitByTwoCaseOption;
impl Splitter for SplitByTwoCaseOption {
  type Consumes = HList![CaseOptionParam];
  type Produces =
    Coprod![(Case1, HList![Split1Param, CommonSplitParam]), (Case2, HList![Split2Param, CommonSplitParam])];

  async fn handle(&self, consumes: <Self::Consumes as ToRef<'_>>::Ref) -> anyhow::Result<Self::Produces> {
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
  type Consumes = HNil;
  type Produces = Coprod![(InnerCase0, HNil), (InnerCase1, HNil)];

  async fn handle(&self, _consumes: <Self::Consumes as ToRef<'_>>::Ref) -> anyhow::Result<Self::Produces> {
    Ok(Self::Produces::inject((InnerCase0, HNil)))
  }
}

impl Splitter for InnerSelectCase<B1> {
  type Consumes = HNil;
  type Produces = Coprod![(InnerCase0, HNil), (InnerCase1, HNil)];

  async fn handle(&self, _consumes: <Self::Consumes as ToRef<'_>>::Ref) -> anyhow::Result<Self::Produces> {
    Ok(Self::Produces::inject((InnerCase1, HNil)))
  }
}

struct SplitByFourCaseOption;
impl Splitter for SplitByFourCaseOption {
  type Consumes = HList![CaseOptionParam];
  type Produces = Coprod![
    (Case1, HList![Split1Param, CommonSplitParam]),
    (Case2, HList![Split2Param, CommonSplitParam]),
    (Case3, HList![Split2Param, CommonSplitParam]),
    (Case4, HList![Split2Param, CommonSplitParam]),
  ];

  async fn handle(&self, consumes: <Self::Consumes as ToRef<'_>>::Ref) -> anyhow::Result<Self::Produces> {
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
  type Consumes = HNil;
  type Produces = HNil;
  type FinalMessage = String;

  async fn handle<'a>(
    &self,
    _consumes: Self::Consumes,
  ) -> anyhow::Result<OperationOutcome<Self::Produces, Self::FinalMessage>> {
    Ok(OperationOutcome::Successful(HNil))
  }
}

struct FinishEarlyOperation;
impl Operation for FinishEarlyOperation {
  type Consumes = HNil;
  type Produces = HNil;
  type FinalMessage = String;

  async fn handle(
    &self,
    _consumes: <Self::Consumes as ToRef<'_>>::Ref,
  ) -> anyhow::Result<OperationOutcome<Self::Produces, Self::FinalMessage>> {
    Ok(OperationOutcome::Finish("Operation finished".into()))
  }
}

#[derive(Serialize, Deserialize)]
struct EmptyFormContext;

struct FinishEarlyForm;
impl Form for FinishEarlyForm {
  type CreateFormConsumes = HNil;
  type ValidateInputConsumes = HNil;
  type Produces = HNil;
  type Context = EmptyFormContext;
  type Messages = Messages;

  async fn create_form(
    &self,
    _consumes: <Self::CreateFormConsumes as ToRef<'_>>::Ref,
    _back_token: Option<BackToken>,
  ) -> anyhow::Result<FormWithContext<String, Self::Context>> {
    Ok(FormWithContext("Finish early form".into(), EmptyFormContext))
  }

  async fn handle_input(
    &self,
    _consumes: <Self::ValidateInputConsumes as ToRef<'_>>::Ref,
    _user_input: String,
    _form_context: Self::Context,
    _back_token: Option<BackToken>,
  ) -> anyhow::Result<InputValidation<Self::Produces, Messages, Self::Context>> {
    Ok(InputValidation::Finish("Form finished".into()))
  }
}

struct SayGoodByAndConsumeCommonParams;
impl Final for SayGoodByAndConsumeCommonParams {
  type Consumes = HList![EntryParam, CommonSplitParam, CommonCaseParam];
  type FinalMessage = String;

  async fn handle(&self, _consumes: Self::Consumes) -> anyhow::Result<String> {
    Ok("Good bye".into())
  }
}

struct CommonCaseParam1Form;
impl Form for CommonCaseParam1Form {
  type CreateFormConsumes = HList![Split1Param];
  type ValidateInputConsumes = HNil;
  type Produces = HList![CommonCaseParam];
  type Context = EmptyFormContext;
  type Messages = Messages;

  async fn create_form(
    &self,
    _consumes: <Self::CreateFormConsumes as ToRef<'_>>::Ref,
    _back_token: Option<BackToken>,
  ) -> anyhow::Result<FormWithContext<String, Self::Context>> {
    Ok(FormWithContext("Enter a number".into(), EmptyFormContext))
  }

  async fn handle_input(
    &self,
    _consumes: <Self::ValidateInputConsumes as ToRef<'_>>::Ref,
    _user_input: String,
    _form_context: Self::Context,
    _back_token: Option<BackToken>,
  ) -> anyhow::Result<InputValidation<Self::Produces, Messages, Self::Context>> {
    Ok(InputValidation::Successful(hlist![CommonCaseParam]))
  }
}

struct CommonCaseParam2Form;
impl Form for CommonCaseParam2Form {
  type CreateFormConsumes = HList![Split2Param];
  type ValidateInputConsumes = HNil;
  type Produces = HList![CommonCaseParam];
  type Context = EmptyFormContext;
  type Messages = Messages;

  async fn create_form(
    &self,
    _consumes: <Self::CreateFormConsumes as ToRef<'_>>::Ref,
    _back_token: Option<BackToken>,
  ) -> anyhow::Result<FormWithContext<String, Self::Context>> {
    Ok(FormWithContext("Enter a number".into(), EmptyFormContext))
  }

  async fn handle_input(
    &self,
    _consumes: <Self::ValidateInputConsumes as ToRef<'_>>::Ref,
    _user_input: String,
    _form_context: Self::Context,
    _back_token: Option<BackToken>,
  ) -> anyhow::Result<InputValidation<Self::Produces, Messages, Self::Context>> {
    Ok(InputValidation::Successful(hlist![CommonCaseParam]))
  }
}

struct NoOpForm;
impl Form for NoOpForm {
  type CreateFormConsumes = HNil;
  type ValidateInputConsumes = HNil;
  type Produces = HNil;
  type Context = EmptyFormContext;
  type Messages = Messages;

  async fn create_form(
    &self,
    _consumes: <Self::CreateFormConsumes as ToRef<'_>>::Ref,
    _back_token: Option<BackToken>,
  ) -> anyhow::Result<FormWithContext<String, Self::Context>> {
    Ok(FormWithContext("Straight to trash".into(), EmptyFormContext))
  }

  async fn handle_input(
    &self,
    _consumes: <Self::ValidateInputConsumes as ToRef<'_>>::Ref,
    _user_input: String,
    _form_context: Self::Context,
    _back_token: Option<BackToken>,
  ) -> anyhow::Result<InputValidation<Self::Produces, Messages, Self::Context>> {
    Ok(InputValidation::Successful(HNil))
  }
}

struct FinishAfterInput;
impl Form for FinishAfterInput {
  type CreateFormConsumes = HNil;
  type ValidateInputConsumes = HNil;
  type Produces = HNil;
  type Context = EmptyFormContext;
  type Messages = Messages;

  async fn create_form(
    &self,
    _consumes: <Self::CreateFormConsumes as ToRef<'_>>::Ref,
    _back_token: Option<BackToken>,
  ) -> anyhow::Result<FormWithContext<String, Self::Context>> {
    Ok(FormWithContext("Last number in the process".into(), EmptyFormContext))
  }

  async fn handle_input(
    &self,
    _consumes: <Self::ValidateInputConsumes as ToRef<'_>>::Ref,
    _user_input: String,
    _form_context: Self::Context,
    _back_token: Option<BackToken>,
  ) -> anyhow::Result<InputValidation<Self::Produces, Messages, Self::Context>> {
    Ok(InputValidation::Finish("Always finish".into()))
  }
}

struct OneInputRetryForm;
impl Form for OneInputRetryForm {
  type CreateFormConsumes = HNil;
  type ValidateInputConsumes = HNil;
  type Produces = HList![CommonCaseParam];
  type Context = u16;
  type Messages = Messages;

  async fn create_form(
    &self,
    _consumes: <Self::CreateFormConsumes as ToRef<'_>>::Ref,
    _back_token: Option<BackToken>,
  ) -> anyhow::Result<FormWithContext<String, Self::Context>> {
    Ok(FormWithContext("This will be discarded".into(), 0))
  }

  async fn handle_input(
    &self,
    _consumes: <Self::ValidateInputConsumes as ToRef<'_>>::Ref,
    user_input: String,
    failed: Self::Context,
    back_token: Option<BackToken>,
  ) -> anyhow::Result<InputValidation<Self::Produces, Messages, Self::Context>> {
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
  type CreateFormConsumes = HNil;
  type ValidateInputConsumes = HNil;
  type Produces = HList![CaseOptionParam];
  type Context = EmptyFormContext;
  type Messages = Messages;

  async fn create_form(
    &self,
    _consumes: <Self::CreateFormConsumes as ToRef<'_>>::Ref,
    _back_token: Option<BackToken>,
  ) -> anyhow::Result<FormWithContext<String, Self::Context>> {
    Ok(FormWithContext("Choose a case".into(), EmptyFormContext))
  }

  async fn handle_input(
    &self,
    _consumes: <Self::ValidateInputConsumes as ToRef<'_>>::Ref,
    user_input: String,
    _form_context: Self::Context,
    _back_token: Option<BackToken>,
  ) -> anyhow::Result<InputValidation<Self::Produces, Messages, Self::Context>> {
    let option = user_input.parse::<u8>().unwrap_or(1);
    Ok(InputValidation::Successful(hlist!(CaseOptionParam(option))))
  }
}

struct FinalNoConsumes;
impl Final for FinalNoConsumes {
  type Consumes = HNil;
  type FinalMessage = String;

  async fn handle(&self, _consumes: Self::Consumes) -> anyhow::Result<String> {
    Ok("Empty good bye".into())
  }
}

struct FinalConsumeCase2Param;
impl Final for FinalConsumeCase2Param {
  type Consumes = HList![Case2Param];
  type FinalMessage = String;

  async fn handle(&self, _consumes: Self::Consumes) -> anyhow::Result<String> {
    Ok("I ate Case2Param".into())
  }
}

struct FinishProcessOperation;
impl Operation for FinishProcessOperation {
  type Consumes = HNil;
  type Produces = HNil;
  type FinalMessage = String;

  async fn handle(
    &self,
    _consumes: <Self::Consumes as ToRef<'_>>::Ref,
  ) -> anyhow::Result<OperationOutcome<Self::Produces, Self::FinalMessage>> {
    Ok(OperationOutcome::Finish("Operation finished".into()))
  }
}

fn session_init_value() -> SessionContext {
  vec![
    (0, postcard::to_allocvec(&"2340000000000".to_string()).unwrap()),
    (1, postcard::to_allocvec(&Operator::MTN).unwrap()),
  ]
}

struct TestFormSplitter;
impl FormSplitter for TestFormSplitter {
  type CreateFormConsumes = HNil;
  type ValidateInputConsumes = HNil;
  type Produces = Coprod![(Case1, HList![Split1Param]), (Case2, HList![Split2Param])];
  type Context = u16;
  type Messages = Messages;

  async fn create_form(
    &self,
    _consumes: <Self::CreateFormConsumes as ToRef<'_>>::Ref,
    _back_token: Option<BackToken>,
  ) -> anyhow::Result<FormWithContext<String, Self::Context>> {
    Ok(FormWithContext("choose case".into(), 0))
  }

  async fn handle_input(
    &self,
    _consumes: <Self::ValidateInputConsumes as ToRef<'_>>::Ref,
    user_input: String,
    failed: Self::Context,
    _back_token: Option<BackToken>,
  ) -> anyhow::Result<InputValidation<Self::Produces, Messages, Self::Context>> {
    match (user_input.as_str(), failed) {
      ("retry", 0) => Ok(InputValidation::Retry("retry again".into(), failed + 1)),
      ("finish", _) => Ok(InputValidation::Finish("finished early".into())),
      ("1", _) => Ok(InputValidation::Successful(Self::Produces::inject((Case1, hlist![Split1Param])))),
      _ => Ok(InputValidation::Successful(Self::Produces::inject((Case2, hlist![Split2Param])))),
    }
  }
}

struct InnerFormSplitter;
impl FormSplitter for InnerFormSplitter {
  type CreateFormConsumes = HNil;
  type ValidateInputConsumes = HNil;
  type Produces = Coprod![(Case1, HList![InnerSplit1Param]), (Case2, HList![InnerSplit2Param])];
  type Context = i16;
  type Messages = Messages;

  async fn create_form(
    &self,
    _consumes: <Self::CreateFormConsumes as ToRef<'_>>::Ref,
    _back_token: Option<BackToken>,
  ) -> anyhow::Result<FormWithContext<String, Self::Context>> {
    Ok(FormWithContext("choose case".into(), 0))
  }

  async fn handle_input(
    &self,
    _consumes: <Self::ValidateInputConsumes as ToRef<'_>>::Ref,
    user_input: String,
    failed: Self::Context,
    _back_token: Option<BackToken>,
  ) -> anyhow::Result<InputValidation<Self::Produces, Messages, Self::Context>> {
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
  process: &RunnableProcess<impl FinalizedProcess<Messages = Messages>>,
  messages: Vec<&str>,
) {
  let mut init_value = session_init_value();
  init_value.pop();
  let run_outcome = process
    .resume_run(init_value, PreviousRunYieldedAt(StepIndex::MIN), messages[0].into(), None, None::<BackToken>)
    .await;
  assert!(run_outcome.is_err_and(|x| format!("{x}") == "Admin error or error on frontend."));
}

async fn test_return_error_on_param_missing_from_context(
  process: &RunnableProcess<impl FinalizedProcess<Messages = Messages>>,
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
    RunOutcome::Yield(msg, mut value, yielded_at, context) => {
      assert_eq!(msg, messages[messages_index]);
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
      assert!(run_outcome.is_err_and(|x| format!("{x}") == "Missing key: 0"));
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
  type CreateFormConsumes = HNil;
  type ValidateInputConsumes = HNil;
  type Produces = HNil;
  type Context = EmptyFormContext;
  type Messages = Messages;

  async fn create_form(
    &self,
    _consumes: <Self::CreateFormConsumes as ToRef<'_>>::Ref,
    _back_token: Option<BackToken>,
  ) -> anyhow::Result<FormWithContext<String, Self::Context>> {
    Ok(FormWithContext("Fancy a retry?".into(), EmptyFormContext))
  }

  async fn handle_input(
    &self,
    _consumes: <Self::ValidateInputConsumes as ToRef<'_>>::Ref,
    user_input: String,
    _form_context: Self::Context,
    _back_token: Option<BackToken>,
  ) -> anyhow::Result<InputValidation<Self::Produces, Messages, Self::Context>> {
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

  assert_eq!(process.get_name(), "test_process");
  assert_eq!(process.get_version(), 123);
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
async fn test_retry_in_case_2_then_resume_in_mixed_split() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByThreeCaseOption)
    .case_end(Case1, |x| x.end(FinalNoConsumes))
    .case_via(Case2, |x| x.show(RetryOnceForm).then(ProduceCaseParam2))
    .case_end(Case3, |x| x.end(FinalNoConsumes))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(
    &process,
    vec!["*123#", "Choose a case", "2", "Fancy a retry?", "retry", "Try again", "accept", "Empty good bye"],
  )
  .await;
}

#[tokio::test]
async fn test_retry_in_case_1_then_resume_in_mixed_split() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByTwoCaseOption)
    .case_via(Case1, |x| x.show(RetryOnceForm).then(ProduceCaseParam1))
    .case_end(Case2, |x| x.end(FinalNoConsumes))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(
    &process,
    vec!["*123#", "Choose a case", "1", "Fancy a retry?", "retry", "Try again", "accept", "Empty good bye"],
  )
  .await;
  test_process_messages(&process, vec!["*123#", "Choose a case", "1", "Fancy a retry?", "anything", "Empty good bye"])
    .await;
}

#[tokio::test]
async fn test_finish_in_case_1_finalized_mixed_split() {
  let process = ExtractMsisdnOperatorAndShortcodeString
    .show(ChooseCaseForm)
    .split(SplitByTwoCaseOption)
    .case_end(Case1, |x| x.then(FinishProcessOperation).end(FinalNoConsumes))
    .case_via(Case2, |x| x.then(ProduceCaseParam2))
    .end(FinalNoConsumes)
    .build("", 0);

  test_process_messages(&process, vec!["*123#", "Choose a case", "1", "Operation finished"]).await;
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

  assert_eq!(48, size_of_val(&process));
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

async fn test_process_messages(
  process: &RunnableProcess<impl FinalizedProcess<Messages = Messages>>,
  messages: Vec<&str>,
) {
  let mut state = SessionState {
    session_context: session_init_value(),
    previous_run_yielded_at: PreviousRunYieldedAt(StepIndex::MIN),
    form_context: None,
    visited_form_steps: Vec::new(),
  };
  let mut index = 0;
  while index < messages.len() {
    let user_input = messages[index];
    index += 1;
    let result = in_memory_process_runner(process, &mut state, user_input).await;
    let expected_message = messages[index];
    index += 1;
    match result {
      Ok(msg) | Err(msg) => {
        assert_eq!(msg, expected_message);
      },
    }
  }
}

// #[test]
// fn test_ever_produced_union_prevents_reusing_removed_params() {
//   let process = ExtractMsisdnOperatorAndShortcodeString
//     .show(ChooseCaseForm)
//     .split(SplitByTwoCaseOption)
//     .case_via(Case1, |x| x.then(ProduceCaseParam1))
//     .case_via(Case2, |x| x.then(ProduceCaseParam2))
//     .end(SayGoodByAndConsumeCommonParams);
//
//   let _ = process;
// }
