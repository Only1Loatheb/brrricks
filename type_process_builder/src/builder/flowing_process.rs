pub mod entry_flowing_process;
pub mod form_flowing_process;
pub mod operation_flowing_process;
pub mod subprocess;

use crate::builder::finalized_process::{FinalizedProcess, FlowingFinalizedProcess};
use crate::builder::form_flowing_process::FormFlowingProcess;
use crate::builder::operation_flowing_process::OperationFlowingProcess;
use crate::builder::split_process_form_splitter::SplitProcessFormSplitter;
use crate::builder::split_process_splitter::SplitProcessSplitter;
use crate::builder::*;
use crate::param_list::ParamList;
use crate::param_list::clone_just::CloneJust;
use crate::param_list::concat::Concat;
use crate::param_list::transform::TransformTo;
use crate::step::{FailedInputValidationAttempts, Final, Form, FormSplitter, Operation, Splitter};
use frunk_core::coproduct::Coproduct;
use std::future::Future;

/// Param value overlap is prevented by making reading them cumbersome <https://github.com/lloydmeta/frunk/issues/187>
/// Well you can work around this `limitation` by providing the indices explicitly
/// or replaceing [Concat] with [intersect::Intersect] in the implementation.
/// Don't do that. The params should be immutable to avoid the need to overwrite them with every session context save.
pub trait FlowingProcess: Sized + Sync {
  // Please specify all associated types at the impl FlowingProcess side for inference to work.
  type ProcessBeforeProduces: ParamList;
  type Produces: ParamList;
  type SubprocessConsumes: ParamList;
  type Messages: ProcessMessages;

  fn resume_run(
    &self,
    previous_run_produced: SessionContext,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
    failed_input_validation_attempts: FailedInputValidationAttempts,
  ) -> impl Future<Output = IntermediateRunResult<Self::Produces, Self::Messages>> + Send;

  fn continue_run(
    &self,
    process_before_produces: Self::ProcessBeforeProduces,
  ) -> impl Future<Output = IntermediateRunResult<Self::Produces, Self::Messages>> + Send;

  fn run_subprocess(
    &self,
    subprocess_consumes: Self::SubprocessConsumes,
  ) -> impl Future<Output = IntermediateRunResult<Self::Produces, Self::Messages>> + Send;

  fn then<
    OperationStep: Operation<FinalMessage = <Self::Messages as ProcessMessages>::FinalMessage>,
    ProcessBeforeProducesToLastStepConsumesIndices: Sync,
  >(
    self,
    step: OperationStep,
  ) -> impl FlowingProcess<
    ProcessBeforeProduces = Self::Produces,
    Produces = <OperationStep::Produces as Concat<Self::Produces>>::Concatenated,
    SubprocessConsumes = Self::SubprocessConsumes,
    Messages = Self::Messages,
  >
  where
    OperationStep::Produces: ParamList + Concat<Self::Produces>,
    for<'a> &'a Self::Produces: CloneJust<OperationStep::Consumes, ProcessBeforeProducesToLastStepConsumesIndices>,
  {
    OperationFlowingProcess {
      process_before: self,
      last_step: step,
      step_index: WILL_BE_RENUMBERED,
      phantom_data: Default::default(),
    }
  }

  fn show<
    FormStep: Form<Messages = Self::Messages>,
    ProcessBeforeProducesToCreateFormConsumesIndices: Sync,
    ProcessBeforeProducesToValidateInputConsumesIndices: Sync,
  >(
    self,
    step: FormStep,
  ) -> impl FlowingProcess<
    ProcessBeforeProduces = Self::Produces,
    Produces = <FormStep::Produces as Concat<Self::Produces>>::Concatenated,
    SubprocessConsumes = Self::SubprocessConsumes,
    Messages = Self::Messages,
  >
  where
    FormStep::Produces: ParamList + Concat<Self::Produces>,
    for<'a> &'a Self::Produces:
      CloneJust<FormStep::CreateFormConsumes, ProcessBeforeProducesToCreateFormConsumesIndices>,
    for<'a> &'a Self::Produces:
      CloneJust<FormStep::ValidateInputConsumes, ProcessBeforeProducesToValidateInputConsumesIndices>,
  {
    FormFlowingProcess {
      process_before: self,
      form_step: step,
      step_index: WILL_BE_RENUMBERED,
      phantom_data: Default::default(),
    }
  }

