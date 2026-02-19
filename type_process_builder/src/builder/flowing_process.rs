pub mod entry_flowing_process;
pub mod form_flowing_process;
pub mod operation_flowing_process;
pub mod subprocess;

use crate::builder::finalized_process::{FinalizedProcess, FlowingFinalizedProcess};
use crate::builder::form_flowing_process::FormFlowingProcess;
use crate::builder::operation_flowing_process::OperationFlowingProcess;
use crate::builder::split_process_splitter::SplitProcessSplitter;
use crate::builder::*;
use crate::param_list::ParamList;
use crate::param_list::clone_just::CloneJust;
use crate::param_list::concat::Concat;
use crate::param_list::transform::TransformTo;
use crate::step::{FailedInputValidationAttempts, Final, Form, Operation, Splitter};
use frunk_core::coproduct::Coproduct;
use std::future::Future;

/// Param value overlap is prevented by making reading them cumbersome <https://github.com/lloydmeta/frunk/issues/187>
/// Well you can work around this `limitation` by providing the indices explicitly
/// or replaceing [Concat] with [intersect::Intersect] in the implementation.
/// Don't do that. The params should be immutable to avoid the need to overwrite them with every session context save.
pub trait FlowingProcess: Sized + Sync {
  type ProcessBeforeProduces: ParamList;
  type Produces: ParamList;
  // add a dependent type for split process to pass values produced by the splitter step to this specific branch.

  fn resume_run(
    &self,
    previous_run_produced: SessionContext,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
    failed_input_validation_attempts: FailedInputValidationAttempts,
  ) -> impl Future<Output = IntermediateRunResult<Self::Produces>>;

  fn continue_run(
    &self,
    process_before_produces: Self::ProcessBeforeProduces,
  ) -> impl Future<Output = IntermediateRunResult<Self::Produces>>;

  fn then<
    OperationConsumes: ParamList,
    OperationProduces: ParamList + Concat<Self::Produces>,
    OperationStep: Operation<Consumes = OperationConsumes, Produces = OperationProduces>,
    ProcessBeforeProducesToLastStepConsumesIndices,
  >(
    self,
    step: OperationStep,
  ) -> impl FlowingProcess<
    ProcessBeforeProduces = Self::Produces,
    Produces = <OperationProduces as Concat<Self::Produces>>::Concatenated,
  >
  where
    for<'a> &'a Self::Produces: CloneJust<OperationConsumes, ProcessBeforeProducesToLastStepConsumesIndices>,
  {
    OperationFlowingProcess {
      process_before: self,
      last_step: step,
      step_index: WILL_BE_RENUMBERED,
      phantom_data: Default::default(),
    }
  }

  fn show<
    CreateFormConsumes: ParamList,
    ValidateInputConsumes: ParamList,
    FormProduces: ParamList + Concat<Self::Produces>,
    FormStep: Form<
        CreateFormConsumes = CreateFormConsumes,
        ValidateInputConsumes = ValidateInputConsumes,
        Produces = FormProduces,
      >,
    ProcessBeforeProducesToCreateFormConsumesIndices,
    ProcessBeforeProducesToValidateInputConsumesIndices,
  >(
    self,
    step: FormStep,
  ) -> impl FlowingProcess<
    ProcessBeforeProduces = Self::Produces,
    Produces = <FormProduces as Concat<Self::Produces>>::Concatenated,
  >
  where
    for<'a> &'a Self::Produces: CloneJust<CreateFormConsumes, ProcessBeforeProducesToCreateFormConsumesIndices>,
    for<'a> &'a Self::Produces: CloneJust<ValidateInputConsumes, ProcessBeforeProducesToValidateInputConsumesIndices>,
  {
    FormFlowingProcess {
      process_before: self,
      form_step: step,
      step_index: WILL_BE_RENUMBERED,
      phantom_data: Default::default(),
    }
  }

  fn split<
    Tag,
    SplitterStepConsumes: ParamList,
    SplitterProducesForFirstCase: ParamList + Concat<Self::Produces>,
    SplitterProducesForOtherCases,
    SplitterStep: Splitter<
        Consumes = SplitterStepConsumes,
        Produces = Coproduct<(Tag, SplitterProducesForFirstCase), SplitterProducesForOtherCases>,
      >,
    ProcessBeforeProducesToSplitterStepConsumesIndices,
  >(
    self,
    step: SplitterStep,
  ) -> impl SplitProcess<
    SplitterProducesForOtherCases,
    ProcessBeforeSplitProduces = Self::Produces,
    SplitterProducesForFirstCase = SplitterProducesForFirstCase,
    SplitterTagForFirstCase = Tag,
  >
  where
    for<'a> &'a Self::Produces: CloneJust<SplitterStepConsumes, ProcessBeforeProducesToSplitterStepConsumesIndices>,
  {
    SplitProcessSplitter::<
      Tag,
      Self,
      SplitterStepConsumes,
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

  fn end<
    FinalConsumes: ParamList,
    FinalStep: Final<Consumes = FinalConsumes>,
    ProcessBeforeProducesToLastStepConsumesIndices,
  >(
    self,
    step: FinalStep,
  ) -> impl FinalizedProcess<ProcessBeforeProduces = Self::Produces>
  where
    Self::Produces: TransformTo<FinalConsumes, ProcessBeforeProducesToLastStepConsumesIndices>,
  {
    FlowingFinalizedProcess {
      process_before: self,
      final_step: step,
      phantom_data: Default::default(),
    }
  }

  fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex;
}
