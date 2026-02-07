#[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Eq, Clone, PartialOrd, Ord, Hash)]
pub struct Message(pub String);

// Should only pass params required in further part of the process, but I don't know what they are.
// todo Make all the methods generic over Serializer
use crate::param_list::ParamList;
use frunk_core::coproduct::Coproduct;
use serde::de::DeserializeOwned;
use std::collections::BTreeMap;
use std::future::Future;

pub trait Entry<RawConsume: DeserializeOwned> {
  type Produces: ParamList;
  fn handle(
    &self,
    consumes: BTreeMap<RawConsume, RawConsume>,
    shortcode_string: String,
  ) -> impl Future<Output = anyhow::Result<Self::Produces>>;
}

pub trait Operation {
  type Consumes: ParamList;
  type Produces: ParamList;
  fn handle(&self, consumes: Self::Consumes) -> impl Future<Output = anyhow::Result<Self::Produces>>;
}

#[derive(Debug, PartialEq, Eq)]
pub enum InputValidation<Produced> {
  Successful(Produced),
  Retry(Message),
  Finish(Message),
}

pub trait Form {
  type CreateFormConsumes: ParamList;
  type ValidateInputConsumes: ParamList;
  type Produces: ParamList;
  fn create_form(&self, consumes: Self::CreateFormConsumes) -> impl Future<Output = anyhow::Result<Message>>;
  fn handle_input(
    &self,
    consumes: Self::ValidateInputConsumes,
    user_input: String,
  ) -> impl Future<Output = anyhow::Result<InputValidation<Self::Produces>>>;
}

pub trait SplitterOutput {}
impl<Tag, ThisCase: ParamList, OtherCase> SplitterOutput for Coproduct<(Tag, ThisCase), OtherCase> {}

/// Works with at least two cases.
/// Just produce link form with a single link using Form step
pub trait Splitter {
  type Consumes: ParamList;
  type Produces: SplitterOutput;
  fn handle(&self, consumes: Self::Consumes) -> impl Future<Output = anyhow::Result<Self::Produces>>;
}

/// Works with at least two cases.
/// Just produce link form with a single link using Form step
pub trait FromSplitter {
  type CreateFormConsumes: ParamList;
  type ValidateInputConsumes: ParamList;
  type Produces: SplitterOutput;
  fn create_form(&self, consumes: Self::CreateFormConsumes) -> impl Future<Output = anyhow::Result<Message>>;
  fn handle_input(
    &self,
    consumes: Self::ValidateInputConsumes,
    user_input: String,
  ) -> impl Future<Output = anyhow::Result<InputValidation<Self::Produces>>>;
}

pub trait Final {
  type Consumes: ParamList;
  fn handle(&self, consumes: Self::Consumes) -> impl Future<Output = anyhow::Result<Message>>;
}
