use serde::{Deserialize, Serialize};
use std::ops::Not;
use type_process_builder::param_list::ParamValue;
use typenum::U0;

#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Msisdn(pub u64);
impl Msisdn {
  pub fn from_string(string: &str) -> Option<Msisdn> {
    string
      .split_at_checked(string.len().checked_sub(10)?)
      .and_then(|(_prefix, suffix)| {
        // deny optional '+' https://doc.rust-lang.org/std/primitive.u64.html#method.from_str
        let _: () = suffix.starts_with('+').not().then_some(())?;
        suffix.parse::<u64>().ok()
      })
      .map(Msisdn)
  }
}

#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Operator {
  mtn,
  airtel,
  glo,
  etisalat,
}

#[derive(PartialEq, Debug, Eq, Clone, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ShortcodeString(pub String);

#[derive(PartialEq, Debug, Eq, Clone, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DialedSessionEntryParam(pub Msisdn, pub Operator, pub ShortcodeString);
impl ParamValue for DialedSessionEntryParam {
  type UID = U0;
}
