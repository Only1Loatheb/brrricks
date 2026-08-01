use serde::{Deserialize, Serialize};
use type_process_builder::builder::{FinalizedProcess, FlowingProcess, RunnableProcess, SessionContext, SplitProcess};
use type_process_builder::step::{
  BackToken, Entry, Final, Form, FormSplitter, FormWithContext, InputValidation, ProcessMessages,
};
use type_process_builder::{Coprod, HList, HNil, ToRef, hlist, hlist_pat};
use typenum::{U0, U1};

use type_process_builder::impl_param_value;

#[derive(Deserialize, Serialize)]
struct ShortcodeString(pub String);

#[derive(Deserialize, Serialize)]
struct Amount(pub u32);

impl_param_value! {
  ShortcodeString => U0,
  Amount => U1,
}

pub struct Messages;
impl ProcessMessages for Messages {
  type FormMessage = String;
  type FinalMessage = String;
}

struct ShortcodeStringEntry;
impl Entry for ShortcodeStringEntry {
  type Produces = HList![ShortcodeString];
  type Messages = Messages;

  async fn handle(
    &self,
    _consumes: SessionContext,
    shortcode_string: String,
  ) -> anyhow::Result<HList![ShortcodeString]> {
    Ok(hlist!(ShortcodeString(shortcode_string)))
  }
}

struct PredefinedAmount;
struct CustomAmount;
struct SelectAmountSource;
impl FormSplitter for SelectAmountSource {
  type CreateFormConsumes = HNil;
  type ValidateInputConsumes = HNil;
  type Produces = Coprod![(PredefinedAmount, HList![Amount]), (CustomAmount, HNil)];
  type Context = EmptyFormContext;
  type Messages = Messages;

  async fn create_form(
    &self,
    _consumes: <Self::CreateFormConsumes as ToRef<'_>>::Ref,
    back_token: Option<BackToken>,
  ) -> anyhow::Result<FormWithContext<String, Self::Context>> {
    let string = back_token.map_or("Enter 1 for 100 or 2 for custom amount".into(), |_| {
      "Enter 1 for 100 or 2 for custom amount. 0 to go back".into()
    });
    Ok(FormWithContext(string, EmptyFormContext))
  }

  async fn handle_input(
    &self,
    _consumes: <Self::ValidateInputConsumes as ToRef<'_>>::Ref,
    user_input: String,
    _form_context: Self::Context,
    back_token: Option<BackToken>,
  ) -> anyhow::Result<InputValidation<Self::Produces, Messages, Self::Context>> {
    Ok(match (user_input.as_str(), back_token) {
      ("0", Some(back_token)) => InputValidation::Back(back_token),
      ("1", _) => InputValidation::Successful(Self::Produces::inject((PredefinedAmount, hlist!(Amount(100))))),
      ("2", _) => InputValidation::Successful(Self::Produces::inject((CustomAmount, HNil))),
      _ => InputValidation::Retry("not 1 or 2".into(), EmptyFormContext),
    })
  }
}

#[derive(Serialize, Deserialize)]
struct EmptyFormContext;

struct AmountForm;
impl Form for AmountForm {
  type CreateFormConsumes = HNil;
  type ValidateInputConsumes = HNil;
  type Produces = HList![Amount];
  type Context = EmptyFormContext;
  type Messages = Messages;

  async fn create_form(
    &self,
    _consumes: <Self::CreateFormConsumes as ToRef<'_>>::Ref,
    back_token: Option<BackToken>,
  ) -> anyhow::Result<FormWithContext<String, Self::Context>> {
    let string = back_token.map_or("Enter a number".into(), |_| "Enter a number. 0 to go back".into());
    Ok(FormWithContext(string, EmptyFormContext))
  }

  async fn handle_input(
    &self,
    _consumes: <Self::ValidateInputConsumes as ToRef<'_>>::Ref,
    user_input: String,
    _form_context: Self::Context,
    back_token: Option<BackToken>,
  ) -> anyhow::Result<InputValidation<Self::Produces, Messages, Self::Context>> {
    if user_input == "0"
      && let Some(token) = back_token
    {
      return Ok(InputValidation::Back(token));
    }
    match user_input.parse::<u32>() {
      Ok(value) => Ok(InputValidation::Successful(hlist![Amount(value)])),
      Err(_) => Ok(InputValidation::Retry("Invalid number".into(), EmptyFormContext)),
    }
  }
}

struct DisplayAmount;
impl Final for DisplayAmount {
  type Consumes = HList![ShortcodeString, Amount];
  type FinalMessage = String;

  async fn handle(&self, consumes: Self::Consumes) -> anyhow::Result<String> {
    let hlist_pat!(_shortcode_string, amount) = consumes;
    Ok(format!("The amount was: {}. Good bye!", amount.0))
  }
}

#[must_use]
pub fn build_demo_process() -> RunnableProcess<impl FinalizedProcess<Messages = Messages>> {
  ShortcodeStringEntry
    .show_split(SelectAmountSource)
    .case_via(PredefinedAmount, |x| x)
    .case_via(CustomAmount, |x| x.show(AmountForm))
    .end(DisplayAmount)
    .build("demo_process", 0)
}
