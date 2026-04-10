#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[allow(clippy::result_unit_err)]
pub mod builder;
pub mod param_list;
pub mod step;

// cargo doc --no-deps --package type_process_builder --features docs
/// View the diagrams
#[cfg(feature = "docs")]
#[doc = simple_mermaid::mermaid!("../doc/brrricks_app_session_flow.mmd")]
#[doc = simple_mermaid::mermaid!("../doc/process_builder_states.mmd")]
#[cfg_attr(not(feature = "docs"), doc = "")]
pub mod documentation_diagrams {}

#[cfg(test)]
mod tests {
  use crate::builder::*;
  use crate::param_list::ParamValue;
  use crate::step::{Entry, FailedInputValidationAttempts, Final, FormSplitter, InputValidation, Operation, Splitter};
  use crate::step::{Form, Message};
  use anyhow::anyhow;
  use frunk_core::hlist::HNil;
  use frunk_core::{Coprod, HList, hlist};
  use serde::{Deserialize, Serialize};
  use serde_value::Value;
  use typenum::*;

  #[derive(Clone, Debug, Deserialize, Serialize)]
  struct Msisdn(pub u64);

  #[allow(clippy::upper_case_acronyms)]
  #[derive(Clone, Debug, Deserialize, Serialize)]
  enum Operator {
    MTN,
  }

  #[derive(Clone, Deserialize, Serialize)]
  struct ShortcodeString(String);

  #[derive(Clone, Deserialize, Serialize)]
  struct EntryParam(Msisdn, Operator, ShortcodeString);
  impl ParamValue for EntryParam {
    type UID = U0;
  }

  #[derive(Clone, Deserialize, Serialize)]
  struct Split1Param;
  impl ParamValue for Split1Param {
    type UID = U1;
  }

  #[derive(Clone, Deserialize, Serialize)]
  struct Split2Param;
  impl ParamValue for Split2Param {
    type UID = U2;
  }

  #[derive(Clone, Deserialize, Serialize)]
  struct CommonSplitParam;
  impl ParamValue for CommonSplitParam {
    type UID = U3;
  }

  #[derive(Clone, Deserialize, Serialize)]
  struct Case1Param;
  impl ParamValue for Case1Param {
    type UID = U4;
  }

  #[derive(Clone, Deserialize, Serialize)]
  struct Case2Param;
  impl ParamValue for Case2Param {
    type UID = U5;
  }

  #[derive(Clone, Deserialize, Serialize)]
  struct CommonCaseParam;
  impl ParamValue for CommonCaseParam {
    type UID = U6;
  }

  struct ExtractMsisdnOperatorAndShortcodeString;
  impl Entry for ExtractMsisdnOperatorAndShortcodeString {
    type Produces = HList![EntryParam];

    #[cfg_attr(coverage_nightly, coverage(off))]
    async fn handle(
      &self,
      mut consumes: SessionContext,
      shortcode_string: String,
    ) -> anyhow::Result<HList![EntryParam]> {
      let operator = consumes.pop().ok_or_else(|| anyhow!("Admin error or error on frontend."))?.1;
      let msisdn_value = consumes.pop().ok_or_else(|| anyhow!("Admin error or error on frontend."))?.1;
      let msisdn = match msisdn_value {
        Value::String(string) => string.parse::<u64>().map_err(|_| anyhow!("Admin error on frontend.")),
        _ => Err(anyhow!("Admin error on frontend.")),
      }?;
      Ok(hlist!(EntryParam(Msisdn(msisdn), Operator::deserialize(operator)?, ShortcodeString(shortcode_string))))
    }
  }

  struct ProduceCaseParam1;
  impl Operation for ProduceCaseParam1 {
    type Consumes = HNil;
    type Produces = HList![Case1Param, CommonCaseParam];

    async fn handle(&self, _consumes: Self::Consumes) -> anyhow::Result<Self::Produces> {
      Ok(hlist!(Case1Param, CommonCaseParam))
    }
  }

  struct ProduceCaseParam2;
  impl Operation for ProduceCaseParam2 {
    type Consumes = HNil;
    type Produces = HList![Case2Param, CommonCaseParam];

