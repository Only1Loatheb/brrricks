use crate::builder::finalized_process::FinalizedProcess;
use crate::builder::{ParamUID, PreviousRunYieldedAt, RunResult, SessionContext, StepIndex};
use crate::step::FailedInputValidationAttempts;
use std::collections::HashSet;

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
    failed_input_validation_attempts: FailedInputValidationAttempts,
  ) -> RunResult<UnderlyingProcess::Messages> {
    self
      .finalized_process
      .resume_run(previous_run_produced, previous_run_yielded_at, user_input, failed_input_validation_attempts)
      .await
  }

  /// [crate::param_list::ParamList::_deserialize]
  pub fn ordered_all_unique_param_uids(&self) -> Vec<ParamUID> {
    let mut all_param_uids = Vec::<ParamUID>::new();
    self.finalized_process.all_param_uids(&mut all_param_uids);

    let mut seen = HashSet::new();
    all_param_uids.into_iter().rev().filter(|c| seen.insert(*c)).collect::<Vec<_>>()
  }

  pub fn get_name(&self) -> &'static str {
    self.name
  }

  pub fn get_version(&self) -> u32 {
    self.version
  }
}
