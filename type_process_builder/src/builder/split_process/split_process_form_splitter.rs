use crate::builder::IntoProcessError;
use crate::builder::split_process::sealed;
use crate::builder::{
  CurrentRunYieldedAt, FlowingProcess, IntermediateFinalizedSplitOutcome, IntermediateFinalizedSplitResult,
  IntermediateRunOutcome, MaybeFormContext, ParamList, PreviousRunYieldedAt, ProcessError, SessionContext, StepIndex,
};
use crate::error_list::union::CoprodUnion;
use crate::frunk::coproduct::Coproduct;
use crate::param_list::borrow_just::BorrowJust;
use crate::param_list::concat::Concat;
use crate::param_list::sealed::ParamList as _;
use crate::step::BackToken;
use crate::step::{FormSplitter, FormWithContext, InputValidation};
use std::marker::PhantomData;

pub struct SplitProcessFormSplitter<
  Tag: Send + Sync,
  ProcessBefore: FlowingProcess,
  SplitterProducesForFirstCase: ParamList,
  SplitterProducesForOtherCases: Send + Sync,
  SplitterStep: FormSplitter<Produces = Coproduct<(Tag, SplitterProducesForFirstCase), SplitterProducesForOtherCases>>,
  ProcessBeforeProducesToCreateFormConsumesIndices,
  ProcessBeforeProducesToValidateInputConsumesIndices,
  ErrorUnionIndices,
> {
  pub(crate) process_before: ProcessBefore,
  pub(crate) splitter: SplitterStep,
  pub(crate) step_index: StepIndex,
  pub(crate) phantom_data: PhantomData<(
    ProcessBeforeProducesToCreateFormConsumesIndices,
    ProcessBeforeProducesToValidateInputConsumesIndices,
    ErrorUnionIndices,
  )>,
}

impl<
  Tag: Send + Sync,
  ProcessBefore: FlowingProcess,
  SplitterProducesForFirstCase: ParamList + Concat<ProcessBefore::Produces> + Concat<ProcessBefore::EverProduced>,
  SplitterProducesForOtherCases: Send + Sync,
  SplitterStep: FormSplitter<
      Produces = Coproduct<(Tag, SplitterProducesForFirstCase), SplitterProducesForOtherCases>,
      Messages = ProcessBefore::Messages,
    >,
  ProcessBeforeProducesToCreateFormConsumesIndices: Sync + Send,
  ProcessBeforeProducesToValidateInputConsumesIndices: Sync + Send,
  ErrorUnionIndices: Sync + Send,
> sealed::SplitProcess<SplitterProducesForOtherCases>
  for SplitProcessFormSplitter<
    Tag,
    ProcessBefore,
    SplitterProducesForFirstCase,
    SplitterProducesForOtherCases,
    SplitterStep,
    ProcessBeforeProducesToCreateFormConsumesIndices,
    ProcessBeforeProducesToValidateInputConsumesIndices,
    ErrorUnionIndices,
  >
