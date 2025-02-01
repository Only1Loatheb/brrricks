use crate::step::param_list::ParamList;
use crate::step::ParamValue;
use frunk_core::hlist::{HCons, HNil};

// todo fail if the same types are in both lists
// Using ParamList instead of HList simplifies where clauses
pub trait Concat<RHS: ParamList>: ParamList {
  type Concatenated: ParamList;

  fn concat(self, rhs: RHS) -> Self::Concatenated;
}

// Base case: Concatenating HNil with another HList results in the other HList
impl<RHS: ParamList> Concat<RHS> for HNil {
  type Concatenated = RHS;

  fn concat(self, rhs: RHS) -> Self::Concatenated {
    rhs
  }
}

// Recursive case: Concatenate HCons
impl<Head: ParamValue, Tail: Concat<RHS> + ParamList, RHS: ParamList> Concat<RHS> for HCons<Head, Tail> {
  type Concatenated = HCons<Head, <Tail as Concat<RHS>>::Concatenated>;

  fn concat(self, rhs: RHS) -> Self::Concatenated {
    HCons {
      head: self.head,
      tail: self.tail.concat(rhs),
    }
  }
}
