pub mod builder;
mod hlist_concat;
mod hlist_intersect;
mod hlist_transform_to;
mod param_list;
pub mod step;

#[cfg(test)]
mod tests {
  use crate::builder::*;
  use crate::param_list::ParamValue;
  use crate::step::step::{Entry, Final, Linear, Splitter};
  use crate::step::Message;
  use anyhow::anyhow;
  use frunk_core::coproduct::{CNil, Coproduct};
  use frunk_core::hlist;
  use frunk_core::hlist::{HCons, HNil};
  use serde_value::Value;
  use std::collections::BTreeMap;

  #[derive(Clone, serde::Deserialize, serde::Serialize)]
  struct Param1;

  impl ParamValue for Param1 {
    const NAME: &'static str = "Param1";
  }

  struct EntryA;
  impl Entry<Value> for EntryA {
    type Produces = HCons<Param1, HNil>;

    async fn handle(&self, mut input: BTreeMap<Value, Value>) -> anyhow::Result<HCons<Param1, HNil>> {
      let key = Value::String("msisdn".into());
      let value = input
        .remove(&key)
        .ok_or_else(|| anyhow!("Admin error or error on frontend."))?;
      Ok(hlist!(value.deserialize_into()?))
    }
  }

  struct LinearA;
  impl Linear<HNil, HNil> for LinearA {
    async fn handle(&self, input: HNil) -> anyhow::Result<HNil> {
      todo!()
    }
  }
  // impl Linear<HNil, HCons<Param1, HNil>> for LinearA {
  //   async fn handle(&self, input: HNil) -> anyhow::Result<(Option<Message>, HCons<Param1, HNil>)> {
  //     todo!()
  //   }
  // }

  struct LinearB;
  impl Linear<HNil, HNil> for LinearB {
    async fn handle(&self, input: HNil) -> anyhow::Result<HNil> {
      todo!()
    }
  }
  // impl Linear<HCons<Param1, HNil>, HNil> for LinearB { async fn handle(&self, input: HCons<Param1, HNil>) -> anyhow::Result<(Option<Message>, HNil)> { todo!() } }

  struct SplitA;
  impl Splitter<HNil, Coproduct<HNil, Coproduct<HNil, Coproduct<HNil, CNil>>>> for SplitA {
    async fn handle(&self, input: HNil) -> anyhow::Result<Coproduct<HNil, Coproduct<HNil, Coproduct<HNil, CNil>>>> {
      todo!()
    }
  }

  struct FinalA;
  impl Final<HCons<Param1, HNil>> for FinalA {
    async fn handle(&self, input: HCons<Param1, HNil>) -> anyhow::Result<Message> {
      todo!()
    }
  }
  // impl Linear<HCons<Param1, HNil>, HNil> for LinearB { async fn handle(&self, input: HCons<Param1, HNil>) -> anyhow::Result<(Option<Message>, HNil)> { todo!() } }

  #[tokio::test]
  async fn test_hcons() {
    let process = EntryA
      .then(LinearA)
      .then(LinearB)
      .split(SplitA)
      .case(|x| x.end(FinalA))
      .case(|x| x.end(FinalA))
      .case(|x| x.end(FinalA))
      .build();
    let run_result = process.run(Value::Map(BTreeMap::new())).await;
    assert!(run_result.is_err());
  }
}
