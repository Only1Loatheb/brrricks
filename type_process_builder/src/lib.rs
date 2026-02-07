pub mod builder;
pub mod param_list;
pub mod step;

// cargo doc --no-deps --package type_process_builder --features docs
/// A sequence diagram
#[cfg(feature = "docs")]
#[doc = simple_mermaid::mermaid!("process_builder_diagram.mmd")]
#[cfg_attr(not(feature = "docs"), doc = "")]
pub mod a {}

#[cfg(test)]
mod tests {
  use crate::builder::*;
  use crate::param_list::ParamValue;
  use crate::step::{Entry, Final, InputValidation, Operation, Splitter};
  use crate::step::{Form, Message};
  use anyhow::anyhow;
  use frunk_core::hlist::HNil;
  use frunk_core::{Coprod, HList, hlist};
  use log::debug;
  use serde::{Deserialize, Serialize};
  use serde_value::Value;
  use std::collections::BTreeMap;
  use std::ops::Not;
  use typenum::*;

  #[derive(Clone, Debug, Deserialize, Serialize)]
  struct Msisdn(u64);

  impl Msisdn {
    fn from(string: String) -> Option<Msisdn> {
      let prefix_len = string.len().checked_sub(10)?;
      // deny optional '+' https://doc.rust-lang.org/std/primitive.u64.html#method.from_str
      let _: () = string.starts_with('+').not().then_some(())?;
      string
        .split_at_checked(prefix_len)
        .and_then(|(_prefix, suffix)| suffix.parse::<u64>().ok())
        .map(|x| Msisdn(x))
    }
  }

  #[derive(Clone, Deserialize, Serialize)]
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

  struct EntryA;
  impl Entry<Value> for EntryA {
    type Produces = HList![EntryParam];

    async fn handle(
      &self,
      mut consumes: BTreeMap<Value, Value>,
      shortcode_string: String,
    ) -> anyhow::Result<HList![EntryParam]> {
      let msisdn_value = consumes
        .remove(&Value::String("msisdn".into()))
        .ok_or_else(|| anyhow!("Admin error or error on frontend."))?;
      let msisdn = match msisdn_value {
        Value::String(string) => Msisdn::from(string).ok_or_else(|| anyhow!("Admin error on frontend.")),
        _ => Err(anyhow!("Admin error on frontend.")),
      }?;
      let operator = consumes
        .remove(&Value::String("operator".into()))
        .ok_or_else(|| anyhow!("Admin error or error on frontend."))?;
      debug!("Operator: {:?}, {:?}", operator, msisdn);
      Ok(hlist!(EntryParam(
        msisdn,
        Operator::deserialize(operator)?,
        ShortcodeString(shortcode_string)
      )))
    }
  }

  struct Linear1;
  impl Operation for Linear1 {
    type Consumes = HNil;
    type Produces = HList![Case1Param, CommonCaseParam];

    async fn handle(&self, _consumes: Self::Consumes) -> anyhow::Result<Self::Produces> {
      Ok(hlist!(Case1Param, CommonCaseParam))
    }
  }

  struct Linear2;
  impl Operation for Linear2 {
    type Consumes = HNil;
    type Produces = HList![Case2Param, CommonCaseParam];

    async fn handle(&self, _consumes: Self::Consumes) -> anyhow::Result<Self::Produces> {
      Ok(hlist!(Case2Param, CommonCaseParam))
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

  struct FinalA;
  impl Final for FinalA {
    type Consumes = HList![EntryParam, CommonSplitParam, CommonCaseParam];

    async fn handle(&self, _consumes: Self::Consumes) -> anyhow::Result<Message> {
      Ok(Message("Good bye".into()))
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
    ) -> anyhow::Result<InputValidation<Self::Produces>> {
      Ok(InputValidation::Successful(hlist![CommonCaseParam]))
    }
  }

  struct FinalNoConsumes;
  impl Final for FinalNoConsumes {
    type Consumes = HList![];

    async fn handle(&self, _consumes: Self::Consumes) -> anyhow::Result<Message> {
      Ok(Message("Empty good bye".into()))
    }
  }

  fn session_init_value() -> Value {
    Value::Map(BTreeMap::from([
      (Value::String("msisdn".into()), Value::String("2340000000000".into())),
      (Value::String("operator".into()), Value::String("MTN".into())),
    ]))
  }

  #[tokio::test]
  async fn test_end() {
    let process = EntryA.end(FinalNoConsumes).build();

    let run_result = process
      .resume_run(session_init_value(), PreviousRunYieldedAt(0), "*123#".to_string())
      .await;
    assert_eq!(
      run_result.unwrap(),
      RunOutcome::Finish(Message("Empty good bye".into()))
    );
  }

  #[tokio::test]
  async fn test_split() {
    let process = EntryA
      .split(SplitA)
      .case_via(Case1, |x| x.then(Linear1))
      .case_via(Case2, |x| x.then(Linear2))
      .end(FinalA)
      .build();

    let run_result = process
      .resume_run(session_init_value(), PreviousRunYieldedAt(0), "*123#".to_string())
      .await;
    assert_eq!(run_result.unwrap(), RunOutcome::Finish(Message("Good bye".into())));
  }

  #[tokio::test]
  async fn test_yield() {
    let process = EntryA
      .split(SplitA)
      .case_via(Case1, |x| x.show(FormA))
      .case_via(Case2, |x| x.then(Linear2))
      .end(FinalA)
      .build();

    let run_result = process
      .resume_run(session_init_value(), PreviousRunYieldedAt(0), "*123#".to_string())
      .await;
    assert!(matches!(run_result.unwrap(), RunOutcome::Yield(message, ..) if message.0 == "Enter a number"));
  }
}
