use crate::frunk::hlist::{HCons, HList, HNil};
use crate::param_list::contains::Contains;
use anyhow::anyhow;
use serde::Serialize;
use serde::de::DeserializeOwned;
use typenum::{B0, Unsigned};

pub mod borrow_just;
pub mod concat;
pub mod contains;
pub mod extract;
pub mod intersect;
pub mod union;

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
/// ```compile_fail,E0119
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

  fn deserialize_from(session_context: SessionContext) -> anyhow::Result<Self>;
}

impl ParamList for HNil {
  fn serialize_into(&self, _: &mut SessionContext) -> anyhow::Result<()> {
    Ok(())
  }

  fn deserialize_from(_session_context: SessionContext) -> anyhow::Result<Self> {
    Ok(HNil)
  }
}

#[diagnostic::on_unimplemented(
  message = "cannot include parameter: duplicate parameter UID for `{Param}` found in ParamList",
  note = "Parameter `{Param}` appears multiple times in the parameter list, or multiple parameters share the same UID"
)]
pub trait PreventDuplicateParamUidInParamList<Param> {}

impl<Param> PreventDuplicateParamUidInParamList<Param> for B0 {}

/// The `where` clause prevents the same [`ParamValue`] from being duplicated in a [`ParamList`].
/// Because uniqueness is checked by `UID`, this also guarantees that two different [`ParamValue`] types cannot share the same `UID` within the list.
impl<Head: ParamValue, Tail: ParamList + Contains<Head>> ParamList for HCons<Head, Tail>
where
  <Tail as Contains<Head>>::IsContained: PreventDuplicateParamUidInParamList<Head>,
{
  fn serialize_into(&self, session_context: &mut SessionContext) -> anyhow::Result<()> {
    self.tail.serialize_into(session_context)?;
    session_context.push((Head::UID::U32, postcard::to_allocvec(&self.head)?));
    Ok(())
  }

  /// <https://isocpp.org/blog/2014/06/stroustrup-lists>
  /// Deserializes a [`ParamList`] from a [`SessionContext`].
  /// Deserializing out-of-order or from a subset `ParamList` is supported, but can degrade parameter lookup
  /// to linear scan ($O(N^2)$ worst-case for [`ParamList::deserialize`] invocation).
  /// We use [`Vec::swap_remove`] because it acts as $O(1)$ [`Vec::pop`] with zero or one element swap and without ever
  /// reallocating the vector.
  /// To keep efficiency gains from using `swap_remove`, we need to serialize [`SessionContext`] in reversed order and
  /// search from the back with [`Iterator::rposition`].
  /// Head-first order is avoided because `swap_remove(0)` would move the last element to index 0 and that would
  /// make us check elements that are unlikely to be used at the beginning of every search.
  /// When deserializing in matching order, `rposition` finds [`Head`] on the first check ($O(1)$) and `swap_remove`
  /// pops from the back with zero element moves, operating as an efficient LIFO stack ($O(N)$ overall).
  fn deserialize_from(mut session_context: SessionContext) -> anyhow::Result<Self> {
    let index = session_context.iter().rposition(|(k, _)| *k == Head::UID::U32).ok_or_else(|| {
      let type_name = std::any::type_name::<Head>();
      let uid: ParamUID = Head::UID::U32;
      anyhow!("Parameter {type_name} with UID {uid} is missing from SessionContext")
    })?;
    let (_, value) = session_context.swap_remove(index);
    let head: Head = postcard::from_bytes(&value)?;
    let tail = Tail::deserialize_from(session_context)?;
    Ok(HCons { head, tail })
  }
}
