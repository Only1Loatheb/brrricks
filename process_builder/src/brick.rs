use std::collections::HashMap;
use std::marker::PhantomData;

use async_trait::async_trait;
use typenum::{Bit, UInt, Unsigned, UTerm};

// #[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]

#[derive(Clone)]
pub struct ParamId(pub usize);

#[derive(Clone)]
pub struct ActionId(pub usize);

#[derive(Clone)]
pub struct Message(pub String);

#[derive(Clone)]
pub struct SplitIndex(pub usize);

#[derive(Clone)]
pub struct InputParams(pub HashMap<ParamId, serde_json::value::Value>);

#[derive(Clone)]
pub struct OutputParams(pub HashMap<ParamId, serde_json::value::Value>);

#[derive(Clone)]
pub struct LinearOutput(pub Option<Message>, pub OutputParams);

#[derive(Clone)]
pub struct SplitterOutput(pub SplitIndex, pub OutputParams);

#[async_trait]
pub trait LinearBrickHandler {
    async fn handle(&self, input: InputParams) -> anyhow::Result<LinearOutput>;
}

#[async_trait]
pub trait SplitterBrickHandler {
    async fn handle(&self, input: InputParams) -> anyhow::Result<SplitterOutput>;
}

#[async_trait]
pub trait FinalBrickHandler {
    async fn handle(&self, input: InputParams) -> anyhow::Result<Message>;
}

pub trait TypeLevelSet: Unsigned {
  fn get() -> Vec<bool>;
}

impl TypeLevelSet for UTerm {
  fn get() -> Vec<bool> {
    vec![]
  }
}

impl<MORE_SIGNIFICANT_BITS: TypeLevelSet, LEAST_SIGNIFICANT_BIT: Bit> TypeLevelSet for UInt<MORE_SIGNIFICANT_BITS, LEAST_SIGNIFICANT_BIT> {
  fn get() -> Vec<bool> {
    let mut a = MORE_SIGNIFICANT_BITS::get();
    a.push(LEAST_SIGNIFICANT_BIT::to_bool());
    a
  }
}

pub struct LinearBrick<
    CONSUMES: TypeLevelSet,
    REQUIRES: TypeLevelSet,
    FORBIDS: TypeLevelSet,
    PRODUCES: TypeLevelSet,
    ACCOMPLISHES: TypeLevelSet,
> {
    pub name: &'static str,
    pub consumes: PhantomData<CONSUMES>,
    pub requires_prior_completion: PhantomData<REQUIRES>,
    pub forbids_prior_completion: PhantomData<FORBIDS>,
    pub produces: PhantomData<PRODUCES>,
    pub accomplishes: PhantomData<ACCOMPLISHES>,
    pub handler: Box<dyn LinearBrickHandler>,
}

// consider https://github.com/rust-phf/rust-phf for SplitIndex
pub struct SplitterBrick<
    SPLITS: TypeLevelSet,
    CONSUMES: TypeLevelSet,
    REQUIRES: TypeLevelSet,
    FORBIDS: TypeLevelSet,
> {
    pub name: &'static str,
    pub splits: PhantomData<SPLITS>,
    pub consumes: PhantomData<CONSUMES>,
    pub requires_prior_completion: PhantomData<REQUIRES>,
    pub forbids_prior_completion: PhantomData<FORBIDS>,
    // pub produces_and_accomplishes: Vec<(Vec<ActionId>, Vec<ParamId>)>,
    pub handler: Box<dyn SplitterBrickHandler>,
}

pub struct FinalBrick<
    CONSUMES: TypeLevelSet,
    REQUIRES: TypeLevelSet,
    FORBIDS: TypeLevelSet,
    ACCOMPLISHES: TypeLevelSet,
> {
    pub name: &'static str,
    pub consumes: PhantomData<CONSUMES>,
    pub requires_prior_completion: PhantomData<REQUIRES>,
    pub forbids_prior_completion: PhantomData<FORBIDS>,
    pub accomplishes: PhantomData<ACCOMPLISHES>,
    pub handler: Box<dyn FinalBrickHandler>,
}
