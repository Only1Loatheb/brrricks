pub mod case_entry;
pub mod entry_flowing_process;
pub mod form_flowing_process;
pub mod operation_flowing_process;
pub mod subprocess_flowing_process;

use crate::builder::finalized_process::FinalizedProcess;
use crate::builder::finalized_process::final_finalized_process::FinalFinalizedProcess;
use crate::builder::finalized_process::subprocess_finalized_process::SubprocessFinalizedProcess;
use crate::builder::flowing_process::form_flowing_process::FormFlowingProcess;
use crate::builder::flowing_process::operation_flowing_process::OperationFlowingProcess;
use crate::builder::flowing_process::subprocess_flowing_process::SubprocessFlowingProcess;
use crate::builder::split_process::split_process_form_splitter::SplitProcessFormSplitter;
use crate::builder::split_process::split_process_splitter::SplitProcessSplitter;
use crate::builder::{ProcessMessages, SplitProcess, WILL_BE_RENUMBERED};
use crate::error_coproduct::union::CoprodUnion;
use crate::frunk::coproduct::Coproduct;
use crate::param_list::ParamList;
use crate::param_list::borrow_just::BorrowJust;
use crate::param_list::concat::Concat;
use crate::param_list::extract::Extract;
use crate::step::{Final, Form, FormSplitter, Operation, Splitter};

pub(crate) mod sealed {
  use crate::builder::{IntermediateRunResult, PreviousRunYieldedAt, StepIndex};
  use crate::error_coproduct::ErrorCoproduct;
  use crate::param_list::ParamList;
  use crate::step::{BackToken, ProcessMessages};
  use std::future::Future;

  pub trait FlowingProcess: Sized + Send + Sync {
    // Please specify all associated types at the impl FlowingProcess side for inference to work.
    type EntryConsumes: ParamList;
    type ProcessBeforeProduces: ParamList;
    type Produces: ParamList;
    type Messages: ProcessMessages;
    type EverProduced: ParamList;
    type Errors: ErrorCoproduct;

    fn resume_run(
      &self,
      previous_run_produced: &[u8],
      previous_run_yielded_at: PreviousRunYieldedAt,
      user_input: String,
      form_context: Option<&[u8]>,
      back_token: Option<BackToken>,
    ) -> impl Future<Output = IntermediateRunResult<Self::Produces, Self::Messages, Self::Errors>> + Send;

    fn continue_run(
      &self,
      process_before_produces: Self::ProcessBeforeProduces,
      back_token: Option<BackToken>,
    ) -> impl Future<Output = IntermediateRunResult<Self::Produces, Self::Messages, Self::Errors>> + Send;

    fn run_from_entry(
      &self,
      entry_consumes: Self::EntryConsumes,
      back_token: Option<BackToken>,
    ) -> impl Future<Output = IntermediateRunResult<Self::Produces, Self::Messages, Self::Errors>> + Send;

    fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex;
  }
}

#[must_use]
pub trait FlowingProcess: sealed::FlowingProcess {
  fn then<
    OperationStep: Operation<FinalMessage = <Self::Messages as ProcessMessages>::FinalMessage>,
    ProcessBeforeProducesToLastStepConsumesIndices: Sync + Send,
    ErrorUnionIndices: Sync + Send,
  >(
    self,
    step: OperationStep,
  ) -> impl FlowingProcess<
    EntryConsumes = Self::EntryConsumes,
    ProcessBeforeProduces = Self::Produces,
    Produces = <OperationStep::Produces as Concat<Self::Produces>>::Concatenated,
    Messages = Self::Messages,
    EverProduced = <OperationStep::Produces as Concat<Self::EverProduced>>::Concatenated,
    Errors = <OperationStep::Error as CoprodUnion<Self::Errors, ErrorUnionIndices>>::Union,
  >
  where
    OperationStep::Produces: ParamList + Concat<Self::Produces> + Concat<Self::EverProduced>,
    OperationStep::Error: CoprodUnion<Self::Errors, ErrorUnionIndices>,
    for<'a> &'a Self::Produces: BorrowJust<'a, OperationStep::Consumes, ProcessBeforeProducesToLastStepConsumesIndices>,
  {
    OperationFlowingProcess { process_before: self, last_step: step, phantom_data: Default::default() }
  }

