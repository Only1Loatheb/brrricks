/// Typeclass for `HList` (Heterogeneous List) behaviour.
pub trait HList: Sized {
  const LEN: usize;
}

/// Represents an empty `HList`.
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash, Default, serde::Serialize, serde::Deserialize)]
pub struct HNil;

impl HList for HNil {
  const LEN: usize = 0;
}

/// Represents a non-empty `HList`, holding a head element and a tail `HList`.
///
/// ### Field Declaration Order (`tail` before `head`)
/// [postcard](https://docs.rs/postcard) encodes a struct as the elements that comprise it, in their order of definition (top to bottom).
/// <https://postcard.jamesmunns.com/wire-format#28---struct>.
/// By declaring `tail` before `head` we ensure that [`HCons`] is serialized like this `[Bytes of tail][Bytes of head]`
/// in postcard's positional byte stream.
///
/// [`crate::builder`] methods put parameters produced by earlier [`crate::step`] in `tail`.
/// Order of serialization and the method of building [`crate::param_list::ParamList`] ensures that earlier step's
/// parameter list is always a strict byte-level prefix of a downstream step's parameter list when we take into
/// account [`crate::step::Form`] and [`crate::step::FormSplitter`] steps that the user interacted with.
/// During `Back` navigation, an earlier step deserializes its required prefix and leaves any unread
/// trailing bytes in the [`crate::param_list::SessionContext`], enabling zero-overhead
/// [`crate::builder::RunOutcome::Back`] navigation without storing UID for every [`crate::param_list::ParamValue`] in
/// `SessionContext` or storing `SessionContext` for every form in the session.
///
/// We don't use this property now because we delete items form `ParamList` when merging [`crate::param_list::intersect`]
#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash, Default, serde::Serialize, serde::Deserialize)]
pub struct HCons<H, T> {
  pub tail: T,
  pub head: H,
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
      tail: $crate::hlist![$($tok)*],
      head: $a,
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
      tail: $crate::hlist_pat![$($tok)*],
      head: $a,
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
