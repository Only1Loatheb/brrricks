use crate::builder::StepIndex;
use crate::error_list::contains::CoprodContains;
use crate::error_list::{ContainsError, ErrorCoproduct, PreventDuplicateErrorUidInCoproduct, StepError};
use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};
use std::marker::PhantomData;

/// Failed to serialize or deserialize a [`crate::builder::SessionContext`] or
/// [`crate::builder::FormContext`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SerializationError {
  WontImplement,
  NotYetImplemented,
  SerializeBufferFull,
  SerializeSeqLengthUnknown,
  DeserializeUnexpectedEnd,
  DeserializeBadVarint,
  DeserializeBadBool,
  DeserializeBadChar,
  DeserializeBadUtf8,
  DeserializeBadOption,
  DeserializeBadEnum,
  DeserializeBadEncoding,
  DeserializeBadCrc,
  SerdeSerCustom,
  SerdeDeCustom,
  CollectStrError,
  Other(String),
}

impl SerializationError {
  pub(crate) fn from_postcard(err: postcard::Error) -> Self {
    match err {
      postcard::Error::WontImplement => Self::WontImplement,
      postcard::Error::NotYetImplemented => Self::NotYetImplemented,
      postcard::Error::SerializeBufferFull => Self::SerializeBufferFull,
      postcard::Error::SerializeSeqLengthUnknown => Self::SerializeSeqLengthUnknown,
      postcard::Error::DeserializeUnexpectedEnd => Self::DeserializeUnexpectedEnd,
      postcard::Error::DeserializeBadVarint => Self::DeserializeBadVarint,
      postcard::Error::DeserializeBadBool => Self::DeserializeBadBool,
      postcard::Error::DeserializeBadChar => Self::DeserializeBadChar,
      postcard::Error::DeserializeBadUtf8 => Self::DeserializeBadUtf8,
      postcard::Error::DeserializeBadOption => Self::DeserializeBadOption,
      postcard::Error::DeserializeBadEnum => Self::DeserializeBadEnum,
      postcard::Error::DeserializeBadEncoding => Self::DeserializeBadEncoding,
      postcard::Error::DeserializeBadCrc => Self::DeserializeBadCrc,
      postcard::Error::SerdeSerCustom => Self::SerdeSerCustom,
      postcard::Error::SerdeDeCustom => Self::SerdeDeCustom,
      postcard::Error::CollectStrError => Self::CollectStrError,
      other => Self::Other(other.to_string()),
    }
  }
}

impl Display for SerializationError {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let detail = match self {
      Self::WontImplement => "unsupported feature",
      Self::NotYetImplemented => "not yet implemented",
      Self::SerializeBufferFull => "serialize buffer is full",
      Self::SerializeSeqLengthUnknown => "sequence length must be known",
      Self::DeserializeUnexpectedEnd => "unexpected end of buffer",
      Self::DeserializeBadVarint => "bad varint",
      Self::DeserializeBadBool => "bad bool discriminant",
      Self::DeserializeBadChar => "invalid unicode char",
      Self::DeserializeBadUtf8 => "invalid utf-8",
      Self::DeserializeBadOption => "bad option discriminant",
      Self::DeserializeBadEnum => "bad enum discriminant",
      Self::DeserializeBadEncoding => "bad encoding",
      Self::DeserializeBadCrc => "bad CRC",
      Self::SerdeSerCustom => "serde serialization error",
      Self::SerdeDeCustom => "serde deserialization error",
      Self::CollectStrError => "collect_str error",
      Self::Other(msg) => msg.as_str(),
    };
    write!(f, "failed to serialize or deserialize process state: {detail}")
  }
}

impl Error for SerializationError {}

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
  pub(crate) fn from_serialization(err: SerializationError) -> Self {
    Self { inner: anyhow::Error::new(err), _marker: PhantomData }
  }

  #[inline(always)]
  pub(crate) fn invalid_step_index(idx: StepIndex) -> Self {
    Self { inner: anyhow::Error::new(InvalidStepIndexError(idx)), _marker: PhantomData }
  }

  #[inline(always)]
  pub(crate) fn from_concrete<E: StepError>(err: E) -> Self {
    Self { inner: anyhow::Error::new(err), _marker: PhantomData }
  }

  #[inline(always)]
  pub(crate) fn from_step_error<E: ErrorCoproduct>(err: E) -> Self {
    err.into_process_error()
  }

  #[inline(always)]
  pub(crate) fn widen<NextErrors: ErrorCoproduct>(self) -> ProcessError<NextErrors> {
    ProcessError { inner: self.inner, _marker: PhantomData }
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

impl ProcessError<crate::CNil> {
  /// Proves at compile time that all error variants have been exhaustively handled.
  ///
  /// Because `ProcessError<CNil>` cannot be constructed, reaching this method means every
  /// possible error in the process's [`ErrorCoproduct`] has been peeled off. If a new error
  /// is added to or removed from the process, the compiler will reject the caller's code.
  pub fn absurd(self) -> ! {
    unreachable!("ProcessError<CNil> is uninhabited: {}", self.inner)
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

impl<T> IntoProcessError<T> for Result<T, SerializationError> {
  #[inline(always)]
  fn into_process_err<Next: ErrorCoproduct>(self) -> Result<T, ProcessError<Next>> {
    self.map_err(ProcessError::from_serialization)
  }
}

impl<T> IntoProcessError<T> for Result<T, postcard::Error> {
  #[inline(always)]
  fn into_process_err<Next: ErrorCoproduct>(self) -> Result<T, ProcessError<Next>> {
    self.map_err(|err| ProcessError::from_serialization(SerializationError::from_postcard(err)))
  }
}

#[cfg(test)]
mod tests {
  use super::SerializationError;

  #[test]
  fn test_serialization_error_from_postcard_and_display() {
    let postcard_errors = [
      postcard::Error::WontImplement,
      postcard::Error::NotYetImplemented,
      postcard::Error::SerializeBufferFull,
      postcard::Error::SerializeSeqLengthUnknown,
      postcard::Error::DeserializeUnexpectedEnd,
      postcard::Error::DeserializeBadVarint,
      postcard::Error::DeserializeBadBool,
      postcard::Error::DeserializeBadChar,
      postcard::Error::DeserializeBadUtf8,
      postcard::Error::DeserializeBadOption,
      postcard::Error::DeserializeBadEnum,
      postcard::Error::DeserializeBadEncoding,
      postcard::Error::DeserializeBadCrc,
      postcard::Error::SerdeSerCustom,
      postcard::Error::SerdeDeCustom,
      postcard::Error::CollectStrError,
    ];
    for err in postcard_errors {
      let ser_err = SerializationError::from_postcard(err);
      assert!(ser_err.to_string().starts_with("failed to serialize or deserialize process state: "));
    }
    let other = SerializationError::Other("custom".to_string());
    assert_eq!(other.to_string(), "failed to serialize or deserialize process state: custom");
  }
}
