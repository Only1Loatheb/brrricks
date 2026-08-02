use crate::builder::contains::Contains;
use crate::builder::{ParamList, ParamValue};
use crate::{HCons, HNil};
use typenum::{B0, B1, Same};

////////// IfNotKeep //////////

/// Like [`crate::param_list::intersect::ThenKeep`], but the logic is negated and omits the method to construct the value
pub trait IfNotKeep<Head, Tail> {
  type Filtered;
}

impl<Head, Tail> IfNotKeep<Head, Tail> for B0 {
  type Filtered = HCons<Head, Tail>;
}

impl<Head, Tail> IfNotKeep<Head, Tail> for B1 {
  type Filtered = Tail;
}

////////// Union //////////

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
