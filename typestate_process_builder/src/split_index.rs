use std::marker::PhantomData;
use typenum::{IsLessOrEqual, NonZero, Unsigned};

pub struct SplitIndex<MaxSplitIndex: Unsigned> {
    value: usize,
    max_split_size: PhantomData<MaxSplitIndex>,
}

impl<MaxSplitIndex: Unsigned> SplitIndex<MaxSplitIndex> {
    pub fn new(value: usize) -> Option<SplitIndex<MaxSplitIndex>> {
        if value <= MaxSplitIndex::USIZE {
            Some(SplitIndex {
                value,
                max_split_size: Default::default(),
            })
        } else {
            None
        }
    }

    pub fn new_const<SplitIndexValue: Unsigned + IsLessOrEqual<MaxSplitIndex>>(
    ) -> SplitIndex<MaxSplitIndex>
    where
        SplitIndexValue::Output: NonZero,
    {
        SplitIndex {
            value: SplitIndexValue::USIZE,
            max_split_size: Default::default(),
        }
    }

    pub fn get(&self) -> usize {
        self.value
    }
}
