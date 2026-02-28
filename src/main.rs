use frunk_core::hlist::HNil;
use frunk_core::{Coprod, HList, hlist};
use serde::{Deserialize, Serialize};
use serde_value::Value;
use std::io::{self, Write};
use type_process_builder::builder::*;
use type_process_builder::step::{Entry, FailedInputValidationAttempts, Final, Form, FormSplitter, InputValidation};
use typenum::*;

#[derive(Clone, Deserialize, Serialize)]
struct EntryParam {
  shortcode_string: String,
}
impl ParamValue for EntryParam {
  type UID = U0;
}

#[derive(Clone, Deserialize, Serialize)]
struct Amount(u32);
impl ParamValue for Amount {
  type UID = U1;
}

struct EntryA;
impl Entry<Value> for EntryA {
  type Produces = HList![EntryParam];

  async fn handle(
    &self,
    _consumes: Vec<(ParamUID, Value)>,
    shortcode_string: String,
  ) -> anyhow::Result<HList![EntryParam]> {
    Ok(hlist!(EntryParam { shortcode_string }))
  }
}

pub struct Case1;
pub struct Case2;
struct SplitA;
impl FormSplitter for SplitA {
  type CreateFormConsumes = HNil;
  type ValidateInputConsumes = HNil;
  type Produces = Coprod![(Case1, HList![Amount]), (Case2, HNil)];

  async fn create_form(&self, _consumes: Self::CreateFormConsumes) -> anyhow::Result<Message> {
    Ok(Message("Enter 1 for 100 or 2 for custom amount ".into()))
  }

  async fn handle_input(
    &self,
    _consumes: Self::ValidateInputConsumes,
    user_input: String,
    _failed_input_validation_attempts: FailedInputValidationAttempts,
  ) -> anyhow::Result<InputValidation<Self::Produces>> {
    Ok(match user_input.as_str() {
      "1" => InputValidation::Successful(Self::Produces::inject((Case1, hlist!(Amount(100))))),
      "2" => InputValidation::Successful(Self::Produces::inject((Case2, HNil))),
      _ => InputValidation::Retry(Message("not 1 or 2".into())),
    })
  }
}

struct FormA;
impl Form for FormA {
  type CreateFormConsumes = HNil;
  type ValidateInputConsumes = HNil;
  type Produces = HList![Amount];

  async fn create_form(&self, _consumes: Self::CreateFormConsumes) -> anyhow::Result<Message> {
    Ok(Message("Enter a number".into()))
  }

  async fn handle_input(
    &self,
    _consumes: Self::ValidateInputConsumes,
    user_input: String,
    _failed_input_validation_attempts: FailedInputValidationAttempts,
  ) -> anyhow::Result<InputValidation<Self::Produces>> {
    match user_input.parse::<u32>() {
      Ok(value) => Ok(InputValidation::Successful(hlist![Amount(value)])),
      Err(_) => Ok(InputValidation::Retry(Message("Invalid number".into()))),
    }
  }
}

struct FinalA;
impl Final for FinalA {
  type Consumes = HList![EntryParam, Amount];

  async fn handle(&self, consumes: Self::Consumes) -> anyhow::Result<Message> {
    let amount = consumes.tail.head.0;
    Ok(Message(format!("The amount was: {amount}. Good bye!")))
  }
}

#[tokio::main]
async fn main() -> io::Result<()> {
  let demo_process = EntryA
    .show_split(SplitA)
    .case_via(Case1, |x| x)
    .case_via(Case2, |x| x.show(FormA))
    .end(FinalA)
    .build("demo_process", 0);

  let mut previous_run_produced = Vec::new();
  let mut previous_run_yielded_at = PreviousRunYieldedAt(StepIndex::MIN);
  let mut failed_attempts = FailedInputValidationAttempts(0);

  print!("Enter a shortcode");
  loop {
    print!("> ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let user_input = input.trim().to_owned();

    match demo_process
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
        println!("yielded: {}", msg.0);
      }
      RunOutcome::RetryUserInput(msg) => {
        failed_attempts = FailedInputValidationAttempts(failed_attempts.0 + 1);
        println!("retry: {}", msg.0);
      }
      RunOutcome::Finish(msg) => {
        println!("finished: {}", msg.0);
        return Ok(());
      }
    }
  }
}
