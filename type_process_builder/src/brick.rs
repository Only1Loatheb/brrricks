use std::collections::HashMap;
use std::marker::PhantomData;

use crate::invariant::Invariant;
use async_trait::async_trait;
use frunk_core::coproduct::{CNil, Coproduct};
use frunk_core::hlist::{HCons, HList, HNil};
use process_builder_common::brick_domain::*;
use serde::{Deserialize, Serialize, Serializer};
// #[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]

pub trait ParamValue: Serialize + for<'de> Deserialize<'de> {}

pub trait ParamList: HList {}

impl ParamList for HNil {}

impl<PARAM_VALUE: ParamValue, TAIL: ParamList> ParamList for HCons<PARAM_VALUE, TAIL> {}

pub struct ParamRepr<'same_process, PARAM_VALUE: ParamValue> {
  pub(crate) same_process_invariant: Invariant<'same_process>,
  pub(crate) param_value: PhantomData<PARAM_VALUE>,
  pub(crate) param_id: ParamId,
}

pub trait ParamReprList: HList {
  type VALUE: ParamList;
}

impl<'same_process, SERIALIZER: Serializer> ParamReprList for HNil {
  type VALUE = HNil;
}

impl<'same_process, PARAM_VALUE: ParamValue, TAIL: ParamReprList> ParamReprList
  for HCons<ParamRepr<'same_process, PARAM_VALUE>, TAIL>
{
  type VALUE = HCons<PARAM_VALUE, TAIL::VALUE>;
}

#[async_trait]
pub trait TypeLinearBrickHandler<'same_process, CONSUMES: ParamReprList, PRODUCES: ParamReprList> {
  async fn handle(&self, input: CONSUMES::VALUE) -> anyhow::Result<(Option<Message>, PRODUCES::VALUE)>;

  // fn produces() -> PRODUCES;
}

// #[async_trait]
// impl<
//         'same_process,
//         SERIALIZER: Serializer,
//         CONSUMES: ParamReprList<'same_process, SERIALIZER>,
//         PRODUCES: ParamReprList<'same_process, SERIALIZER>,
//     > LinearBrickHandler
//     for dyn TypeLinearBrickHandler<'same_process, SERIALIZER, CONSUMES, PRODUCES>
// {
//     async fn handle(&self, input: InputParams) -> anyhow::Result<LinearOutput> {
//         let result = self.handle(input).await?;
//         let param_ids: Vec<ParamId> = vec![];
//         let param_values = result.1.get_params(serializer);
//         anyhow::Ok(LinearOutput(
//             result.0,
//             OutputParams(param_ids.iter().zip(param_values).into()),
//         ))
//     }
// }

/// We can add a list of completed actions later
pub struct LinearBrick<
  'same_process,
  SERIALIZER: Serializer,
  CONSUMES: ParamReprList<'same_process, SERIALIZER>,
  PRODUCES: ParamReprList<'same_process, SERIALIZER>,
> {
  pub name: String,
  pub produces: PRODUCES,
  pub handler: Box<dyn TypeLinearBrickHandler<'same_process, SERIALIZER, CONSUMES, PRODUCES>>,
}

/// Let's hope rust will optimise frunk::coproduct::Coproduct just fine :copium:
pub trait SplitterOutputRepr<'same_process> {
  fn get_param_ids(self) -> Vec<ParamId>;
  fn index(self) -> usize;
}

impl<'same_process, SERIALIZER: Serializer, CASE_THIS: ParamReprList<'same_process, SERIALIZER>>
  SplitterOutputRepr<'same_process> for Coproduct<CASE_THIS, CNil>
{
  fn get_param_ids(self) -> Vec<ParamId> {
    match self {
      Coproduct::Inl(inl) => inl.get_param_ids(),
      Coproduct::Inr(cnil) => match cnil {},
    }
  }

  fn index(self) -> usize {
    match self {
      Coproduct::Inl(_) => 0,
      Coproduct::Inr(cnil) => match cnil {},
    }
  }
}

impl<
    'same_process,
    SERIALIZER: Serializer,
    CASE_THIS: ParamReprList<'same_process, SERIALIZER>,
    CASE_OTHER: SplitterOutputRepr<'same_process>,
  > SplitterOutputRepr<'same_process> for Coproduct<CASE_THIS, CASE_OTHER>
{
  fn get_param_ids(self) -> Vec<ParamId> {
    match self {
      Coproduct::Inl(inl) => inl.get_param_ids(),
      Coproduct::Inr(inr) => inr.get_param_ids(),
    }
  }

  fn index(self) -> usize {
    match self {
      Coproduct::Inl(_) => 0,
      Coproduct::Inr(tail) => 1 + tail.index(),
    }
  }
}

/// At least two cases, I guess
#[async_trait]
pub trait TypeSplitterBrickHandler<'same_process, SPLITTER_OUTPUT_REPR: SplitterOutputRepr<'same_process>> {
  async fn handle(&self, input: InputParams) -> anyhow::Result<SPLITTER_OUTPUT_REPR>;
}

// #[async_trait]
// impl<'same_process, SPLITTER_OUTPUT_REPR: SplitterOutputRepr<'same_process>> SplitterBrickHandler
//     for dyn TypeSplitterBrickHandler<'same_process, SPLITTER_OUTPUT_REPR>
// {
//     async fn handle(&self, input: InputParams) -> anyhow::Result<SplitterOutput> {
//         let result = self.handle(input).await?;
//         anyhow::Ok(SplitterOutput(
//             SplitIndex(result.index()),
//             OutputParams(result.get_param_ids()),
//         ))
//     }
// }

/// We can add a list of completed actions later
pub struct SplitterBrick<
  'same_process,
  CASE_THIS: ParamReprList<'same_process>,
  CASE_OTHER: SplitterOutputRepr<'same_process>,
> {
  pub name: String,
  pub produces: Coproduct<CASE_THIS, CASE_OTHER>,
  pub handler: Box<dyn TypeSplitterBrickHandler<'same_process, CASE_THIS, CASE_OTHER>>,
}

#[async_trait]
pub trait TypeFinalBrickHandler<'same_process, CONSUMES: ParamReprList<'same_process>> {
  async fn handle(&self, input: CONSUMES::VALUE) -> anyhow::Result<Message>;
}

pub struct FinalBrick<'same_process, CONSUMES: ParamReprList<'same_process>> {
  pub name: String,
  pub handler: Box<dyn TypeFinalBrickHandler<'same_process, CONSUMES>>,
}
