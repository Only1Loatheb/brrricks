#![feature(generic_const_exprs)]
use frunk_core::hlist::{HCons, HList, HNil};

/// Trait to compute the intersection of two HLists
pub trait Intersect<RHS>: HList {
  type Output: HList;

  fn intersect(self, rhs: RHS) -> Self::Output;
}

/// Trait to check if an HList contains a specific type
pub trait Contains<T>: HList {
  const VALUE: bool;
}

impl<T> Contains<T> for HNil {
  const VALUE: bool = false;
}

impl<Head, Tail, T> Contains<T> for HCons<Head, Tail>
where
  Tail: Contains<T>,
{
  const VALUE: bool = std::any::TypeId::of::<Head>() == std::any::TypeId::of::<T>() || Tail::VALUE;
}

// Base case: Intersection with an empty HList is empty
impl<RHS: HList> Intersect<RHS> for HNil {
  type Output = HNil;

  fn intersect(self, _: RHS) -> Self::Output {
    HNil
  }
}

// Recursive case: Check if the head exists in RHS
impl<Head, Tail: Intersect<RHS>, RHS: Contains<Head> + HList> Intersect<RHS> for HCons<Head, Tail>
where
  Assert<RHS::VALUE>: IsTrue,
{
  type Output = HCons<Head, Tail::Output>;

  fn intersect(self, rhs: RHS) -> Self::Output {
    HCons {
      head: self.head,
      tail: self.tail.intersect(rhs),
    }
  }
}

impl<Head, Tail: Intersect<RHS>, RHS: Contains<Head> + HList> Intersect<RHS> for HCons<Head, Tail>
where
  Assert<RHS::VALUE>: IsFalse,
{
  type Output = Tail::Output;

  fn intersect(self, rhs: RHS) -> Self::Output {
    self.tail.intersect(rhs)
  }
}

pub enum Assert<const CHECK: bool> {}

pub trait IsTrue {}
impl IsTrue for Assert<true> {}

pub trait IsFalse {}
impl IsFalse for Assert<false> {}
