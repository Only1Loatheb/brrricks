/// Typeclass for `HList` (Heterogeneous List) behaviour.
pub trait HList: Sized {
  const LEN: usize;
}

/// Represents an empty `HList`.
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash, Default)]
pub struct HNil;

impl HList for HNil {
  const LEN: usize = 0;
}

/// Represents a non-empty `HList`, holding a head element and a tail `HList`.
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash, Default)]
pub struct HCons<H, T> {
  pub head: H,
  pub tail: T,
}

impl<H, T: HList> HList for HCons<H, T> {
  const LEN: usize = 1 + <T as HList>::LEN;
}

/// Construct an `HList` value from arguments.
#[macro_export]
macro_rules! hlist {
  () => { $crate::frunk::hlist::HNil };
  (...$rest:expr) => { $rest };
  ($a:expr) => { $crate::hlist![$a,] };
  ($a:expr, $($tok:tt)*) => {
    $crate::frunk::hlist::HCons {
      head: $a,
      tail: $crate::hlist![$($tok)*],
    }
  };
}

/// Pattern match on an `HList`.
#[macro_export]
macro_rules! hlist_pat {
  () => { $crate::frunk::hlist::HNil };
  (...) => { _ };
  (...$rest:pat) => { $rest };
  (_) => { $crate::hlist_pat![_,] };
  ($a:pat) => { $crate::hlist_pat![$a,] };
  (_, $($tok:tt)*) => {
    $crate::frunk::hlist::HCons {
      tail: $crate::hlist_pat![$($tok)*],
      ..
    }
  };
  ($a:pat, $($tok:tt)*) => {
    $crate::frunk::hlist::HCons {
      head: $a,
      tail: $crate::hlist_pat![$($tok)*],
    }
  };
}

/// Type macro for creating `HList` type signatures.
#[macro_export]
macro_rules! HList {
  () => { $crate::frunk::hlist::HNil };
  (...$Rest:ty) => { $Rest };
  ($A:ty) => { $crate::HList![$A,] };
  ($A:ty, $($tok:tt)*) => {
    $crate::frunk::hlist::HCons<$A, $crate::HList![$($tok)*]>
  };
}
