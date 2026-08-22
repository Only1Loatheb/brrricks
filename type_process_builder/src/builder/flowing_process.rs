use crate::step::BackToken;
pub mod entry_flowing_process;
pub mod form_flowing_process;
pub mod operation_flowing_process;
pub mod subprocess;

use crate::builder::borrow_just::BorrowJust;
use crate::builder::finalized_process::{FinalizedProcess, FlowingFinalizedProcess};
use crate::builder::form_flowing_process::FormFlowingProcess;
use crate::builder::operation_flowing_process::OperationFlowingProcess;
use crate::builder::split_process_form_splitter::SplitProcessFormSplitter;
use crate::builder::split_process_splitter::SplitProcessSplitter;
use crate::builder::{
  IntermediateRunResult, MaybeFormContext, PreviousRunYieldedAt, ProcessMessages, SessionContext, SplitProcess,
  StepIndex, WILL_BE_RENUMBERED,
};
use crate::frunk::coproduct::Coproduct;
use crate::param_list::ParamList;
use crate::param_list::concat::Concat;
use crate::param_list::extract::Extract;
use crate::step::{Final, Form, FormSplitter, Operation, Splitter};
use std::future::Future;

pub trait FlowingProcess: Sized + Send + Sync {
  // Please specify all associated types at the impl FlowingProcess side for inference to work.
  type ProcessBeforeProduces: ParamList;
  type Produces: ParamList;
  type SubprocessConsumes: ParamList;
  type Messages: ProcessMessages;
  type EverProduced: ParamList;

  fn resume_run(
    &self,
    previous_run_produced: SessionContext,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
    form_context: MaybeFormContext,
    back_token: Option<BackToken>,
  ) -> impl Future<Output = IntermediateRunResult<Self::Produces, Self::Messages>> + Send;

  fn continue_run(
    &self,
    process_before_produces: Self::ProcessBeforeProduces,
    back_token: Option<BackToken>,
  ) -> impl Future<Output = IntermediateRunResult<Self::Produces, Self::Messages>> + Send;

  fn run_subprocess(
    &self,
    subprocess_consumes: Self::SubprocessConsumes,
    back_token: Option<BackToken>,
  ) -> impl Future<Output = IntermediateRunResult<Self::Produces, Self::Messages>> + Send;

  fn then<
    OperationStep: Operation<FinalMessage = <Self::Messages as ProcessMessages>::FinalMessage>,
    ProcessBeforeProducesToLastStepConsumesIndices: Sync + Send,
  >(
    self,
    step: OperationStep,
  ) -> impl FlowingProcess<
    ProcessBeforeProduces = Self::Produces,
    Produces = <OperationStep::Produces as Concat<Self::Produces>>::Concatenated,
    SubprocessConsumes = Self::SubprocessConsumes,
    Messages = Self::Messages,
    EverProduced = <OperationStep::Produces as Concat<Self::EverProduced>>::Concatenated,
  >
  where
    OperationStep::Produces: ParamList + Concat<Self::Produces> + Concat<Self::EverProduced>,
    for<'a> &'a Self::Produces: BorrowJust<'a, OperationStep::Consumes, ProcessBeforeProducesToLastStepConsumesIndices>,
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
    ProcessBeforeProducesToCreateFormConsumesIndices: Sync + Send,
    ProcessBeforeProducesToValidateInputConsumesIndices: Sync + Send,
  >(
    self,
    step: FormStep,
  ) -> impl FlowingProcess<
    ProcessBeforeProduces = Self::Produces,
    Produces = <FormStep::Produces as Concat<Self::Produces>>::Concatenated,
    SubprocessConsumes = Self::SubprocessConsumes,
    Messages = Self::Messages,
    EverProduced = <FormStep::Produces as Concat<Self::EverProduced>>::Concatenated,
  >
  where
    FormStep::Produces: ParamList + Concat<Self::Produces> + Concat<Self::EverProduced>,
    for<'a> &'a Self::Produces:
      BorrowJust<'a, FormStep::CreateFormConsumes, ProcessBeforeProducesToCreateFormConsumesIndices>,
    for<'a> &'a Self::Produces:
      BorrowJust<'a, FormStep::ValidateInputConsumes, ProcessBeforeProducesToValidateInputConsumesIndices>,
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
    SplitterProducesForFirstCase: ParamList + Concat<Self::Produces> + Concat<Self::EverProduced>,
    SplitterProducesForOtherCases: Send + Sync,
    SplitterStep: Splitter<Produces = Coproduct<(Tag, SplitterProducesForFirstCase), SplitterProducesForOtherCases>>,
    ProcessBeforeProducesToSplitterStepConsumesIndices: Sync + Send,
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
    ProcessBeforeSplitEverProduced = Self::EverProduced,
    EverProduced = Self::EverProduced,
  >
  where
    for<'a> &'a Self::Produces:
      BorrowJust<'a, SplitterStep::Consumes, ProcessBeforeProducesToSplitterStepConsumesIndices>,
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

