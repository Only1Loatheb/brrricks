use crate::builder::flowing_process::sealed;
use crate::builder::{
  IntermediateRunOutcome, IntermediateRunResult, MaybeFormContext, ParamList, PreviousRunYieldedAt, ProcessError,
  SessionContext, StepIndex,
};
use crate::frunk::coproduct::CNil;
use crate::step::BackToken;
use crate::step::ProcessMessages;
use std::marker::PhantomData;

// maybe can use dependent types and a trait, but is okay for now
pub struct CaseEntry<EntryConsumes, ProcessBeforeProduces, EverProduced, Messages> {
  pub(crate) phantom_data: PhantomData<(EntryConsumes, ProcessBeforeProduces, EverProduced, Messages)>,
}

impl<EntryConsumes: ParamList, ProcessBeforeProduces: ParamList, EverProduced: ParamList, Messages: ProcessMessages>
  sealed::FlowingProcess for CaseEntry<EntryConsumes, ProcessBeforeProduces, EverProduced, Messages>
{
  type EntryConsumes = EntryConsumes;
  type ProcessBeforeProduces = ProcessBeforeProduces;
  type Produces = ProcessBeforeProduces;
  type SubprocessConsumes = ProcessBeforeProduces;
  type Messages = Messages;
  type EverProduced = EverProduced; // Includes params produced specific for this case in split
  type Errors = CNil;

  fn resume_run(
    &self,
    _previous_run_produced: SessionContext,
    previous_run_yielded_at: PreviousRunYieldedAt,
    _user_input: String,
    _form_context: MaybeFormContext,
    _back_token: Option<BackToken>,
  ) -> impl Future<Output = IntermediateRunResult<Self::Produces, Self::Messages, Self::Errors>> {
    std::future::ready(Err(ProcessError::invalid_step_index(previous_run_yielded_at.0)))
  }

  fn continue_run(
    &self,
    process_before_produces: Self::ProcessBeforeProduces,
    _back_token: Option<BackToken>,
  ) -> impl Future<Output = IntermediateRunResult<Self::Produces, Self::Messages, Self::Errors>> {
    std::future::ready(Ok(IntermediateRunOutcome::Continue(process_before_produces)))
  }

  async fn run_subprocess(
    &self,
    subprocess_consumes: Self::SubprocessConsumes,
    back_token: Option<BackToken>,
  ) -> IntermediateRunResult<Self::Produces, Self::Messages, Self::Errors> {
    self.continue_run(subprocess_consumes, back_token).await
  }

  fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex {
    last_used_index
  }
}

#[must_use]
pub(crate) fn case_entry<
  EntryConsumes: ParamList,
  ProcessBeforeProduces: ParamList,
  EverProduced: ParamList,
  Messages: ProcessMessages,
>() -> CaseEntry<EntryConsumes, ProcessBeforeProduces, EverProduced, Messages> {
  CaseEntry { phantom_data: Default::default() }
}
