use crate::frunk::hlist::{HCons, HList, HNil};
use crate::param_list::intersect::Contains;
use anyhow::anyhow;
use serde::Serialize;
use serde::de::DeserializeOwned;
use typenum::{B0, Same, Unsigned};

pub mod borrow_just;
pub mod concat;
pub mod intersect;
pub mod transform;

pub type ParamUID = u32;

pub type SessionContext = Vec<(ParamUID, Vec<u8>)>;

/// Use [`typenum::op`] to generate UID if the desired typenum const is missing.
pub trait ParamValue: Serialize + DeserializeOwned + Send + Sync {
  type UID: Unsigned;
}

/// Macro to implement [`ParamValue`] for multiple types intended for use in a single process.
///
/// Ensures that no duplicate [`Unsigned`] are passed within one invocation at compile time.
///
/// # Usage:
/// ```
/// use type_process_builder::impl_param_value;
/// use serde::{Serialize, Deserialize};
/// use typenum::{U0, U1, U2};
///
/// #[derive(Serialize, Deserialize)]
/// struct ShortcodeString;
/// #[derive(Serialize, Deserialize)]
/// struct EntryParam;
/// #[derive(Serialize, Deserialize)]
/// struct Split1Param;
///
/// impl_param_value!(ShortcodeString, U0);
///
/// impl_param_value! {
///   EntryParam => U1,
///   Split1Param => U2,
/// }
/// ```
///
/// Passing duplicate typenum UIDs within the same invocation results in a compile-time error:
/// ```compile_fail
/// use type_process_builder::impl_param_value;
/// use serde::{Serialize, Deserialize};
/// use typenum::U0;
///
/// #[derive(Serialize, Deserialize)]
/// struct ParamA;
/// #[derive(Serialize, Deserialize)]
/// struct ParamB;
///
/// impl_param_value! {
///   ParamA => U0,
///   ParamB => U0, // conflicting implementations of trait `DuplicateParamUidInMacroInvocation` for type `UTerm`
/// }
/// ```
#[macro_export]
macro_rules! impl_param_value {
  ($type:ty, $uid:ty) => {
    impl $crate::param_list::ParamValue for $type {
      type UID = $uid;
    }
  };
  ($($type:ty => $uid:ty),* $(,)?) => {
    $(
      $crate::impl_param_value!($type, $uid);
    )*
    const _: () = {
      trait DuplicateParamUidInMacroInvocation {}
      $(
        impl DuplicateParamUidInMacroInvocation for $uid {}
      )*
    };
  };
}

pub trait ParamList: HList + Send + Sync {
  // https://serde.rs/impl-serialize.html#serializing-a-sequence-or-map
  fn serialize(&self) -> anyhow::Result<SessionContext> {
    let mut session_context = Vec::with_capacity(Self::LEN);
    self.serialize_into(&mut session_context)?;
    Ok(session_context)
  }
  fn serialize_into(&self, serialize_map: &mut SessionContext) -> anyhow::Result<()>;

  // https://serde.rs/deserialize-map.html
  // todo: We should only deserialize values required in further part of the process up to the next interaction, but I don't know what they are.
  fn deserialize(session_context: SessionContext) -> anyhow::Result<Self> {
    Self::deserialize_from(session_context)
  }
  /// [`crate::builder::RunnableProcess::ordered_all_unique_param_uids`]
  fn deserialize_from(session_context: SessionContext) -> anyhow::Result<Self>;

  fn all_param_uids(acc: &mut Vec<ParamUID>);
}

impl ParamList for HNil {
  fn serialize_into(&self, _: &mut SessionContext) -> anyhow::Result<()> {
    Ok(())
  }

  fn deserialize_from(_session_context: SessionContext) -> anyhow::Result<Self> {
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
  fn serialize_into(&self, session_context: &mut SessionContext) -> anyhow::Result<()> {
    self.tail.serialize_into(session_context)?;
    session_context.push((Head::UID::U32, postcard::to_allocvec(&self.head)?));
    Ok(())
  }

  /// <https://isocpp.org/blog/2014/06/stroustrup-lists>
  fn deserialize_from(mut session_context: SessionContext) -> anyhow::Result<Self> {
    let index = session_context.iter().rposition(|(k, _)| *k == Head::UID::U32).ok_or({
      let head_param_uid: ParamUID = Head::UID::U32;
      anyhow!("Missing key: {head_param_uid}")
    })?;
    let (_, value) = session_context.swap_remove(index);
    let head: Head = postcard::from_bytes(&value)?;
    let tail = Tail::deserialize_from(session_context)?;
    Ok(HCons { head, tail })
  }

  fn all_param_uids(acc: &mut Vec<ParamUID>) {
    acc.push(Head::UID::U32);
    Tail::all_param_uids(acc);
  }
}
