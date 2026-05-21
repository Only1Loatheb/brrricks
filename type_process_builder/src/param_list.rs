use crate::builder::{ParamUID, SessionContext};
use crate::param_list::intersect::Contains;
use anyhow::anyhow;
use frunk_core::hlist::{HCons, HList, HNil};
use serde::Serialize;
use serde::de::DeserializeOwned;
use typenum::{B0, Same, Unsigned};

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
  fn serialize(&self) -> anyhow::Result<SessionContext> {
    let mut session_context = Vec::with_capacity(self.len());
    self._serialize(&mut session_context)?;
    Ok(session_context)
  }
  fn _serialize(&self, serialize_map: &mut SessionContext) -> anyhow::Result<()>;

  // https://serde.rs/deserialize-map.html
  // todo: We should only deserialize values required in further part of the process up to the next interaction, but I don't know what they are.
  fn deserialize(session_context: SessionContext) -> anyhow::Result<Self> {
    Self::_deserialize(session_context)
  }
  /// [crate::builder::RunnableProcess::ordered_all_unique_param_uids]
  fn _deserialize(session_context: SessionContext) -> anyhow::Result<Self>;

  fn all_param_uids(acc: &mut Vec<ParamUID>);
}

impl ParamList for HNil {
  fn _serialize(&self, _: &mut SessionContext) -> anyhow::Result<()> {
    Ok(())
  }

  fn _deserialize(_session_context: SessionContext) -> anyhow::Result<Self> {
    Ok(HNil)
  }

  fn all_param_uids(_acc: &mut Vec<ParamUID>) {}
}

/// The `where` clause prevents the same [`ParamValue`] from being duplicated in a [`ParamList`].
/// Because uniqueness is checked by `UID`, this also guarantees that two different [`ParamValue`] types cannot share the same `UID` within the list.
impl<Head: ParamValue, Tail: ParamList + Contains<Head>> ParamList for HCons<Head, Tail>
where
  <Tail as Contains<Head>>::IsContained: Same<B0>,
{
  fn _serialize(&self, session_context: &mut SessionContext) -> anyhow::Result<()> {
    self.tail._serialize(session_context)?;
    session_context.push((Head::UID::U32, postcard::to_allocvec(&self.head)?));
    Ok(())
  }

  /// <https://isocpp.org/blog/2014/06/stroustrup-lists>
  fn _deserialize(mut session_context: SessionContext) -> anyhow::Result<Self> {
    let index = session_context.iter().rposition(|(k, _)| *k == Head::UID::U32).ok_or_else(|| {
      let head_param_uid: ParamUID = Head::UID::U32;
      anyhow!("Missing key: {head_param_uid}")
    })?;
    let (_, value) = session_context.swap_remove(index);
    let head: Head = postcard::from_bytes(&value)?;
    let tail = Tail::_deserialize(session_context)?;
    Ok(HCons { head, tail })
  }

  fn all_param_uids(acc: &mut Vec<ParamUID>) {
    acc.push(Head::UID::U32);
    Tail::all_param_uids(acc)
  }
}
