use crate::param_list::{ParamList, ParamValue};
use frunk_core::hlist::{HCons, HNil};

/// Using ParamList instead of HList simplifies where clauses
/// Like `Add` and `extend` in [frunk_core::hlist], but with appropriate name
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

impl<Head: ParamValue, Tail: Concat<RHS> + ParamList, RHS: ParamList> Concat<RHS> for HCons<Head, Tail> {
  type Concatenated = HCons<Head, <Tail as Concat<RHS>>::Concatenated>;

  #[inline(always)]
  fn concat(self, rhs: RHS) -> Self::Concatenated {
    HCons {
      head: self.head,
      tail: self.tail.concat(rhs),
    }
  }
}
