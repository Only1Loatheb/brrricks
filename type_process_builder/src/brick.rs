use crate::brick::param_repr_list::ParamReprList;
use crate::brick::split::SplitterOutputRepr;
use crate::invariant::Invariant;
use async_trait::async_trait;
use process_builder_common::brick_domain::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub trait ParamValue: Serialize + for<'de> Deserialize<'de> {}

mod param_list {
  use crate::brick::ParamValue;
  use frunk_core::hlist::{HCons, HList, HNil};

  pub trait ParamList: HList {}

  impl ParamList for HNil {}

  impl<PARAM_VALUE: ParamValue, TAIL: ParamList> ParamList for HCons<PARAM_VALUE, TAIL> {}
}

pub struct ParamRepr<'same_process, PARAM_VALUE: ParamValue> {
  pub(crate) same_process_invariant: Invariant<'same_process>,
  pub(crate) param_value: PhantomData<PARAM_VALUE>,
  pub(crate) param_id: ParamId,
}

mod param_repr_list {
  use crate::brick::param_list::ParamList;
  use crate::brick::{ParamRepr, ParamValue};
  use frunk_core::hlist::{HCons, HList, HNil};

  pub trait ParamReprList: HList {
    type VALUE: ParamList;
  }

  impl<'same_process> ParamReprList for HNil {
    type VALUE = HNil;
  }

  impl<'same_process, PARAM_VALUE: ParamValue, TAIL: ParamReprList> ParamReprList
    for HCons<ParamRepr<'same_process, PARAM_VALUE>, TAIL>
  {
    type VALUE = HCons<PARAM_VALUE, TAIL::VALUE>;
  }
}

mod split {
  use crate::brick::param_list::ParamList;
  use crate::brick::param_repr_list::ParamReprList;
  use frunk_core::coproduct::{CNil, Coproduct};

  pub trait SplitterOutputRepr {
    type VALUE: ParamList;
  }

  impl<CASE_THIS: ParamReprList> SplitterOutputRepr for Coproduct<CASE_THIS, CNil> {
    type VALUE = CASE_THIS::VALUE;
  }

  impl<CASE_THIS: ParamReprList, CASE_OTHER: SplitterOutputRepr> SplitterOutputRepr
    for Coproduct<CASE_THIS, CASE_OTHER>
  {
    type VALUE = Coproduct<CASE_THIS::VALUE, CASE_OTHER::VALUE>;
  }
}

#[async_trait]
pub trait TypeLinearBrickHandler<CONSUMES: ParamReprList, PRODUCES: ParamReprList> {
  async fn handle(&self, input: CONSUMES::VALUE) -> anyhow::Result<(Option<Message>, PRODUCES::VALUE)>;
}

#[async_trait]
pub trait TypeSplitterBrickHandler<CONSUMES: ParamReprList, SPLITTER_OUTPUT: SplitterOutputRepr> {
  async fn handle(&self, input: CONSUMES::VALUE) -> anyhow::Result<SPLITTER_OUTPUT::VALUE>;
}

#[async_trait]
pub trait TypeFinalBrickHandler<'same_process, CONSUMES: ParamReprList> {
  async fn handle(&self, input: CONSUMES::VALUE) -> anyhow::Result<Message>;
}
