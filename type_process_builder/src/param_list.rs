use crate::frunk::hlist::{HCons, HList, HNil};
use crate::param_list::contains::Contains;
use serde::Serialize;
use serde::de::DeserializeOwned;
use typenum::{B0, Unsigned};

pub mod borrow_just;
pub mod concat;
pub mod contains;
pub mod extract;
pub mod intersect;
pub mod union;

pub type SessionContext = Vec<u8>;

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

pub trait ParamList: HList + Serialize + DeserializeOwned + Send + Sync {
  fn serialize_param_list(&self) -> anyhow::Result<SessionContext> {
    Ok(postcard::to_allocvec(self)?)
  }

  fn deserialize_param_list(session_context: SessionContext) -> anyhow::Result<Self> {
    Ok(postcard::from_bytes(&session_context)?)
  }
}

impl ParamList for HNil {}

#[diagnostic::on_unimplemented(
  message = "cannot include parameter: duplicate parameter UID for `{Param}` found in ParamList",
  note = "Parameter `{Param}` appears multiple times in the parameter list, or multiple parameters share the same UID"
)]
pub trait PreventDuplicateParamUidInParamList<Param> {}

impl<Param> PreventDuplicateParamUidInParamList<Param> for B0 {}

impl<Head: ParamValue, Tail: ParamList + Contains<Head>> ParamList for HCons<Head, Tail> where
  <Tail as Contains<Head>>::IsContained: PreventDuplicateParamUidInParamList<Head>
{
}
