use crate::builder::{
  FlowingProcess, IntermediateRunOutcome, IntermediateRunResult, MaybeFormContext, ParamList, PreviousRunYieldedAt,
  SessionContext, StepIndex,
};
use crate::step::BackToken;
use crate::step::ProcessMessages;
use std::marker::PhantomData;

pub struct Subprocess<ProcessBeforeProduces, Messages> {
  pub phantom_data: PhantomData<(ProcessBeforeProduces, Messages)>,
}

impl<ProcessBeforeProduces: ParamList, Messages: ProcessMessages> FlowingProcess
  for Subprocess<ProcessBeforeProduces, Messages>
{
  type ProcessBeforeProduces = ProcessBeforeProduces;
  type Produces = ProcessBeforeProduces;
  type SubprocessConsumes = ProcessBeforeProduces;
  type Messages = Messages;
  type EverProduced = ProcessBeforeProduces;

  async fn resume_run(
    &self,
    previous_run_produced: SessionContext,
    _previous_run_yielded_at: PreviousRunYieldedAt,
    _user_input: String,
    _form_context: MaybeFormContext,
    back_token: Option<BackToken>,
  ) -> IntermediateRunResult<Self::Produces, Self::Messages> {
    let process_before_produces = ProcessBeforeProduces::deserialize(previous_run_produced)?;
    self.continue_run(process_before_produces, back_token).await
  }

  async fn continue_run(
    &self,
    process_before_produces: Self::ProcessBeforeProduces,
    _back_token: Option<BackToken>,
  ) -> IntermediateRunResult<Self::Produces, Self::Messages> {
    Ok(IntermediateRunOutcome::Continue(process_before_produces))
  }

  async fn run_subprocess(
    &self,
    subprocess_consumes: Self::SubprocessConsumes,
    back_token: Option<BackToken>,
  ) -> IntermediateRunResult<Self::Produces, Self::Messages> {
    self.continue_run(subprocess_consumes, back_token).await
  }

  fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex {
    last_used_index
  }
}

#[must_use]
pub fn subprocess<ProcessBeforeProduces: ParamList, Messages: ProcessMessages>()
-> Subprocess<ProcessBeforeProduces, Messages> {
  Subprocess { phantom_data: Default::default() }
}
