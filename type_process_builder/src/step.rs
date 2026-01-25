#[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Eq, Clone, PartialOrd, Ord, Hash)]
pub struct Message(pub String);

pub mod splitter_output_repr {
  use crate::param_list::ParamList;
  use frunk_core::coproduct::Coproduct;

  pub trait SplitterOutput {}
  impl<Tag, ThisCase: ParamList, OtherCase> SplitterOutput for Coproduct<(Tag, ThisCase), OtherCase> {}
}

// Should only pass params required in further part of the process, but I don't know what they are.
// todo Make all the methods generic over Serializer
pub mod step {
  use crate::param_list::ParamList;
  use crate::step::splitter_output_repr::SplitterOutput;
  use crate::step::Message;
  use serde::de::DeserializeOwned;
  use std::collections::BTreeMap;
  use std::future::Future;

  pub trait Entry<RawConsume: DeserializeOwned> {
    type Produces: ParamList;
    fn handle(
      &self,
      consumes: BTreeMap<RawConsume, RawConsume>,
      shortcode_string: String,
    ) -> impl Future<Output = anyhow::Result<Self::Produces>>;
  }

  pub trait Operation {
    type Consumes: ParamList;
    type Produces: ParamList;
    fn handle(&self, consumes: Self::Consumes) -> impl Future<Output = anyhow::Result<Self::Produces>>;
  }

  pub trait Form {
    type Consumes: ParamList;
    type Produces: ParamList;
    fn show_form(&self, consumes: Self::Consumes) -> impl Future<Output = anyhow::Result<Message>>;
    fn handle_input(
      &self,
      consumes: Self::Consumes,
      user_input: String,
    ) -> impl Future<Output = anyhow::Result<Self::Produces>>;
  }

  /// Works with at lease two cases.
  /// just produce link form with a single link using Form step
  pub trait Splitter {
    type Consumes: ParamList;
    type Produces: SplitterOutput;
    fn handle(&self, consumes: Self::Consumes) -> impl Future<Output = anyhow::Result<Self::Produces>>;
  }

  pub trait Final {
    type Consumes: ParamList;
    fn handle(&self, consumes: Self::Consumes) -> impl Future<Output = anyhow::Result<Message>>;
  }
}
