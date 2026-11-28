use crate::builder::IntoProcessError;
use crate::builder::flowing_process::sealed;
use crate::builder::{
  FlowingProcess, IntermediateRunOutcome, IntermediateRunResult, IntermediateStartOutcome, IntermediateStartResult,
  ParamList, PreviousRunYieldedAt, StepIndex,
};
use crate::error_coproduct::union::CoprodUnion;
use crate::param_list::borrow_just::BorrowJust;
use crate::param_list::concat::Concat;
use crate::step::BackToken;
use crate::step::{Operation, OperationOutcome, ProcessMessages};
use std::marker::PhantomData;

pub struct OperationFlowingProcess<
  ProcessBefore: FlowingProcess,
  OperationStep: Operation,
  ProcessBeforeProducesToLastStepConsumesIndices,
  ErrorUnionIndices,
> {
  pub(crate) process_before: ProcessBefore,
  pub(crate) last_step: OperationStep,
  pub(crate) phantom_data: PhantomData<(ProcessBeforeProducesToLastStepConsumesIndices, ErrorUnionIndices)>,
}

impl<
  ProcessBefore: FlowingProcess,
  OperationStep: Operation<FinalMessage = <ProcessBefore::Messages as ProcessMessages>::FinalMessage>,
  ProcessBeforeProducesToLastStepConsumesIndices: Sync + Send,
  ErrorUnionIndices: Sync + Send,
> sealed::FlowingProcess
  for OperationFlowingProcess<
    ProcessBefore,
    OperationStep,
    ProcessBeforeProducesToLastStepConsumesIndices,
    ErrorUnionIndices,
  >
where
  OperationStep::Produces: ParamList + Concat<ProcessBefore::Produces> + Concat<ProcessBefore::EverProduced>,
  OperationStep::Error: CoprodUnion<ProcessBefore::Errors, ErrorUnionIndices>,
  for<'a> &'a ProcessBefore::Produces:
    BorrowJust<'a, OperationStep::Consumes, ProcessBeforeProducesToLastStepConsumesIndices>,
{
  type EntryConsumes = ProcessBefore::EntryConsumes;
  type Produces = <OperationStep::Produces as Concat<ProcessBefore::Produces>>::Concatenated;
  type Messages = ProcessBefore::Messages;
  type EverProduced = <OperationStep::Produces as Concat<ProcessBefore::EverProduced>>::Concatenated;
  type Errors = <OperationStep::Error as CoprodUnion<ProcessBefore::Errors, ErrorUnionIndices>>::Union;

  async fn resume_run(
    &self,
    previous_run_produced: &[u8],
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: &str,
    form_context: Option<&[u8]>,
    back_token: Option<BackToken>,
  ) -> IntermediateRunResult<Self::Produces, Self::Messages, Self::Errors> {
    let process_before_output = self
      .process_before
      .resume_run(previous_run_produced, previous_run_yielded_at, user_input, form_context, back_token)
      .await
      .into_process_err()?;
    match process_before_output {
      IntermediateRunOutcome::Continue(process_before_produces) => {
        self.continue_run(process_before_produces).await.map(Into::into)
      },
      IntermediateRunOutcome::Yield(a, b, c, d) => Ok(IntermediateRunOutcome::Yield(a, b, c, d)),
      IntermediateRunOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
      IntermediateRunOutcome::RetryUserInput(a, b) => Ok(IntermediateRunOutcome::RetryUserInput(a, b)),
      IntermediateRunOutcome::Back => Ok(IntermediateRunOutcome::Back),
    }
  }

  async fn run_from_entry(
    &self,
    entry_consumes: Self::EntryConsumes,
    back_token: Option<BackToken>,
  ) -> IntermediateStartResult<Self::Produces, Self::Messages, Self::Errors> {
    let process_before_output =
      self.process_before.run_from_entry(entry_consumes, back_token).await.into_process_err()?;
    match process_before_output {
      IntermediateStartOutcome::Continue(process_before_produces) => self.continue_run(process_before_produces).await,
      IntermediateStartOutcome::Yield(a, b, c, d) => Ok(IntermediateStartOutcome::Yield(a, b, c, d)),
      IntermediateStartOutcome::Finish(a) => Ok(IntermediateStartOutcome::Finish(a)),
    }
  }

  /// Assigning an index is unnecessary because a process never yields at [`Operation`] step
  fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex {
    self.process_before.enumerate_steps(last_used_index)
  }
}

impl<
  ProcessBefore: FlowingProcess,
  OperationStep: Operation<FinalMessage = <ProcessBefore::Messages as ProcessMessages>::FinalMessage>,
  ProcessBeforeProducesToLastStepConsumesIndices: Sync + Send,
  ErrorUnionIndices: Sync + Send,
>
  OperationFlowingProcess<
    ProcessBefore,
    OperationStep,
    ProcessBeforeProducesToLastStepConsumesIndices,
    ErrorUnionIndices,
  >
where
  OperationStep::Produces: ParamList + Concat<ProcessBefore::Produces> + Concat<ProcessBefore::EverProduced>,
  OperationStep::Error: CoprodUnion<ProcessBefore::Errors, ErrorUnionIndices>,
  for<'a> &'a ProcessBefore::Produces:
    BorrowJust<'a, OperationStep::Consumes, ProcessBeforeProducesToLastStepConsumesIndices>,
{
  async fn continue_run(
    &self,
    process_before_produces: ProcessBefore::Produces,
  ) -> IntermediateStartResult<
    <Self as sealed::FlowingProcess>::Produces,
    ProcessBefore::Messages,
    <Self as sealed::FlowingProcess>::Errors,
  > {
    let last_step_consumes = (&process_before_produces).borrow_just();
    let last_step_outcome = self.last_step.handle(last_step_consumes).await.into_process_err()?;
    Ok(match last_step_outcome {
      OperationOutcome::Successful(last_step_output) => {
        IntermediateStartOutcome::Continue(last_step_output.concat(process_before_produces))
      },
      OperationOutcome::Finish(a) => IntermediateStartOutcome::Finish(a),
    })
  }
}
