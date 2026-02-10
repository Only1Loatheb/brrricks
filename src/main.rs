use frunk_core::hlist::HNil;
use frunk_core::{Coprod, HList, hlist};
use serde::{Deserialize, Serialize};
use serde_value::Value;
use std::collections::HashMap;
use std::io::{self, Write};
use type_process_builder::builder::*;
use type_process_builder::step::{
  Entry, FailedInputValidationAttempts, Final, Form, InputValidation, Operation, Splitter,
};
use typenum::*;

#[derive(Clone, Deserialize, Serialize)]
struct EntryParam {
  shortcode_string: String,
}
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
struct Case2Param;
impl ParamValue for Case2Param {
  type UID = U4;
}

#[derive(Clone, Deserialize, Serialize)]
struct CommonCaseParam;
impl ParamValue for CommonCaseParam {
  type UID = U5;
}

struct EntryA;
impl Entry<Value> for EntryA {
  type Produces = HList![EntryParam];

  async fn handle(
    &self,
    _consumes: HashMap<u64, Value>,
    shortcode_string: String,
  ) -> anyhow::Result<HList![EntryParam]> {
    Ok(hlist!(EntryParam { shortcode_string }))
  }
}

pub struct Case1;
pub struct Case2;
struct SplitA;
impl Splitter for SplitA {
  type Consumes = HNil;
  type Produces = Coprod![
    (Case1, HList![Split1Param, CommonSplitParam]),
    (Case2, HList![Split2Param, CommonSplitParam])
  ];

  async fn handle(&self, _consumes: Self::Consumes) -> anyhow::Result<Self::Produces> {
    Ok(Self::Produces::inject((Case1, hlist!(Split1Param, CommonSplitParam))))
  }
}

struct FormA;
impl Form for FormA {
  type CreateFormConsumes = HNil;
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

struct Operation2;
impl Operation for Operation2 {
  type Consumes = HNil;
  type Produces = HList![Case2Param, CommonCaseParam];

  async fn handle(&self, _consumes: Self::Consumes) -> anyhow::Result<Self::Produces> {
    Ok(hlist!(Case2Param, CommonCaseParam))
  }
}

struct FinalA;
impl Final for FinalA {
  type Consumes = HList![EntryParam, CommonSplitParam, CommonCaseParam];

  async fn handle(&self, _consumes: Self::Consumes) -> anyhow::Result<Message> {
    Ok(Message("Good bye".into()))
  }
}

#[tokio::main]
async fn main() -> io::Result<()> {
  let process = EntryA
    .split(SplitA)
    .case_via(Case1, |x| x.show(FormA))
    .case_via(Case2, |x| x.then(Operation2))
    .end(FinalA)
    .build();

  let mut previous_run_produced = HashMap::new();
  let mut previous_run_yielded_at = PreviousRunYieldedAt(0);
  let mut failed_attempts = FailedInputValidationAttempts(0);

  loop {
    print!("> ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let user_input = input.trim().to_owned();

    match process
      .resume_run(
        previous_run_produced.clone(),
        previous_run_yielded_at.clone(),
        user_input,
        failed_attempts.clone(),
      )
      .await
      .map_err(io::Error::other)?
    {
      RunOutcome::Yield(msg, value, yielded_at) => {
        previous_run_produced = value;
        previous_run_yielded_at = PreviousRunYieldedAt(yielded_at.0);
        failed_attempts = FailedInputValidationAttempts(0);
        println!("yielded: {:?}", msg);
      }
      RunOutcome::RetryUserInput(msg) => {
        failed_attempts = FailedInputValidationAttempts(failed_attempts.0 + 1);
        println!("retry: {:?}", msg);
      }
      RunOutcome::Finish(msg) => {
        println!("finished: {:?}", msg);
        return Ok(());
      }
    }
  }
}
