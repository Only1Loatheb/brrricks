use crate::frunk::hlist::{HCons, HNil};

/// Trait to create reference representations of `HLists`.
pub trait ToRef<'a> {
  type Output;

  fn to_ref(&'a self) -> Self::Output;
}

impl<'a> ToRef<'a> for HNil {
  type Output = HNil;

  #[inline]
  fn to_ref(&'a self) -> Self::Output {
    HNil
  }
}

impl<'a, H: 'a, T> ToRef<'a> for HCons<H, T>
where
  T: ToRef<'a>,
{
  type Output = HCons<&'a H, <T as ToRef<'a>>::Output>;

  #[inline]
  fn to_ref(&'a self) -> Self::Output {
    HCons { head: &self.head, tail: self.tail.to_ref() }
  }
}
