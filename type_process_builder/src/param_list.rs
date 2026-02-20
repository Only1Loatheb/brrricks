use crate::builder::SessionContext;
use frunk_core::hlist::{HCons, HList, HNil};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_value::{DeserializerError, SerializerError, to_value};
use typenum::Unsigned;

pub mod clone_just;
pub mod concat;
pub mod intersect;
pub mod transform;

/// clone (required by run method) should be used in brick instead
/// Use [typenum::op] to generate UID if the desired typenum const is missing.
pub trait ParamValue: Clone + Serialize + DeserializeOwned + Send + Sync {
  type UID: Unsigned;
}

pub trait ParamList: HList + Clone + Send + Sync {
  // https://serde.rs/impl-serialize.html#serializing-a-sequence-or-map
  fn serialize(&self) -> Result<SessionContext, SerializerError> {
    let mut session_context = Vec::with_capacity(self.len());
    self._serialize(&mut session_context)?;
    Ok(session_context)
  }
  fn _serialize(&self, serialize_map: &mut SessionContext) -> Result<(), SerializerError>;

  // https://serde.rs/deserialize-map.html
  fn deserialize(session_context: SessionContext) -> Result<Self, DeserializerError> {
    Self::_deserialize(session_context)
  }
  fn _deserialize(session_context: SessionContext) -> Result<Self, DeserializerError>;
}

impl ParamList for HNil {
  fn _serialize(&self, _: &mut SessionContext) -> Result<(), SerializerError> {
    Ok(())
  }

  fn _deserialize(_session_context: SessionContext) -> Result<Self, DeserializerError> {
    Ok(HNil)
  }
}

impl<Head: ParamValue, Tail: ParamList> ParamList for HCons<Head, Tail> {
  fn _serialize(&self, session_context: &mut SessionContext) -> Result<(), SerializerError> {
    self.tail._serialize(session_context)?;
    session_context.push((Head::UID::U32, to_value(&self.head)?));
    Ok(())
  }

  /// https://isocpp.org/blog/2014/06/stroustrup-lists
  fn _deserialize(mut session_context: SessionContext) -> Result<Self, DeserializerError> {
    let position = session_context
      .iter()
      .rev()
      .position(|(k, _)| *k == Head::UID::U32)
      .ok_or_else(|| DeserializerError::Custom(format!("Missing key: {}", Head::UID::U64)))?;
    let (_, value) = session_context.swap_remove((session_context.len() - 1) - position);
    let head: Head = Head::deserialize(value)?;
    let tail = Tail::_deserialize(session_context)?;
    Ok(HCons { head, tail })
  }
}
