pub mod builder;
mod hlist_concat;
mod hlist_intersect;
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
  use frunk_core::hlist::{HCons, HNil};
  use frunk_core::{hlist, Coprod, HList};
  use serde::{Deserialize, Serialize};
  use serde_value::Value;
  use std::collections::BTreeMap;
  use typenum::*;

  #[derive(Clone, Deserialize, Serialize)]
  struct Param0;
  impl ParamValue for Param0 {
    type UID = U0;
  }

  #[derive(Clone, Deserialize, Serialize)]
  struct Param1;
  impl ParamValue for Param1 {
    type UID = U1;
  }

  #[derive(Clone, Deserialize, Serialize)]
  struct Param2;
  impl ParamValue for Param2 {
    type UID = U2;
  }

  #[derive(Clone, Deserialize, Serialize)]
  struct Param3;
  impl ParamValue for Param3 {
    type UID = U3;
  }

  struct EntryA;
  impl Entry<Value> for EntryA {
    type Produces = HCons<Param0, HNil>;

    async fn handle(
      &self,
      mut consumes: BTreeMap<Value, Value>,
      shortcode_string: String,
    ) -> anyhow::Result<HCons<Param0, HNil>> {
      let key = Value::String("msisdn".into());
      let value = consumes
        .remove(&key)
        .ok_or_else(|| anyhow!("Admin error or error on frontend."))?;
      Ok(hlist!(value.deserialize_into()?))
    }
  }

  struct Linear1;
  impl Operation<HNil, HList![Param1, Param3]> for Linear1 {
    async fn handle(&self, consumes: HNil) -> anyhow::Result<HList![Param1, Param3]> {
      todo!()
    }
  }

  struct Linear2;
  impl Operation<HNil, HList![Param2, Param3]> for Linear2 {
    async fn handle(&self, consumes: HNil) -> anyhow::Result<HList![Param2, Param3]> {
      todo!()
    }
  }

  pub struct Case1;
  pub struct Case2;
  struct SplitA;
  impl Splitter<HNil, Coprod![(Case1, HNil), (Case2, HNil)]> for SplitA {
    async fn handle(&self, consumes: HNil) -> anyhow::Result<Coprod![(Case1, HNil), (Case2, HNil)]> {
      todo!()
    }
  }

  struct FinalA;
  impl Final<HCons<Param0, HNil>> for FinalA {
    async fn handle(&self, consumes: HCons<Param0, HNil>) -> anyhow::Result<Message> {
      todo!()
    }
  }
  // impl Linear<HCons<Param1, HNil>, HNil> for LinearB { async fn handle(&self, consumes: HCons<Param1, HNil>) -> anyhow::Result<(Option<Message>, HNil)> { todo!() } }

  #[tokio::test]
  async fn test_hcons() {
    let process = EntryA
      .split(SplitA)
      .case_flowing::<Case1, _>(|x| x.then(Linear1))
      .case_flowing::<Case2, _, _>(|x| x.then(Linear2))
      .end(FinalA);
    //   .then(FinalA)
    //   .build();
    // let run_result = process
    //   .continue_run(
    //     Value::Map(BTreeMap::new()),
    //     PreviousRunYieldedAt(0),
    //     "*123#".to_string(),
    //   )
    //   .await;
    // assert!(run_result.is_err());
  }
}
