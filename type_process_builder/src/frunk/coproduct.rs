use crate::frunk::plucker::{Here, There};

/// Enum type representing a Coproduct (ad-hoc sum type).
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub enum Coproduct<H, T> {
  Inl(H),
  Inr(T),
}

/// Empty enum for terminating a Coproduct type signature.
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub enum CNil {}

/// Trait for injecting a value into a Coproduct by type.
pub trait CoprodInjector<InjectType, Index> {
  fn inject(to_insert: InjectType) -> Self;
}

impl<I, Tail> CoprodInjector<I, Here> for Coproduct<I, Tail> {
  #[inline]
  fn inject(to_insert: I) -> Self {
    Coproduct::Inl(to_insert)
  }
}

impl<Head, I, Tail, TailIndex> CoprodInjector<I, There<TailIndex>> for Coproduct<Head, Tail>
where
  Tail: CoprodInjector<I, TailIndex>,
{
  #[inline]
  fn inject(to_insert: I) -> Self {
    let tail_inserted = <Tail as CoprodInjector<I, TailIndex>>::inject(to_insert);
    Coproduct::Inr(tail_inserted)
  }
}

impl<Head, Tail> Coproduct<Head, Tail> {
  #[inline]
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
