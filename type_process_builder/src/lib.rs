pub mod builder;
mod hlist_concat;
mod hlist_transform_to;
mod param_list;
pub mod step;
mod type_eq;

// cargo doc --no-deps --package type_process_builder --features docs
/// A sequence diagram
#[cfg(feature = "docs")]
#[doc = simple_mermaid::mermaid!("process_builder_diagram.mermaid")]
#[cfg_attr(not(feature = "docs"), doc = "")]
pub mod a {}

#[cfg(test)]
mod tests {
  use crate::builder::*;
  use crate::param_list::ParamValue;
  use crate::step::step::{Entry, Final, Operation, Splitter};
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

    async fn handle(
      &self,
      mut consumes: BTreeMap<Value, Value>,
      shortcode_string: String,
    ) -> anyhow::Result<HCons<Param1, HNil>> {
      let key = Value::String("msisdn".into());
      let value = consumes
        .remove(&key)
        .ok_or_else(|| anyhow!("Admin error or error on frontend."))?;
      Ok(hlist!(value.deserialize_into()?))
    }
  }

  struct LinearA;
  impl Operation<HNil, HNil> for LinearA {
    async fn handle(&self, consumes: HNil) -> anyhow::Result<HNil> {
      todo!()
    }
  }
  // impl Linear<HNil, HCons<Param1, HNil>> for LinearA {
  //   async fn handle(&self, consumes: HNil) -> anyhow::Result<(Option<Message>, HCons<Param1, HNil>)> {
  //     todo!()
  //   }
  // }

  struct LinearB;
  impl Operation<HNil, HNil> for LinearB {
    async fn handle(&self, consumes: HNil) -> anyhow::Result<HNil> {
      todo!()
    }
  }
  // impl Linear<HCons<Param1, HNil>, HNil> for LinearB { async fn handle(&self, consumes: HCons<Param1, HNil>) -> anyhow::Result<(Option<Message>, HNil)> { todo!() } }

  pub struct Case1;
  pub struct Case2;
  pub struct Case3;
  struct SplitA;
  impl Splitter<HNil, Coproduct<(Case1, HNil), Coproduct<(Case2, HNil), Coproduct<(Case3, HNil), CNil>>>> for SplitA {
    async fn handle(
      &self,
      consumes: HNil,
    ) -> anyhow::Result<Coproduct<(Case1, HNil), Coproduct<(Case2, HNil), Coproduct<(Case3, HNil), CNil>>>> {
      todo!()
    }
  }

  struct FinalA;
  impl Final<HCons<Param1, HNil>> for FinalA {
    async fn handle(&self, consumes: HCons<Param1, HNil>) -> anyhow::Result<Message> {
      todo!()
    }
  }
  // impl Linear<HCons<Param1, HNil>, HNil> for LinearB { async fn handle(&self, consumes: HCons<Param1, HNil>) -> anyhow::Result<(Option<Message>, HNil)> { todo!() } }

  #[tokio::test]
  async fn test_hcons() {
    let process = EntryA
      .then(LinearA)
      .then(LinearB)
      .split(SplitA)
      .case::<Case1, _, _>(|x| x.end(FinalA))
      .case::<Case2, _, _>(|x| x.end(FinalA))
      .case::<Case3, _, _>(|x| x.end(FinalA))
      .build();
    let run_result = process
      .continue_run(
        Value::Map(BTreeMap::new()),
        PreviousRunYieldedAt(0),
        "*123#".to_string(),
      )
      .await;
    assert!(run_result.is_err());
  }
}
