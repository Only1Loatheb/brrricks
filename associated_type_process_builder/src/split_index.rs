use std::marker::PhantomData;

use typenum::{IsLess, NonZero, Unsigned};

#[derive(Clone)]
pub struct TypeSplitIndex<CASES_LEN: Unsigned> {
  value: usize,
  cases_len: PhantomData<CASES_LEN>,
}

impl<CASES_LEN: Unsigned> TypeSplitIndex<CASES_LEN> {
  pub fn new(value: usize) -> Option<TypeSplitIndex<CASES_LEN>> {
    if value < CASES_LEN::USIZE {
      Some(TypeSplitIndex { value, cases_len: Default::default() })
    } else {
      None
    }
  }

  pub fn new_const<TypeSplitIndexValue: Unsigned + IsLess<CASES_LEN>>() -> TypeSplitIndex<CASES_LEN>
  where
    TypeSplitIndexValue::Output: NonZero,
  {
    TypeSplitIndex {
      value: TypeSplitIndexValue::USIZE,
      cases_len: Default::default(),
    }
  }

  pub fn get(&self) -> usize {
    self.value
  }
}