    async fn handle(&self, _consumes: Self::Consumes) -> anyhow::Result<Self::Produces> {
      Ok(hlist!(Case2Param, CommonCaseParam))
    }
  }

  pub struct Case1;
  pub struct Case2;
  struct SelectCase1;
  impl Splitter for SelectCase1 {
    type Consumes = HNil;
    type Produces =
      Coprod![(Case1, HList![Split1Param, CommonSplitParam]), (Case2, HList![Split2Param, CommonSplitParam])];

    async fn handle(&self, _consumes: Self::Consumes) -> anyhow::Result<Self::Produces> {
      Ok(Self::Produces::inject((Case1, hlist!(Split1Param, CommonSplitParam))))
    }
  }

  struct SelectCase2;
  impl Splitter for SelectCase2 {
    type Consumes = HNil;
    type Produces =
      Coprod![(Case1, HList![Split1Param, CommonSplitParam]), (Case2, HList![Split2Param, CommonSplitParam])];

    async fn handle(&self, _consumes: Self::Consumes) -> anyhow::Result<Self::Produces> {
      Ok(Self::Produces::inject((Case2, hlist!(Split2Param, CommonSplitParam))))
    }
  }

  pub struct InnerCase1;
  pub struct InnerCase2;
  struct InnerSelectCase2;
  impl Splitter for InnerSelectCase2 {
    type Consumes = HNil;
    type Produces = Coprod![(InnerCase1, HNil), (InnerCase2, HNil)];
    async fn handle(&self, _consumes: Self::Consumes) -> anyhow::Result<Self::Produces> {
      Ok(Self::Produces::inject((InnerCase2, HNil)))
    }
  }

  pub struct Case3;
  struct SelectCase3;
  impl Splitter for SelectCase3 {
    type Consumes = HNil;
    type Produces = Coprod![
      (Case1, HList![Split1Param, CommonSplitParam]),
      (Case2, HList![Split2Param, CommonSplitParam]),
      (Case3, HList![CommonSplitParam]),
    ];

    async fn handle(&self, _consumes: Self::Consumes) -> anyhow::Result<Self::Produces> {
      Ok(Self::Produces::inject((Case3, hlist![CommonSplitParam])))
    }
  }

  struct SayGoodByAndConsumeCommonParams;
  impl Final for SayGoodByAndConsumeCommonParams {
    type Consumes = HList![EntryParam, CommonSplitParam, CommonCaseParam];

    async fn handle(&self, _consumes: Self::Consumes) -> anyhow::Result<Message> {
      Ok(Message("Good bye".into()))
    }
  }

  struct CommonCaseParamNumberForm;
  impl Form for CommonCaseParamNumberForm {
    type CreateFormConsumes = HList![Split1Param];
    type ValidateInputConsumes = HNil;
    type Produces = HList![CommonCaseParam];

    async fn create_form(&self, _consumes: Self::CreateFormConsumes) -> anyhow::Result<Message> {
      Ok(Message("Enter a number".into()))
    }

    async fn handle_input(
      &self,
      _consumes: Self::ValidateInputConsumes,
      _user_input: String,
      _failed_input_validation_attempts: FailedInputValidationAttempts,
    ) -> anyhow::Result<InputValidation<Self::Produces>> {
      Ok(InputValidation::Successful(hlist![CommonCaseParam]))
    }
  }

  struct NoOpForm;
  impl Form for NoOpForm {
    type CreateFormConsumes = HNil;
    type ValidateInputConsumes = HNil;
    type Produces = HNil;

    async fn create_form(&self, _consumes: Self::CreateFormConsumes) -> anyhow::Result<Message> {
      Ok(Message("Straight to trash".into()))
    }

    async fn handle_input(
      &self,
      _consumes: Self::ValidateInputConsumes,
      _user_input: String,
      _failed_input_validation_attempts: FailedInputValidationAttempts,
    ) -> anyhow::Result<InputValidation<Self::Produces>> {
      Ok(InputValidation::Successful(HNil))
    }
  }

  struct FinishAfterInput;
  impl Form for FinishAfterInput {
    type CreateFormConsumes = HNil;
    type ValidateInputConsumes = HNil;
    type Produces = HList![CommonCaseParam];

