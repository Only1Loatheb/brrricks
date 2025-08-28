use frunk_core::hlist::*;
use frunk_core::indices::{Here, There};

pub trait SelectOrAbsent<Target, Index> {
  fn select(&self) -> Option<&Target>;

  fn select_mut(&mut self) -> Option<&mut Target>;
}

impl<Target, Tail> SelectOrAbsent<Target, Here> for HCons<Target, Tail> {
  fn select(&self) -> Option<&Target> {
    Some(&self.head)
  }

  fn select_mut(&mut self) -> Option<&mut Target> {
    Some(&mut self.head)
  }
}

impl<Target> SelectOrAbsent<Target, Here> for HNil {
  fn select(&self) -> Option<&Target> {
    None
  }

  fn select_mut(&mut self) -> Option<&mut Target> {
    None
  }
}

impl<Head, Tail, Target, Index> SelectOrAbsent<Target, There<Index>> for HCons<Head, Tail>
where
  Tail: SelectOrAbsent<Target, Index>,
{
  fn select(&self) -> Option<&Target> {
    self.tail.select()
  }

  fn select_mut(&mut self) -> Option<&mut Target> {
    self.tail.select_mut()
  }
}

#[cfg(test)]
mod tests {
  use crate::hlist_intersect::*;
  use frunk_core::hlist;
  use frunk_core::hlist::{HCons, HNil};

  #[test]
  fn test_select_or_absent() {
    let mut my_list = hlist![42u32, "hello", true];

    // Example 1: existing type
    let val: Option<&u32> = <HCons<u32, HCons<&str, HCons<bool, HNil>>> as SelectOrAbsent<_, _>>::select(&my_list);
    println!("Found u32? {:?}", val); // Some(42)

    // Example 2: mut reference
    if let Some(val_mut) = <HCons<u32, HCons<&str, HCons<bool, HNil>>>>::select_mut(&mut my_list) {
      *val_mut = 99;
    }
    println!("Mutated list: {:?}", my_list);

    // Example 3: type not in HList (Missing)
    let missing: Option<&f64> = my_list.select();
    println!("Found f64? {:?}", missing);
    // None
    // let a = hlist![1u8, "hello", true];
    // let b = hlist![false, 1u8, 3.14f32];
    //
    // let intersection = a.intersect(b);
    //
    // assert_eq!(intersection, hlist![1u8, true]);
  }
}
