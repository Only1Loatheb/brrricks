use serde::de::DeserializeOwned;
use serde::Serialize;

/// clone (required by run method) should be used in brick instead
pub trait ParamValue: Clone + Serialize + DeserializeOwned {
  const NAME: &'static str;
}

pub mod param_list {
  use crate::step::ParamValue;
  use frunk_core::hlist::{HCons, HList, HNil};
  use serde::de::MapAccess;
  use serde::ser::SerializeMap;
  use serde::{Deserializer, Serializer};

  pub trait ParamList: HList + Clone {
    fn _serialize<S: Serializer>(&self, serialize_map: &mut S::SerializeMap) -> Result<(), S::Error>;

    /// maybe serialize as a list if the same list is always serialized and deserialized?
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
      // https://serde.rs/impl-serialize.html#serializing-a-sequence-or-map
      let mut serialize_map = serializer.serialize_map(Some(self.len()))?;
      self._serialize::<S>(&mut serialize_map)?;
      serialize_map.end()
    }
  }

  impl ParamList for HNil {
    fn _serialize<S: Serializer>(&self, serialize_map: &mut S::SerializeMap) -> Result<(), S::Error> {
      Ok(())
    }
  }

  impl<PARAM_VALUE: ParamValue, TAIL: ParamList> ParamList for HCons<PARAM_VALUE, TAIL> {
    fn _serialize<S: Serializer>(&self, serialize_map: &mut S::SerializeMap) -> Result<(), S::Error> {
      self.tail._serialize::<S>(serialize_map)?;
      serialize_map.serialize_entry(PARAM_VALUE::NAME, &self.head)
    }
  }
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

  impl<CASE_THIS: ParamList, CASE_OTHER: SplitterOutput> SplitterOutput for Coproduct<CASE_THIS, CASE_OTHER> {
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

  pub trait Final<CONSUMES: ParamList> {
    async fn handle(&self, input: CONSUMES) -> anyhow::Result<Message>;
  }
}
