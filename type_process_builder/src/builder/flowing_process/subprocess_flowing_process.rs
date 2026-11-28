use crate::builder::flowing_process::sealed;
use crate::builder::{
  FlowingProcess, IntermediateRunOutcome, IntermediateRunResult, IntermediateStartOutcome, IntermediateStartResult,
  IntoProcessError, ParamList, PreviousRunYieldedAt, ProcessError, StepIndex,
};
use crate::error_coproduct::union::CoprodUnion;
use crate::param_list::concat::Concat;
use crate::param_list::extract::Extract;
use crate::param_list::sealed::ParamList as _;
use crate::step::BackToken;
use std::marker::PhantomData;

pub struct SubprocessFlowingProcess<
  ProcessBefore: FlowingProcess,
  Sub: FlowingProcess,
  ProcessBeforeProducesToEntryConsumesIndices,
  SubEverProducedExtractIndices,
  ErrorUnionIndices,
> {
  pub(crate) process_before: ProcessBefore,
  pub(crate) sub_process: Sub,
  pub(crate) sub_start_index: StepIndex,
  pub(crate) sub_end_index: StepIndex,
  pub(crate) phantom_data:
    PhantomData<(ProcessBeforeProducesToEntryConsumesIndices, SubEverProducedExtractIndices, ErrorUnionIndices)>,
}

impl<
  ProcessBefore: FlowingProcess,
  Sub: FlowingProcess<Messages = ProcessBefore::Messages>,
  ProcessBeforeProducesToEntryConsumesIndices: Sync + Send,
  SubEverProducedExtractIndices: Sync + Send,
  ErrorUnionIndices: Sync + Send,
> sealed::FlowingProcess
  for SubprocessFlowingProcess<
    ProcessBefore,
    Sub,
    ProcessBeforeProducesToEntryConsumesIndices,
    SubEverProducedExtractIndices,
    ErrorUnionIndices,
  >
where
  ProcessBefore::Produces: Extract<Sub::EntryConsumes, ProcessBeforeProducesToEntryConsumesIndices>,
  <ProcessBefore::Produces as Extract<Sub::EntryConsumes, ProcessBeforeProducesToEntryConsumesIndices>>::Remainder:
    ParamList,
  Sub::Produces: Concat<
    <ProcessBefore::Produces as Extract<Sub::EntryConsumes, ProcessBeforeProducesToEntryConsumesIndices>>::Remainder,
  >,
  Sub::EverProduced: Extract<Sub::EntryConsumes, SubEverProducedExtractIndices>,
  <Sub::EverProduced as Extract<Sub::EntryConsumes, SubEverProducedExtractIndices>>::Remainder:
    ParamList + Concat<ProcessBefore::EverProduced>,
  Sub::Errors: CoprodUnion<ProcessBefore::Errors, ErrorUnionIndices>,
{
  type EntryConsumes = ProcessBefore::EntryConsumes;
  type Produces = <Sub::Produces as Concat<
    <ProcessBefore::Produces as Extract<Sub::EntryConsumes, ProcessBeforeProducesToEntryConsumesIndices>>::Remainder,
  >>::Concatenated;
  type Messages = ProcessBefore::Messages;
  type EverProduced =
    <<Sub::EverProduced as Extract<Sub::EntryConsumes, SubEverProducedExtractIndices>>::Remainder as Concat<
      ProcessBefore::EverProduced,
    >>::Concatenated;
  type Errors = <Sub::Errors as CoprodUnion<ProcessBefore::Errors, ErrorUnionIndices>>::Union;

  async fn resume_run(
    &self,
    previous_run_produced: &[u8],
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: &str,
    form_context: Option<&[u8]>,
    back_token: Option<BackToken>,
  ) -> IntermediateRunResult<Self::Produces, Self::Messages, Self::Errors> {
    if previous_run_yielded_at.0 <= self.sub_start_index {
      let outcome = self
        .process_before
        .resume_run(previous_run_produced, previous_run_yielded_at, user_input, form_context, back_token)
        .await
        .into_process_err()?;
      match outcome {
        IntermediateRunOutcome::Continue(val) => self.continue_run(val, back_token).await.map(Into::into),
        IntermediateRunOutcome::Yield(a, b, c, d) => Ok(IntermediateRunOutcome::Yield(a, b, c, d)),
        IntermediateRunOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
        IntermediateRunOutcome::RetryUserInput(a, b) => Ok(IntermediateRunOutcome::RetryUserInput(a, b)),
        IntermediateRunOutcome::Back => Ok(IntermediateRunOutcome::Back),
      }
    } else if previous_run_yielded_at.0 <= self.sub_end_index {
      let (sub_ctx, outer_ctx): (&[u8], &[u8]) = postcard::take_from_bytes(previous_run_produced).into_process_err()?;
      let outcome = self
        .sub_process
        .resume_run(sub_ctx, previous_run_yielded_at, user_input, form_context, back_token)
        .await
        .into_process_err()?;
      match outcome {
        IntermediateRunOutcome::Continue(sub_produces) => {
          let outer_remainder = <ProcessBefore::Produces as Extract<
            Sub::EntryConsumes,
            ProcessBeforeProducesToEntryConsumesIndices,
          >>::Remainder::deserialize_param_list(outer_ctx)
          .into_process_err()?;
          Ok(IntermediateRunOutcome::Continue(sub_produces.concat(outer_remainder)))
        },
        IntermediateRunOutcome::Yield(msg, new_sub_ctx, yielded_at, form_ctx) => {
          // `postcard` encodes `&[u8]` with a LEB128 varint length prefix (7 payload bits/byte),
          // which is at most ceil(64 / 7) = 10 bytes for a 64-bit `usize`.
          let mut packed =
            postcard::to_extend(&new_sub_ctx.as_slice(), Vec::with_capacity(new_sub_ctx.len() + outer_ctx.len() + 10))
              .into_process_err()?;
          packed.extend_from_slice(outer_ctx);
          Ok(IntermediateRunOutcome::Yield(msg, packed, yielded_at, form_ctx))
        },
        IntermediateRunOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
        IntermediateRunOutcome::RetryUserInput(a, b) => Ok(IntermediateRunOutcome::RetryUserInput(a, b)),
        IntermediateRunOutcome::Back => Ok(IntermediateRunOutcome::Back),
      }
    } else {
      Err(ProcessError::invalid_step_index(previous_run_yielded_at.0))
    }
  }

  async fn run_from_entry(
    &self,
    entry_consumes: Self::EntryConsumes,
    back_token: Option<BackToken>,
  ) -> IntermediateStartResult<Self::Produces, Self::Messages, Self::Errors> {
    let outcome = self.process_before.run_from_entry(entry_consumes, back_token).await.into_process_err()?;
    match outcome {
      IntermediateStartOutcome::Continue(val) => self.continue_run(val, back_token).await,
      IntermediateStartOutcome::Yield(a, b, c, d) => Ok(IntermediateStartOutcome::Yield(a, b, c, d)),
      IntermediateStartOutcome::Finish(a) => Ok(IntermediateStartOutcome::Finish(a)),
    }
  }

  fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex {
    let last_used_index = self.process_before.enumerate_steps(last_used_index);
    self.sub_start_index = last_used_index;
    let last_used_index = self.sub_process.enumerate_steps(last_used_index);
    self.sub_end_index = last_used_index;
    last_used_index
  }
}

