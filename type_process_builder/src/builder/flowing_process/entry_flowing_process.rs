use crate::builder::flowing_process::sealed;
use crate::builder::{
  IntermediateRunOutcome, IntermediateRunResult, LibraryErrors, PreviousRunYieldedAt, ProcessError, StepIndex,
};
use crate::frunk::hlist::HNil;
use crate::param_list::ParamList;
use crate::step::{BackToken, Entry, ProcessMessages};
use std::future::Future;

impl<Produces: ParamList, Messages: ProcessMessages> sealed::FlowingProcess for Entry<Produces, Messages> {
  type EntryConsumes = Produces;
  type ProcessBeforeProduces = HNil;
  type Produces = Produces;
  type Messages = Messages;
  type EverProduced = Produces;
  type Errors = LibraryErrors;

  fn resume_run(
    &self,
    previous_run_produced: &[u8],
    previous_run_yielded_at: PreviousRunYieldedAt,
    _user_input: &str,
    _form_context: Option<&[u8]>,
    _back_token: Option<BackToken>,
  ) -> impl Future<Output = IntermediateRunResult<Self::Produces, Self::Messages, Self::Errors>> {
    let result = if previous_run_yielded_at.0 == StepIndex::MIN {
      Produces::deserialize_param_list(previous_run_produced)
        .map(IntermediateRunOutcome::Continue)
        .map_err(ProcessError::from_serialization)
    } else {
      Err(ProcessError::invalid_step_index(previous_run_yielded_at.0))
    };
    std::future::ready(result)
  }

  fn continue_run(
    &self,
    _: Self::ProcessBeforeProduces,
    _back_token: Option<BackToken>,
  ) -> impl Future<Output = IntermediateRunResult<Self::Produces, Self::Messages, Self::Errors>> {
    #[allow(unreachable_code)]
    std::future::ready(unreachable!("We never continue from entry step"))
  }

  fn run_from_entry(
    &self,
    entry_consumes: Self::EntryConsumes,
    _back_token: Option<BackToken>,
  ) -> impl Future<Output = IntermediateRunResult<Self::Produces, Self::Messages, Self::Errors>> {
    std::future::ready(Ok(IntermediateRunOutcome::Continue(entry_consumes)))
  }

  fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex {
    last_used_index
  }
}
