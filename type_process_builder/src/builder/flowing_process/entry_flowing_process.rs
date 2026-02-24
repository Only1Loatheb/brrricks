use crate::builder::*;
use crate::param_list::ParamList;
use crate::step::{Entry, FailedInputValidationAttempts};
use frunk_core::hlist::HNil;
use serde_value::Value;

impl<Produces: ParamList, EntryStep: Entry<Value, Produces = Produces>> FlowingProcess for EntryStep {
  type ProcessBeforeProduces = HNil;
  type Produces = EntryStep::Produces;

  async fn resume_run(
    &self,
    previous_run_produced: SessionContext,
    _: PreviousRunYieldedAt,
    user_input: String,
    _failed_input_validation_attempts: FailedInputValidationAttempts,
  ) -> IntermediateRunResult<Self::Produces> {
    let result: Produces = EntryStep::handle(self, previous_run_produced, user_input).await?;
    Ok(IntermediateRunOutcome::Continue(result))
  }

  async fn continue_run(&self, _: Self::ProcessBeforeProduces) -> IntermediateRunResult<Self::Produces> {
    unreachable!("We never continue run first step")
  }

  fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex {
    last_used_index
  }

  fn all_param_uids(&self, acc: &mut Vec<ParamUID>) {
    EntryStep::Produces::all_param_uids(acc);
  }
}
