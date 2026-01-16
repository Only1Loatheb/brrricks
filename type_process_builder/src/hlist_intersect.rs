use crate::param_list::ParamValue;
use frunk_core::hlist::{HCons, HNil};
use std::ops::BitOr;
use typenum::private::IsEqualPrivate;
use typenum::{Bit, Cmp, IsEqual, B0, B1};

////////// UIDEquals //////////

trait UIDEquals {
  type Output: Bit;
}

impl<Left: ParamValue, Right: ParamValue> UIDEquals for (Left, Right)
where
  Left::UID: Cmp<Right::UID>,
  Left::UID: IsEqualPrivate<Right::UID, <Left::UID as Cmp<Right::UID>>::Output>,
{
  type Output = <Left::UID as IsEqual<Right::UID>>::Output;
}

////////// Contains //////////

trait Contains<Needle: ParamValue> {
  type Output: Bit;
}

impl<Needle: ParamValue> Contains<Needle> for HNil {
  type Output = B0;
}

impl<Needle: ParamValue, Head: ParamValue, Tail: Contains<Needle>> Contains<Needle> for HCons<Head, Tail>
where
  Needle::UID: Cmp<Head::UID>,
  Needle::UID: IsEqualPrivate<Head::UID, <Needle::UID as Cmp<Head::UID>>::Output>,
  <Needle::UID as IsEqualPrivate<Head::UID, <Needle::UID as Cmp<Head::UID>>::Output>>::Output:
    BitOr<<Tail as Contains<Needle>>::Output>,
  <<Needle::UID as IsEqualPrivate<Head::UID, <Needle::UID as Cmp<Head::UID>>::Output>>::Output as BitOr<
    <Tail as Contains<Needle>>::Output,
  >>::Output: Bit,
{
  type Output = <<(Needle, Head) as UIDEquals>::Output as BitOr<<Tail as Contains<Needle>>::Output>>::Output;
}

////////// KeepIf //////////

trait KeepIf<Head, Tail> {
  type Output;
  fn kept(head: Head, tail: Tail) -> Self::Output;
}

impl<Head, Tail> KeepIf<Head, Tail> for B1 {
  type Output = HCons<Head, Tail>;

  #[inline(always)]
  fn kept(head: Head, tail: Tail) -> Self::Output {
    HCons { head, tail }
  }
}

impl<Head, Tail> KeepIf<Head, Tail> for B0 {
  type Output = Tail;

  #[inline(always)]
  fn kept(_head: Head, tail: Tail) -> Self::Output {
    tail
  }
}

////////// Intersection //////////

trait Intersection<RHS> {
  type Output;

  fn intersect(self, RHS: RHS) -> Self::Output;
}

impl<RHS> Intersection<RHS> for HNil {
  type Output = HNil;

  #[inline(always)]
  fn intersect(self, rhs: RHS) -> Self::Output {
    HNil
  }
}

impl<Head: ParamValue, Tail, RHS> Intersection<RHS> for HCons<Head, Tail>
where
  Tail: Intersection<RHS>,
  RHS: Contains<Head>,
  <RHS as Contains<Head>>::Output: KeepIf<Head, <Tail as Intersection<RHS>>::Output>,
{
  type Output = <<RHS as Contains<Head>>::Output as KeepIf<Head, <Tail as Intersection<RHS>>::Output>>::Output;

  #[inline(always)]
  fn intersect(self, rhs: RHS) -> Self::Output {
    todo!()
  }
}
