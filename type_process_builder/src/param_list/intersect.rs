use crate::{
  builder::{contains::Contains, filter::Filter},
  param_list::ParamValue,
};
use frunk_core::hlist::{HCons, HNil};

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
  <RHS as Contains<Head>>::IsContained: Filter<Head, <Tail as Intersect<RHS>>::Intersection>,
{
  type Intersection =
    <<RHS as Contains<Head>>::IsContained as Filter<Head, <Tail as Intersect<RHS>>::Intersection>>::Filtered;

  #[inline(always)]
  fn intersect(self) -> Self::Intersection {
    let intersected_tail = self.tail.intersect();
    <<RHS as Contains<Head>>::IsContained as Filter<Head, <Tail as Intersect<RHS>>::Intersection>>::filter(
      self.head,
      intersected_tail,
    )
  }
}
