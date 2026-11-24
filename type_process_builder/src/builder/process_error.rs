use crate::builder::StepIndex;
use crate::error_list::contains::CoprodContains;
use crate::error_list::{ContainsError, ErrorCoproduct, PreventDuplicateErrorUidInCoproduct, StepError};
use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};
use std::marker::PhantomData;

/// Failed to serialize or deserialize a [`crate::builder::SessionContext`] or
/// [`crate::builder::FormContext`].
#[derive(Debug)]
pub struct SerializationError(pub postcard::Error);

impl Display for SerializationError {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "failed to serialize or deserialize process state: {}", self.0)
  }
}

impl Error for SerializationError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    Some(&self.0)
  }
}

impl StepError for SerializationError {
  type UID = typenum::U0;
}

/// The supplied [`crate::builder::PreviousRunYieldedAt`] does not match any valid resume point in
/// the process.
#[derive(Debug)]
pub struct InvalidStepIndexError(pub StepIndex);

impl Display for InvalidStepIndexError {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "invalid step index: {}", self.0)
  }
}

impl Error for InvalidStepIndexError {}

impl StepError for InvalidStepIndexError {
  type UID = typenum::U1;
}

/// Library-level errors injected by [`crate::step::Entry`] and present in every process.
pub type LibraryErrors = crate::Coprod![SerializationError, InvalidStepIndexError];

/// Type-erased, constant-size error returned by process execution, tagged with the
/// compile-time [`ErrorCoproduct`] `StepErrors` of all possible error types.
pub struct ProcessError<StepErrors: ErrorCoproduct> {
  inner: anyhow::Error,
  _marker: PhantomData<StepErrors>,
}

impl<StepErrors: ErrorCoproduct> Debug for ProcessError<StepErrors> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    Debug::fmt(&self.inner, f)
  }
}

impl<StepErrors: ErrorCoproduct> Display for ProcessError<StepErrors> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    Display::fmt(&self.inner, f)
  }
}

impl<StepErrors: ErrorCoproduct> Error for ProcessError<StepErrors> {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    self.inner.source()
  }
}

impl<StepErrors: ErrorCoproduct> ProcessError<StepErrors> {
  #[inline(always)]
  pub(crate) fn from_serialization(err: postcard::Error) -> Self {
    Self { inner: anyhow::Error::new(SerializationError(err)), _marker: PhantomData }
  }

  #[inline(always)]
  pub(crate) fn invalid_step_index(idx: StepIndex) -> Self {
    Self { inner: anyhow::Error::new(InvalidStepIndexError(idx)), _marker: PhantomData }
  }

  #[inline(always)]
  pub(crate) fn from_step_error<E: ErrorCoproduct>(err: E) -> Self {
    Self { inner: err.into_anyhow(), _marker: PhantomData }
  }

  #[inline(always)]
  pub(crate) fn widen<NextErrors: ErrorCoproduct>(self) -> ProcessError<NextErrors> {
    ProcessError { inner: self.inner, _marker: PhantomData }
  }

  /// Returns `true` if this error is of concrete type `E`.
  ///
  /// Only compiles when `E` is a member of `StepErrors`.
  #[must_use]
  pub fn is<E: StepError>(&self) -> bool
  where
    StepErrors: ContainsError<E>,
  {
    self.inner.is::<E>()
  }

  /// Attempts to borrow this error as concrete type `E`.
  ///
  /// Only compiles when `E` is a member of `StepErrors`.
  #[must_use]
  pub fn downcast_ref<E: StepError>(&self) -> Option<&E>
  where
    StepErrors: ContainsError<E>,
  {
    self.inner.downcast_ref::<E>()
  }

  /// Attempts to take ownership of this error as concrete type `E`, returning `Err(self)` otherwise.
  ///
  /// Only compiles when `E` is a member of `StepErrors`.
  pub fn downcast<E: StepError>(self) -> Result<E, Self>
  where
    StepErrors: ContainsError<E>,
  {
    self.inner.downcast::<E>().map_err(|inner| Self { inner, _marker: PhantomData })
  }
}

impl<Head: StepError, Tail: ErrorCoproduct + CoprodContains<Head>> ProcessError<crate::Coproduct<Head, Tail>>
where
  <Tail as CoprodContains<Head>>::IsContained: PreventDuplicateErrorUidInCoproduct<Head>,
{
  /// Peels off the head error type `Head`, returning `Ok(Head)` if it matches or
  /// `Err(ProcessError<Tail>)` with the remaining possible error types.
  pub fn peel(self) -> Result<Head, ProcessError<Tail>> {
    self.inner.downcast::<Head>().map_err(|inner| ProcessError { inner, _marker: PhantomData })
  }
}

impl<Head: StepError> ProcessError<crate::Coproduct<Head, crate::CNil>> {
  /// Extracts the sole remaining error variant once all other variants have been peeled off.
  ///
  /// # Panics
  /// Never panics for errors produced by the process builder (the type-level `StepErrors`
  /// coproduct guarantees `Head` is the only possible variant).
  #[must_use]
  pub fn extract(self) -> Head {
    self.inner.downcast::<Head>().expect("ProcessError contained an error type outside its StepErrors coproduct")
  }
}

pub(crate) trait IntoProcessError<T>: Sized {
  fn into_process_err<Next: ErrorCoproduct>(self) -> Result<T, ProcessError<Next>>;
}

impl<T, E: ErrorCoproduct> IntoProcessError<T> for Result<T, ProcessError<E>> {
  #[inline(always)]
  fn into_process_err<Next: ErrorCoproduct>(self) -> Result<T, ProcessError<Next>> {
    self.map_err(ProcessError::widen)
  }
}

impl<T, E: ErrorCoproduct> IntoProcessError<T> for Result<T, E> {
  #[inline(always)]
  fn into_process_err<Next: ErrorCoproduct>(self) -> Result<T, ProcessError<Next>> {
    self.map_err(ProcessError::from_step_error)
  }
}

impl<T> IntoProcessError<T> for Result<T, postcard::Error> {
  #[inline(always)]
  fn into_process_err<Next: ErrorCoproduct>(self) -> Result<T, ProcessError<Next>> {
    self.map_err(ProcessError::from_serialization)
  }
}
