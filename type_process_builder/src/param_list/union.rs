use crate::{
  builder::{ParamList, concat::Concat, contains::Contains, filter::Filter},
  param_list::ParamValue,
};
use frunk_core::hlist::{HCons, HNil};

////////// Subtraction (LHS - RHS) //////////

pub trait Subtract<RHS> {
  type Remainder;
  fn subtract(self) -> Self::Remainder;
}

impl<RHS> Subtract<RHS> for HNil {
  type Remainder = HNil;

  #[inline(always)]
  fn subtract(self) -> Self::Remainder {
    HNil
  }
}

impl<Head: ParamValue, Tail: Subtract<RHS>, RHS: Contains<Head>> Subtract<RHS> for HCons<Head, Tail>
where
  <RHS as Contains<Head>>::IsContained: std::ops::Not,
  <<RHS as Contains<Head>>::IsContained as std::ops::Not>::Output: Filter<Head, <Tail as Subtract<RHS>>::Remainder>,
{
  type Remainder = <<<RHS as Contains<Head>>::IsContained as std::ops::Not>::Output as Filter<
    Head,
    <Tail as Subtract<RHS>>::Remainder,
  >>::Filtered;

  #[inline(always)]
  fn subtract(self) -> Self::Remainder {
    let remainder_tail = self.tail.subtract();
    <<<RHS as Contains<Head>>::IsContained as std::ops::Not>::Output as Filter<
      Head,
      <Tail as Subtract<RHS>>::Remainder,
    >>::filter(self.head, remainder_tail)
  }
}

////////// Union //////////

pub trait Union<RHS> {
  type Output;
  fn union(self, rhs: RHS) -> Self::Output;
}

// To implement Union, we take all of LHS and append (RHS - LHS)
// This ensures that if an element is in both, we only keep the one from LHS.
impl<LHS, RHS: ParamList> Union<RHS> for LHS
where
  RHS: Subtract<LHS>,
  <RHS as Subtract<LHS>>::Remainder: ParamList,
  LHS: Concat<<RHS as Subtract<LHS>>::Remainder>,
{
  type Output = <LHS as Concat<<RHS as Subtract<LHS>>::Remainder>>::Concatenated;

  #[inline(always)]
  fn union(self, rhs: RHS) -> Self::Output {
    let unique_from_rhs = rhs.subtract();
    self.concat(unique_from_rhs)
  }
}
