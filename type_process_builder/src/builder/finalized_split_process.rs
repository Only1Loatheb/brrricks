use crate::builder::finalized_process::FinalizedProcess;
use crate::builder::flowing_process::FlowingProcess;
use crate::builder::{PreviousRunYieldedAt, RunResult};
use crate::param_list::ParamList;
use crate::step::splitter_output_repr::SplitterOutput;
use crate::step::step::Splitter;
use frunk_core::coproduct::Coproduct;
use serde_value::Value;
use std::future::Future;
use std::marker::PhantomData;

pub trait FinalizedSplitProcess: Sized {
  fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded: PreviousRunYieldedAt,
  ) -> impl Future<Output = RunResult>;

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize;
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
  > FinalizedSplitProcess
  for FirstCaseOfFinalizedSplitProcess<
    PROCESS_BEFORE,
    SPLITTER_CONSUMES,
    Coproduct<CASE_THIS, CASE_OTHER>,
    SPLITTER_STEP,
    FIRST_CASE,
  >
{
  // type SplitterOutput = <CASE_THIS as Concat<PROCESS_BEFORE::Produces>>::Concatenated;

  async fn continue_run(&self, previous_run_produced: Value, previous_run_yielded: PreviousRunYieldedAt) -> RunResult {
    todo!()
    // if last_run.0 < self.step_index {
    //   // no yielding from splitter step todo maybe implement
    //   let process_before_output = self
    //     .process_before
    //     .continue_run(previous_run_produced, last_run)
    //     .await?;
    //   match process_before_output {
    //     Continue(process_before_produces) => {
    //       let splitter_output = self.splitter.handle(process_before_produces).await?;
    //       match splitter_output {
    //         Coproduct::Inl(a) => self.first_case.continue_run(previous_run_produced, last_run),
    //         Coproduct::Inr(b) => {}
    //       }
    //     }
    //     result @ Yield(_, _, _) => Ok(result),
    //     result @ Finish(_) => Ok(result),
    //   }
    // } else {
    //   let params = serde_json::from_value::<SPLITTER_PRODUCES>(previous_run_produced)?;
    //   Ok(Continue(params))
    // }
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    let used_index = self.process_before.enumerate_steps(last_used_index);
    self.step_index = used_index + 1;
    self.step_index
  }
}

// maybe the case_step_index overlaps with FinalizedProcess or maybe it allows for a skip
pub struct NextCaseOfFinalizedSplitProcess<PROCESS_BEFORE: FinalizedSplitProcess, NEXT_CASE: FinalizedProcess> {
  pub split_process_before: PROCESS_BEFORE,
  pub case_step_index: usize,
  pub next_case: NEXT_CASE,
}

impl<PROCESS_BEFORE: FinalizedSplitProcess, NEXT_CASE: FinalizedProcess> FinalizedSplitProcess
  for NextCaseOfFinalizedSplitProcess<PROCESS_BEFORE, NEXT_CASE>
{
  async fn continue_run(&self, previous_run_produced: Value, previous_run_yielded: PreviousRunYieldedAt) -> RunResult {
    //Self::SplitterOutput
    todo!()
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    let used_index = self.split_process_before.enumerate_steps(last_used_index);
    self.case_step_index = used_index + 1;
    self.case_step_index
  }
}
