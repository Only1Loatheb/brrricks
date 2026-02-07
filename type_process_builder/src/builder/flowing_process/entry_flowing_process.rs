use crate::builder::*;
use crate::param_list::ParamList;
use crate::step::Entry;
use anyhow::anyhow;
use frunk_core::hlist::HNil;
use serde_value::Value;
use std::hint::unreachable_unchecked;

impl<Produces: ParamList, EntryStep: Entry<Value, Produces = Produces>> FlowingProcess for EntryStep {
  type ProcessBeforeProduces = HNil;
  type Produces = EntryStep::Produces;

  async fn resume_run(
    &self,
    previous_run_produced: Value,
    _: PreviousRunYieldedAt,
    user_input: String,
  ) -> IntermediateRunResult<Self::Produces> {
    let map = match previous_run_produced {
      Value::Map(m) => m,
      _ => return Err(anyhow!("Not a map")),
    };
    let result: Produces = EntryStep::handle(self, map, user_input).await?;
    Ok(IntermediateRunOutcome::Continue(result))
  }

  async fn continue_run(&self, _: Self::ProcessBeforeProduces) -> IntermediateRunResult<Self::Produces> {
    unsafe { unreachable_unchecked() }
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    last_used_index
  }
}
