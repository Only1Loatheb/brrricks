use crate::builder::finalized_process::FinalizedProcess;
use crate::builder::flowing_process::FlowingProcess;
use crate::builder::IntermediateRunOutcome::{Continue, Finish, Yield};
use crate::builder::{PreviousRunYieldedAt, RunOutcome, RunResult};
use crate::param_list::ParamList;
use crate::step::splitter_output_repr::SplitterOutput;
use crate::step::step::Splitter;
use frunk_core::coproduct::Coproduct;
use serde_value::Value;
use std::future::Future;
use std::marker::PhantomData;

pub trait FinalizedSplitProcess: Sized {
  type ProcessBeforeProduces: ParamList;

  fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
  ) -> impl Future<Output = RunResult>;

  fn run(&self, process_before_produces: Self::ProcessBeforeProduces) -> impl Future<Output = RunResult>;

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize;
}

pub struct FirstCaseOfFinalizedSplitProcess<
  ProcessBefore: FlowingProcess,
  SplitterConsumes: ParamList,
  SplitterProduces: SplitterOutput,
  SplitterStep: Splitter<SplitterConsumes, SplitterProduces>,
  FirstCase: FinalizedProcess,
> {
  pub process_before: ProcessBefore,
  pub splitter: SplitterStep,
  pub split_step_index: usize,
  pub first_case: FirstCase,
  pub phantom_data: PhantomData<(SplitterConsumes, SplitterProduces)>,
}

impl<
    ProcessBefore: FlowingProcess,
    SplitterConsumes: ParamList,
    ThisCase: ParamList,
    OtherCase: SplitterOutput,
    SplitterStep: Splitter<SplitterConsumes, Coproduct<ThisCase, OtherCase>>,
    FirstCase: FinalizedProcess,
  > FinalizedSplitProcess
  for FirstCaseOfFinalizedSplitProcess<
    ProcessBefore,
    SplitterConsumes,
    Coproduct<ThisCase, OtherCase>,
    SplitterStep,
    FirstCase,
  >
{
  type ProcessBeforeProduces = ProcessBefore::Produces;

  async fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
  ) -> RunResult {
    if previous_run_yielded_at.0 < self.split_step_index {
      let process_before_output = self
        .process_before
        .continue_run(previous_run_produced, previous_run_yielded_at)
        .await?;
      match process_before_output {
        Continue(process_before_produces) => self.run(process_before_produces).await,
        Yield(a, b, c) => Ok(RunOutcome::Yield(a, b, c)),
        Finish(a) => Ok(RunOutcome::Finish(a)),
      }
    } else {
      let process_before_produces: ProcessBefore::Produces =
        ProcessBefore::Produces::deserialize(previous_run_produced)?;
      self.run(process_before_produces).await
    }
  }

  async fn run(&self, process_before_produces: Self::ProcessBeforeProduces) -> RunResult {
    // Continue(process_before_produces) => {
    //   let splitter_output = self.splitter.handle(process_before_produces).await?;
    //   match splitter_output {
    //     Coproduct::Inl(a) => self.first_case.continue_run(previous_run_produced, last_run),
    //     Coproduct::Inr(b) => {}
    //   }
    // }
    // result @ Yield(_, _, _) => Ok(result),
    // result @ Finish(_) => Ok(result),
    todo!()
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    let used_index = self.process_before.enumerate_steps(last_used_index);
    self.split_step_index = used_index + 1;
    self.split_step_index
  }
}

// maybe the case_step_index overlaps with FinalizedProcess or maybe it allows for a skip
pub struct NextCaseOfFinalizedSplitProcess<ProcessBefore: FinalizedSplitProcess, NextCase: FinalizedProcess> {
  pub split_process_before: ProcessBefore,
  pub case_step_index: usize,
  pub next_case: NextCase,
}

impl<ProcessBefore: FinalizedSplitProcess, NextCase: FinalizedProcess> FinalizedSplitProcess
  for NextCaseOfFinalizedSplitProcess<ProcessBefore, NextCase>
{
  type ProcessBeforeProduces = ProcessBefore::ProcessBeforeProduces;

  async fn continue_run(
    &self,
    previous_run_produced: Value,
    previous_run_yielded_at: PreviousRunYieldedAt,
  ) -> RunResult {
    //Self::SplitterOutput
    todo!()
  }

  async fn run(&self, process_before_produces: Self::ProcessBeforeProduces) -> RunResult {
    todo!()
  }

  fn enumerate_steps(&mut self, last_used_index: usize) -> usize {
    let used_index = self.split_process_before.enumerate_steps(last_used_index);
    self.case_step_index = used_index + 1;
    self.case_step_index
  }
}
