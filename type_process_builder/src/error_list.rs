use crate::error_list::contains::CoprodContains;
use crate::{CNil, Coproduct};
use std::error::Error;
use typenum::{B0, Unsigned};

pub mod contains;
pub mod union;

/// Trait implemented by user-defined step error types so the builder can deduplicate errors
/// across a process using compile-time UIDs.
///
/// Use [`crate::impl_step_error!`] to implement this trait and check for duplicate UIDs at compile time.
/// `U0` and `U1` are reserved for built-in [`crate::builder::SerializationError`] and
/// [`crate::builder::InvalidStepIndexError`]; [`crate::impl_step_error!`] offsets user UIDs by `+2` automatically.
pub trait StepError: Error + Send + Sync + 'static {
  type UID: Unsigned;
}

/// Macro to implement [`StepError`] for one or more error types.
///
/// Ensures that no duplicate [`Unsigned`] UIDs are passed within a single invocation at compile time.
///
/// # Usage:
/// ```
/// use type_process_builder::impl_step_error;
/// use typenum::{U0, U1};
///
/// #[derive(Debug, thiserror::Error)]
/// #[error("db error")]
/// struct DbError;
///
/// #[derive(Debug, thiserror::Error)]
/// #[error("network error")]
/// struct NetworkError;
///
/// impl_step_error! {
///   DbError => U0,
///   NetworkError => U1,
/// }
/// ```
///
/// Passing duplicate typenum UIDs within the same invocation results in a compile-time error:
/// ```compile_fail,E0119
/// use type_process_builder::impl_step_error;
/// use typenum::U0;
///
/// #[derive(Debug, thiserror::Error)]
/// #[error("err a")]
/// struct ErrA;
///
/// #[derive(Debug, thiserror::Error)]
/// #[error("err b")]
/// struct ErrB;
///
/// impl_step_error! {
///   ErrA => U0,
///   ErrB => U0,
/// }
/// ```
#[macro_export]
macro_rules! impl_step_error {
  ($type:ty, $uid:ty) => {
    impl $crate::error_list::StepError for $type {
      type UID = <$uid as ::core::ops::Add<$crate::typenum::U2>>::Output;
    }
  };
  ($($type:ty => $uid:ty),* $(,)?) => {
    $(
      $crate::impl_step_error!($type, $uid);
    )*
    const _: () = {
      #[allow(dead_code)]
      trait DuplicateErrorUidInMacroInvocation {}
      $(
        impl DuplicateErrorUidInMacroInvocation for $uid {}
      )*
    };
  };
}

mod sealed {
  use super::ErrorCoproduct;

  pub trait Sealed {}
  impl Sealed for typenum::B0 {}

  pub trait ConvertStepError {
    fn into_process_error<Target: ErrorCoproduct>(self) -> crate::builder::ProcessError<Target>;
  }
}

#[diagnostic::on_unimplemented(
  message = "cannot include error: duplicate error UID for `{Err}` found in ErrorCoproduct",
  note = "Error `{Err}` appears multiple times in the error coproduct, or multiple errors share the same UID"
)]
pub trait PreventDuplicateErrorUidInCoproduct<Err>: sealed::Sealed {}

impl<Err> PreventDuplicateErrorUidInCoproduct<Err> for B0 {}

/// Sealed marker trait for a deduplicated [`Coproduct`] of [`StepError`]s (terminated by [`CNil`]).
pub trait ErrorCoproduct: crate::sealed::Sealed + sealed::ConvertStepError + Send + Sync + 'static {}

impl sealed::ConvertStepError for CNil {
  #[inline(always)]
  fn into_process_error<Target: ErrorCoproduct>(self) -> crate::builder::ProcessError<Target> {
    match self {}
  }
}

impl ErrorCoproduct for CNil {}

impl<Head: StepError, Tail: ErrorCoproduct + CoprodContains<Head>> sealed::ConvertStepError for Coproduct<Head, Tail>
where
  <Tail as CoprodContains<Head>>::IsContained: PreventDuplicateErrorUidInCoproduct<Head>,
{
  #[inline(always)]
  fn into_process_error<Target: ErrorCoproduct>(self) -> crate::builder::ProcessError<Target> {
    match self {
      Coproduct::Inl(h) => crate::builder::ProcessError::from_concrete(h),
      Coproduct::Inr(t) => t.into_process_error(),
    }
  }
}

impl<Head: StepError, Tail: ErrorCoproduct + CoprodContains<Head>> ErrorCoproduct for Coproduct<Head, Tail> where
  <Tail as CoprodContains<Head>>::IsContained: PreventDuplicateErrorUidInCoproduct<Head>
{
}

/// Marker trait proving that [`ErrorCoproduct`] `Self` contains step error `E`.
pub trait ContainsError<E: StepError>: ErrorCoproduct {}

impl<E: StepError, C: ErrorCoproduct + CoprodContains<E, IsContained = typenum::B1>> ContainsError<E> for C {}
