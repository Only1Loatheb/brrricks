use crate::builder::{
  FlowingProcess, IntermediateRunOutcome, IntermediateRunResult, MaybeFormContext, ParamList, PreviousRunYieldedAt,
  SessionContext, StepIndex,
};
use crate::param_list::borrow_just::BorrowJust;
use crate::param_list::concat::Concat;
use crate::step::BackToken;
use crate::step::{Operation, OperationOutcome, ProcessMessages};
use std::marker::PhantomData;

pub struct OperationFlowingProcess<
  ProcessBefore: FlowingProcess,
  OperationStep: Operation,
  ProcessBeforeProducesToLastStepConsumesIndices,
> {
  pub process_before: ProcessBefore,
  pub last_step: OperationStep,
  pub phantom_data: PhantomData<ProcessBeforeProducesToLastStepConsumesIndices>,
}

impl<
  ProcessBefore: FlowingProcess,
  OperationStep: Operation<FinalMessage = <ProcessBefore::Messages as ProcessMessages>::FinalMessage>,
  ProcessBeforeProducesToLastStepConsumesIndices: Sync + Send,
> FlowingProcess
  for OperationFlowingProcess<ProcessBefore, OperationStep, ProcessBeforeProducesToLastStepConsumesIndices>
where
  OperationStep::Produces: ParamList + Concat<ProcessBefore::Produces> + Concat<ProcessBefore::EverProduced>,
  for<'a> &'a ProcessBefore::Produces:
    BorrowJust<'a, OperationStep::Consumes, ProcessBeforeProducesToLastStepConsumesIndices>,
{
  type EntryConsumes = ProcessBefore::EntryConsumes;
  type ProcessBeforeProduces = ProcessBefore::Produces;
  type Produces = <OperationStep::Produces as Concat<ProcessBefore::Produces>>::Concatenated;
  type SubprocessConsumes = ProcessBefore::SubprocessConsumes;
  type Messages = ProcessBefore::Messages;
  type EverProduced = <OperationStep::Produces as Concat<ProcessBefore::EverProduced>>::Concatenated;

  async fn resume_run(
    &self,
    previous_run_produced: SessionContext,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
    form_context: MaybeFormContext,
    back_token: Option<BackToken>,
  ) -> IntermediateRunResult<Self::Produces, Self::Messages> {
    let process_before_output = self
      .process_before
      .resume_run(previous_run_produced, previous_run_yielded_at, user_input, form_context, back_token)
      .await?;
    match process_before_output {
      IntermediateRunOutcome::Continue(process_before_produces) => {
        self.continue_run(process_before_produces, back_token).await
      },
      IntermediateRunOutcome::Yield(a, b, c, d) => Ok(IntermediateRunOutcome::Yield(a, b, c, d)),
      IntermediateRunOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
      IntermediateRunOutcome::RetryUserInput(a, b) => Ok(IntermediateRunOutcome::RetryUserInput(a, b)),
      IntermediateRunOutcome::Back => Ok(IntermediateRunOutcome::Back),
    }
  }

  async fn continue_run(
    &self,
    process_before_produces: Self::ProcessBeforeProduces,
    _back_token: Option<BackToken>,
  ) -> IntermediateRunResult<Self::Produces, Self::Messages> {
    let last_step_consumes = (&process_before_produces).borrow_just();
    let last_step_outcome = self.last_step.handle(last_step_consumes).await?;
    Ok(match last_step_outcome {
      OperationOutcome::Successful(last_step_output) => {
        IntermediateRunOutcome::Continue(last_step_output.concat(process_before_produces))
      },
      OperationOutcome::Finish(a) => IntermediateRunOutcome::Finish(a),
    })
  }

  async fn run_subprocess(
    &self,
    subprocess_consumes: Self::SubprocessConsumes,
    back_token: Option<BackToken>,
  ) -> IntermediateRunResult<Self::Produces, Self::Messages> {
    let process_before_output = self.process_before.run_subprocess(subprocess_consumes, back_token).await?;
    match process_before_output {
      IntermediateRunOutcome::Continue(process_before_produces) => {
        self.continue_run(process_before_produces, back_token).await
      },
      IntermediateRunOutcome::Yield(a, b, c, d) => Ok(IntermediateRunOutcome::Yield(a, b, c, d)),
      IntermediateRunOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
      IntermediateRunOutcome::RetryUserInput(a, b) => Ok(IntermediateRunOutcome::RetryUserInput(a, b)),
      IntermediateRunOutcome::Back => Ok(IntermediateRunOutcome::Back),
    }
  }

  /// Assigning an index is unnecessary because a process never yields at [`Operation`] step
  fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex {
    self.process_before.enumerate_steps(last_used_index)
  }
}
