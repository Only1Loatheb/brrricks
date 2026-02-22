use crate::builder::{ParamUID, SessionContext};
use frunk_core::hlist::{HCons, HList, HNil};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_value::{DeserializerError, SerializerError, to_value};
use std::collections::HashSet;
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

  fn all_param_uids(acc: &mut HashSet<ParamUID>);
}

impl ParamList for HNil {
  fn _serialize(&self, _: &mut SessionContext) -> Result<(), SerializerError> {
    Ok(())
  }

  fn _deserialize(_session_context: SessionContext) -> Result<Self, DeserializerError> {
    Ok(HNil)
  }

  fn all_param_uids(_acc: &mut HashSet<ParamUID>) {}
}

impl<Head: ParamValue, Tail: ParamList> ParamList for HCons<Head, Tail> {
  fn _serialize(&self, session_context: &mut SessionContext) -> Result<(), SerializerError> {
    self.tail._serialize(session_context)?;
    session_context.push((Head::UID::U32, to_value(&self.head)?));
    Ok(())
  }

  /// https://isocpp.org/blog/2014/06/stroustrup-lists
  fn _deserialize(mut session_context: SessionContext) -> Result<Self, DeserializerError> {
    let index = session_context
      .iter()
      .rposition(|(k, _)| *k == Head::UID::U32)
      .ok_or_else(|| {
        let head_param_uid: ParamUID = Head::UID::U32;
        DeserializerError::Custom(format!("Missing key: {head_param_uid}"))
      })?;
    let (_, value) = session_context.swap_remove(index);
    let head: Head = Head::deserialize(value)?;
    let tail = Tail::_deserialize(session_context)?;
    Ok(HCons { head, tail })
  }

  fn all_param_uids(acc: &mut HashSet<ParamUID>) {
    acc.insert(Head::UID::U32);
    Tail::all_param_uids(acc)
  }
}
