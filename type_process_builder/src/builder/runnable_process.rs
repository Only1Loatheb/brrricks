use crate::builder::finalized_process::FinalizedProcess;
use crate::builder::{MaybeFormContext, PreviousRunYieldedAt, RunResult, SessionContext, StepIndex};
use crate::step::BackToken;

pub struct RunnableProcess<UnderlyingProcess: FinalizedProcess> {
  finalized_process: UnderlyingProcess, // shouldn't be public
  name: &'static str,                   // immutable
  version: u32,                         // immutable
}

impl<UnderlyingProcess: FinalizedProcess> RunnableProcess<UnderlyingProcess> {
  pub fn new(mut finalized_process: UnderlyingProcess, name: &'static str, version: u32) -> Self {
    finalized_process.enumerate_steps(StepIndex::MIN);
    Self { finalized_process, name, version }
  }

  pub async fn resume_run(
    &self,
    previous_run_produced: SessionContext,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
    form_context: MaybeFormContext,
    back_token: Option<BackToken>,
  ) -> RunResult<UnderlyingProcess::Messages> {
    self
      .finalized_process
      .resume_run(previous_run_produced, previous_run_yielded_at, user_input, form_context, back_token)
      .await
  }

  pub fn get_name(&self) -> &'static str {
    self.name
  }

  pub fn get_version(&self) -> u32 {
    self.version
  }
}
