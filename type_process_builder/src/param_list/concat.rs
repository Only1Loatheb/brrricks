use crate::builder::PreventDuplicateParamUidInParamList;
use crate::frunk::hlist::{HCons, HNil};
use crate::param_list::contains::Contains;
use crate::param_list::{ParamList, ParamValue};
use typenum::B0;

#[diagnostic::on_unimplemented(
  message = "cannot overwrite parameter `{Param}`: it was already produced by a previous step in this process flow",
  note = "The step being added produces parameter `{Param}`, but `{Param}` is already present in the preceding process parameters. Process steps cannot produce or overwrite existing parameters."
)]
pub trait PreventOverwritingParamInProcess<Param> {}

impl<Param> PreventOverwritingParamInProcess<Param> for B0 {}

/// Using `ParamList` instead of `HList` simplifies where clauses.
/// Replaces `Add` and `extend` from `frunk::hlist`.
pub trait Concat<RHS: ParamList>: ParamList {
  type Concatenated: ParamList;

  fn concat(self, rhs: RHS) -> Self::Concatenated;
}

impl<RHS: ParamList> Concat<RHS> for HNil {
  type Concatenated = RHS;

  #[inline(always)]
  fn concat(self, rhs: RHS) -> Self::Concatenated {
    rhs
  }
}

impl<Head: ParamValue, Tail: Concat<RHS> + ParamList + Contains<Head>, RHS: ParamList> Concat<RHS> for HCons<Head, Tail>
where
  <Tail as Concat<RHS>>::Concatenated: Contains<Head>,
  <<Tail as Concat<RHS>>::Concatenated as Contains<Head>>::IsContained: PreventOverwritingParamInProcess<Head>,
  <<Tail as Concat<RHS>>::Concatenated as Contains<Head>>::IsContained: PreventDuplicateParamUidInParamList<Head>,
  <Tail as Contains<Head>>::IsContained: PreventDuplicateParamUidInParamList<Head>,
{
  type Concatenated = HCons<Head, <Tail as Concat<RHS>>::Concatenated>;

  #[inline(always)]
  fn concat(self, rhs: RHS) -> Self::Concatenated {
    HCons { head: self.head, tail: self.tail.concat(rhs) }
  }
}
