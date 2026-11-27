use crate::builder::finalized_process::{FinalizedProcess, sealed};
use crate::builder::flowing_process::sealed::FlowingProcess;
use crate::builder::{
  IntermediateRunOutcome, IntoProcessError, MaybeFormContext, ParamList, PreviousRunYieldedAt, ProcessError,
  RunOutcome, RunResult, SessionContext, StepIndex,
};
use crate::error_list::union::CoprodUnion;
use crate::param_list::concat::Concat;
use crate::param_list::extract::Extract;
use crate::step::BackToken;
use std::marker::PhantomData;

pub struct SubprocessFinalizedProcess<
  ProcessBefore: FlowingProcess,
  Sub: FinalizedProcess,
  ProcessBeforeProducesToEntryConsumesIndices,
  SubEverProducedExtractIndices,
  ErrorUnionIndices,
> {
  pub(crate) process_before: ProcessBefore,
  pub(crate) sub_process: Sub,
  pub(crate) sub_start_index: StepIndex,
  pub(crate) sub_end_index: StepIndex,
  pub(crate) phantom_data:
    PhantomData<(ProcessBeforeProducesToEntryConsumesIndices, SubEverProducedExtractIndices, ErrorUnionIndices)>,
}

impl<
  ProcessBefore: FlowingProcess,
  Sub: FinalizedProcess<Messages = ProcessBefore::Messages>,
  ProcessBeforeProducesToEntryConsumesIndices: Sync + Send,
  SubEverProducedExtractIndices: Sync + Send,
  ErrorUnionIndices: Sync + Send,
> sealed::FinalizedProcess
  for SubprocessFinalizedProcess<
    ProcessBefore,
    Sub,
    ProcessBeforeProducesToEntryConsumesIndices,
    SubEverProducedExtractIndices,
    ErrorUnionIndices,
  >
where
  Sub::EverProduced: Extract<Sub::EntryConsumes, SubEverProducedExtractIndices>,
  <Sub::EverProduced as Extract<Sub::EntryConsumes, SubEverProducedExtractIndices>>::Remainder:
    ParamList + Concat<ProcessBefore::EverProduced>,
  ProcessBefore::Produces: Extract<Sub::EntryConsumes, ProcessBeforeProducesToEntryConsumesIndices>,
  Sub::Errors: CoprodUnion<ProcessBefore::Errors, ErrorUnionIndices>,
{
  type EntryConsumes = ProcessBefore::EntryConsumes;
  type ProcessBeforeProduces = ProcessBefore::Produces;
  type Messages = ProcessBefore::Messages;
  type EverProduced =
    <<Sub::EverProduced as Extract<Sub::EntryConsumes, SubEverProducedExtractIndices>>::Remainder as Concat<
      ProcessBefore::EverProduced,
    >>::Concatenated;
  type Errors = <Sub::Errors as CoprodUnion<ProcessBefore::Errors, ErrorUnionIndices>>::Union;

  async fn resume_run(
    &self,
    previous_run_produced: SessionContext,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
    form_context: MaybeFormContext,
    back_token: Option<BackToken>,
  ) -> RunResult<Self::Messages, Self::Errors> {
    if previous_run_yielded_at.0 <= self.sub_start_index {
      let outcome = self
        .process_before
        .resume_run(previous_run_produced, previous_run_yielded_at, user_input, form_context, back_token)
        .await
        .into_process_err()?;
      match outcome {
        IntermediateRunOutcome::Continue(val) => self.continue_run(val, back_token).await,
        IntermediateRunOutcome::Yield(a, b, c, d) => Ok(RunOutcome::Yield(a, b, c, d)),
        IntermediateRunOutcome::Finish(a) => Ok(RunOutcome::Finish(a)),
        IntermediateRunOutcome::RetryUserInput(a, b) => Ok(RunOutcome::RetryUserInput(a, b)),
        IntermediateRunOutcome::Back => Ok(RunOutcome::Back),
      }
    } else if previous_run_yielded_at.0 <= self.sub_end_index {
      self
        .sub_process
        .resume_run(previous_run_produced, previous_run_yielded_at, user_input, form_context, back_token)
        .await
        .into_process_err()
    } else {
      Err(ProcessError::invalid_step_index(previous_run_yielded_at.0))
    }
  }

  async fn continue_run(
    &self,
    process_before_produces: Self::ProcessBeforeProduces,
    back_token: Option<BackToken>,
  ) -> RunResult<Self::Messages, Self::Errors> {
    self.sub_process.run_subprocess(process_before_produces.extract(), back_token).await.into_process_err()
  }

  async fn run_subprocess(
    &self,
    entry_consumes: Self::EntryConsumes,
    back_token: Option<BackToken>,
  ) -> RunResult<Self::Messages, Self::Errors> {
    let outcome = self.process_before.run_subprocess(entry_consumes, back_token).await.into_process_err()?;
    match outcome {
      IntermediateRunOutcome::Continue(val) => self.continue_run(val, back_token).await,
      IntermediateRunOutcome::Yield(a, b, c, d) => Ok(RunOutcome::Yield(a, b, c, d)),
      IntermediateRunOutcome::Finish(a) => Ok(RunOutcome::Finish(a)),
      IntermediateRunOutcome::RetryUserInput(a, b) => Ok(RunOutcome::RetryUserInput(a, b)),
      IntermediateRunOutcome::Back => Ok(RunOutcome::Back),
    }
  }

  fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex {
    let last_used_index = self.process_before.enumerate_steps(last_used_index);
    self.sub_start_index = last_used_index;
    let last_used_index = self.sub_process.enumerate_steps(last_used_index);
    self.sub_end_index = last_used_index;
    last_used_index
  }
}
