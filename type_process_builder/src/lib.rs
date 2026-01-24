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
  impl Operation<HNil, HList![Case1Param, CommonCaseParam]> for Linear1 {
    async fn handle(&self, consumes: HNil) -> anyhow::Result<HList![Case1Param, CommonCaseParam]> {
      todo!()
    }
  }

  struct Linear2;
  impl Operation<HNil, HList![Case2Param, CommonCaseParam]> for Linear2 {
    async fn handle(&self, consumes: HNil) -> anyhow::Result<HList![Case2Param, CommonCaseParam]> {
      todo!()
    }
  }

  pub struct Case1;
  pub struct Case2;
  struct SplitA;
  impl
    Splitter<
      HNil,
      Coprod![
        (Case1, HList![Split1Param, CommonSplitParam]),
        (Case2, HList![Split2Param, CommonSplitParam])
      ],
    > for SplitA
  {
    async fn handle(
      &self,
      consumes: HNil,
    ) -> anyhow::Result<
      Coprod![
        (Case1, HList![Split1Param, CommonSplitParam]),
        (Case2, HList![Split2Param, CommonSplitParam])
      ],
    > {
      todo!()
    }
  }

  struct FinalA;
  impl Final<HList![EntryParam, CommonSplitParam, CommonCaseParam]> for FinalA {
    async fn handle(&self, consumes: HList![EntryParam, CommonSplitParam, CommonCaseParam]) -> anyhow::Result<Message> {
      todo!()
    }
  }

  #[tokio::test]
  async fn test_hcons() {
    let process = EntryA
      .split(SplitA)
      .case_flowing(Case1, |x| x.then(Linear1))
      .case_flowing(Case2, |x| x.then(Linear2))
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
