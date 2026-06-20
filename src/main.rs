mod standard_io_process_runner;

use crate::standard_io_process_runner::{Message, Messages, standard_io_process_runner};
use frunk_core::hlist::HNil;
use frunk_core::traits::ToRef;
use frunk_core::{Coprod, HList, hlist, hlist_pat};
use serde::{Deserialize, Serialize};
use type_process_builder::builder::*;
use type_process_builder::step::{Entry, Final, Form, FormSplitter, FormWithContext, InputValidation};
use typenum::*;

#[derive(Deserialize, Serialize)]
struct ShortcodeString(String);
impl ParamValue for ShortcodeString {
  type UID = U0;
}

#[derive(Deserialize, Serialize)]
struct Amount(u32);
impl ParamValue for Amount {
  type UID = U1;
}

struct ShortcodeStringEntry;
impl Entry for ShortcodeStringEntry {
  type Produces = HList![ShortcodeString];
  type Messages = Messages;

  async fn handle(
    &self,
    _consumes: Vec<(ParamUID, Vec<u8>)>,
    shortcode_string: String,
  ) -> anyhow::Result<HList![ShortcodeString]> {
    Ok(hlist!(ShortcodeString(shortcode_string)))
  }
}

pub struct PredefinedAmount;
pub struct CustomAmount;
struct SelectAmountSource;
impl FormSplitter for SelectAmountSource {
  type CreateFormConsumes = HNil;
  type ValidateInputConsumes = HNil;
  type Produces = Coprod![(PredefinedAmount, HList![Amount]), (CustomAmount, HNil)];
  type Context = EmptyContext;
  type Messages = Messages;

  async fn create_form<'a>(
    &self,
    _consumes: <Self::CreateFormConsumes as ToRef<'a>>::Output,
  ) -> anyhow::Result<FormWithContext<Message, Self::Context>> {
    Ok(FormWithContext(Message("Enter 1 for 100 or 2 for custom amount".into()), EmptyContext))
  }

  async fn handle_input<'a>(
    &self,
    _consumes: <Self::ValidateInputConsumes as ToRef<'a>>::Output,
    user_input: String,
    _form_context: Self::Context,
  ) -> anyhow::Result<InputValidation<Self::Produces, Messages, Self::Context>> {
    Ok(match user_input.as_str() {
      "1" => InputValidation::Successful(Self::Produces::inject((PredefinedAmount, hlist!(Amount(100))))),
      "2" => InputValidation::Successful(Self::Produces::inject((CustomAmount, HNil))),
      _ => InputValidation::Retry(Message("not 1 or 2".into()), EmptyContext),
    })
  }
}

#[derive(Serialize, Deserialize)]
struct EmptyContext;

struct AmountForm;
impl Form for AmountForm {
  type CreateFormConsumes = HNil;
  type ValidateInputConsumes = HNil;
  type Produces = HList![Amount];
  type Context = EmptyContext;
  type Messages = Messages;

  async fn create_form<'a>(
    &self,
    _consumes: <Self::CreateFormConsumes as ToRef<'a>>::Output,
  ) -> anyhow::Result<FormWithContext<Message, Self::Context>> {
    Ok(FormWithContext(Message("Enter a number".into()), EmptyContext))
  }

  async fn handle_input<'a>(
    &self,
    _consumes: <Self::ValidateInputConsumes as ToRef<'a>>::Output,
    user_input: String,
    _form_context: Self::Context,
  ) -> anyhow::Result<InputValidation<Self::Produces, Messages, Self::Context>> {
    match user_input.parse::<u32>() {
      Ok(value) => Ok(InputValidation::Successful(hlist![Amount(value)])),
      Err(_) => Ok(InputValidation::Retry(Message("Invalid number".into()), EmptyContext)),
    }
  }
}

struct DisplayAmount;
impl Final for DisplayAmount {
  type Consumes = HList![ShortcodeString, Amount];
  type FinalMessage = Message;

  async fn handle(&self, consumes: Self::Consumes) -> anyhow::Result<Message> {
    let hlist_pat!(_shortcode_string, amount) = consumes;
    Ok(Message(format!("The amount was: {}. Good bye!", amount.0)))
  }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
  let process = ShortcodeStringEntry
    .show_split(SelectAmountSource)
    .case_via(PredefinedAmount, |x| x)
    .case_via(CustomAmount, |x| x.show(AmountForm))
    .end(DisplayAmount)
    .build("demo_process", 0);
  standard_io_process_runner(process).await
}
