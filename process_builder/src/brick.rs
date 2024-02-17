use std::collections::HashMap;
use std::marker::PhantomData;

use async_trait::async_trait;
use typenum::Unsigned;

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

pub struct LinearBrick<
    CONSUMES: Unsigned,
    REQUIRES: Unsigned,
    FORBIDS: Unsigned,
    PRODUCES: Unsigned,
    ACCOMPLISHES: Unsigned,
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
pub struct SplitterBrick {
    pub name: &'static str,
    pub consumes: Vec<ParamId>,
    pub requires_prior_completion: Vec<ActionId>,
    pub forbids_prior_completion: Vec<ActionId>,
    pub produces_and_accomplishes: Vec<(Vec<ActionId>, Vec<ParamId>)>,
    pub handler: Box<dyn SplitterBrickHandler>,
}

pub struct FinalBrick<
    CONSUMES: Unsigned,
    REQUIRES: Unsigned,
    FORBIDS: Unsigned,
> {
    pub name: &'static str,
    pub consumes: PhantomData<CONSUMES>,
    pub requires_prior_completion: PhantomData<REQUIRES>,
    pub forbids_prior_completion: PhantomData<FORBIDS>,
    pub handler: Box<dyn FinalBrickHandler>,
}
