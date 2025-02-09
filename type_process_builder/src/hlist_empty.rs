use frunk_core::hlist::HNil;
use crate::hlist_empty::sealed::Sealed;

// https://github.com/paholg/typenum/commit/500ef759e1ddaa8ad2b37862f958903d785582c9
mod sealed {
  use frunk_core::hlist::HNil;

  pub trait Sealed {}
  impl Sealed for HNil {}
}

pub trait Empty: Sealed {}

impl Empty for HNil {}
