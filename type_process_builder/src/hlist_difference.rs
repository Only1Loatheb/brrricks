use frunk_core::hlist::{HCons, HNil, Plucker};
use frunk_core::indices::{Here, There};

/// better test this bad boy
pub trait Minus<Value, Index> {
  /// What is left after you pluck the target from the Self
  type Remainder;

  /// Remove an element by type from an HList.
  ///
  /// Please see the [inherent method] for more information.
  ///
  /// The only difference between that inherent method and this
  /// trait method is the location of the type parameters.
  /// (here, they are on the trait rather than the method)
  ///
  /// [inherent method]: struct.HCons.html#method.pluck
  fn minus(self) -> Self::Remainder;
}

/// Implementation when the minus target is in head
impl<Value, Index> Minus<Value, Index> for HNil {
  type Remainder = HNil;

  fn minus(self) -> Self::Remainder {
    HNil
  }
}

/// Implementation when the minus target is head
impl<Head, Tail> Minus<Head, Here> for HCons<Head, Tail> {
  type Remainder = Tail;

  fn minus(self) -> Self::Remainder {
    self.tail
  }
}

/// Implementation when the minus target is not head
impl<Value, Head, Tail, TailIndex> Minus<Value, There<TailIndex>> for HCons<Head, Tail> {
  type Remainder = HCons<Head, Tail>;

  fn minus(self) -> Self::Remainder {
    self
  }
}

#[cfg(test)]
mod tests {
  use frunk_core::hlist::h_cons;
  use super::*;

  #[test]
  fn test_hcons() {
    let hlist1 = h_cons(1, HNil);
    let h = <HCons<i32, HNil> as Minus<i32, _>>::minus(hlist1);
    assert_eq!(h, HNil);

    let hlist2 = h_cons("hello", h_cons(1, HNil));
    let (h2, tail2) = hlist2.pop();
    let (h1, _) = tail2.pop();
    assert_eq!(h2, "hello");
    assert_eq!(h1, 1);
  }
}

// /// Implementation when the pluck target is in the tail
// impl<Head, Tail, FromTail, TailIndex> Minus<FromTail, There<TailIndex>> for HCons<Head, Tail>
// where
//   Tail: Minus<FromTail, TailIndex>,
// {
//   type Remainder = HCons<Head, <Tail as Minus<FromTail, TailIndex>>::Remainder>;
// 
//   fn pluck(self) -> (FromTail, Self::Remainder) {
//     let (target, tail_remainder): (
//       FromTail,
//       <Tail as Minus<FromTail, TailIndex>>::Remainder,
//     ) = <Tail as Minus<FromTail, TailIndex>>::pluck(self.tail);
//     (
//       target,
//       HCons {
//         head: self.head,
//         tail: tail_remainder,
//       },
//     )
//   }
// }

// /// Trait for computing the difference of two HLists
// pub trait Minus<Other> {
//   type Output;
//   fn minus(self) -> Self::Output;
// }
//
// /// Base case: Subtracting anything from an empty list yields an empty list
// impl<Other> Minus<Other> for HNil {
//   type Output = HNil;
//   fn minus(self) -> Self::Output {
//     HNil
//   }
// }
//
// /// Recursive case: Remove elements in `Other` from `Self`
// impl<SourceHead, SourceTail: Minus<HCons<OtherHead, OtherTail>>, OtherHead, OtherTail>
//   Minus<HCons<OtherHead, OtherTail>> for HCons<SourceHead, SourceTail>
// where
//   HCons<SourceHead, SourceTail>: Plucker<SourceHead, Here, Remainder = SourceTail>,
// {
//   type Output = <SourceTail as Minus<Other>>::Output;
//
//   fn minus(self) -> Self::Output {
//     let (_, remainder) = self.pluck();
//     remainder.minus()
//   }
// }
//
// // /// Case when the head element is not in `Other`, retain it
// // impl<Head, Tail, Other, FilteredTail> Minus<Other> for HCons<Head, Tail>
// // where
// //     Tail: Minus<Other, Output = FilteredTail>,
// //     HCons<Head, FilteredTail>: Sized,
// // {
// //     type Output = HCons<Head, FilteredTail>;
// //
// //     fn minus(self, other: Other) -> Self::Output {
// //         HCons {
// //             head: self.head,
// //             tail: self.tail.minus(other),
// //         }
// //     }
// // }


//

//   use frunk_core::hlist::{HCons, HNil, Plucker};
// use frunk_core::indices::Here;
// 
// /// Trait for computing the difference of two HLists
// pub trait Minus<Other> {
//     type Output;
//     fn minus(self, other: Other) -> Self::Output;
// }
// 
// /// Base case: Subtracting anything from an empty list yields an empty list
// impl<Other> Minus<Other> for HNil {
//     type Output = HNil;
//     fn minus(self, _: Other) -> Self::Output {
//         HNil
//     }
// }
// 
// /// Recursive case: Remove elements in `Other` from `Self`
// impl<Head, Tail, Other, Remainder> Minus<Other> for HCons<Head, Tail>
// where
//     HCons<Head, Tail>: Plucker<Head, Here, Remainder = Tail>,
//     Remainder: Minus<Other>,
// {
//     type Output = <Remainder as Minus<Other>>::Output;
//     
//     fn minus(self, other: Other) -> Self::Output {
//         let (_, remainder) = self.pluck();
//         remainder.minus(other)
//     }
// }
// 
// /// Case when the head element is not in `Other`, retain it
// impl<Head, Tail, Other, FilteredTail> Minus<Other> for HCons<Head, Tail>
// where
//     Tail: Minus<Other, Output = FilteredTail>,
//     HCons<Head, FilteredTail>: Sized,
// {
//     type Output = HCons<Head, FilteredTail>;
//     
//     fn minus(self, other: Other) -> Self::Output {
//         HCons {
//             head: self.head,
//             tail: self.tail.minus(other),
//         }
//     }
// }
