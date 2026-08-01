use crate::frunk::hlist::{HCons, HNil};
use crate::param_list::{ParamList, ParamValue};
use std::ops::BitOr;
use typenum::{B0, B1, Bit, IsEqual, Same};

////////// Contains ////////// fixme move to contains.rs

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

////////// ThenKeep //////////

pub trait ThenKeep<Head, Tail> {
  type Filtered;
  fn filter(head: Head, tail: Tail) -> Self::Filtered;
}

impl<Head, Tail> ThenKeep<Head, Tail> for B1 {
  type Filtered = HCons<Head, Tail>;

  #[inline(always)]
  fn filter(head: Head, tail: Tail) -> Self::Filtered {
    HCons { head, tail }
  }
}

impl<Head, Tail> ThenKeep<Head, Tail> for B0 {
  type Filtered = Tail;

  #[inline(always)]
  fn filter(_head: Head, tail: Tail) -> Self::Filtered {
    tail
  }
}

////////// Intersection ////////// fixme move to separate file with Union

pub trait IfNotKeep<Head, Tail> {
  type Filtered;
}

impl<Head, Tail> IfNotKeep<Head, Tail> for B0 {
  type Filtered = HCons<Head, Tail>;
}

impl<Head, Tail> IfNotKeep<Head, Tail> for B1 {
  type Filtered = Tail;
}

////////// Intersection //////////

pub trait Intersect<RHS> {
  type Intersection;

  fn intersect(self) -> Self::Intersection;
}

impl<RHS> Intersect<RHS> for HNil {
  type Intersection = HNil;

  #[inline(always)]
  fn intersect(self) -> Self::Intersection {
    HNil
  }
}

impl<Head: ParamValue, Tail: Intersect<RHS>, RHS: Contains<Head>> Intersect<RHS> for HCons<Head, Tail>
where
  <RHS as Contains<Head>>::IsContained: ThenKeep<Head, <Tail as Intersect<RHS>>::Intersection>,
{
  type Intersection =
    <<RHS as Contains<Head>>::IsContained as ThenKeep<Head, <Tail as Intersect<RHS>>::Intersection>>::Filtered;

  #[inline(always)]
  fn intersect(self) -> Self::Intersection {
    let intersected_tail = self.tail.intersect();
    <<RHS as Contains<Head>>::IsContained as ThenKeep<Head, <Tail as Intersect<RHS>>::Intersection>>::filter(
      self.head,
      intersected_tail,
    )
  }
}

////////// Union ////////// fixme move to separate file with Intersection

pub trait Union<RHS: ParamList>: ParamList {
  type Union: ParamList;
}

impl<RHS: ParamList> Union<RHS> for HNil {
  type Union = RHS;
}

impl<Head: ParamValue, Tail: Union<RHS> + ParamList + Contains<Head>, RHS: ParamList> Union<RHS> for HCons<Head, Tail>
where
  <Tail as Contains<Head>>::IsContained: Same<B0>,
  <Tail as Union<RHS>>::Union: Contains<Head>,
  <<Tail as Union<RHS>>::Union as Contains<Head>>::IsContained: IfNotKeep<Head, <Tail as Union<RHS>>::Union>,
  <<<Tail as Union<RHS>>::Union as Contains<Head>>::IsContained as IfNotKeep<Head, <Tail as Union<RHS>>::Union>>::Filtered: ParamList,
{
  type Union = <<<Tail as Union<RHS>>::Union as Contains<Head>>::IsContained as IfNotKeep<Head, <Tail as Union<RHS>>::Union>>::Filtered;
}