  fn then_via<
    Sub: FlowingProcess<Messages = Self::Messages>,
    ProcessBeforeProducesToEntryConsumesIndices: Sync + Send,
    SubEverProducedExtractIndices: Sync + Send,
    ErrorUnionIndices: Sync + Send,
  >(
    self,
    sub: Sub,
  ) -> impl FlowingProcess<
    EntryConsumes = Self::EntryConsumes,
    ProcessBeforeProduces = Self::Produces,
    Produces = <Sub::Produces as Concat<
      <Self::Produces as Extract<
        Sub::EntryConsumes,
        ProcessBeforeProducesToEntryConsumesIndices,
      >>::Remainder,
    >>::Concatenated,
    Messages = Self::Messages,
    EverProduced = <<Sub::EverProduced as Extract<
      Sub::EntryConsumes,
      SubEverProducedExtractIndices,
    >>::Remainder as Concat<Self::EverProduced>>::Concatenated,
    Errors = <Sub::Errors as CoprodUnion<Self::Errors, ErrorUnionIndices>>::Union,
  >
  where
    Self::Produces: Extract<Sub::EntryConsumes, ProcessBeforeProducesToEntryConsumesIndices>,
    <Self::Produces as Extract<
      Sub::EntryConsumes,
      ProcessBeforeProducesToEntryConsumesIndices,
    >>::Remainder: ParamList,
    Sub::Produces: Concat<
      <Self::Produces as Extract<
        Sub::EntryConsumes,
        ProcessBeforeProducesToEntryConsumesIndices,
      >>::Remainder,
    >,
    Sub::EverProduced: Extract<Sub::EntryConsumes, SubEverProducedExtractIndices>,
    <Sub::EverProduced as Extract<Sub::EntryConsumes, SubEverProducedExtractIndices>>::Remainder:
      ParamList + Concat<Self::EverProduced>,
    Sub::Errors: CoprodUnion<Self::Errors, ErrorUnionIndices>,
{
    SubprocessFlowingProcess {
      process_before: self,
      sub_process: sub,
      sub_start_index: WILL_BE_RENUMBERED,
      sub_end_index: WILL_BE_RENUMBERED,
      phantom_data: Default::default(),
    }
  }

