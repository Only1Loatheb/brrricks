use frunk_core::hlist::{HCons, HList, HNil};
use serde_value::SerializerError::Custom;
use serde_value::{to_value, DeserializerError, SerializerError, Value};
use std::collections::BTreeMap;
use serde::de::DeserializeOwned;
use serde::Serialize;

/// clone (required by run method) should be used in brick instead
pub trait ParamValue: Clone + Serialize + DeserializeOwned {
  const NAME: &'static str;
}

// use io:Read or Serializer and Deserializer wrappers instead of serde_value
// could be BTreeMap<String, Value> or even BTreeMap<type_id, Value> but there is no need for unnecessary type conversions
pub trait ParamList: HList + Clone {
  /// maybe serialize as a list if the same list is always serialized and deserialized?
  fn serialize(&self) -> Result<Value, SerializerError> {
    let mut serialize_map = BTreeMap::new();
    self._serialize(&mut serialize_map)?;
    Ok(Value::Map(serialize_map))
  }
  fn _serialize(&self, serialize_map: &mut BTreeMap<Value, Value>) -> Result<(), SerializerError>;

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

  fn _deserialize(map: &mut BTreeMap<Value, Value>) -> Result<Self, DeserializerError> {
    let key = Value::String(PARAM_VALUE::NAME.into());
    let value = map
      .remove(&key)
      .ok_or_else(|| DeserializerError::Custom(format!("Missing key: {}", PARAM_VALUE::NAME)))?;

    let head: PARAM_VALUE = PARAM_VALUE::deserialize(value)?;
    let tail = TAIL::_deserialize(map)?;
    Ok(HCons { head, tail })
  }
}
