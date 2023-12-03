use generic_array::GenericArray;
use serde::{Deserialize, Serialize};
use typenum::Unsigned;

// #[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]

pub trait Named {
    fn name() -> &'static str;
}

pub trait Param: Named + Deserialize<'static> + Serialize {}

pub trait SplitParam: Named {
    type MaxSplitIndex: Unsigned;
}

#[derive(Clone)]
pub struct LinearBrickData<ConsumesCount: Unsigned, ProducesCount: Unsigned> {
    pub name: &'static str,
    pub consumes: GenericArray<dyn Param, ConsumesCount>,
    pub produces: GenericArray<dyn Param, ProducesCount>,
}

#[derive(Clone)] // consider https://github.com/rust-phf/rust-phf for SplitterBrick
pub struct SplitterBrickData<ConsumesCount: Unsigned, MaxSplitIndex: Unsigned> {
    pub name: &'static str,
    pub consumes: GenericArray<dyn Param, ConsumesCount>,
    pub produces: GenericArray<dyn Param, MaxSplitIndex>,
}

#[derive(Clone)]
pub struct FinalBrickkData<ConsumesCount: Unsigned> {
    pub name: &'static str,
    pub consumes: GenericArray<dyn Param, ConsumesCount>,
}

// add fn handle()
pub trait LinearBrick {
    type ConsumesCount: Unsigned;
    type ProducesCount: Unsigned;
    fn data(&self) -> LinearBrickData<Self::ConsumesCount, Self::ProducesCount>;
}

pub trait SplitterBrick {
    type ConsumesCount: Unsigned;
    type MaxSplitIndex: Unsigned;
    fn data(&self) -> SplitterBrickData<Self::ConsumesCount, Self::MaxSplitIndex>;
}

pub trait FinalBrick {
    type ConsumesCount: Unsigned;
    fn data(&self) -> FinalBrickkData<Self::ConsumesCount>;
}
