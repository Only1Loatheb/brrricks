pub mod builder;
mod hlist_concat;
mod hlist_intersect;
mod hlist_transform_to;
mod param_list;
pub mod step;

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
  use frunk_core::hlist::HNil;
  use frunk_core::{hlist, Coprod, HList};
  use serde::{Deserialize, Serialize};
  use serde_value::Value;
  use std::collections::BTreeMap;
  use typenum::*;

  #[derive(Clone, Deserialize, Serialize)]
  struct EntryParam;
  impl ParamValue for EntryParam {
    type UID = U0;
  }

  #[derive(Clone, Deserialize, Serialize)]
  struct Split1Param;
  impl ParamValue for Split1Param {
    type UID = U1;
  }

  #[derive(Clone, Deserialize, Serialize)]
  struct Split2Param;
  impl ParamValue for Split2Param {
    type UID = U2;
  }

  #[derive(Clone, Deserialize, Serialize)]
  struct CommonSplitParam;
  impl ParamValue for CommonSplitParam {
    type UID = U3;
  }

  #[derive(Clone, Deserialize, Serialize)]
  struct Case1Param;
  impl ParamValue for Case1Param {
    type UID = U4;
  }

  #[derive(Clone, Deserialize, Serialize)]
  struct Case2Param;
  impl ParamValue for Case2Param {
    type UID = U5;
  }

  #[derive(Clone, Deserialize, Serialize)]
  struct CommonCaseParam;
  impl ParamValue for CommonCaseParam {
    type UID = U6;
  }

  struct EntryA;
  impl Entry<Value> for EntryA {
    type Produces = HList![EntryParam];

    async fn handle(
      &self,
      mut consumes: BTreeMap<Value, Value>,
      shortcode_string: String,
    ) -> anyhow::Result<HList![EntryParam]> {
      let key = Value::String("msisdn".into());
      let value = consumes
        .remove(&key)
        .ok_or_else(|| anyhow!("Admin error or error on frontend."))?;
      Ok(hlist!(value.deserialize_into()?))
    }
  }

  struct Linear1;
  impl Operation for Linear1 {
    type Consumes = HNil;
    type Produces = HList![Case1Param, CommonCaseParam];

    async fn handle(&self, consumes: Self::Consumes) -> anyhow::Result<Self::Produces> {
      todo!()
    }
  }

  struct Linear2;
  impl Operation for Linear2 {
    type Consumes = HNil;
    type Produces = HList![Case2Param, CommonCaseParam];

    async fn handle(&self, consumes: Self::Consumes) -> anyhow::Result<Self::Produces> {
      todo!()
    }
  }

  pub struct Case1;
  pub struct Case2;
  struct SplitA;
  impl Splitter for SplitA {
    type Consumes = HNil;
    type Produces = Coprod![
      (Case1, HList![Split1Param, CommonSplitParam]),
      (Case2, HList![Split2Param, CommonSplitParam])
    ];

    async fn handle(&self, consumes: Self::Consumes) -> anyhow::Result<Self::Produces> {
      todo!()
    }
  }

  struct FinalA;
  impl Final for FinalA {
    type Consumes = HList![EntryParam, CommonSplitParam, CommonCaseParam];

    async fn handle(&self, consumes: Self::Consumes) -> anyhow::Result<Message> {
      todo!()
    }
  }

  #[tokio::test]
  async fn test_hcons() {
    let process = EntryA
      .split(SplitA)
      .case_via(Case1, |x| x.then(Linear1))
      .case_via(Case2, |x| x.then(Linear2))
      .end(FinalA);
    //   .then(FinalA)
    //   .build();
    // let run_result = process
    //   .resume_run(
    //     Value::Map(BTreeMap::new()),
    //     PreviousRunYieldedAt(0),
    //     "*123#".to_string(),
    //   )
    //   .await;
    // assert!(run_result.is_err());
  }
}
