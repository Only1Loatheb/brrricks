use crate::invariant::Invariant;
use process_builder_common::process_domain::ParamId;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub trait ParamValue: Serialize + for<'de> Deserialize<'de> {}

pub mod param_list {
  use crate::step::ParamValue;
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

pub mod param_repr_list {
  use crate::step::param_list::ParamList;
  use crate::step::{ParamRepr, ParamValue};
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

pub mod splitter_output_repr {
  use crate::step::param_list::ParamList;
  use crate::step::param_repr_list::ParamReprList;
  use frunk_core::coproduct::{CNil, Coproduct};

  pub trait SplitterOutputRepr {
    type VALUE: ParamList;
  }

  impl<CASE_THIS: ParamReprList> SplitterOutputRepr for Coproduct<CASE_THIS, CNil> {
    type VALUE = CASE_THIS::VALUE;
  }

  impl<CASE_THIS: ParamReprList, CASE_OTHER: SplitterOutputRepr> SplitterOutputRepr for Coproduct<CASE_THIS, CASE_OTHER> {
    type VALUE = Coproduct<CASE_THIS::VALUE, CASE_OTHER::VALUE>;
  }
}

pub mod step {
  use crate::step::param_repr_list::ParamReprList;
  use crate::step::splitter_output_repr::SplitterOutputRepr;
  use async_trait::async_trait;
  use process_builder_common::process_domain::Message;

  #[async_trait]
  pub trait Linear {
    type CONSUMES: ParamReprList;
    type PRODUCES: ParamReprList;
    async fn handle(&self, input: Self::CONSUMES::VALUE) -> anyhow::Result<(Option<Message>, Self::PRODUCES::VALUE)>;
  }

  #[async_trait]
  pub trait Splitter {
    type CONSUMES: ParamReprList;
    type PRODUCES: SplitterOutputRepr;
    async fn handle(&self, input: Self::CONSUMES::VALUE) -> anyhow::Result<Self::PRODUCES::VALUE>;
  }

  #[async_trait]
  pub trait Final {
    type CONSUMES: ParamReprList;
    async fn handle(&self, input: Self::CONSUMES::VALUE) -> anyhow::Result<Message>;
  }
}
