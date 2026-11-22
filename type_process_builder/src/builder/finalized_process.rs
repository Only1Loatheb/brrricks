use crate::builder::MapProcessError;
use crate::builder::MapStepErr;
use crate::builder::flowing_process::FlowingProcess;
use crate::builder::runnable_process::RunnableProcess;
use crate::builder::{
  IntermediateRunOutcome, MaybeFormContext, PreviousRunYieldedAt, RunOutcome, RunResult, SessionContext, StepIndex,
};
use crate::error_list::union::CoprodUnion;
use crate::param_list::extract::Extract;
use crate::step::BackToken;
use crate::step::{Final, ProcessMessages};
use std::marker::PhantomData;

pub(crate) mod sealed {
  use crate::builder::{MaybeFormContext, PreviousRunYieldedAt, RunResult, SessionContext, StepIndex};
  use crate::error_list::ErrorCoproduct;
  use crate::param_list::ParamList;
  use crate::step::{BackToken, ProcessMessages};
  use std::future::Future;

  pub trait FinalizedProcess: Sized + Send + Sync {
    // Please specify all associated types at the impl FinalizedProcess side for inference to work.
    type EntryConsumes: ParamList;
    type ProcessBeforeProduces: ParamList;
    type SubprocessConsumes: ParamList;
    type Messages: ProcessMessages;
    type EverProduced: ParamList;
    type Errors: ErrorCoproduct;

    fn resume_run(
      &self,
      previous_run_produced: SessionContext,
      previous_run_yielded_at: PreviousRunYieldedAt,
      user_input: String,
      form_context: MaybeFormContext,
      back_token: Option<BackToken>,
    ) -> impl Future<Output = RunResult<Self::Messages, Self::Errors>> + Send;

    fn continue_run(
      &self,
      process_before_produces: Self::ProcessBeforeProduces,
      back_token: Option<BackToken>,
    ) -> impl Future<Output = RunResult<Self::Messages, Self::Errors>> + Send;

    fn run_subprocess(
      &self,
      subprocess_consumes: Self::SubprocessConsumes,
      back_token: Option<BackToken>,
    ) -> impl Future<Output = RunResult<Self::Messages, Self::Errors>> + Send;

    fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex;
  }
}

pub trait FinalizedProcess: sealed::FinalizedProcess {
  fn build(mut self, name: &'static str, version: u32) -> RunnableProcess<Self> {
    self.enumerate_steps(StepIndex::MIN);
    RunnableProcess { finalized_process: self, name, version }
  }
}

impl<T: sealed::FinalizedProcess> FinalizedProcess for T {}

pub struct FlowingFinalizedProcess<
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
  for FlowingFinalizedProcess<
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
  type ProcessBeforeProduces = ProcessBefore::Produces;
  type SubprocessConsumes = ProcessBefore::SubprocessConsumes;
  type Messages = ProcessBefore::Messages;
  type EverProduced = ProcessBefore::EverProduced;
  type Errors = <FinalStep::Error as CoprodUnion<ProcessBefore::Errors, ErrorUnionIndices>>::Union;

  async fn resume_run(
    // check where to resume when copying and pasting to finalized proces with finalized process instead of last case
    &self,
    previous_run_produced: SessionContext,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
    form_context: MaybeFormContext,
    back_token: Option<BackToken>,
  ) -> RunResult<Self::Messages, Self::Errors> {
    let outcome = self
      .process_before
      .resume_run(previous_run_produced, previous_run_yielded_at, user_input, form_context, back_token)
      .await
      .map_rhs::<FinalStep::Error, _>()?;
    match outcome {
      IntermediateRunOutcome::Continue(val) => self.continue_run(val, back_token).await,
      IntermediateRunOutcome::Yield(a, b, c, d) => Ok(RunOutcome::Yield(a, b, c, d)),
      IntermediateRunOutcome::Finish(a) => Ok(RunOutcome::Finish(a)),
      IntermediateRunOutcome::RetryUserInput(a, b) => Ok(RunOutcome::RetryUserInput(a, b)),
      IntermediateRunOutcome::Back => Ok(RunOutcome::Back),
    }
  }

  async fn continue_run(
    &self,
    process_before_produces: Self::ProcessBeforeProduces,
    _back_token: Option<BackToken>,
  ) -> RunResult<Self::Messages, Self::Errors> {
    Ok(RunOutcome::Finish(self.final_step.handle(process_before_produces.extract()).await.map_step_lhs()?))
  }

  async fn run_subprocess(
    &self,
    subprocess_consumes: Self::SubprocessConsumes,
    back_token: Option<BackToken>,
  ) -> RunResult<Self::Messages, Self::Errors> {
    let outcome =
      self.process_before.run_subprocess(subprocess_consumes, back_token).await.map_rhs::<FinalStep::Error, _>()?;
    match outcome {
      IntermediateRunOutcome::Continue(val) => self.continue_run(val, back_token).await,
      IntermediateRunOutcome::Yield(a, b, c, d) => Ok(RunOutcome::Yield(a, b, c, d)),
      IntermediateRunOutcome::Finish(a) => Ok(RunOutcome::Finish(a)),
      IntermediateRunOutcome::RetryUserInput(a, b) => Ok(RunOutcome::RetryUserInput(a, b)),
      IntermediateRunOutcome::Back => Ok(RunOutcome::Back),
    }
  }

  fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex {
    // most likely not worth to assign an index to final steps, but maybe test
    self.process_before.enumerate_steps(last_used_index)
  }
}
