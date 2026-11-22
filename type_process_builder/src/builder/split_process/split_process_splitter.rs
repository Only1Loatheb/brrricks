use crate::builder::MapProcessError;
use crate::builder::MapStepErr;
use crate::builder::split_process::sealed;
use crate::builder::{
  FlowingProcess, IntermediateFinalizedSplitOutcome, IntermediateFinalizedSplitResult, IntermediateRunOutcome,
  MaybeFormContext, ParamList, PreviousRunYieldedAt, SessionContext, StepIndex,
};
use crate::error_list::union::CoprodUnion;
use crate::frunk::coproduct::Coproduct;
use crate::param_list::borrow_just::BorrowJust;
use crate::param_list::concat::Concat;
use crate::step::BackToken;
use crate::step::Splitter;
use std::marker::PhantomData;

pub struct SplitProcessSplitter<
  Tag: Send + Sync,
  ProcessBefore: FlowingProcess,
  SplitterProducesForFirstCase: ParamList,
  SplitterProducesForOtherCases,
  SplitterStep: Splitter<Produces = Coproduct<(Tag, SplitterProducesForFirstCase), SplitterProducesForOtherCases>>,
  ProcessBeforeProducesToSplitterStepConsumesIndices,
  ErrorUnionIndices,
> {
  pub(crate) process_before: ProcessBefore,
  pub(crate) splitter: SplitterStep,
  pub(crate) phantom_data: PhantomData<(ProcessBeforeProducesToSplitterStepConsumesIndices, ErrorUnionIndices)>,
}

impl<
  Tag: Send + Sync,
  ProcessBefore: FlowingProcess,
  SplitterProducesForFirstCase: ParamList + Concat<ProcessBefore::Produces> + Concat<ProcessBefore::EverProduced>,
  SplitterProducesForOtherCases: Send + Sync,
  SplitterStep: Splitter<Produces = Coproduct<(Tag, SplitterProducesForFirstCase), SplitterProducesForOtherCases>>,
  ProcessBeforeProducesToSplitterStepConsumesIndices: Sync + Send,
  ErrorUnionIndices: Sync + Send,
> sealed::SplitProcess<SplitterProducesForOtherCases>
  for SplitProcessSplitter<
    Tag,
    ProcessBefore,
    SplitterProducesForFirstCase,
    SplitterProducesForOtherCases,
    SplitterStep,
    ProcessBeforeProducesToSplitterStepConsumesIndices,
    ErrorUnionIndices,
  >
where
  SplitterStep::Error: CoprodUnion<ProcessBefore::Errors, ErrorUnionIndices>,
  for<'a> &'a ProcessBefore::Produces:
    BorrowJust<'a, SplitterStep::Consumes, ProcessBeforeProducesToSplitterStepConsumesIndices>,
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
    let process_before_output = self
      .process_before
      .resume_run(previous_run_produced, previous_run_yielded_at, user_input, form_context, back_token)
      .await
      .map_rhs::<SplitterStep::Error, _>()?;
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

  async fn continue_run(
    &self,
    process_before_split_produced: Self::ProcessBeforeSplitProduces,
    _back_token: Option<BackToken>,
  ) -> IntermediateFinalizedSplitResult<
    Self::ProcessBeforeSplitProduces,
    Coproduct<Self::SplitterProducesForFirstCase, SplitterProducesForOtherCases>,
    Self::Messages,
    Self::Errors,
  > {
    let splitter_step_consumes = (&process_before_split_produced).borrow_just();
    let splitter_produces_to_other_cases = match self.splitter.handle(splitter_step_consumes).await.map_step_lhs()? {
      Coproduct::Inl(a) => Coproduct::Inl(a.1),
      Coproduct::Inr(b) => Coproduct::Inr(b),
    };
    Ok(IntermediateFinalizedSplitOutcome::GoToCase { process_before_split_produced, splitter_produces_to_other_cases })
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
      self.process_before.run_subprocess(subprocess_consumes, back_token).await.map_rhs::<SplitterStep::Error, _>()?;
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

  /// Assigning an index is unnecessary because a process never yields at [`Splitter`] step
  fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex {
    self.process_before.enumerate_steps(last_used_index)
  }
}
