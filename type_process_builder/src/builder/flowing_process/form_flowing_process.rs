use crate::builder::borrow_just::BorrowJust;
use crate::builder::{
  CurrentRunYieldedAt, FlowingProcess, MaybeFormContext, IntermediateRunOutcome, IntermediateRunResult, ParamList, ParamUID,
  PreviousRunYieldedAt, SessionContext, StepIndex,
};
use crate::param_list::concat::Concat;
use crate::step::{Form, InputValidation};
use anyhow::anyhow;
use std::marker::PhantomData;

pub struct FormFlowingProcess<
  ProcessBefore: FlowingProcess,
  FormStep: Form,
  ProcessBeforeProducesToCreateFormConsumesIndices,
  ProcessBeforeProducesToValidateInputConsumesIndices,
> {
  pub process_before: ProcessBefore,
  pub form_step: FormStep,
  pub step_index: StepIndex,
  pub phantom_data: PhantomData<(
    ProcessBeforeProducesToCreateFormConsumesIndices,
    ProcessBeforeProducesToValidateInputConsumesIndices,
  )>,
}

impl<
  ProcessBefore: FlowingProcess,
  FormStep: Form<Messages = ProcessBefore::Messages>,
  ProcessBeforeProducesToCreateFormConsumesIndices: Sync + Send,
  ProcessBeforeProducesToValidateInputConsumesIndices: Sync + Send,
> FlowingProcess
  for FormFlowingProcess<
    ProcessBefore,
    FormStep,
    ProcessBeforeProducesToCreateFormConsumesIndices,
    ProcessBeforeProducesToValidateInputConsumesIndices,
  >
where
  FormStep::Produces: Concat<ProcessBefore::Produces>,
  for<'a> &'a ProcessBefore::Produces:
    BorrowJust<'a, FormStep::CreateFormConsumes, ProcessBeforeProducesToCreateFormConsumesIndices>,
  for<'a> &'a ProcessBefore::Produces:
    BorrowJust<'a, FormStep::ValidateInputConsumes, ProcessBeforeProducesToValidateInputConsumesIndices>,
{
  type ProcessBeforeProduces = ProcessBefore::Produces;
  type Produces = <FormStep::Produces as Concat<ProcessBefore::Produces>>::Concatenated;
  type SubprocessConsumes = ProcessBefore::SubprocessConsumes;
  type Messages = ProcessBefore::Messages;

  async fn resume_run(
    &self,
    previous_run_produced: SessionContext,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
    form_context: MaybeFormContext,
  ) -> IntermediateRunResult<Self::Produces, Self::Messages> {
    if previous_run_yielded_at.0 < self.step_index {
      let process_before_output = self
        .process_before
        .resume_run(previous_run_produced, previous_run_yielded_at, user_input, form_context)
        .await?;
      match process_before_output {
        IntermediateRunOutcome::Continue(process_before_produces) => self.continue_run(process_before_produces).await,
        IntermediateRunOutcome::Yield(a, b, c) => Ok(IntermediateRunOutcome::Yield(a, b, c)),
        IntermediateRunOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
        IntermediateRunOutcome::RetryUserInput(a, b) => Ok(IntermediateRunOutcome::RetryUserInput(a, b)),
      }
    } else {
      let process_before_produces = ProcessBefore::Produces::deserialize(previous_run_produced)?;
      let last_step_consumes =
        <&ProcessBefore::Produces as BorrowJust<'_, FormStep::ValidateInputConsumes, _>>::borrow_just(
          &process_before_produces,
        );
      let context: FormStep::Context = postcard::from_bytes(&form_context.ok_or(anyhow!("Missing FormContext"))?)?;
      match self.form_step.handle_input(last_step_consumes, user_input, context).await? {
        InputValidation::Successful(a) => Ok(IntermediateRunOutcome::Continue(a.concat(process_before_produces))),
        InputValidation::Retry(a) => Ok(IntermediateRunOutcome::RetryUserInput(a, todo!())),
        InputValidation::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
      }
    }
  }

  async fn continue_run(
    &self,
    process_before_produces: Self::ProcessBeforeProduces,
  ) -> IntermediateRunResult<Self::Produces, Self::Messages> {
    let last_step_consumes = <&ProcessBefore::Produces as BorrowJust<'_, FormStep::CreateFormConsumes, _>>::borrow_just(
      &process_before_produces,
    );
    Ok(IntermediateRunOutcome::Yield(
      self.form_step.create_form(last_step_consumes).await?,
      process_before_produces.serialize()?,
      CurrentRunYieldedAt(self.step_index),
    ))
  }

  async fn run_subprocess(
    &self,
    subprocess_consumes: Self::SubprocessConsumes,
  ) -> IntermediateRunResult<Self::Produces, Self::Messages> {
    let process_before_output = self.process_before.run_subprocess(subprocess_consumes).await?;
    match process_before_output {
      IntermediateRunOutcome::Continue(process_before_produces) => self.continue_run(process_before_produces).await,
      IntermediateRunOutcome::Yield(a, b, c) => Ok(IntermediateRunOutcome::Yield(a, b, c)),
      IntermediateRunOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
      IntermediateRunOutcome::RetryUserInput(a, b) => Ok(IntermediateRunOutcome::RetryUserInput(a, b)),
    }
  }

  fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex {
    let used_index = self.process_before.enumerate_steps(last_used_index);
    self.step_index = used_index + 1;
    self.step_index
  }

  fn all_param_uids(&self, acc: &mut Vec<ParamUID>) {
    self.process_before.all_param_uids(acc);
    FormStep::Produces::all_param_uids(acc);
  }
}
