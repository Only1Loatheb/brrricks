use crate::builder::{
  FlowingProcess, IntermediateRunOutcome, IntermediateRunResult, MaybeFormContext, PreviousRunYieldedAt,
  SessionContext, StepIndex,
};
use crate::frunk::hlist::HNil;
use crate::param_list::ParamList;
use crate::step::{BackToken, Entry, ProcessMessages};
use std::future::Future;

impl<Produces: ParamList, Messages: ProcessMessages> FlowingProcess for Entry<Produces, Messages> {
  type EntryConsumes = Produces;
  type ProcessBeforeProduces = HNil;
  type Produces = Produces;
  type SubprocessConsumes = HNil;
  type Messages = Messages;
  type EverProduced = Produces;

  fn resume_run(
    &self,
    previous_run_produced: SessionContext,
    _: PreviousRunYieldedAt,
    _user_input: String,
    _form_context: MaybeFormContext,
    _back_token: Option<BackToken>,
  ) -> impl Future<Output = IntermediateRunResult<Self::Produces, Self::Messages>> {
    std::future::ready(Produces::deserialize_param_list(previous_run_produced).map(IntermediateRunOutcome::Continue))
  }

  fn continue_run(
    &self,
    _: Self::ProcessBeforeProduces,
    _back_token: Option<BackToken>,
  ) -> impl Future<Output = IntermediateRunResult<Self::Produces, Self::Messages>> {
    #[allow(unreachable_code)]
    std::future::ready(unreachable!("We never continue from entry step"))
  }

  fn run_subprocess(
    &self,
    _: Self::SubprocessConsumes,
    _back_token: Option<BackToken>,
  ) -> impl Future<Output = IntermediateRunResult<Self::Produces, Self::Messages>> {
    #[allow(unreachable_code)]
    std::future::ready(unreachable!("Entry step never starts subprocess"))
  }

  fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex {
    last_used_index
  }
}
