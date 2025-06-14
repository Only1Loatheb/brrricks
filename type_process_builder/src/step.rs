pub mod splitter_output_repr {
  use crate::param_list::ParamList;
  use frunk_core::coproduct::{CNil, Coproduct};

  pub trait SplitterOutput {
    type NonEmptyCoproduct;
  }

  impl<ThisCase: ParamList> SplitterOutput for Coproduct<ThisCase, CNil> {
    type NonEmptyCoproduct = Coproduct<ThisCase, CNil>;
  }

  impl<ThisCase: ParamList, OtherCase: SplitterOutput> SplitterOutput for Coproduct<ThisCase, OtherCase> {
    type NonEmptyCoproduct = Coproduct<ThisCase, OtherCase::NonEmptyCoproduct>;
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

  pub trait Entry<RawConsume: DeserializeOwned> {
    type Produces: ParamList;
    fn handle(&self, input: BTreeMap<RawConsume, RawConsume>) -> impl Future<Output = anyhow::Result<Self::Produces>>;
  }

  pub trait Linear<Consumes: ParamList, Produces: ParamList> {
    fn handle(&self, input: Consumes) -> impl Future<Output = anyhow::Result<(Option<Message>, Produces)>>;
  }

  pub trait Splitter<Consumes: ParamList, Produces: SplitterOutput> {
    fn handle(&self, input: Consumes) -> impl Future<Output = anyhow::Result<Produces>>;
  }

  pub trait Final<Consumes: ParamList> {
    fn handle(&self, input: Consumes) -> impl Future<Output = anyhow::Result<Message>>;
  }
}
