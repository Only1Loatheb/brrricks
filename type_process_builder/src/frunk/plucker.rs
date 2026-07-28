use crate::frunk::hlist::HCons;
use crate::frunk::to_ref::ToRef;
use std::marker::PhantomData;

pub struct Here;

pub struct There<T>(PhantomData<T>);

/// Trait for plucking/extracting an element by type from an `HList`.
pub trait Plucker<Target, Index> {
  type Remainder;

  fn pluck(self) -> (Target, Self::Remainder);
}

impl<Head, Tail> Plucker<Head, Here> for HCons<Head, Tail> {
  type Remainder = Tail;

  #[inline(always)]
  fn pluck(self) -> (Head, Self::Remainder) {
    (self.head, self.tail)
  }
}

impl<Head, Tail, FromTail, TailIndex> Plucker<FromTail, There<TailIndex>> for HCons<Head, Tail>
where
  Tail: Plucker<FromTail, TailIndex>,
{
  type Remainder = HCons<Head, <Tail as Plucker<FromTail, TailIndex>>::Remainder>;

  #[inline(always)]
  fn pluck(self) -> (FromTail, Self::Remainder) {
    let (target, tail_remainder) = <Tail as Plucker<FromTail, TailIndex>>::pluck(self.tail);
    (target, HCons { head: self.head, tail: tail_remainder })
  }
}

/// Implementation when target is reference and the pluck target is in head
impl<'a, Head, Tail: ToRef<'a>> Plucker<&'a Head, Here> for &'a HCons<Head, Tail> {
  type Remainder = <Tail as ToRef<'a>>::Ref;

  #[inline(always)]
  fn pluck(self) -> (&'a Head, Self::Remainder) {
    (&self.head, self.tail.to_ref())
  }
}

/// Implementation when target is reference and the pluck target is in the tail
impl<'a, Head, Tail, FromTail, TailIndex> Plucker<&'a FromTail, There<TailIndex>> for &'a HCons<Head, Tail>
where
  &'a Tail: Plucker<&'a FromTail, TailIndex>,
{
  type Remainder = HCons<&'a Head, <&'a Tail as Plucker<&'a FromTail, TailIndex>>::Remainder>;

  #[inline(always)]
  fn pluck(self) -> (&'a FromTail, Self::Remainder) {
    let (target, tail_remainder) = <&'a Tail as Plucker<&'a FromTail, TailIndex>>::pluck(&self.tail);
    (target, HCons { head: &self.head, tail: tail_remainder })
  }
}
