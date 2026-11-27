use crate::builder::IntoProcessError;
use crate::builder::borrow_just::BorrowJust;
use crate::builder::flowing_process::sealed;
use crate::builder::{
  CurrentRunYieldedAt, FlowingProcess, IntermediateRunOutcome, IntermediateRunResult, MaybeFormContext, ParamList,
  PreviousRunYieldedAt, ProcessError, SessionContext, StepIndex,
};
use crate::error_list::union::CoprodUnion;
use crate::param_list::concat::Concat;
use crate::param_list::sealed::ParamList as _;
use crate::step::BackToken;
use crate::step::{Form, FormWithContext, InputValidation};
use std::marker::PhantomData;

pub struct FormFlowingProcess<
  ProcessBefore: FlowingProcess,
  FormStep: Form,
  ProcessBeforeProducesToCreateFormConsumesIndices,
  ProcessBeforeProducesToValidateInputConsumesIndices,
  ErrorUnionIndices,
> {
  pub(crate) process_before: ProcessBefore,
  pub(crate) form_step: FormStep,
  pub(crate) step_index: StepIndex,
  pub(crate) phantom_data: PhantomData<(
    ProcessBeforeProducesToCreateFormConsumesIndices,
    ProcessBeforeProducesToValidateInputConsumesIndices,
    ErrorUnionIndices,
  )>,
}

impl<
  ProcessBefore: FlowingProcess,
  FormStep: Form<Messages = ProcessBefore::Messages>,
  ProcessBeforeProducesToCreateFormConsumesIndices: Sync + Send,
  ProcessBeforeProducesToValidateInputConsumesIndices: Sync + Send,
  ErrorUnionIndices: Sync + Send,
> sealed::FlowingProcess
  for FormFlowingProcess<
    ProcessBefore,
    FormStep,
    ProcessBeforeProducesToCreateFormConsumesIndices,
    ProcessBeforeProducesToValidateInputConsumesIndices,
    ErrorUnionIndices,
  >
where
  FormStep::Produces: Concat<ProcessBefore::Produces> + Concat<ProcessBefore::EverProduced>,
  FormStep::Error: CoprodUnion<ProcessBefore::Errors, ErrorUnionIndices>,
  for<'a> &'a ProcessBefore::Produces:
    BorrowJust<'a, FormStep::CreateFormConsumes, ProcessBeforeProducesToCreateFormConsumesIndices>,
  for<'a> &'a ProcessBefore::Produces:
    BorrowJust<'a, FormStep::ValidateInputConsumes, ProcessBeforeProducesToValidateInputConsumesIndices>,
{
  type EntryConsumes = ProcessBefore::EntryConsumes;
  type ProcessBeforeProduces = ProcessBefore::Produces;
  type Produces = <FormStep::Produces as Concat<ProcessBefore::Produces>>::Concatenated;
  type Messages = ProcessBefore::Messages;
  type EverProduced = <FormStep::Produces as Concat<ProcessBefore::EverProduced>>::Concatenated;
  type Errors = <FormStep::Error as CoprodUnion<ProcessBefore::Errors, ErrorUnionIndices>>::Union;

  async fn resume_run(
    &self,
    previous_run_produced: SessionContext,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
    form_context: MaybeFormContext,
    back_token: Option<BackToken>,
  ) -> IntermediateRunResult<Self::Produces, Self::Messages, Self::Errors> {
    if previous_run_yielded_at.0 < self.step_index {
      let process_before_output = self
        .process_before
        .resume_run(previous_run_produced, previous_run_yielded_at, user_input, form_context, back_token)
        .await
        .into_process_err()?;
      match process_before_output {
        IntermediateRunOutcome::Continue(process_before_produces) => {
          self.continue_run(process_before_produces, back_token).await
        },
        IntermediateRunOutcome::Yield(a, b, c, d) => Ok(IntermediateRunOutcome::Yield(a, b, c, d)),
        IntermediateRunOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
        IntermediateRunOutcome::RetryUserInput(a, b) => Ok(IntermediateRunOutcome::RetryUserInput(a, b)),
        IntermediateRunOutcome::Back => Ok(IntermediateRunOutcome::Back),
      }
    } else if previous_run_yielded_at.0 != self.step_index {
      Err(ProcessError::invalid_step_index(previous_run_yielded_at.0))
    } else if let Some(form_context) = form_context {
      let process_before_produces =
        ProcessBefore::Produces::deserialize_param_list(previous_run_produced).into_process_err()?;
      let last_step_consumes =
        <&ProcessBefore::Produces as BorrowJust<'_, FormStep::ValidateInputConsumes, _>>::borrow_just(
          &process_before_produces,
        );
      let context: FormStep::Context = postcard::from_bytes(&form_context).into_process_err()?;
      match self.form_step.handle_input(last_step_consumes, user_input, context, back_token).await.into_process_err()? {
        InputValidation::Successful(a) => Ok(IntermediateRunOutcome::Continue(a.concat(process_before_produces))),
        InputValidation::Retry(a, b) => {
          Ok(IntermediateRunOutcome::RetryUserInput(a, postcard::to_allocvec(&b).into_process_err()?))
        },
        InputValidation::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
        InputValidation::Back(_) => Ok(IntermediateRunOutcome::Back),
      }
    } else {
      let process_before_produces =
        ProcessBefore::Produces::deserialize_param_list(previous_run_produced).into_process_err()?;
      self.continue_run(process_before_produces, back_token).await
    }
  }

  async fn continue_run(
    &self,
    process_before_produces: Self::ProcessBeforeProduces,
    back_token: Option<BackToken>,
  ) -> IntermediateRunResult<Self::Produces, Self::Messages, Self::Errors> {
    let last_step_consumes = <&ProcessBefore::Produces as BorrowJust<'_, FormStep::CreateFormConsumes, _>>::borrow_just(
      &process_before_produces,
    );
    let FormWithContext(form, form_context) =
      self.form_step.create_form(last_step_consumes, back_token).await.into_process_err()?;
    Ok(IntermediateRunOutcome::Yield(
      form,
      process_before_produces.serialize_param_list().into_process_err()?,
      CurrentRunYieldedAt(self.step_index),
      postcard::to_allocvec(&form_context).into_process_err()?,
    ))
  }

  async fn run_subprocess(
    &self,
    entry_consumes: Self::EntryConsumes,
    back_token: Option<BackToken>,
  ) -> IntermediateRunResult<Self::Produces, Self::Messages, Self::Errors> {
    let process_before_output =
      self.process_before.run_subprocess(entry_consumes, back_token).await.into_process_err()?;
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

  fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex {
    let used_index = self.process_before.enumerate_steps(last_used_index);
    self.step_index = used_index + 1;
    self.step_index
  }
}
