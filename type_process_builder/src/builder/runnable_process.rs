use crate::builder::finalized_process::FinalizedProcess;
use crate::builder::{MaybeFormContext, PreviousRunYieldedAt, RunResult, SessionContext};
use crate::step::BackToken;

pub struct RunnableProcess<UnderlyingProcess: FinalizedProcess> {
  pub(crate) finalized_process: UnderlyingProcess,
  pub(crate) name: &'static str,
  pub(crate) version: u32,
}

impl<UnderlyingProcess: FinalizedProcess> RunnableProcess<UnderlyingProcess> {
  pub async fn start_run(
    &self,
    entry_consumes: UnderlyingProcess::EntryConsumes,
  ) -> RunResult<UnderlyingProcess::Messages, UnderlyingProcess::Errors> {
    self.finalized_process.run_from_entry(entry_consumes, None).await
  }

  pub async fn resume_run(
    &self,
    previous_run_produced: SessionContext,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
    form_context: MaybeFormContext,
    back_token: Option<BackToken>,
  ) -> RunResult<UnderlyingProcess::Messages, UnderlyingProcess::Errors> {
    self
      .finalized_process
      .resume_run(&previous_run_produced, previous_run_yielded_at, user_input, form_context.as_deref(), back_token)
      .await
  }

  pub fn name(&self) -> &'static str {
    self.name
  }

  pub fn version(&self) -> u32 {
    self.version
  }
}