    async fn create_form(&self, _consumes: Self::CreateFormConsumes) -> anyhow::Result<Message> {
      Ok(Message("Last number in the process".into()))
    }

    async fn handle_input(
      &self,
      _consumes: Self::ValidateInputConsumes,
      _user_input: String,
      _failed_input_validation_attempts: FailedInputValidationAttempts,
    ) -> anyhow::Result<InputValidation<Self::Produces>> {
      Ok(InputValidation::Finish(Message("Always finnish".into())))
    }
  }

  struct OneInputRetryForm;
  impl Form for OneInputRetryForm {
    type CreateFormConsumes = HNil;
    type ValidateInputConsumes = HNil;
    type Produces = HList![CommonCaseParam];

    async fn create_form(&self, _consumes: Self::CreateFormConsumes) -> anyhow::Result<Message> {
      Ok(Message("This will be discarded".into()))
    }

    async fn handle_input(
      &self,
      _consumes: Self::ValidateInputConsumes,
      _user_input: String,
      failed_input_validation_attempts: FailedInputValidationAttempts,
    ) -> anyhow::Result<InputValidation<Self::Produces>> {
      match failed_input_validation_attempts.0 {
        0 => Ok(InputValidation::Retry(Message("This will be accepted".into()))),
        _ => Ok(InputValidation::Successful(hlist![CommonCaseParam])),
      }
    }
  }

  struct FinalNoConsumes;
  impl Final for FinalNoConsumes {
    type Consumes = HNil;

    async fn handle(&self, _consumes: Self::Consumes) -> anyhow::Result<Message> {
      Ok(Message("Empty good bye".into()))
    }
  }

  struct FinalConsumeCase2Param;
  impl Final for FinalConsumeCase2Param {
    type Consumes = HList![Case2Param];

    async fn handle(&self, _consumes: Self::Consumes) -> anyhow::Result<Message> {
      Ok(Message("I ate Case2Param".into()))
    }
  }

  fn session_init_value() -> SessionContext {
    vec![(0, Value::String("2340000000000".into())), (1, Value::String("MTN".into()))]
  }

  struct TestFormSplitter;

  impl FormSplitter for TestFormSplitter {
    type CreateFormConsumes = HNil;
    type ValidateInputConsumes = HNil;

    type Produces = Coprod![(Case1, HList![Split1Param]), (Case2, HList![Split2Param])];

    async fn create_form(&self, _consumes: Self::CreateFormConsumes) -> anyhow::Result<Message> {
      Ok(Message("choose case".into()))
    }

