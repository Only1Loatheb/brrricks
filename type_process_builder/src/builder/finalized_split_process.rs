use crate::builder::finalized_process::FinalizedProcess;
use crate::builder::flowing_process::FlowingProcess;
use crate::builder::InterpretationOutcome::*;
use crate::builder::{InterpretationResult, PreviousInterpretationYieldedAt, ProcessBuilder};
use crate::step::param_list::ParamList;
use crate::step::splitter_output_repr::SplitterOutput;
use crate::step::step::Splitter;
use frunk_core::coproduct::Coproduct;
use frunk_core::hlist::HNil;
use serde_json::Value;
use std::marker::PhantomData;

pub trait FinalizedSplitProcess: ProcessBuilder {
  async fn interpret_resume(
    &self,
    previous_interpretation_produced: Value,
    previous_interpretation_yielded: PreviousInterpretationYieldedAt,
  ) -> InterpretationResult<HNil>; // fixme create result for finalised process, or undo changes
}

pub struct FirstCaseOfFinalizedSplitProcess<
  PROCESS_BEFORE: FlowingProcess,
  SPLITTER_CONSUMES: ParamList,
  SPLITTER_PRODUCES: SplitterOutput,
  SPLITTER_STEP: Splitter<SPLITTER_CONSUMES, SPLITTER_PRODUCES>,
  FIRST_CASE: FinalizedProcess,
> {
  pub process_before: PROCESS_BEFORE,
  pub splitter: SPLITTER_STEP,
  pub step_index: usize,
  pub first_case: FIRST_CASE,
  pub phantom_data: PhantomData<(SPLITTER_CONSUMES, SPLITTER_PRODUCES)>,
}

impl<
    PROCESS_BEFORE: FlowingProcess,
    SPLITTER_CONSUMES: ParamList,
    CASE_THIS: ParamList,
    CASE_OTHER: SplitterOutput,
    SPLITTER_STEP: Splitter<SPLITTER_CONSUMES, Coproduct<CASE_THIS, CASE_OTHER>>,
    FIRST_CASE: FinalizedProcess,
  > ProcessBuilder
  for FirstCaseOfFinalizedSplitProcess<PROCESS_BEFORE, SPLITTER_CONSUMES, Coproduct<CASE_THIS, CASE_OTHER>, SPLITTER_STEP, FIRST_CASE>
{
  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    let used_index = self.process_before.enumerate_steps(last_used_index);
    self.step_index = used_index + 1;
    self.step_index
  }
}

impl<
    PROCESS_BEFORE: FlowingProcess,
    SPLITTER_CONSUMES: ParamList,
    CASE_THIS: ParamList,
    CASE_OTHER: SplitterOutput,
    SPLITTER_STEP: Splitter<SPLITTER_CONSUMES, Coproduct<CASE_THIS, CASE_OTHER>>,
    FIRST_CASE: FinalizedProcess,
  > FinalizedSplitProcess
  for FirstCaseOfFinalizedSplitProcess<PROCESS_BEFORE, SPLITTER_CONSUMES, Coproduct<CASE_THIS, CASE_OTHER>, SPLITTER_STEP, FIRST_CASE>
{
  // type SplitterOutput = <CASE_THIS as Concat<PROCESS_BEFORE::Produces>>::Concatenated;

  async fn interpret_resume(
    &self,
    previous_interpretation_produced: Value,
    previous_interpretation_yielded: PreviousInterpretationYieldedAt,
  ) -> InterpretationResult<HNil> {
    todo!()
    // if last_interpreted.0 < self.step_index {
    //   // no yielding from splitter step todo maybe implement
    //   let process_before_output = self
    //     .process_before
    //     .interpret_resume(previous_interpretation_produced, last_interpreted)
    //     .await?;
    //   match process_before_output {
    //     Continue(process_before_produces) => {
    //       let splitter_output = self.splitter.handle(process_before_produces).await?;
    //       match splitter_output {
    //         Coproduct::Inl(a) => self.first_case.interpret_resume(previous_interpretation_produced, last_interpreted),
    //         Coproduct::Inr(b) => {}
    //       }
    //     }
    //     result @ Yield(_, _, _) => Ok(result),
    //     result @ Finish(_) => Ok(result),
    //   }
    // } else {
    //   let params = serde_json::from_value::<SPLITTER_PRODUCES>(previous_interpretation_produced)?;
    //   Ok(Continue(params))
    // }
  }
}

// maybe the case_step_index overlaps with FinalizedProcess or maybe it allows for a skip
pub struct NextCaseOfFinalizedSplitProcess<PROCESS_BEFORE: FinalizedSplitProcess, NEXT_CASE: FinalizedProcess> {
  pub split_process_before: PROCESS_BEFORE,
  pub case_step_index: usize,
  pub next_case: NEXT_CASE,
}

impl<PROCESS_BEFORE: FinalizedSplitProcess, NEXT_CASE: FinalizedProcess> ProcessBuilder
  for NextCaseOfFinalizedSplitProcess<PROCESS_BEFORE, NEXT_CASE>
{
  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    let used_index = self.split_process_before.enumerate_steps(last_used_index);
    self.case_step_index = used_index + 1;
    self.case_step_index
  }
}

impl<PROCESS_BEFORE: FinalizedSplitProcess, NEXT_CASE: FinalizedProcess> FinalizedSplitProcess
  for NextCaseOfFinalizedSplitProcess<PROCESS_BEFORE, NEXT_CASE>
{
  async fn interpret_resume(
    &self,
    previous_interpretation_produced: Value,
    previous_interpretation_yielded: PreviousInterpretationYieldedAt,
  ) -> InterpretationResult<HNil> {
    //Self::SplitterOutput
    todo!()
  }
}
