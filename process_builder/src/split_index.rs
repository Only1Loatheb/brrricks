use typenum::{IsLessOrEqual, NonZero, Unsigned};

pub struct SplitIndex<MaxSplitIndex: Unsigned> {
  value: usize,
}

impl<MaxSplitIndex: Unsigned> SplitIndex<MaxSplitIndex> {
  pub fn new(value: usize) -> Option<SplitIndex<MaxSplitIndex>> {
    if value <= MaxSplitIndex::USIZE { Some(SplitIndex { value }) } else { None }
  }

  pub fn new_const<SplitIndexValue: Unsigned + IsLessOrEqual<MaxSplitIndex>>() -> SplitIndex<MaxSplitIndex>
    where SplitIndexValue::Output: NonZero {
    SplitIndex { value: SplitIndexValue::USIZE }
  }

  pub fn get(&self) -> usize { self.value }
}
