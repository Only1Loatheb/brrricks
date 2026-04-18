// Should only pass params required in further part of the process, but I don't know what they are.
// todo Make all the methods generic over Serializer
use crate::builder::ParamUID;
use crate::param_list::ParamList;
use frunk_core::coproduct::Coproduct;
use serde_value::Value;
use std::future::Future;

pub trait ProcessMessages: Send + Sync {
  type FormMessage: Send + Sync;
  type FinalMessage: Send + Sync;
}

pub trait Entry: Sync {
  type Produces: ParamList;
  type Messages: ProcessMessages;
  fn handle(
    &self,
    consumes: Vec<(ParamUID, Value)>,
    initial_input: String,
  ) -> impl Future<Output = anyhow::Result<Self::Produces>> + Send;
}

pub trait Operation: Sync {
  type Consumes: ParamList;
  type Produces: ParamList;
  fn handle(&self, consumes: Self::Consumes) -> impl Future<Output = anyhow::Result<Self::Produces>> + Send;
}

#[derive(PartialEq, Debug, Eq, Clone, PartialOrd, Ord, Hash)]
pub struct FailedInputValidationAttempts(pub u8);

#[derive(Debug, PartialEq, Eq)]
pub enum InputValidation<Produced, Messages: ProcessMessages> {
  Successful(Produced),
  Retry(Messages::FormMessage),
  Finish(Messages::FinalMessage),
}

pub trait Form: Sync {
  type CreateFormConsumes: ParamList;
  type ValidateInputConsumes: ParamList;
  type Produces: ParamList;
  type Messages: ProcessMessages;
  fn create_form(
    &self,
    consumes: Self::CreateFormConsumes,
  ) -> impl Future<Output = anyhow::Result<<Self::Messages as ProcessMessages>::FormMessage>> + Send;
  fn handle_input(
    &self,
    consumes: Self::ValidateInputConsumes,
    user_input: String,
    failed_input_validation_attempts: FailedInputValidationAttempts,
  ) -> impl Future<Output = anyhow::Result<InputValidation<Self::Produces, Self::Messages>>> + Send;
}

pub trait SplitterOutput: Send + Sync {}
impl<Tag: Send + Sync, ThisCase: ParamList, OtherCase: Send + Sync> SplitterOutput
  for Coproduct<(Tag, ThisCase), OtherCase>
{
}

/// Works with at least two cases.
/// If you want single option form, just produce link form with a single link using Form step
pub trait Splitter: Sync {
  type Consumes: ParamList;
  type Produces: SplitterOutput;
  fn handle(&self, consumes: Self::Consumes) -> impl Future<Output = anyhow::Result<Self::Produces>> + Send;
}

/// Works with at least two cases.
/// Just produce link form with a single link using Form step
pub trait FormSplitter: Sync {
  type CreateFormConsumes: ParamList;
  type ValidateInputConsumes: ParamList;
  type Produces: SplitterOutput;
  type Messages: ProcessMessages;
  fn create_form(
    &self,
    consumes: Self::CreateFormConsumes,
  ) -> impl Future<Output = anyhow::Result<<Self::Messages as ProcessMessages>::FormMessage>> + Send;
  fn handle_input(
    &self,
    consumes: Self::ValidateInputConsumes,
    user_input: String,
    failed_input_validation_attempts: FailedInputValidationAttempts,
  ) -> impl Future<Output = anyhow::Result<InputValidation<Self::Produces, Self::Messages>>> + Send;
}

pub trait Final: Sync {
  type Consumes: ParamList;
  type FinalMessage: Send + Sync;
  fn handle(&self, consumes: Self::Consumes) -> impl Future<Output = anyhow::Result<Self::FinalMessage>> + Send;
}