  fn show<
    FormStep: Form<Messages = Self::Messages>,
    ProcessBeforeProducesToCreateFormConsumesIndices: Sync + Send,
    ProcessBeforeProducesToValidateInputConsumesIndices: Sync + Send,
    ErrorUnionIndices: Sync + Send,
  >(
    self,
    step: FormStep,
  ) -> impl FlowingProcess<
    EntryConsumes = Self::EntryConsumes,
    ProcessBeforeProduces = Self::Produces,
    Produces = <FormStep::Produces as Concat<Self::Produces>>::Concatenated,
    Messages = Self::Messages,
    EverProduced = <FormStep::Produces as Concat<Self::EverProduced>>::Concatenated,
    Errors = <FormStep::Error as CoprodUnion<Self::Errors, ErrorUnionIndices>>::Union,
  >
  where
    FormStep::Produces: ParamList + Concat<Self::Produces> + Concat<Self::EverProduced>,
    FormStep::Error: CoprodUnion<Self::Errors, ErrorUnionIndices>,
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
    ErrorUnionIndices: Sync + Send,
  >(
    self,
    step: SplitterStep,
  ) -> impl SplitProcess<
    SplitterProducesForOtherCases,
    EntryConsumes = Self::EntryConsumes,
    ProcessBeforeSplitProduces = Self::Produces,
    SplitterProducesForFirstCase = SplitterProducesForFirstCase,
    SplitterTagForFirstCase = Tag,
    Messages = Self::Messages,
    ProcessBeforeSplitEverProduced = Self::EverProduced,
    EverProduced = Self::EverProduced,
    Errors = <SplitterStep::Error as CoprodUnion<Self::Errors, ErrorUnionIndices>>::Union,
  >
  where
    SplitterStep::Error: CoprodUnion<Self::Errors, ErrorUnionIndices>,
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
      ErrorUnionIndices,
    > {
      process_before: self,
      splitter: step,
      phantom_data: Default::default(),
    }
  }

  /// The process execution will call `SplitProcess::continue_run` instead of the individual case
  /// `FlowingProcess::continue_run` method to avoid excessive nesting:
  /// ```compile_fail,E0425
  /// let _ = entry::<HNil, Msg>()
  ///   .show_split(SplitA, |case_entry|
  ///     case_entry
  ///       .case_via(Case1, |x| x)
  ///       .case_via(Case2, |x| x.show(FormA))
  ///   )
  ///   .end(FinalA);
  /// ```
  /// and use the builder like this instead:
  /// ```
  /// # use type_process_builder::builder::*;
  /// # use type_process_builder::step::*;
  /// # use type_process_builder::{CNil, Coprod, HNil, ToRef};
  /// # struct Msg;
  /// # impl ProcessMessages for Msg { type FormMessage = String; type FinalMessage = String; }
  /// # struct Case1; struct Case2; struct SplitA;
  /// # impl FormSplitter for SplitA {
  /// #   type CreateFormConsumes = HNil; type ValidateInputConsumes = HNil; type Produces = Coprod![(Case1, HNil), (Case2, HNil)]; type Context = (); type Messages = Msg; type Error = CNil;
  /// #   async fn create_form(&self, _: <Self::CreateFormConsumes as ToRef<'_>>::Ref, _: Option<BackToken>) -> Result<FormWithContext<String, ()>, CNil> { Ok(FormWithContext("".into(), ())) }
  /// #   async fn handle_input(&self, _: <Self::ValidateInputConsumes as ToRef<'_>>::Ref, _: String, _: (), _: Option<BackToken>) -> Result<InputValidation<Self::Produces, Msg, ()>, CNil> { Ok(InputValidation::Successful(Self::Produces::inject((Case1, HNil)))) }
  /// # }
  /// # struct FormA;
  /// # impl Form for FormA {
  /// #   type CreateFormConsumes = HNil; type ValidateInputConsumes = HNil; type Produces = HNil; type Context = (); type Messages = Msg; type Error = CNil;
  /// #   async fn create_form(&self, _: <Self::CreateFormConsumes as ToRef<'_>>::Ref, _: Option<BackToken>) -> Result<FormWithContext<String, ()>, CNil> { Ok(FormWithContext("".into(), ())) }
  /// #   async fn handle_input(&self, _: <Self::ValidateInputConsumes as ToRef<'_>>::Ref, _: String, _: (), _: Option<BackToken>) -> Result<InputValidation<HNil, Msg, ()>, CNil> { Ok(InputValidation::Successful(HNil)) }
  /// # }
  /// # struct FinalA;
  /// # impl Final for FinalA { type Consumes = HNil; type FinalMessage = String; type Error = CNil; async fn handle(&self, _: HNil) -> Result<String, CNil> { Ok("".into()) } }
  /// let _ = entry::<HNil, Msg>()
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
    ErrorUnionIndices: Sync + Send,
  >(
    self,
    step: SplitterStep,
  ) -> impl SplitProcess<
    SplitterProducesForOtherCases,
    EntryConsumes = Self::EntryConsumes,
    ProcessBeforeSplitProduces = Self::Produces,
    SplitterProducesForFirstCase = SplitterProducesForFirstCase,
    SplitterTagForFirstCase = Tag,
    Messages = Self::Messages,
    ProcessBeforeSplitEverProduced = Self::EverProduced,
    EverProduced = Self::EverProduced,
    Errors = <SplitterStep::Error as CoprodUnion<Self::Errors, ErrorUnionIndices>>::Union,
  >
  where
    SplitterStep::Error: CoprodUnion<Self::Errors, ErrorUnionIndices>,
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
      ErrorUnionIndices,
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
    ErrorUnionIndices: Sync + Send,
  >(
    self,
    step: FinalStep,
  ) -> impl FinalizedProcess<
    EntryConsumes = Self::EntryConsumes,
    ProcessBeforeProduces = Self::Produces,
    Messages = Self::Messages,
    EverProduced = Self::EverProduced,
    Errors = <FinalStep::Error as CoprodUnion<Self::Errors, ErrorUnionIndices>>::Union,
  >
  where
    FinalStep::Error: CoprodUnion<Self::Errors, ErrorUnionIndices>,
    Self::Produces: Extract<FinalStep::Consumes, ProcessBeforeProducesToLastStepConsumesIndices>,
  {
    FinalFinalizedProcess { process_before: self, final_step: step, phantom_data: Default::default() }
  }

  fn then_end<
    Sub: FinalizedProcess<Messages = Self::Messages>,
    ProcessBeforeProducesToEntryConsumesIndices: Sync + Send,
    SubEverProducedExtractIndices: Sync + Send,
    ErrorUnionIndices: Sync + Send,
  >(
    self,
    sub: Sub,
  ) -> impl FinalizedProcess<
    EntryConsumes = Self::EntryConsumes,
    ProcessBeforeProduces = Self::Produces,
    Messages = Self::Messages,
    EverProduced = <<Sub::EverProduced as Extract<
      Sub::EntryConsumes,
      SubEverProducedExtractIndices,
    >>::Remainder as Concat<Self::EverProduced>>::Concatenated,
    Errors = <Sub::Errors as CoprodUnion<Self::Errors, ErrorUnionIndices>>::Union,
  >
  where
    Sub::EverProduced: Extract<Sub::EntryConsumes, SubEverProducedExtractIndices>,
    <Sub::EverProduced as Extract<Sub::EntryConsumes, SubEverProducedExtractIndices>>::Remainder:
      ParamList + Concat<Self::EverProduced>,
    Self::Produces: Extract<Sub::EntryConsumes, ProcessBeforeProducesToEntryConsumesIndices>,
    Sub::Errors: CoprodUnion<Self::Errors, ErrorUnionIndices>,
{
    SubprocessFinalizedProcess {
      process_before: self,
      sub_process: sub,
      sub_start_index: WILL_BE_RENUMBERED,
      phantom_data: Default::default(),
    }
  }
}

impl<T: sealed::FlowingProcess> FlowingProcess for T {}
