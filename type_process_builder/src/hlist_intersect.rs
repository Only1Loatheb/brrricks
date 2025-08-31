// use frunk_core::hlist::*;
// use frunk_core::indices::{Here, There};
// use std::ops::Index;
//
// /// NotSameAs
// pub trait NotSameAs<T> {}
// impl<A, B> NotSameAs<B> for A {}
//
// /// NotContains
// pub trait NotContains<T> {}
//
// // Empty list never contains anything => trivially NotContains
// impl<T> NotContains<T> for HNil {}
//
// impl<T, Head, Tail> NotContains<T> for HCons<Head, Tail>
// where
//   Tail: NotContains<T>,
//   T: NotSameAs<Head>,
// {
// }
//
// pub trait Contains<Target, Index> {
//   /// What is left after you pluck the target from the Self
//   type Remainder;
//
//   /// Remove an element by type from an HList.
//   ///
//   /// Please see the [inherent method] for more information.
//   ///
//   /// The only difference between that inherent method and this
//   /// trait method is the location of the type parameters.
//   /// (here, they are on the trait rather than the method)
//   ///
//   /// [inherent method]: struct.HCons.html#method.pluck
//   fn pluck(self) -> (Target, Self::Remainder);
// }
//
// /// Implementation when the pluck target is in head
// impl<T, Tail> Contains<T, Here> for HCons<T, Tail> {
//   type Remainder = Tail;
//
//   fn pluck(self) -> (T, Self::Remainder) {
//     (self.head, self.tail)
//   }
// }
//
// /// Implementation when the pluck target is in the tail
// impl<Head, Tail, FromTail, TailIndex> Contains<FromTail, There<TailIndex>> for HCons<Head, Tail>
// where
//   Tail: Contains<FromTail, TailIndex>,
// {
//   type Remainder = HCons<Head, <Tail as Contains<FromTail, TailIndex>>::Remainder>;
//
//   fn pluck(self) -> (FromTail, Self::Remainder) {
//     let (target, tail_remainder): (FromTail, <Tail as Contains<FromTail, TailIndex>>::Remainder) =
//       <Tail as Contains<FromTail, TailIndex>>::pluck(self.tail);
//     (
//       target,
//       HCons {
//         head: self.head,
//         tail: tail_remainder,
//       },
//     )
//   }
// }
//
// /// HListIntersect
// pub trait HListIntersect<Other> {
//   type Output;
//
//   fn intersect(self, other: &Other) -> Self::Output;
// }
//
// impl<Other> HListIntersect<Other> for HNil {
//   type Output = HNil;
//
//   fn intersect(self, _other: &Other) -> Self::Output {
//     HNil
//   }
// }
//
// /// HListIntersect Contains
// impl<Head, Tail, OtherTail> HListIntersect<HCons<Head, OtherTail>> for HCons<Head, Tail>
// where
//   Tail: HListIntersect<HCons<Head, OtherTail>>,
//   Head: Clone,
// {
//   type Output = HCons<Head, <Tail as HListIntersect<HCons<Head, OtherTail>>>::Output>;
//
//   fn intersect(self, other: &HCons<Head, OtherTail>) -> Self::Output {
//     let HCons { head, tail } = self;
//     HCons {
//       head: head.clone(),
//       tail: tail.intersect(other),
//     }
//   }
// }
//
// impl<Head, Tail, Index, OtherHead, OtherTail: Contains<Head, There<Index>>> HListIntersect<HCons<OtherHead, OtherTail>>
//   for HCons<Head, Tail>
// where
//   Tail: HListIntersect<HCons<OtherHead, OtherTail>>,
//   Head: Clone,
// {
//   type Output = HCons<Head, <Tail as HListIntersect<HCons<Head, OtherTail>>>::Output>;
//
//   fn intersect(self, other: &HCons<Head, OtherTail>) -> Self::Output {
//     let HCons { head, tail } = self;
//     HCons {
//       head: head.clone(),
//       tail: tail.intersect(other),
//     }
//   }
// }
//
// // /// HListIntersect NotContains
// // impl<Head, Tail, Other> HListIntersect<Other> for HCons<Head, Tail>
// // where
// //   Tail: HListIntersect<Other>,
// //   Other: NotContains<Head>,
// // {
// //   type Output = <Tail as HListIntersect<Other>>::Output;
// //
// //   fn intersect(self, other: &Other) -> Self::Output {
// //     self.tail.intersect(other)
// //   }
// // }
//
// #[cfg(test)]
// mod tests {
//   use crate::hlist_intersect::HListIntersect;
//   use frunk_core::{hlist, HList};
//
//   #[test]
//   fn test_add() {
//     type L1 = HList![u8, u16, u32];
//     type L2 = HList![u16, bool, u8];
//
//     let list1 = hlist![1u8, 2u16, 3u32];
//     let list2 = hlist![4u16, true, 5u8];
//
//     // Type-level intersection
//     type Intersection = <L1 as HListIntersect<L2>>::Output;
//     // Intersection == Hlist![u8, u16]
//
//     // Value-level intersection
//     let intersected: Intersection = list1.intersect(&list2);
//
//     println!("{:?}", intersected); // prints (1, 2)
//   }
// }