impl<
  ProcessBefore: FlowingProcess,
  Sub: FlowingProcess<Messages = ProcessBefore::Messages>,
  ProcessBeforeProducesToEntryConsumesIndices: Sync + Send,
  SubEverProducedExtractIndices: Sync + Send,
  ErrorUnionIndices: Sync + Send,
>
  SubprocessFlowingProcess<
    ProcessBefore,
    Sub,
    ProcessBeforeProducesToEntryConsumesIndices,
    SubEverProducedExtractIndices,
    ErrorUnionIndices,
  >
where
  ProcessBefore::Produces: Extract<Sub::EntryConsumes, ProcessBeforeProducesToEntryConsumesIndices>,
  <ProcessBefore::Produces as Extract<Sub::EntryConsumes, ProcessBeforeProducesToEntryConsumesIndices>>::Remainder:
    ParamList,
  Sub::Produces: Concat<
    <ProcessBefore::Produces as Extract<Sub::EntryConsumes, ProcessBeforeProducesToEntryConsumesIndices>>::Remainder,
  >,
  Sub::EverProduced: Extract<Sub::EntryConsumes, SubEverProducedExtractIndices>,
  <Sub::EverProduced as Extract<Sub::EntryConsumes, SubEverProducedExtractIndices>>::Remainder:
    ParamList + Concat<ProcessBefore::EverProduced>,
  Sub::Errors: CoprodUnion<ProcessBefore::Errors, ErrorUnionIndices>,
{
  async fn continue_run(
    &self,
    process_before_produces: ProcessBefore::Produces,
    back_token: Option<BackToken>,
  ) -> IntermediateStartResult<
    <Self as sealed::FlowingProcess>::Produces,
    ProcessBefore::Messages,
    <Self as sealed::FlowingProcess>::Errors,
  > {
    let (sub_consumes, outer_remainder): (Sub::EntryConsumes, _) = process_before_produces.extract_with_remainder();
    let outcome = self.sub_process.run_from_entry(sub_consumes, back_token).await.into_process_err()?;
    match outcome {
      IntermediateStartOutcome::Continue(sub_produces) => {
        Ok(IntermediateStartOutcome::Continue(sub_produces.concat(outer_remainder)))
      },
      IntermediateStartOutcome::Yield(msg, sub_ctx, yielded_at, form_ctx) => {
        // 10 bytes max for the `sub_ctx` LEB128 varint length prefix + 6 bytes headroom for small `outer_remainder`s.
        let packed =
          postcard::to_extend(&sub_ctx.as_slice(), Vec::with_capacity(sub_ctx.len() + 16)).into_process_err()?;
        let packed = postcard::to_extend(&outer_remainder, packed).into_process_err()?;
        Ok(IntermediateStartOutcome::Yield(msg, packed, yielded_at, form_ctx))
      },
      IntermediateStartOutcome::Finish(a) => Ok(IntermediateStartOutcome::Finish(a)),
    }
  }
}
