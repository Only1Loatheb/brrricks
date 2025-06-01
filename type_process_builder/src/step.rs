use serde::de::DeserializeOwned;
use serde::de::{Deserialize, Deserializer, MapAccess, Visitor};
use serde::Serialize;

pub mod splitter_output_repr {
  use frunk_core::coproduct::{CNil, Coproduct};
  use crate::param_list::ParamList;

  pub trait SplitterOutput {
    type VALUE;
  }

  impl<CASE_THIS: ParamList> SplitterOutput for Coproduct<CASE_THIS, CNil> {
    type VALUE = Coproduct<CASE_THIS, CNil>;
  }

  impl<CASE_THIS: ParamList, CASE_OTHER: SplitterOutput> SplitterOutput for Coproduct<CASE_THIS, CASE_OTHER> {
    type VALUE = Coproduct<CASE_THIS, CASE_OTHER::VALUE>;
  }
}

pub mod step {
  use crate::step::splitter_output_repr::SplitterOutput;
  use process_builder_common::process_domain::Message;
  use crate::param_list::ParamList;

  pub trait Linear<CONSUMES: ParamList, PRODUCES: ParamList> {
    async fn handle(&self, input: CONSUMES) -> anyhow::Result<(Option<Message>, PRODUCES)>;
  }

  pub trait Splitter<CONSUMES: ParamList, PRODUCES: SplitterOutput> {
    async fn handle(&self, input: CONSUMES) -> anyhow::Result<PRODUCES>;
  }

  pub trait Final<CONSUMES: ParamList> {
    async fn handle(&self, input: CONSUMES) -> anyhow::Result<Message>;
  }
}
