use crate::builder::SessionContext;
use frunk_core::hlist::{HCons, HList, HNil};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_value::SerializerError::Custom;
use serde_value::{DeserializerError, SerializerError, to_value};
use std::collections::HashMap;
use typenum::Unsigned;

pub mod clone_just;
pub mod concat;
pub mod intersect;
pub mod transform;

/// clone (required by run method) should be used in brick instead
/// Use [typenum::op] to generate UID if the desired typenum const is missing.
pub trait ParamValue: Clone + Serialize + DeserializeOwned {
  type UID: Unsigned;
}

// use io:Read or Serializer and Deserializer wrappers instead of serde_value
pub trait ParamList: HList + Clone {
  // https://serde.rs/impl-serialize.html#serializing-a-sequence-or-map
  fn serialize(&self) -> Result<SessionContext, SerializerError> {
    let mut serialize_map = HashMap::new();
    self._serialize(&mut serialize_map)?;
    Ok(serialize_map)
  }
  fn _serialize(&self, serialize_map: &mut SessionContext) -> Result<(), SerializerError>;

  // https://serde.rs/deserialize-map.html
  fn deserialize(value: SessionContext) -> Result<Self, DeserializerError> {
    Self::_deserialize(value)
  }
  fn _deserialize(map: SessionContext) -> Result<Self, DeserializerError>;
}

impl ParamList for HNil {
  fn _serialize(&self, _: &mut SessionContext) -> Result<(), SerializerError> {
    Ok(())
  }

  fn _deserialize(_map: SessionContext) -> Result<Self, DeserializerError> {
    Ok(HNil)
  }
}

impl<Head: ParamValue, Tail: ParamList> ParamList for HCons<Head, Tail> {
  fn _serialize(&self, serialize_map: &mut SessionContext) -> Result<(), SerializerError> {
    self.tail._serialize(serialize_map)?;
    let old_value = serialize_map.insert(Head::UID::U64, to_value(&self.head)?);
    match old_value {
      None => Ok(()),
      Some(_) => Err(Custom(format!(
        "Two ParamValues have the same name: {}",
        Head::UID::U64
      ))),
    }
  }

  fn _deserialize(mut map: SessionContext) -> Result<Self, DeserializerError> {
    let value = map
      .remove(&Head::UID::U64)
      .ok_or_else(|| DeserializerError::Custom(format!("Missing key: {}", Head::UID::U64)))?;

    let head: Head = Head::deserialize(value)?;
    let tail = Tail::_deserialize(map)?;
    Ok(HCons { head, tail })
  }
}
