use crate::builder::*;
use crate::param_list::ParamList;
use crate::step::{Entry, FailedInputValidationAttempts};
use frunk_core::hlist::HNil;

impl<EntryStep: Entry> FlowingProcess for EntryStep {
  type ProcessBeforeProduces = HNil;
  type Produces = EntryStep::Produces;
  type SubprocessConsumes = HNil;

  async fn resume_run(
    &self,
    previous_run_produced: SessionContext,
    _: PreviousRunYieldedAt,
    user_input: String,
    _failed_input_validation_attempts: FailedInputValidationAttempts,
  ) -> IntermediateRunResult<Self::Produces> {
    let result: EntryStep::Produces = EntryStep::handle(self, previous_run_produced, user_input).await?;
    Ok(IntermediateRunOutcome::Continue(result))
  }

  #[cfg_attr(coverage_nightly, coverage(off))]
  async fn continue_run(&self, _: Self::ProcessBeforeProduces) -> IntermediateRunResult<Self::Produces> {
    unreachable!("We never continue from entry step")
  }

  #[cfg_attr(coverage_nightly, coverage(off))]
  async fn run_subprocess(&self, _: Self::SubprocessConsumes) -> IntermediateRunResult<Self::Produces> {
    unreachable!("Entry step never starts subprocess")
  }

  fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex {
    last_used_index
  }

  fn all_param_uids(&self, acc: &mut Vec<ParamUID>) {
    EntryStep::Produces::all_param_uids(acc);
  }
}
