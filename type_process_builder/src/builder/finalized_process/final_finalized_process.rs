use crate::builder::IntoProcessError;
use crate::builder::finalized_process::sealed;
use crate::builder::flowing_process::FlowingProcess;
use crate::builder::{IntermediateRunOutcome, PreviousRunYieldedAt, RunOutcome, RunResult, StepIndex};
use crate::error_coproduct::union::CoprodUnion;
use crate::param_list::extract::Extract;
use crate::step::{BackToken, Final, ProcessMessages};
use std::marker::PhantomData;

pub struct FinalFinalizedProcess<
  ProcessBefore: FlowingProcess,
  FinalStep: Final<FinalMessage = <ProcessBefore::Messages as ProcessMessages>::FinalMessage>,
  ProcessBeforeProducesTransformToFinalConsumesIndices,
  ErrorUnionIndices,
> {
  pub(crate) process_before: ProcessBefore,
  pub(crate) final_step: FinalStep,
  pub(crate) phantom_data: PhantomData<(ProcessBeforeProducesTransformToFinalConsumesIndices, ErrorUnionIndices)>,
}

impl<
  ProcessBefore: FlowingProcess,
  FinalStep: Final<FinalMessage = <ProcessBefore::Messages as ProcessMessages>::FinalMessage>,
  ProcessBeforeProducesTransformToFinalConsumesIndices: Sync + Send,
  ErrorUnionIndices: Sync + Send,
> sealed::FinalizedProcess
  for FinalFinalizedProcess<
    ProcessBefore,
    FinalStep,
    ProcessBeforeProducesTransformToFinalConsumesIndices,
    ErrorUnionIndices,
  >
where
  FinalStep::Error: CoprodUnion<ProcessBefore::Errors, ErrorUnionIndices>,
  ProcessBefore::Produces: Extract<FinalStep::Consumes, ProcessBeforeProducesTransformToFinalConsumesIndices>,
{
  type EntryConsumes = ProcessBefore::EntryConsumes;
  type Messages = ProcessBefore::Messages;
  type EverProduced = ProcessBefore::EverProduced;
  type Errors = <FinalStep::Error as CoprodUnion<ProcessBefore::Errors, ErrorUnionIndices>>::Union;

  async fn resume_run(
    &self,
    previous_run_produced: &[u8],
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: &str,
    form_context: Option<&[u8]>,
    back_token: Option<BackToken>,
  ) -> RunResult<Self::Messages, Self::Errors> {
    let outcome = self
      .process_before
      .resume_run(previous_run_produced, previous_run_yielded_at, user_input, form_context, back_token)
      .await
      .into_process_err()?;
    match outcome {
      IntermediateRunOutcome::Continue(val) => self.continue_run(val).await,
      IntermediateRunOutcome::Yield(a, b, c, d) => Ok(RunOutcome::Yield(a, b, c, d)),
      IntermediateRunOutcome::Finish(a) => Ok(RunOutcome::Finish(a)),
      IntermediateRunOutcome::RetryUserInput(a, b) => Ok(RunOutcome::RetryUserInput(a, b)),
      IntermediateRunOutcome::Back => Ok(RunOutcome::Back),
    }
  }

  async fn run_from_entry(
    &self,
    entry_consumes: Self::EntryConsumes,
    back_token: Option<BackToken>,
  ) -> RunResult<Self::Messages, Self::Errors> {
    let outcome = self.process_before.run_from_entry(entry_consumes, back_token).await.into_process_err()?;
    match outcome {
      IntermediateRunOutcome::Continue(val) => self.continue_run(val).await,
      IntermediateRunOutcome::Yield(a, b, c, d) => Ok(RunOutcome::Yield(a, b, c, d)),
      IntermediateRunOutcome::Finish(a) => Ok(RunOutcome::Finish(a)),
      IntermediateRunOutcome::RetryUserInput(a, b) => Ok(RunOutcome::RetryUserInput(a, b)),
      IntermediateRunOutcome::Back => Ok(RunOutcome::Back),
    }
  }

  /// Assigning an index is unnecessary because a process never yields at [`Final`] step
  fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex {
    self.process_before.enumerate_steps(last_used_index)
  }
}

impl<
  ProcessBefore: FlowingProcess,
  FinalStep: Final<FinalMessage = <ProcessBefore::Messages as ProcessMessages>::FinalMessage>,
  ProcessBeforeProducesTransformToFinalConsumesIndices: Sync + Send,
  ErrorUnionIndices: Sync + Send,
>
  FinalFinalizedProcess<
    ProcessBefore,
    FinalStep,
    ProcessBeforeProducesTransformToFinalConsumesIndices,
    ErrorUnionIndices,
  >
where
  FinalStep::Error: CoprodUnion<ProcessBefore::Errors, ErrorUnionIndices>,
  ProcessBefore::Produces: Extract<FinalStep::Consumes, ProcessBeforeProducesTransformToFinalConsumesIndices>,
{
  async fn continue_run(
    &self,
    process_before_produces: ProcessBefore::Produces,
  ) -> RunResult<ProcessBefore::Messages, <Self as sealed::FinalizedProcess>::Errors> {
    Ok(RunOutcome::Finish(self.final_step.handle(process_before_produces.extract()).await.into_process_err()?))
  }
}
