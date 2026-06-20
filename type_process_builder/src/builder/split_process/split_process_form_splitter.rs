use crate::builder::{
  CurrentRunYieldedAt, FlowingProcess, IntermediateFinalizedSplitOutcome, IntermediateFinalizedSplitResult,
  IntermediateRunOutcome, MaybeFormContext, ParamList, ParamUID, PreviousRunYieldedAt, SessionContext, SplitProcess,
  StepIndex,
};
use crate::param_list::borrow_just::BorrowJust;
use crate::param_list::concat::Concat;
use crate::step::{FormSplitter, FormWithContext, InputValidation};
use anyhow::anyhow;
use frunk_core::coproduct::Coproduct;
use std::marker::PhantomData;

pub struct SplitProcessFormSplitter<
  Tag: Send + Sync,
  ProcessBefore: FlowingProcess,
  SplitterProducesForFirstCase: ParamList,
  SplitterProducesForOtherCases: Send + Sync,
  SplitterStep: FormSplitter<Produces = Coproduct<(Tag, SplitterProducesForFirstCase), SplitterProducesForOtherCases>>,
  ProcessBeforeProducesToCreateFormConsumesIndices,
  ProcessBeforeProducesToValidateInputConsumesIndices,
> {
  pub process_before: ProcessBefore,
  pub splitter: SplitterStep,
  pub step_index: StepIndex,
  pub phantom_data: PhantomData<(
    ProcessBeforeProducesToCreateFormConsumesIndices,
    ProcessBeforeProducesToValidateInputConsumesIndices,
  )>,
}

impl<
  Tag: Send + Sync,
  ProcessBefore: FlowingProcess,
  SplitterProducesForFirstCase: ParamList + Concat<ProcessBefore::Produces>,
  SplitterProducesForOtherCases: Send + Sync,
  SplitterStep: FormSplitter<
      Produces = Coproduct<(Tag, SplitterProducesForFirstCase), SplitterProducesForOtherCases>,
      Messages = ProcessBefore::Messages,
    >,
  ProcessBeforeProducesToCreateFormConsumesIndices: Sync + Send,
  ProcessBeforeProducesToValidateInputConsumesIndices: Sync + Send,
> SplitProcess<SplitterProducesForOtherCases>
  for SplitProcessFormSplitter<
    Tag,
    ProcessBefore,
    SplitterProducesForFirstCase,
    SplitterProducesForOtherCases,
    SplitterStep,
    ProcessBeforeProducesToCreateFormConsumesIndices,
    ProcessBeforeProducesToValidateInputConsumesIndices,
  >
where
  for<'a> &'a ProcessBefore::Produces:
    BorrowJust<'a, SplitterStep::CreateFormConsumes, ProcessBeforeProducesToCreateFormConsumesIndices>,
  for<'a> &'a ProcessBefore::Produces:
    BorrowJust<'a, SplitterStep::ValidateInputConsumes, ProcessBeforeProducesToValidateInputConsumesIndices>,
{
  type ProcessBeforeSplitProduces = ProcessBefore::Produces;
  type SplitterProducesForFirstCase = SplitterProducesForFirstCase;
  type SplitterTagForFirstCase = Tag;
  type SubprocessConsumes = ProcessBefore::SubprocessConsumes;
  type Messages = ProcessBefore::Messages;

  async fn resume_run(
    &self,
    previous_run_produced: SessionContext,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
    form_context: MaybeFormContext,
  ) -> IntermediateFinalizedSplitResult<
    Self::ProcessBeforeSplitProduces,
    Coproduct<Self::SplitterProducesForFirstCase, SplitterProducesForOtherCases>,
    Self::Messages,
  > {
    if previous_run_yielded_at.0 < self.step_index {
      let process_before_output = self
        .process_before
        .resume_run(previous_run_produced, previous_run_yielded_at, user_input, form_context)
        .await?;
      match process_before_output {
        IntermediateRunOutcome::Continue(process_before_split_produced) => {
          self.continue_run(process_before_split_produced).await
        },
        IntermediateRunOutcome::Yield(a, b, c, d) => Ok(IntermediateFinalizedSplitOutcome::Yield(a, b, c, d)),
        IntermediateRunOutcome::Finish(a) => Ok(IntermediateFinalizedSplitOutcome::Finish(a)),
        IntermediateRunOutcome::RetryUserInput(a, b) => Ok(IntermediateFinalizedSplitOutcome::RetryUserInput(a, b)),
      }
    } else {
      let process_before_split_produced = ProcessBefore::Produces::deserialize(previous_run_produced)?;
      let last_step_consumes =
        <&ProcessBefore::Produces as BorrowJust<'_, SplitterStep::ValidateInputConsumes, _>>::borrow_just(
          &process_before_split_produced,
        );
      let context: SplitterStep::Context = postcard::from_bytes(&form_context.ok_or(anyhow!("Missing FormContext"))?)?;
      match self.splitter.handle_input(last_step_consumes, user_input, context).await? {
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
          Ok(IntermediateFinalizedSplitOutcome::RetryUserInput(a, postcard::to_allocvec(&b)?))
        },
        InputValidation::Finish(a) => Ok(IntermediateFinalizedSplitOutcome::Finish(a)),
      }
    }
  }

  async fn continue_run(
    &self,
    process_before_split_produced: Self::ProcessBeforeSplitProduces,
  ) -> IntermediateFinalizedSplitResult<
    Self::ProcessBeforeSplitProduces,
    Coproduct<Self::SplitterProducesForFirstCase, SplitterProducesForOtherCases>,
    Self::Messages,
  > {
    let splitter_step_consumes =
      <&ProcessBefore::Produces as BorrowJust<'_, SplitterStep::CreateFormConsumes, _>>::borrow_just(
        &process_before_split_produced,
      );
    let FormWithContext(form, form_context) = self.splitter.create_form(splitter_step_consumes).await?;
    Ok(IntermediateFinalizedSplitOutcome::Yield(
      form,
      process_before_split_produced.serialize()?,
      CurrentRunYieldedAt(self.step_index),
      postcard::to_allocvec(&form_context)?,
    ))
  }

  async fn run_subprocess(
    &self,
    subprocess_consumes: Self::SubprocessConsumes,
  ) -> IntermediateFinalizedSplitResult<
    Self::ProcessBeforeSplitProduces,
    Coproduct<Self::SplitterProducesForFirstCase, SplitterProducesForOtherCases>,
    Self::Messages,
  > {
    let process_before_output = self.process_before.run_subprocess(subprocess_consumes).await?;
    match process_before_output {
      IntermediateRunOutcome::Continue(process_before_split_produced) => {
        self.continue_run(process_before_split_produced).await
      },
      IntermediateRunOutcome::Yield(a, b, c, d) => Ok(IntermediateFinalizedSplitOutcome::Yield(a, b, c, d)),
      IntermediateRunOutcome::Finish(a) => Ok(IntermediateFinalizedSplitOutcome::Finish(a)),
      IntermediateRunOutcome::RetryUserInput(a, b) => Ok(IntermediateFinalizedSplitOutcome::RetryUserInput(a, b)),
    }
  }

  fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex {
    let used_index = self.process_before.enumerate_steps(last_used_index);
    self.step_index = used_index + 1;
    self.step_index
  }

  fn all_param_uids(&self, acc: &mut Vec<ParamUID>) {
    self.process_before.all_param_uids(acc);
  }
}
