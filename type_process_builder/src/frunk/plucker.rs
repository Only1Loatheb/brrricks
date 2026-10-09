use crate::frunk::hlist::HCons;
use std::marker::PhantomData;

pub struct Here;

pub struct There<T>(PhantomData<T>);

/// Trait for plucking/extracting an element by type from an `HList`.
#[diagnostic::on_unimplemented(
  message = "cannot consume parameter `{Target}`: it has not been produced by any previous step in this process flow",
  note = "The step being added requires parameter `{Target}`, but it has not been produced yet by any preceding step."
)]
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
    (target, HCons { tail: tail_remainder, head: self.head })
  }
}
