use frunk_core::hlist::*;
use frunk_core::traits::*;

/// Trait to compute intersection of two HLists
pub trait Intersect<Rhs> {
  type Output;
  fn intersect(self, rhs: Rhs) -> Self::Output;
}

/// Base case: empty LHS results in empty output
impl<Rhs> Intersect<Rhs> for HNil {
  type Output = HNil;
  fn intersect(self, _: Rhs) -> Self::Output {
    HNil
  }
}

/// Recursive case: if Head in RHS, keep it; else skip it
impl<Head, TailL, Rhs, TailOut> Intersect<Rhs> for HCons<Head, TailL>
where
  Rhs: Plucker<Head>,
  TailL: Intersect<Rhs, Output = TailOut>,
{
  type Output = HCons<Head, TailOut>;
  fn intersect(self, rhs: Rhs) -> Self::Output {
    let (_, rhs_without_head) = rhs.pluck();
    HCons {
      head: self.head,
      tail: self.tail.intersect(rhs_without_head),
    }
  }
}

// When Head not in Rhs, skip it
impl<Head, TailL, Rhs, TailOut> Intersect<Rhs> for HCons<Head, TailL>
where
  Rhs: NotContains<Head>,
  TailL: Intersect<Rhs, Output = TailOut>,
{
  type Output = TailOut;
  fn intersect(self, rhs: Rhs) -> Self::Output {
    self.tail.intersect(rhs)
  }
}

/// Trait that proves type is NOT contained in HList
pub trait NotContains<T> {}
impl<T> NotContains<T> for HNil {}

impl<Head, Tail, T> NotContains<T> for HCons<Head, Tail>
where
  Head: NotSame<T>,
  Tail: NotContains<T>,
{
}

/// Helper trait to prove two types are NOT equal
pub trait NotSame<T> {}
impl<T, U> NotSame<U> for T
where
  T: std::any::Any,
  U: std::any::Any,
{
}

#[cfg(test)]
mod tests {
  use frunk_core::hlist;

  #[test]
  fn test_add() {
    let a = hlist![1u8, "hello", true];
    let b = hlist![false, 1u8, 3.14f32];

    let intersection = a.intersect(b);

    assert_eq!(intersection, hlist![1u8, true]);
  }
}
