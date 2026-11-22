use crate::error_list::ErrorCoproduct;
use crate::frunk::coproduct::Coproduct;
use crate::frunk::to_ref::ToRef;
use crate::param_list::ParamList;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::future::Future;
use std::marker::PhantomData;

pub trait ProcessMessages: Send + Sync {
  type FormMessage: Send + Sync;
  type FinalMessage: Send + Sync;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entry<Produces: ParamList, Messages: ProcessMessages> {
  pub(crate) phantom_data: PhantomData<(Produces, Messages)>,
}

#[must_use]
pub const fn entry<Produces: ParamList, Messages: ProcessMessages>() -> Entry<Produces, Messages> {
  Entry { phantom_data: PhantomData }
}

#[derive(Debug, PartialEq, Eq)]
pub enum OperationOutcome<Produced, FinalMessage: Send + Sync> {
  Successful(Produced),
  Finish(FinalMessage),
}

pub trait Operation: Send + Sync {
  type Consumes: ParamList + for<'a> ToRef<'a>;
  type Produces: ParamList;
  type FinalMessage: Send + Sync;
  type Error: ErrorCoproduct;
  fn handle(
    &self,
    consumes: <Self::Consumes as ToRef<'_>>::Ref,
  ) -> impl Future<Output = Result<OperationOutcome<Self::Produces, Self::FinalMessage>, Self::Error>> + Send;
}

/// I could make it type safe, but I want to allow for backing up from process that was composed in a dynamic GUI editor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackToken(pub(crate) ());

impl BackToken {
  /// Creates a [`BackToken`] that enables `InputValidation::Back` in form steps.
  ///
  /// Intended for process runner crates.
  #[cfg(feature = "process-runner-api")]
  #[must_use]
  pub fn create() -> Self {
    Self(())
  }
}

#[derive(Debug, PartialEq, Eq)]
pub enum InputValidation<Produced, Messages: ProcessMessages, FormContext: Serialize> {
  Successful(Produced),
  Retry(Messages::FormMessage, FormContext),
  Finish(Messages::FinalMessage),
  Back(BackToken),
}

pub struct FormWithContext<FormMessage, FormContext>(pub FormMessage, pub FormContext);

pub trait Form: Send + Sync {
  type CreateFormConsumes: ParamList + for<'a> ToRef<'a>;
  type ValidateInputConsumes: ParamList + for<'a> ToRef<'a>;
  type Produces: ParamList;
  type Context: Serialize + DeserializeOwned;
  type Messages: ProcessMessages;
  type Error: ErrorCoproduct;
  fn create_form(
    &self,
    consumes: <Self::CreateFormConsumes as ToRef<'_>>::Ref,
    back_token: Option<BackToken>,
  ) -> impl Future<
    Output = Result<FormWithContext<<Self::Messages as ProcessMessages>::FormMessage, Self::Context>, Self::Error>,
  > + Send;
  fn handle_input(
    &self,
    consumes: <Self::ValidateInputConsumes as ToRef<'_>>::Ref,
    user_input: String,
    form_context: Self::Context,
    back_token: Option<BackToken>,
  ) -> impl Future<Output = Result<InputValidation<Self::Produces, Self::Messages, Self::Context>, Self::Error>> + Send;
}

pub trait SplitterOutput: crate::sealed::Sealed + Send + Sync {}
impl<
  FirstTag: Send + Sync,
  FirstCase: ParamList,
  SecondTag: Send + Sync,
  SecondCase: ParamList,
  OtherCases: Send + Sync,
> SplitterOutput for Coproduct<(FirstTag, FirstCase), Coproduct<(SecondTag, SecondCase), OtherCases>>
{
}

/// Works with at least two cases.
/// If you want single option form, just produce link form with a single link using Form step
pub trait Splitter: Send + Sync {
  type Consumes: ParamList + for<'a> ToRef<'a>;
  type Produces: SplitterOutput;
  type Error: ErrorCoproduct;
  fn handle(
    &self,
    consumes: <Self::Consumes as ToRef<'_>>::Ref,
  ) -> impl Future<Output = Result<Self::Produces, Self::Error>> + Send;
}

/// Works with at least two cases.
/// Just produce link form with a single link using Form step
pub trait FormSplitter: Send + Sync {
  type CreateFormConsumes: ParamList + for<'a> ToRef<'a>;
  type ValidateInputConsumes: ParamList + for<'a> ToRef<'a>;
  type Produces: SplitterOutput;
  type Context: Serialize + DeserializeOwned;
  type Messages: ProcessMessages;
  type Error: ErrorCoproduct;
  fn create_form(
    &self,
    consumes: <Self::CreateFormConsumes as ToRef<'_>>::Ref,
    back_token: Option<BackToken>,
  ) -> impl Future<
    Output = Result<FormWithContext<<Self::Messages as ProcessMessages>::FormMessage, Self::Context>, Self::Error>,
  > + Send;
  fn handle_input(
    &self,
    consumes: <Self::ValidateInputConsumes as ToRef<'_>>::Ref,
    user_input: String,
    form_context: Self::Context,
    back_token: Option<BackToken>,
  ) -> impl Future<Output = Result<InputValidation<Self::Produces, Self::Messages, Self::Context>, Self::Error>> + Send;
}

pub trait Final: Send + Sync {
  type Consumes: ParamList;
  type FinalMessage: Send + Sync;
  type Error: ErrorCoproduct;
  fn handle(&self, consumes: Self::Consumes) -> impl Future<Output = Result<Self::FinalMessage, Self::Error>> + Send;
}
