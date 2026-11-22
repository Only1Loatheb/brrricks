use crate::frunk::plucker::{Here, There};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// Enum type representing a Coproduct (ad-hoc sum type).
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub enum Coproduct<H, T> {
  Inl(H),
  Inr(T),
}

/// Empty enum for terminating a Coproduct type signature.
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub enum CNil {}

impl crate::sealed::Sealed for CNil {}

impl Display for CNil {
  fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
    match *self {}
  }
}

impl Error for CNil {}

impl<H: Display, T: Display> Display for Coproduct<H, T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::Inl(h) => h.fmt(f),
      Self::Inr(t) => t.fmt(f),
    }
  }
}

impl<H: Error + 'static, T: Error + 'static> Error for Coproduct<H, T> {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      Self::Inl(h) => h.source(),
      Self::Inr(t) => t.source(),
    }
  }
}

/// Trait for injecting a value into a Coproduct by type.
#[diagnostic::on_unimplemented(
  message = "cannot inject `{InjectType}` into error coproduct `{Self}`",
  note = "If merging step errors across steps, ensure `{InjectType}` does not share the same `StepError::UID` with a different error type"
)]
pub trait CoprodInjector<InjectType, Index>: crate::sealed::Sealed {
  fn inject(to_insert: InjectType) -> Self;
}

impl<H, T> crate::sealed::Sealed for Coproduct<H, T> {}

impl<I, Tail> CoprodInjector<I, Here> for Coproduct<I, Tail> {
  #[inline(always)]
  fn inject(to_insert: I) -> Self {
    Coproduct::Inl(to_insert)
  }
}

impl<Head, I, Tail, TailIndex> CoprodInjector<I, There<TailIndex>> for Coproduct<Head, Tail>
where
  Tail: CoprodInjector<I, TailIndex>,
{
  #[inline(always)]
  fn inject(to_insert: I) -> Self {
    let tail_inserted = <Tail as CoprodInjector<I, TailIndex>>::inject(to_insert);
    Coproduct::Inr(tail_inserted)
  }
}

impl<Head, Tail> Coproduct<Head, Tail> {
  #[inline(always)]
  pub fn inject<T, Index>(to_insert: T) -> Self
  where
    Self: CoprodInjector<T, Index>,
  {
    CoprodInjector::inject(to_insert)
  }
}

/// Type macro for creating `Coproduct` type signatures.
#[macro_export]
macro_rules! Coprod {
  () => { $crate::frunk::coproduct::CNil };
  (...$Rest:ty) => { $Rest };
  ($A:ty) => { $crate::Coprod![$A,] };
  ($A:ty, $($tok:tt)*) => {
    $crate::frunk::coproduct::Coproduct<$A, $crate::Coprod![$($tok)*]>
  };
}