  fn split<
    Tag: Send + Sync,
    SplitterProducesForFirstCase: ParamList + Concat<Self::Produces>,
    SplitterProducesForOtherCases: Send + Sync,
    SplitterStep: Splitter<Produces = Coproduct<(Tag, SplitterProducesForFirstCase), SplitterProducesForOtherCases>>,
    ProcessBeforeProducesToSplitterStepConsumesIndices: Sync,
  >(
    self,
    step: SplitterStep,
  ) -> impl SplitProcess<
    SplitterProducesForOtherCases,
    ProcessBeforeSplitProduces = Self::Produces,
    SplitterProducesForFirstCase = SplitterProducesForFirstCase,
    SplitterTagForFirstCase = Tag,
    SubprocessConsumes = Self::SubprocessConsumes,
    Messages = Self::Messages,
  >
  where
    for<'a> &'a Self::Produces: CloneJust<SplitterStep::Consumes, ProcessBeforeProducesToSplitterStepConsumesIndices>,
  {
    SplitProcessSplitter::<
      Tag,
      Self,
      SplitterProducesForFirstCase,
      SplitterProducesForOtherCases,
      SplitterStep,
      ProcessBeforeProducesToSplitterStepConsumesIndices,
    > {
      process_before: self,
      splitter: step,
      step_index: WILL_BE_RENUMBERED,
      phantom_data: Default::default(),
    }
  }

  fn show_split<
    Tag: Send + Sync,
    SplitterProducesForFirstCase: ParamList + Concat<Self::Produces>,
    SplitterProducesForOtherCases: Send + Sync,
    SplitterStep: FormSplitter<
        Produces = Coproduct<(Tag, SplitterProducesForFirstCase), SplitterProducesForOtherCases>,
        Messages = Self::Messages,
      >,
    ProcessBeforeProducesToCreateFormConsumesIndices: Sync,
    ProcessBeforeProducesToValidateInputConsumesIndices: Sync,
  >(
    self,
    step: SplitterStep,
  ) -> impl SplitProcess<
    SplitterProducesForOtherCases,
    ProcessBeforeSplitProduces = Self::Produces,
    SplitterProducesForFirstCase = SplitterProducesForFirstCase,
    SplitterTagForFirstCase = Tag,
    SubprocessConsumes = Self::SubprocessConsumes,
    Messages = Self::Messages,
  >
  where
    for<'a> &'a Self::Produces:
      CloneJust<SplitterStep::CreateFormConsumes, ProcessBeforeProducesToCreateFormConsumesIndices>,
    for<'a> &'a Self::Produces:
      CloneJust<SplitterStep::ValidateInputConsumes, ProcessBeforeProducesToValidateInputConsumesIndices>,
  {
    SplitProcessFormSplitter::<
      Tag,
      Self,
      SplitterProducesForFirstCase,
      SplitterProducesForOtherCases,
      SplitterStep,
      ProcessBeforeProducesToCreateFormConsumesIndices,
      ProcessBeforeProducesToValidateInputConsumesIndices,
    > {
      process_before: self,
      splitter: step,
      step_index: WILL_BE_RENUMBERED,
      phantom_data: Default::default(),
    }
  }

  fn end<
    FinalStep: Final<FinalMessage = <Self::Messages as ProcessMessages>::FinalMessage>,
    ProcessBeforeProducesToLastStepConsumesIndices: Sync,
  >(
    self,
    step: FinalStep,
  ) -> impl FinalizedProcess<
    ProcessBeforeProduces = Self::Produces,
    SubprocessConsumes = Self::SubprocessConsumes,
    Messages = Self::Messages,
  >
  where
    Self::Produces: TransformTo<FinalStep::Consumes, ProcessBeforeProducesToLastStepConsumesIndices>,
  {
    FlowingFinalizedProcess { process_before: self, final_step: step, phantom_data: Default::default() }
  }

  fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex;

  fn all_param_uids(&self, acc: &mut Vec<ParamUID>);
}
