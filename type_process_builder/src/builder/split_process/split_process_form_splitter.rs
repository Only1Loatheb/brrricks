use crate::builder::{
  CurrentRunYieldedAt, FlowingProcess, IntermediateFinalizedSplitOutcome, IntermediateFinalizedSplitResult,
  IntermediateRunOutcome, ParamList, ParamUID, PreviousRunYieldedAt, SessionContext, SplitProcess, StepIndex,
};
use crate::param_list::clone_just::CloneJust;
use crate::param_list::concat::Concat;
use crate::step::{FailedInputValidationAttempts, FormSplitter, InputValidation};
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
  ProcessBeforeProducesToCreateFormConsumesIndices: Sync,
  ProcessBeforeProducesToValidateInputConsumesIndices: Sync,
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
    CloneJust<SplitterStep::CreateFormConsumes, ProcessBeforeProducesToCreateFormConsumesIndices>,
  for<'a> &'a ProcessBefore::Produces:
    CloneJust<SplitterStep::ValidateInputConsumes, ProcessBeforeProducesToValidateInputConsumesIndices>,
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
    failed_input_validation_attempts: FailedInputValidationAttempts,
  ) -> IntermediateFinalizedSplitResult<
    Self::ProcessBeforeSplitProduces,
    Coproduct<Self::SplitterProducesForFirstCase, SplitterProducesForOtherCases>,
    Self::Messages,
  > {
    if previous_run_yielded_at.0 < self.step_index {
      let process_before_output = self
        .process_before
        .resume_run(previous_run_produced, previous_run_yielded_at, user_input, failed_input_validation_attempts)
        .await?;
      match process_before_output {
        IntermediateRunOutcome::Continue(process_before_split_produced) => {
          self.continue_run(process_before_split_produced).await
        },
        IntermediateRunOutcome::Yield(a, b, c) => Ok(IntermediateFinalizedSplitOutcome::Yield(a, b, c)),
        IntermediateRunOutcome::Finish(a) => Ok(IntermediateFinalizedSplitOutcome::Finish(a)),
        IntermediateRunOutcome::RetryUserInput(a) => Ok(IntermediateFinalizedSplitOutcome::RetryUserInput(a)),
      }
    } else {
      let process_before_split_produced = ProcessBefore::Produces::deserialize(previous_run_produced)?;
      let last_step_consumes = (&process_before_split_produced).clone_just();
      match self.splitter.handle_input(last_step_consumes, user_input, failed_input_validation_attempts).await? {
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
        InputValidation::Retry(a) => Ok(IntermediateFinalizedSplitOutcome::RetryUserInput(a)),
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
    let splitter_step_consumes = (&process_before_split_produced).clone_just();
    Ok(IntermediateFinalizedSplitOutcome::Yield(
      self.splitter.create_form(splitter_step_consumes).await?,
      process_before_split_produced.serialize()?,
      CurrentRunYieldedAt(self.step_index),
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
      IntermediateRunOutcome::Yield(a, b, c) => Ok(IntermediateFinalizedSplitOutcome::Yield(a, b, c)),
      IntermediateRunOutcome::Finish(a) => Ok(IntermediateFinalizedSplitOutcome::Finish(a)),
      IntermediateRunOutcome::RetryUserInput(a) => Ok(IntermediateFinalizedSplitOutcome::RetryUserInput(a)),
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
