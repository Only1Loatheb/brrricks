use anyhow::anyhow;
use frunk_core::{HList, hlist};
use serde::{Deserialize, Serialize};
use serde_value::Value;
use std::collections::HashMap;
use std::ops::Not;
use type_process_builder::param_list::ParamValue;
use type_process_builder::step::Entry;
use typenum::U0;

#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Msisdn(u64);
impl Msisdn {
  pub fn from_string(string: String) -> Option<Msisdn> {
    string
      .split_at_checked(string.len().checked_sub(10)?)
      .and_then(|(_prefix, suffix)| {
        // deny optional '+' https://doc.rust-lang.org/std/primitive.u64.html#method.from_str
        let _: () = suffix.starts_with('+').not().then_some(())?;
        suffix.parse::<u64>().ok()
      })
      .map(|x| Msisdn(x))
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
pub struct ShortcodeString(String);

#[derive(PartialEq, Debug, Eq, Clone, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DialedSessionEntryParam(Msisdn, Operator, ShortcodeString);
impl ParamValue for DialedSessionEntryParam {
  type UID = U0;
}

pub struct DialedSessionEntry;
impl Entry<Value> for DialedSessionEntry {
  type Produces = HList![DialedSessionEntryParam];

  async fn handle(
    &self,
    mut consumes: HashMap<u64, Value>,
    shortcode_string: String,
  ) -> anyhow::Result<Self::Produces> {
    let msisdn_value = consumes
      .remove(&0)
      .ok_or_else(|| anyhow!("Admin error or error on frontend."))?;
    let msisdn = match msisdn_value {
      Value::String(string) => Msisdn::from_string(string).ok_or_else(|| anyhow!("Admin error on frontend.")),
      _ => Err(anyhow!("Admin error on frontend.")),
    }?;
    let operator = consumes
      .remove(&1)
      .ok_or_else(|| anyhow!("Admin error or error on frontend."))?;
    Ok(hlist!(DialedSessionEntryParam(
      msisdn,
      Operator::deserialize(operator)?,
      ShortcodeString(shortcode_string)
    )))
  }
}
