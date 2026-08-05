use crate::frunk::hlist::{HCons, HNil};
use crate::param_list::ParamValue;
use crate::param_list::contains::Contains;
use typenum::{B0, B1};

////////// ThenKeep //////////

/// Like [`crate::param_list::union::IfNotKeep`], but the logic is negated and exposes runtime method for
/// constructing the value
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
