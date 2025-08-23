// use frunk_core::hlist::*;
// use frunk_core::indices::{Here, There};
//
// pub trait Selector<S, I> {
//   fn get(&self) -> Option<&S>;
//
//   fn get_mut(&mut self) -> Option<&mut S>;
// }
//
// pub struct Missing {
//   _priv: (),
// }
//
// impl<T, Tail> Selector<T, Here> for HCons<T, Tail> {
//   fn get(&self) -> Option<&T> {
//     Some(&self.head)
//   }
//
//   fn get_mut(&mut self) -> Option<&mut T> {
//     Some(&mut self.head)
//   }
// }
//
// impl<T: std::any::Any, NotT: std::any::Any> Selector<T, Missing> for HCons<NotT, HNil> {
//   fn get(&self) -> Option<&T> {
//     None
//   }
//
//   fn get_mut(&mut self) -> Option<&mut T> {
//     None
//   }
// }
//
// impl<Head, Tail, FromTail, TailIndex> Selector<FromTail, There<TailIndex>> for HCons<Head, Tail>
// where
//   Tail: Selector<FromTail, TailIndex>,
// {
//   fn get(&self) -> Option<&FromTail> {
//     self.tail.get()
//   }
//
//   fn get_mut(&mut self) -> Option<&mut FromTail> {
//     self.tail.get_mut()
//   }
// }
//
// #[cfg(test)]
// mod tests {
//   use frunk_core::hlist;
//
//   #[test]
//   fn test_add() {
//     let a = hlist![1u8, "hello", true];
//     let b = hlist![false, 1u8, 3.14f32];
//
//     let intersection = a.intersect(b);
//
//     assert_eq!(intersection, hlist![1u8, true]);
//   }
// }