  /// The process execution will call [`SplitProcess::continue_run`] instead of the individual case
  /// [`FlowingProcess::continue_run`] method to avoid excessive nesting:
  /// ```compile_fail,E0425
  /// let _ = EntryA
  ///   .show_split(SplitA, |subprocess|
  ///     subprocess
  ///       .case_via(Case1, |x| x)
  ///       .case_via(Case2, |x| x.show(FormA))
  ///   )
  ///   .end(FinalA);
  /// ```
  /// and use the builder like this instead:
  /// ```
  /// # use type_process_builder::builder::*;
  /// # use type_process_builder::step::*;
  /// # use type_process_builder::{Coprod, HNil, ToRef};
  /// # struct Msg;
  /// # impl ProcessMessages for Msg { type FormMessage = String; type FinalMessage = String; }
  /// # struct EntryA;
  /// # impl Entry for EntryA { type Produces = HNil; type Messages = Msg; async fn handle(&self, _: SessionContext, _: String) -> anyhow::Result<HNil> { Ok(HNil) } }
  /// # struct Case1; struct Case2; struct SplitA;
  /// # impl FormSplitter for SplitA {
  /// #   type CreateFormConsumes = HNil; type ValidateInputConsumes = HNil; type Produces = Coprod![(Case1, HNil), (Case2, HNil)]; type Context = (); type Messages = Msg;
  /// #   async fn create_form(&self, _: <Self::CreateFormConsumes as ToRef<'_>>::Ref, _: Option<BackToken>) -> anyhow::Result<FormWithContext<String, ()>> { Ok(FormWithContext("".into(), ())) }
  /// #   async fn handle_input(&self, _: <Self::ValidateInputConsumes as ToRef<'_>>::Ref, _: String, _: (), _: Option<BackToken>) -> anyhow::Result<InputValidation<Self::Produces, Msg, ()>> { Ok(InputValidation::Successful(Self::Produces::inject((Case1, HNil)))) }
  /// # }
  /// # struct FormA;
  /// # impl Form for FormA {
  /// #   type CreateFormConsumes = HNil; type ValidateInputConsumes = HNil; type Produces = HNil; type Context = (); type Messages = Msg;
  /// #   async fn create_form(&self, _: <Self::CreateFormConsumes as ToRef<'_>>::Ref, _: Option<BackToken>) -> anyhow::Result<FormWithContext<String, ()>> { Ok(FormWithContext("".into(), ())) }
  /// #   async fn handle_input(&self, _: <Self::ValidateInputConsumes as ToRef<'_>>::Ref, _: String, _: (), _: Option<BackToken>) -> anyhow::Result<InputValidation<HNil, Msg, ()>> { Ok(InputValidation::Successful(HNil)) }
  /// # }
  /// # struct FinalA;
  /// # impl Final for FinalA { type Consumes = HNil; type FinalMessage = String; async fn handle(&self, _: HNil) -> anyhow::Result<String> { Ok("".into()) } }
  /// let _ = EntryA
  ///   .show_split(SplitA)
  ///   .case_via(Case1, |x| x)
  ///   .case_via(Case2, |x| x.show(FormA))
  ///   .end(FinalA);
  /// ```
  fn show_split<
    Tag: Send + Sync,
    SplitterProducesForFirstCase: ParamList + Concat<Self::Produces> + Concat<Self::EverProduced>,
    SplitterProducesForOtherCases: Send + Sync,
    SplitterStep: FormSplitter<
        Produces = Coproduct<(Tag, SplitterProducesForFirstCase), SplitterProducesForOtherCases>,
        Messages = Self::Messages,
      >,
    ProcessBeforeProducesToCreateFormConsumesIndices: Sync + Send,
    ProcessBeforeProducesToValidateInputConsumesIndices: Sync + Send,
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
    ProcessBeforeSplitEverProduced = Self::EverProduced,
    EverProduced = Self::EverProduced,
  >
  where
    for<'a> &'a Self::Produces:
      BorrowJust<'a, SplitterStep::CreateFormConsumes, ProcessBeforeProducesToCreateFormConsumesIndices>,
    for<'a> &'a Self::Produces:
      BorrowJust<'a, SplitterStep::ValidateInputConsumes, ProcessBeforeProducesToValidateInputConsumesIndices>,
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
    ProcessBeforeProducesToLastStepConsumesIndices: Sync + Send,
  >(
    self,
    step: FinalStep,
  ) -> impl FinalizedProcess<
    ProcessBeforeProduces = Self::Produces,
    SubprocessConsumes = Self::SubprocessConsumes,
    Messages = Self::Messages,
    EverProduced = Self::EverProduced,
  >
  where
    Self::Produces: Extract<FinalStep::Consumes, ProcessBeforeProducesToLastStepConsumesIndices>,
  {
    FlowingFinalizedProcess { process_before: self, final_step: step, phantom_data: Default::default() }
  }

  fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex;
}
