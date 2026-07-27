use crate::builder::{
  FlowingProcess, IntermediateRunOutcome, IntermediateRunResult, MaybeFormContext, ParamUID, PreviousRunYieldedAt,
  SessionContext, StepIndex,
};
use crate::frunk::hlist::HNil;
use crate::param_list::ParamList;
use crate::step::Entry;

impl<EntryStep: Entry> FlowingProcess for EntryStep {
  type ProcessBeforeProduces = HNil;
  type Produces = EntryStep::Produces;
  type SubprocessConsumes = HNil;
  type Messages = EntryStep::Messages;
  type EverProduced = EntryStep::Produces;

  async fn resume_run(
    &self,
    previous_run_produced: SessionContext,
    _: PreviousRunYieldedAt,
    user_input: String,
    _form_context: MaybeFormContext,
    _back_navigation_available: bool,
  ) -> IntermediateRunResult<Self::Produces, Self::Messages> {
    let result: EntryStep::Produces = EntryStep::handle(self, previous_run_produced, user_input).await?;
    Ok(IntermediateRunOutcome::Continue(result))
  }

  async fn continue_run(
    &self,
    _: Self::ProcessBeforeProduces,
    _back_navigation_available: bool,
  ) -> IntermediateRunResult<Self::Produces, Self::Messages> {
    unreachable!("We never continue from entry step")
  }

  async fn run_subprocess(
    &self,
    _: Self::SubprocessConsumes,
    _back_navigation_available: bool,
  ) -> IntermediateRunResult<Self::Produces, Self::Messages> {
    unreachable!("Entry step never starts subprocess")
  }


  fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex {
    last_used_index
  }

  fn all_param_uids(&self, acc: &mut Vec<ParamUID>) {
    EntryStep::Produces::all_param_uids(acc);
  }
}
