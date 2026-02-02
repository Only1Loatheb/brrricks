use frunk_core::hlist::{HCons, HList, HNil};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_value::SerializerError::Custom;
use serde_value::{DeserializerError, SerializerError, Value, to_value};
use std::collections::BTreeMap;
use typenum::Unsigned;

/// clone (required by run method) should be used in brick instead
/// Use [typenum::op] to generate UID if the desired typenum const is missing.
pub trait ParamValue: Clone + Serialize + DeserializeOwned {
  type UID: Unsigned;
}

// use io:Read or Serializer and Deserializer wrappers instead of serde_value
// could be BTreeMap<String, Value> or even BTreeMap<type_id, Value> but there is no need for unnecessary type conversions
pub trait ParamList: HList + Clone {
  // https://serde.rs/impl-serialize.html#serializing-a-sequence-or-map
  fn serialize(&self) -> Result<Value, SerializerError> {
    let mut serialize_map = BTreeMap::new();
    self._serialize(&mut serialize_map)?;
    Ok(Value::Map(serialize_map))
  }
  fn _serialize(&self, serialize_map: &mut BTreeMap<Value, Value>) -> Result<(), SerializerError>;

  // https://serde.rs/deserialize-map.html
  fn deserialize(value: Value) -> Result<Self, DeserializerError> {
    let mut map = match value {
      Value::Map(m) => m,
      _ => return Err(DeserializerError::Custom("Expected map".into())),
    };
    Self::_deserialize(&mut map)
  }
  fn _deserialize(map: &mut BTreeMap<Value, Value>) -> Result<Self, DeserializerError>;
}

impl ParamList for HNil {
  fn _serialize(&self, _: &mut BTreeMap<Value, Value>) -> Result<(), SerializerError> {
    Ok(())
  }

  fn _deserialize(_map: &mut BTreeMap<Value, Value>) -> Result<Self, DeserializerError> {
    Ok(HNil)
  }
}

impl<Head: ParamValue, Tail: ParamList> ParamList for HCons<Head, Tail> {
  fn _serialize(&self, serialize_map: &mut BTreeMap<Value, Value>) -> Result<(), SerializerError> {
    self.tail._serialize(serialize_map)?;
    let old_value = serialize_map.insert(Value::U64(Head::UID::U64), to_value(&self.head)?);
    match old_value {
      None => Ok(()),
      Some(_) => Err(Custom(format!(
        "Two ParamValues have the same name: {}",
        Head::UID::U64
      ))),
    }
  }

  fn _deserialize(map: &mut BTreeMap<Value, Value>) -> Result<Self, DeserializerError> {
    let key = Value::U64(Head::UID::U64);
    let value = map
      .remove(&key)
      .ok_or_else(|| DeserializerError::Custom(format!("Missing key: {}", Head::UID::U64)))?;

    let head: Head = Head::deserialize(value)?;
    let tail = Tail::_deserialize(map)?;
    Ok(HCons { head, tail })
  }
}
