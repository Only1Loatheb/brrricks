use crate::builder::flowing_process::sealed;
use crate::builder::{
  IntermediateRunResult, IntermediateStartOutcome, IntermediateStartResult, ParamList, PreviousRunYieldedAt,
  ProcessError, StepIndex,
};
use crate::frunk::coproduct::CNil;
use crate::step::BackToken;
use crate::step::ProcessMessages;
use std::marker::PhantomData;

/// Seeds a split branch with the parameters produced before the split plus the splitter's output for that case.
#[must_use]
pub struct CaseEntry<ProcessBeforeProduces, EverProduced, Messages> {
  pub(crate) phantom_data: PhantomData<(ProcessBeforeProduces, EverProduced, Messages)>,
}

impl<ProcessBeforeProduces: ParamList, EverProduced: ParamList, Messages: ProcessMessages> sealed::FlowingProcess
  for CaseEntry<ProcessBeforeProduces, EverProduced, Messages>
{
  type EntryConsumes = ProcessBeforeProduces;
  type Produces = ProcessBeforeProduces;
  type Messages = Messages;
  type EverProduced = EverProduced; // Includes params produced specific for this case in split
  type Errors = CNil;

  fn resume_run(
    &self,
    _previous_run_produced: &[u8],
    previous_run_yielded_at: PreviousRunYieldedAt,
    _user_input: &str,
    _form_context: Option<&[u8]>,
    _back_token: Option<BackToken>,
  ) -> impl Future<Output = IntermediateRunResult<Self::Produces, Self::Messages, Self::Errors>> {
    std::future::ready(Err(ProcessError::invalid_step_index(previous_run_yielded_at.0)))
  }

  fn run_from_entry(
    &self,
    entry_consumes: Self::EntryConsumes,
    _back_token: Option<BackToken>,
  ) -> impl Future<Output = IntermediateStartResult<Self::Produces, Self::Messages, Self::Errors>> {
    std::future::ready(Ok(IntermediateStartOutcome::Continue(entry_consumes)))
  }

  fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex {
    last_used_index
  }
}

pub(crate) fn case_entry<ProcessBeforeProduces: ParamList, EverProduced: ParamList, Messages: ProcessMessages>()
-> CaseEntry<ProcessBeforeProduces, EverProduced, Messages> {
  CaseEntry { phantom_data: Default::default() }
}
