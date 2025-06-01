use crate::step::param_list::ParamList;
use serde::de::DeserializeOwned;
use serde::de::{Deserialize, Deserializer, MapAccess, Visitor};
use serde::Serialize;

/// clone (required by run method) should be used in brick instead
pub trait ParamValue: Clone + Serialize + DeserializeOwned {
  const NAME: &'static str;
}

pub mod param_list {
  use crate::step::ParamValue;
  use frunk_core::hlist::{HCons, HList, HNil};
  use serde_value::SerializerError::Custom;
  use serde_value::{to_value, SerializerError, Value};
  use std::collections::BTreeMap;

  pub trait ParamList: HList + Clone {
    /// maybe serialize as a list if the same list is always serialized and deserialized?
    fn serialize(&self) -> Result<Value, SerializerError> {
      let mut serialize_map = BTreeMap::new();
      self._serialize(&mut serialize_map)?;
      Ok(Value::Map(serialize_map))
    }
    fn _serialize(&self, serialize_map: &mut BTreeMap<Value, Value>) -> Result<(), SerializerError>;
  }

  impl ParamList for HNil {
    fn _serialize(&self, serialize_map: &mut BTreeMap<Value, Value>) -> Result<(), SerializerError> {
      Ok(())
    }
  }

  impl<PARAM_VALUE: ParamValue, TAIL: ParamList> ParamList for HCons<PARAM_VALUE, TAIL> {
    fn _serialize(&self, serialize_map: &mut BTreeMap<Value, Value>) -> Result<(), SerializerError> {
      self.tail._serialize(serialize_map)?;
      let old_value = serialize_map.insert(Value::String(PARAM_VALUE::NAME.into()), to_value(&self.head)?);
      match old_value {
        None => Ok(()),
        Some(_) => Err(Custom(format!(
          "Two ParamValues have the same name: {}",
          PARAM_VALUE::NAME
        ))),
      }
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
