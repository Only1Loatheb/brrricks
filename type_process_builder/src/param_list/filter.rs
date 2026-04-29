use frunk_core::hlist::HCons;
use typenum::{B0, B1};

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
