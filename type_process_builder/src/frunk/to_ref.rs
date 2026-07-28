use crate::frunk::hlist::{HCons, HNil};

/// Trait to create reference representations of `HLists`.
pub trait ToRef<'a> {
  type Ref;

  fn to_ref(&'a self) -> Self::Ref;
}

impl<'a> ToRef<'a> for HNil {
  type Ref = HNil;

  #[inline(always)]
  fn to_ref(&'a self) -> Self::Ref {
    HNil
  }
}

impl<'a, H: 'a, T> ToRef<'a> for HCons<H, T>
where
  T: ToRef<'a>,
{
  type Ref = HCons<&'a H, <T as ToRef<'a>>::Ref>;

  #[inline(always)]
  fn to_ref(&'a self) -> Self::Ref {
    HCons { head: &self.head, tail: self.tail.to_ref() }
  }
}
