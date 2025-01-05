use serde::{Deserialize, Serialize};

pub trait ParamValue: Serialize + for<'de> Deserialize<'de> {}

pub mod param_list {
  use crate::step::ParamValue;
  use frunk_core::hlist::{HCons, HList, HNil};

  pub trait ParamList: HList {}

  impl ParamList for HNil {}

  impl<PARAM_VALUE: ParamValue, TAIL: ParamList> ParamList for HCons<PARAM_VALUE, TAIL> {}
}

pub mod splitter_output_repr {
  use crate::step::param_list::ParamList;
  use frunk_core::coproduct::{CNil, Coproduct};

  pub trait SplitterOutput {
    type VALUE;
  }

  impl<CASE_THIS: ParamList> SplitterOutput for Coproduct<CASE_THIS, CNil> {
    type VALUE = Coproduct<CASE_THIS, CNil>;
  }

  impl<CASE_THIS: ParamList, CASE_OTHER: SplitterOutput> SplitterOutput
    for Coproduct<CASE_THIS, CASE_OTHER>
  {
    type VALUE = Coproduct<CASE_THIS, CASE_OTHER::VALUE>;
  }
}

pub mod step {
  use crate::step::param_list::ParamList;
  use crate::step::splitter_output_repr::SplitterOutput;
  use process_builder_common::process_domain::Message;

  pub trait Linear<CONSUMES: ParamList, PRODUCES: ParamList> {
    async fn handle(&self, input: CONSUMES) -> anyhow::Result<(Option<Message>, PRODUCES)>;
  }

  pub trait Splitter<CONSUMES: ParamList, PRODUCES: SplitterOutput> {
    async fn handle(&self, input: CONSUMES) -> anyhow::Result<PRODUCES>;
  }

  pub trait Final<CONSUMES: ParamList>{
    async fn handle(&self, input: CONSUMES) -> anyhow::Result<Message>;
  }
}
