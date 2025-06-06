pub mod splitter_output_repr {
  use crate::param_list::ParamList;
  use frunk_core::coproduct::{CNil, Coproduct};

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

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Eq, Clone, PartialOrd, Ord, Hash)]
pub struct Message(pub String);

pub mod step {
  use crate::param_list::ParamList;
  use crate::step::splitter_output_repr::SplitterOutput;
  use crate::step::Message;
  use serde::de::DeserializeOwned;
  use std::collections::BTreeMap;
  use std::future::Future;

  pub trait Entry<VALUE: DeserializeOwned> {
    type Produces: ParamList;
    fn handle(&self, input: BTreeMap<VALUE, VALUE>) -> impl Future<Output = anyhow::Result<Self::Produces>>;
  }

  pub trait Linear<CONSUMES: ParamList, PRODUCES: ParamList> {
    fn handle(&self, input: CONSUMES) -> impl Future<Output = anyhow::Result<(Option<Message>, PRODUCES)>>;
  }

  pub trait Splitter<CONSUMES: ParamList, PRODUCES: SplitterOutput> {
    fn handle(&self, input: CONSUMES) -> impl Future<Output = anyhow::Result<PRODUCES>>;
  }

  pub trait Final<CONSUMES: ParamList> {
    fn handle(&self, input: CONSUMES) -> impl Future<Output = anyhow::Result<Message>>;
  }
}
