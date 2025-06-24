#[derive(serde::Deserialize, serde::Serialize, PartialEq, Debug, Eq, Clone, PartialOrd, Ord, Hash)]
pub struct Message(pub String);

// We want to allow defining split with one case for Confirmation forms
pub mod splitter_output_repr {
  use crate::param_list::ParamList;
  use frunk_core::coproduct::Coproduct;

  pub trait SplitterOutput {
    type NonEmptyCoproduct;
  }
  impl<ThisCase: ParamList, OtherCase> SplitterOutput for Coproduct<ThisCase, OtherCase> {
    type NonEmptyCoproduct = Coproduct<ThisCase, OtherCase>;
  }
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
    ) -> impl Future<Output = anyhow::Result<Self::Produces>>;
  }

  pub trait Operation<Consumes: ParamList, Produces: ParamList> {
    fn handle(&self, consumes: Consumes) -> impl Future<Output = anyhow::Result<Produces>>;
  }

  pub trait Form<Consumes: ParamList, Produces: ParamList> {
    fn proompt(&self, consumes: Consumes) -> impl Future<Output = anyhow::Result<Message>>;
    fn handle_input(&self, consumes: Consumes, user_input: String) -> impl Future<Output = anyhow::Result<Produces>>;
  }

  pub trait Splitter<Consumes: ParamList, Produces: SplitterOutput> {
    fn handle(&self, consumes: Consumes) -> impl Future<Output = anyhow::Result<Produces>>;
  }

  pub trait Final<Consumes: ParamList> {
    fn handle(&self, consumes: Consumes) -> impl Future<Output = anyhow::Result<Message>>;
  }
}