where
  SplitterStep::Error: CoprodUnion<ProcessBefore::Errors, ErrorUnionIndices>,
  for<'a> &'a ProcessBefore::Produces:
    BorrowJust<'a, SplitterStep::CreateFormConsumes, ProcessBeforeProducesToCreateFormConsumesIndices>,
  for<'a> &'a ProcessBefore::Produces:
    BorrowJust<'a, SplitterStep::ValidateInputConsumes, ProcessBeforeProducesToValidateInputConsumesIndices>,
{
  type EntryConsumes = ProcessBefore::EntryConsumes;
  type ProcessBeforeSplitProduces = ProcessBefore::Produces;
  type SplitterProducesForFirstCase = SplitterProducesForFirstCase;
  type SplitterTagForFirstCase = Tag;
  type SubprocessConsumes = ProcessBefore::SubprocessConsumes;
  type Messages = ProcessBefore::Messages;
  type ProcessBeforeSplitEverProduced = ProcessBefore::EverProduced;
  type EverProduced = ProcessBefore::EverProduced; // SplitterProducesForFirstCase is included in first case produces when handing first case
  type Errors = <SplitterStep::Error as CoprodUnion<ProcessBefore::Errors, ErrorUnionIndices>>::Union;

  async fn resume_run(
    &self,
    previous_run_produced: SessionContext,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
    form_context: MaybeFormContext,
    back_token: Option<BackToken>,
  ) -> IntermediateFinalizedSplitResult<
    Self::ProcessBeforeSplitProduces,
    Coproduct<Self::SplitterProducesForFirstCase, SplitterProducesForOtherCases>,
    Self::Messages,
    Self::Errors,
  > {
    if previous_run_yielded_at.0 < self.step_index {
      let process_before_output = self
        .process_before
        .resume_run(previous_run_produced, previous_run_yielded_at, user_input, form_context, back_token)
        .await
        .into_process_err()?;
      match process_before_output {
        IntermediateRunOutcome::Continue(process_before_split_produced) => {
          self.continue_run(process_before_split_produced, back_token).await
        },
        IntermediateRunOutcome::Yield(a, b, c, d) => Ok(IntermediateFinalizedSplitOutcome::Yield(a, b, c, d)),
        IntermediateRunOutcome::Finish(a) => Ok(IntermediateFinalizedSplitOutcome::Finish(a)),
        IntermediateRunOutcome::RetryUserInput(a, b) => Ok(IntermediateFinalizedSplitOutcome::RetryUserInput(a, b)),
        IntermediateRunOutcome::Back => Ok(IntermediateFinalizedSplitOutcome::Back),
      }
    } else if previous_run_yielded_at.0 != self.step_index {
      Err(ProcessError::invalid_step_index(previous_run_yielded_at.0))
    } else if let Some(form_context) = form_context {
      let process_before_split_produced =
        ProcessBefore::Produces::deserialize_param_list(previous_run_produced).into_process_err()?;
      let last_step_consumes =
        <&ProcessBefore::Produces as BorrowJust<'_, SplitterStep::ValidateInputConsumes, _>>::borrow_just(
          &process_before_split_produced,
        );
      let context: SplitterStep::Context = postcard::from_bytes(&form_context).into_process_err()?;
      match self.splitter.handle_input(last_step_consumes, user_input, context, back_token).await.into_process_err()? {
        InputValidation::Successful(splitter_produces) => {
          let splitter_produces_to_other_cases = match splitter_produces {
            Coproduct::Inl(a) => Coproduct::Inl(a.1),
            Coproduct::Inr(b) => Coproduct::Inr(b),
          };
          Ok(IntermediateFinalizedSplitOutcome::GoToCase {
            process_before_split_produced,
            splitter_produces_to_other_cases,
          })
        },
        InputValidation::Retry(a, b) => {
          Ok(IntermediateFinalizedSplitOutcome::RetryUserInput(a, postcard::to_allocvec(&b).into_process_err()?))
        },
        InputValidation::Finish(a) => Ok(IntermediateFinalizedSplitOutcome::Finish(a)),
        InputValidation::Back(_) => Ok(IntermediateFinalizedSplitOutcome::Back),
      }
    } else {
      let process_before_split_produced =
        ProcessBefore::Produces::deserialize_param_list(previous_run_produced).into_process_err()?;
      self.continue_run(process_before_split_produced, back_token).await
    }
  }

  async fn continue_run(
    &self,
    process_before_split_produced: Self::ProcessBeforeSplitProduces,
    back_token: Option<BackToken>,
  ) -> IntermediateFinalizedSplitResult<
    Self::ProcessBeforeSplitProduces,
    Coproduct<Self::SplitterProducesForFirstCase, SplitterProducesForOtherCases>,
    Self::Messages,
    Self::Errors,
  > {
    let splitter_step_consumes =
      <&ProcessBefore::Produces as BorrowJust<'_, SplitterStep::CreateFormConsumes, _>>::borrow_just(
        &process_before_split_produced,
      );
    let FormWithContext(form, form_context) =
      self.splitter.create_form(splitter_step_consumes, back_token).await.into_process_err()?;
    Ok(IntermediateFinalizedSplitOutcome::Yield(
      form,
      process_before_split_produced.serialize_param_list().into_process_err()?,
      CurrentRunYieldedAt(self.step_index),
      postcard::to_allocvec(&form_context).into_process_err()?,
    ))
  }

  async fn run_subprocess(
    &self,
    subprocess_consumes: Self::SubprocessConsumes,
    back_token: Option<BackToken>,
  ) -> IntermediateFinalizedSplitResult<
    Self::ProcessBeforeSplitProduces,
    Coproduct<Self::SplitterProducesForFirstCase, SplitterProducesForOtherCases>,
    Self::Messages,
    Self::Errors,
  > {
    let process_before_output =
      self.process_before.run_subprocess(subprocess_consumes, back_token).await.into_process_err()?;
    match process_before_output {
      IntermediateRunOutcome::Continue(process_before_split_produced) => {
        self.continue_run(process_before_split_produced, back_token).await
      },
      IntermediateRunOutcome::Yield(a, b, c, d) => Ok(IntermediateFinalizedSplitOutcome::Yield(a, b, c, d)),
      IntermediateRunOutcome::Finish(a) => Ok(IntermediateFinalizedSplitOutcome::Finish(a)),
      IntermediateRunOutcome::RetryUserInput(a, b) => Ok(IntermediateFinalizedSplitOutcome::RetryUserInput(a, b)),
      IntermediateRunOutcome::Back => Ok(IntermediateFinalizedSplitOutcome::Back),
    }
  }

  fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex {
    let used_index = self.process_before.enumerate_steps(last_used_index);
    self.step_index = used_index + 1;
    self.step_index
  }
}
