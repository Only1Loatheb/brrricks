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

////////// PrependIf //////////

trait PrependIf {
  type Output;
}

impl<Head, Tail> PrependIf for (B1, Head, Tail) {
  type Output = HCons<Head, Tail>;
}

impl<Head, Tail> PrependIf for (B0, Head, Tail) {
  type Output = Tail;
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

impl<Head: ParamValue, Tail, RHS, TailFilterOutput> Intersection<RHS> for HCons<Head, Tail>
where
  Tail: Intersection<RHS, Output = TailFilterOutput>,
  RHS: Contains<Head>,
  (<RHS as Contains<Head>>::Output, Head, TailFilterOutput): PrependIf,
{
  type Output = <(
    <RHS as Contains<Head>>::Output,
    Head,
    <Tail as Intersection<RHS>>::Output,
  ) as PrependIf>::Output;

  #[inline(always)]
  fn intersect(self, rhs: RHS) -> Self::Output {
    todo!()
  }
}
