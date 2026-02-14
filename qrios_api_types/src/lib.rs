use std::ops::Not;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash, Serialize, Deserialize)]
struct Msisdn(u64);
impl Msisdn {
  fn from(string: String) -> Option<Msisdn> {
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
enum Operator {
  mtn,
  airtel,
  glo,
  etisalat,
}
