use frunk_core::hlist::*;

/// NotSameAs
pub trait NotSameAs<T> {}
impl<A, B> NotSameAs<B> for A {}

/// NotContains
pub trait NotContains<T> {}

// Empty list never contains anything => trivially NotContains
impl<T> NotContains<T> for HNil {}

impl<T, Head, Tail> NotContains<T> for HCons<Head, Tail>
where
  Tail: NotContains<T>,
  T: NotSameAs<Head>,
{
}

/// HListIntersect
pub trait HListIntersect<Other> {
  type Output;

  fn intersect(self, other: &Other) -> Self::Output;
}

impl<Other> HListIntersect<Other> for HNil {
  type Output = HNil;

  fn intersect(self, _other: &Other) -> Self::Output {
    HNil
  }
}

/// HListIntersect Contains
impl<Head, Tail, Other, Index> HListIntersect<Other> for HCons<Head, Tail>
where
  Tail: HListIntersect<Other>,
  Other: Plucker<Head, Index>,
  Head: Clone,
{
  type Output = HCons<Head, <Tail as HListIntersect<Other>>::Output>;

  fn intersect(self, other: &Other) -> Self::Output {
    let HCons { head, tail } = self;
    HCons {
      head: head.clone(),
      tail: tail.intersect(other),
    }
  }
}

/// HListIntersect NotContains
impl<Head, Tail, Other> HListIntersect<Other> for HCons<Head, Tail>
where
  Tail: HListIntersect<Other>,
  Other: NotContains<Head>,
{
  type Output = <Tail as HListIntersect<Other>>::Output;

  fn intersect(self, other: &Other) -> Self::Output {
    self.tail.intersect(other)
  }
}

#[cfg(test)]
mod tests {
  use crate::hlist_intersect::HListIntersect;
  use frunk_core::{hlist, HList};

  #[test]
  fn test_add() {
    type L1 = HList![u8, u16, u32];
    type L2 = HList![u16, bool, u8];

    let list1 = hlist![1u8, 2u16, 3u32];
    let list2 = hlist![4u16, true, 5u8];

    // Type-level intersection
    type Intersection = <L1 as HListIntersect<L2>>::Output;
    // Intersection == Hlist![u8, u16]

    // Value-level intersection
    let intersected: Intersection = list1.intersect(&list2);

    println!("{:?}", intersected); // prints (1, 2)
  }
}
