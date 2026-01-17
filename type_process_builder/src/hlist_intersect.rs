use crate::param_list::ParamValue;
use frunk_core::hlist::{HCons, HNil};
use std::ops::BitOr;
use typenum::{Bit, IsEqual, B0, B1};

////////// Contains //////////

pub trait Contains<Needle: ParamValue> {
  type IsContained: Bit;
}

impl<Needle: ParamValue> Contains<Needle> for HNil {
  type IsContained = B0;
}

impl<Needle: ParamValue, Head: ParamValue, Tail: Contains<Needle>> Contains<Needle> for HCons<Head, Tail>
where
  Needle::UID: IsEqual<Head::UID>,
  <Needle::UID as IsEqual<Head::UID>>::Output: BitOr<<Tail as Contains<Needle>>::IsContained>,
  <<Needle::UID as IsEqual<Head::UID>>::Output as BitOr<<Tail as Contains<Needle>>::IsContained>>::Output: Bit,
{
  type IsContained =
    <<Needle::UID as IsEqual<Head::UID>>::Output as BitOr<<Tail as Contains<Needle>>::IsContained>>::Output;
}

////////// Filter //////////

pub trait Filter<Head, Tail> {
  type Filtered;
  fn filter(head: Head, tail: Tail) -> Self::Filtered;
}

impl<Head, Tail> Filter<Head, Tail> for B1 {
  type Filtered = HCons<Head, Tail>;

  #[inline(always)]
  fn filter(head: Head, tail: Tail) -> Self::Filtered {
    HCons { head, tail }
  }
}

impl<Head, Tail> Filter<Head, Tail> for B0 {
  type Filtered = Tail;

  #[inline(always)]
  fn filter(_head: Head, tail: Tail) -> Self::Filtered {
    tail
  }
}

////////// Intersection //////////

pub trait Intersect<RHS> {
  type Intersection;

  fn intersect(self, rhs: RHS) -> Self::Intersection;
}

impl<RHS> Intersect<RHS> for HNil {
  type Intersection = HNil;

  #[inline(always)]
  fn intersect(self, rhs: RHS) -> Self::Intersection {
    HNil
  }
}

impl<Head: ParamValue, Tail: Intersect<RHS>, RHS: Contains<Head>> Intersect<RHS> for HCons<Head, Tail>
where
  <RHS as Contains<Head>>::IsContained: Filter<Head, <Tail as Intersect<RHS>>::Intersection>,
{
  type Intersection =
    <<RHS as Contains<Head>>::IsContained as Filter<Head, <Tail as Intersect<RHS>>::Intersection>>::Filtered;

  #[inline(always)]
  fn intersect(self, rhs: RHS) -> Self::Intersection {
    let intersected_tail = self.tail.intersect(rhs);
    <<RHS as Contains<Head>>::IsContained as Filter<Head, <Tail as Intersect<RHS>>::Intersection>>::filter(
      self.head,
      intersected_tail,
    )
  }
}
