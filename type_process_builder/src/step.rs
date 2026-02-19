#[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Eq, Clone, PartialOrd, Ord, Hash)]
pub struct Message(pub String);

use std::collections::HashMap;
// Should only pass params required in further part of the process, but I don't know what they are.
// todo Make all the methods generic over Serializer
use crate::param_list::ParamList;
use frunk_core::coproduct::Coproduct;
use serde::de::DeserializeOwned;
use serde_value::Value;
use std::future::Future;

pub trait Entry<RawConsume: DeserializeOwned>: Sync {
  type Produces: ParamList;
  fn handle(
    &self,
    consumes: HashMap<u64, Value>,
    shortcode_string: String,
  ) -> impl Future<Output = anyhow::Result<Self::Produces>>;
}

pub trait Operation: Sync {
  type Consumes: ParamList;
  type Produces: ParamList;
  fn handle(&self, consumes: Self::Consumes) -> impl Future<Output = anyhow::Result<Self::Produces>>;
}

#[derive(PartialEq, Debug, Eq, Clone, PartialOrd, Ord, Hash)]
pub struct FailedInputValidationAttempts(pub u32);

#[derive(Debug, PartialEq, Eq)]
pub enum InputValidation<Produced> {
  Successful(Produced),
  Retry(Message),
  Finish(Message),
}

pub trait Form: Sync {
  type CreateFormConsumes: ParamList;
  type ValidateInputConsumes: ParamList;
  type Produces: ParamList;
  fn create_form(&self, consumes: Self::CreateFormConsumes) -> impl Future<Output = anyhow::Result<Message>>;
  fn handle_input(
    &self,
    consumes: Self::ValidateInputConsumes,
    user_input: String,
    failed_input_validation_attempts: FailedInputValidationAttempts,
  ) -> impl Future<Output = anyhow::Result<InputValidation<Self::Produces>>>;
}

pub trait SplitterOutput: Send + Sync{}
impl<Tag: Send + Sync, ThisCase: ParamList, OtherCase: Send + Sync> SplitterOutput for Coproduct<(Tag, ThisCase), 
  OtherCase> {}

/// Works with at least two cases.
/// Just produce link form with a single link using Form step
pub trait Splitter: Sync {
  type Consumes: ParamList;
  type Produces: SplitterOutput;
  fn handle(&self, consumes: Self::Consumes) -> impl Future<Output = anyhow::Result<Self::Produces>>;
}

/// Works with at least two cases.
/// Just produce link form with a single link using Form step
pub trait FromSplitter: Sync {
  type CreateFormConsumes: ParamList;
  type ValidateInputConsumes: ParamList;
  type Produces: SplitterOutput;
  fn create_form(&self, consumes: Self::CreateFormConsumes) -> impl Future<Output = anyhow::Result<Message>>;
  fn handle_input(
    &self,
    consumes: Self::ValidateInputConsumes,
    user_input: String,
    failed_input_validation_attempts: FailedInputValidationAttempts,
  ) -> impl Future<Output = anyhow::Result<InputValidation<Self::Produces>>>;
}

pub trait Final: Sync {
  type Consumes: ParamList;
  fn handle(&self, consumes: Self::Consumes) -> impl Future<Output = anyhow::Result<Message>>;
}
