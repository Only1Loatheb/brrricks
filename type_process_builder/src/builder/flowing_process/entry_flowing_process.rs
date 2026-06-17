use crate::builder::*;
use crate::param_list::ParamList;
use crate::step::Entry;
use frunk_core::hlist::HNil;

impl<EntryStep: Entry> FlowingProcess for EntryStep {
  type ProcessBeforeProduces = HNil;
  type Produces = EntryStep::Produces;
  type SubprocessConsumes = HNil;
  type Messages = EntryStep::Messages;

  async fn resume_run(
    &self,
    previous_run_produced: SessionContext,
    _: PreviousRunYieldedAt,
    user_input: String,
    _form_context: RawFormContext,
  ) -> IntermediateRunResult<Self::Produces, Self::Messages> {
    let result: EntryStep::Produces = EntryStep::handle(self, previous_run_produced, user_input).await?;
    Ok(IntermediateRunOutcome::Continue(result))
  }

  async fn continue_run(
    &self,
    _: Self::ProcessBeforeProduces,
  ) -> IntermediateRunResult<Self::Produces, Self::Messages> {
    unreachable!("We never continue from entry step")
  }

  async fn run_subprocess(&self, _: Self::SubprocessConsumes) -> IntermediateRunResult<Self::Produces, Self::Messages> {
    unreachable!("Entry step never starts subprocess")
  }

  fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex {
    last_used_index
  }

  fn all_param_uids(&self, acc: &mut Vec<ParamUID>) {
    EntryStep::Produces::all_param_uids(acc);
  }
}