    async fn handle_input(
      &self,
      _consumes: Self::ValidateInputConsumes,
      user_input: String,
      failed: FailedInputValidationAttempts,
    ) -> anyhow::Result<InputValidation<Self::Produces>> {
      match (user_input.as_str(), failed.0) {
        ("retry", 0) => Ok(InputValidation::Retry(Message("retry again".into()))),
        ("finish", _) => Ok(InputValidation::Finish(Message("finished early".into()))),
        ("1", _) => Ok(InputValidation::Successful(Self::Produces::inject((Case1, hlist![Split1Param])))),
        _ => Ok(InputValidation::Successful(Self::Produces::inject((Case2, hlist![Split2Param])))),
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
      .split(SelectCase1)
      .case_via(Case1, |x| x.then(ProduceCaseParam1))
      .case_via(Case2, |x| x.then(ProduceCaseParam2))
      .end(SayGoodByAndConsumeCommonParams)
      .build("", 0);
    let messages = vec!["*123#", "Good bye"];
    test_process_messages(&process, messages).await;
  }

  #[tokio::test]
  async fn test_end_emitted_in_form_step() {
    let process = ExtractMsisdnOperatorAndShortcodeString.show(FinishAfterInput).end(FinalNoConsumes).build("", 0);
    let messages = vec!["*123#", "Last number in the process", "10", "Always finnish"];
    test_process_messages(&process, messages.clone()).await;

    #[cfg_attr(coverage_nightly, coverage(off))]
    async fn test_return_error_on_param_missing_from_init_value(
      process: &RunnableProcess<impl FinalizedProcess>,
      messages: Vec<&str>,
    ) {
      let mut init_value = session_init_value();
      init_value.pop();
      let run_outcome = process
        .resume_run(
          init_value,
          PreviousRunYieldedAt(StepIndex::MIN),
          messages[0].into(),
          FailedInputValidationAttempts(0),
        )
        .await;
      assert!(run_outcome.is_err_and(|x| format!("{x}") == "Admin error or error on frontend."))
    }
    test_return_error_on_param_missing_from_init_value(&process, vec!["*123#"]).await;

    #[cfg_attr(coverage_nightly, coverage(off))]
    async fn test_return_error_on_param_missing_from_context(
      process: &RunnableProcess<impl FinalizedProcess>,
      messages: Vec<&str>,
    ) {
      let mut messages_index = 0;
      let run_outcome = process
        .resume_run(
          session_init_value(),
          PreviousRunYieldedAt(StepIndex::MIN),
          messages[messages_index].into(),
          FailedInputValidationAttempts(0),
        )
        .await
        .expect("Test failed");
      messages_index += 1;
      match run_outcome {
        RunOutcome::Yield(msg, mut value, yielded_at) => {
          assert_eq!(msg.0, messages[messages_index]);
          value.pop();
          let run_outcome = process
            .resume_run(
              value,
              PreviousRunYieldedAt(yielded_at.0),
              messages[messages_index].into(),
              FailedInputValidationAttempts(0),
            )
            .await;
          assert!(run_outcome.is_err_and(|x| format!("{x}") == "Missing key: 0"))
        },
        RunOutcome::RetryUserInput(msg) => {
          assert_eq!(msg.0, messages[messages_index])
        },
        RunOutcome::Finish(msg) => {
          assert_eq!(msg.0, messages[messages_index])
        },
      }
    }
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
      .split(SelectCase1)
      .case_via(Case1, |x| x.show(CommonCaseParamNumberForm))
      .case_via(Case2, |x| x.then(ProduceCaseParam2))
      .end(SayGoodByAndConsumeCommonParams)
      .build("", 0);
    let messages = vec!["*123#", "Enter a number", "a number", "Good bye"];
    test_process_messages(&process, messages).await;
  }

  #[tokio::test]
  async fn test_end_first_case_of_finalized_split_process() {
    let process = ExtractMsisdnOperatorAndShortcodeString
      .split(SelectCase1)
      .case_end(Case1, |x| x.end(FinalNoConsumes))
      .case_end(Case2, |x| x.end(FinalNoConsumes))
      .build("", 0);
    let messages = vec!["*123#", "Empty good bye"];
    test_process_messages(&process, messages).await;
  }

  #[tokio::test]
  async fn test_yield_first_case_of_finalized_split_process() {
    let process = ExtractMsisdnOperatorAndShortcodeString
      .split(SelectCase1)
      .case_end(Case1, |x| x.show(NoOpForm).end(FinalNoConsumes))
      .case_end(Case2, |x| x.end(FinalNoConsumes))
      .build("", 0);
    let messages = vec!["*123#", "Straight to trash", "10", "Empty good bye"];
    test_process_messages(&process, messages).await;
  }

  #[tokio::test]
  async fn test_yield_first_case_of_finalized_split_process_split() {
    let process = ExtractMsisdnOperatorAndShortcodeString
      .split(SelectCase2)
      .case_end(Case1, |x| x.end(FinalNoConsumes))
      .case_end(Case2, |x| {
        x.split(InnerSelectCase2)
          .case_end(InnerCase1, |x| x.end(FinalNoConsumes))
          .case_end(InnerCase2, |x| x.end(FinalNoConsumes))
      })
      .build("", 0);
    let messages = vec!["*123#", "Empty good bye"];
    test_process_messages(&process, messages).await;
  }

  #[tokio::test]
  async fn test_flowing_case_of_finalized_split_process() {
    let process = ExtractMsisdnOperatorAndShortcodeString
      .split(SelectCase2)
      .case_end(Case1, |x| x.end(FinalNoConsumes))
      .case_via(Case2, |x| x.then(ProduceCaseParam2))
      .end(FinalConsumeCase2Param)
      .build("", 0);
    let messages = vec!["*123#", "I ate Case2Param"];
    test_process_messages(&process, messages).await;
  }

  #[tokio::test]
  async fn test_flowing_next_case_of_finalized_split_process() {
    let process = ExtractMsisdnOperatorAndShortcodeString
      .split(SelectCase3)
      .case_end(Case1, |x| x.end(FinalNoConsumes))
      .case_via(Case2, |x| x.then(ProduceCaseParam2))
      .case_end(Case3, |x| x.end(FinalNoConsumes))
      .end(FinalConsumeCase2Param)
      .build("", 0);
    let messages = vec!["*123#", "Empty good bye"];
    test_process_messages(&process, messages).await;
  }

  #[tokio::test]
  async fn test_flowing_split_with_nested_split_and_mixed_cases() {
    let process = ExtractMsisdnOperatorAndShortcodeString
      .split(SelectCase3)
      .case_via(Case1, |x| x.show(OneInputRetryForm))
      .case_end(Case2, |x| x.end(FinalNoConsumes))
      .case_via(Case3, |x| {
        x.split(InnerSelectCase2)
          .case_end(InnerCase1, |x| x.end(FinalNoConsumes))
          .case_via(InnerCase2, |x| x.show(NoOpForm))
          .then(ProduceCaseParam2)
      })
      .end(SayGoodByAndConsumeCommonParams)
      .build("", 0);
    test_process_messages(&process, vec!["*123#", "Straight to trash", "20", "Good bye"]).await;
  }

  #[tokio::test]
  async fn test_split_process_form_splitter() {
    let process = ExtractMsisdnOperatorAndShortcodeString
      .show_split(TestFormSplitter)
      .case_via(Case1, |x| x.show(NoOpForm))
      .case_end(Case2, |x| x.end(FinalNoConsumes))
      .end(FinalNoConsumes)
      .build("", 0);

    test_process_messages(&process, vec!["*123#", "choose case", "1", "Straight to trash", "10", "Empty good bye"])
      .await;
    test_process_messages(
      &process,
      vec!["*123#", "choose case", "retry", "retry again", "1", "Straight to trash", "10", "Empty good bye"],
    )
    .await;
    test_process_messages(&process, vec!["*123#", "choose case", "finish", "finished early"]).await;
  }

  #[tokio::test]
  async fn test_form_splitter_passthrough_with_multi_step_process_before() {
    let process = ExtractMsisdnOperatorAndShortcodeString
      .show(OneInputRetryForm)
      .then(ProduceCaseParam1)
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
      "Always finnish",
    ];
    test_process_messages(&process, messages).await;
  }

  async fn test_process_messages(process: &RunnableProcess<impl FinalizedProcess>, messages: Vec<&str>) {
    let mut previous_run_produced = session_init_value();
    let mut previous_run_yielded_at = PreviousRunYieldedAt(StepIndex::MIN);
    let mut failed_attempts = FailedInputValidationAttempts(0);
    let mut messages_index = 0;
    // run ordered_all_unique_param_uids in tests to check what code is reachable and it does not panic
    let _ = process.ordered_all_unique_param_uids();
    loop {
      let run_outcome = process
        .resume_run(
          previous_run_produced.clone(),
          previous_run_yielded_at.clone(),
          messages[messages_index].into(),
          failed_attempts.clone(),
        )
        .await
        .expect("Test failed");
      messages_index += 1;
      match run_outcome {
        RunOutcome::Yield(msg, value, yielded_at) => {
          previous_run_produced = value;
          previous_run_yielded_at = PreviousRunYieldedAt(yielded_at.0);
          failed_attempts = FailedInputValidationAttempts(0);

          assert_eq!(msg.0, messages[messages_index])
        },
        RunOutcome::RetryUserInput(msg) => {
          failed_attempts = FailedInputValidationAttempts(failed_attempts.0 + 1);

          assert_eq!(msg.0, messages[messages_index])
        },
        RunOutcome::Finish(msg) => {
          assert_eq!(msg.0, messages[messages_index]);
          break;
        },
      }
      messages_index += 1;
    }
    assert_eq!(messages_index + 1, messages.len());
  }
}
