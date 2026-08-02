use crate::builder::subprocess::{Subprocess, subprocess};
use crate::builder::{
  FinalizedCaseOfFlowingSplitProcess, FinalizedProcess, FinalizedSplitProcess, FlowingCaseOfFlowingSplitProcess,
  FlowingProcess, FlowingSplitProcess, IntermediateFinalizedSplitOutcome, IntermediateFlowingSplitOutcome,
  IntermediateFlowingSplitResult, IntermediateRunOutcome, IntermediateRunResult, MaybeFormContext, ParamList,
  PreviousRunYieldedAt, SessionContext, StepIndex, WILL_BE_RENUMBERED,
};
use crate::frunk::coproduct::{CNil, Coproduct};
use crate::param_list::concat::Concat;
use crate::param_list::union::Union;
use crate::step::BackToken;
use std::marker::PhantomData;

pub struct FlowingCaseOfFinalizedSplitProcess<
  ThisTag: Send + Sync,
  SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  SplitterProducesForOtherCases: Send + Sync,
  ProcessBefore: FinalizedSplitProcess<Coproduct<(ThisTag, SplitterProducesForThisCase), SplitterProducesForOtherCases>>,
  ThisCase: FlowingProcess<SubprocessConsumes=
  <SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated
  >,
>
{
  pub split_process_before: ProcessBefore,
  pub case_index: StepIndex,
  pub this_case: ThisCase,
  pub phantom_data: PhantomData<(
    ThisTag,
    SplitterProducesForThisCase,
    SplitterProducesForOtherCases,
  )>,
}

impl<
  ThisTag: Send + Sync,
  NextTag: Send + Sync,
  SplitterProducesForOtherCases: Send + Sync,
  ProcessBefore: FinalizedSplitProcess<
    Coproduct<(ThisTag, SplitterProducesForThisCase), Coproduct<(NextTag, SplitterProducesForNextCase), SplitterProducesForOtherCases>>,
  >,
  SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  SplitterProducesForNextCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  ThisCase: FlowingProcess<
    SubprocessConsumes=<SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
    Messages=ProcessBefore::Messages,
  >,
>
FlowingCaseOfFinalizedSplitProcess<
  ThisTag,
  SplitterProducesForThisCase,
  Coproduct<(NextTag, SplitterProducesForNextCase), SplitterProducesForOtherCases>,
  ProcessBefore,
  ThisCase,
>
where
  ThisCase::EverProduced: Union<ProcessBefore::EverProduced>,
  <ThisCase::EverProduced as Union<ProcessBefore::EverProduced>>::Union: ParamList,
{
  pub fn case_end<
    NextCase: FinalizedProcess<SubprocessConsumes=<SplitterProducesForNextCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated>,
  >(
    self,
    _assumed_tag: NextTag,
    create_case: impl FnOnce(Subprocess<
      <SplitterProducesForNextCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
      ProcessBefore::Messages,
    >) -> NextCase,
  ) -> FinalizedCaseOfFlowingSplitProcess<
    NextTag,
    SplitterProducesForNextCase,
    SplitterProducesForOtherCases,
    Self,
    NextCase,
  >
  {
    FinalizedCaseOfFlowingSplitProcess {
      split_process_before: self,
      case_index: WILL_BE_RENUMBERED,
      this_case: create_case(subprocess::<
        <SplitterProducesForNextCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
        ProcessBefore::Messages,
      >()),
      phantom_data: Default::default(),
    }
  }

  pub fn case_via<
    NextCase: FlowingProcess<
      SubprocessConsumes=<SplitterProducesForNextCase as
      Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
      Messages=ProcessBefore::Messages,
    >,
    Indices: Sync + Send,
  >(
    self,
    _assumed_tag: NextTag,
    create_case: impl FnOnce(Subprocess<
      <SplitterProducesForNextCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
      ProcessBefore::Messages,
    >) -> NextCase,
  ) -> FlowingCaseOfFlowingSplitProcess<
    NextTag,
    SplitterProducesForNextCase,
    SplitterProducesForOtherCases,
    Self,
    NextCase,
    Indices,
  >
  {
    FlowingCaseOfFlowingSplitProcess {
      split_process_before: self,
      case_index: WILL_BE_RENUMBERED,
      this_case: create_case(subprocess::<
        <SplitterProducesForNextCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
        ProcessBefore::Messages,
      >()),
      phantom_data: Default::default(),
    }
  }
}

impl<
  ThisTag: Send + Sync,
  SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  SplitterProducesForOtherCases: Send + Sync,
  ProcessBefore: FinalizedSplitProcess<Coproduct<(ThisTag, SplitterProducesForThisCase), SplitterProducesForOtherCases>>,
  ThisCase: FlowingProcess<
    SubprocessConsumes=<SplitterProducesForThisCase as
    Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
    Messages=ProcessBefore::Messages,
  >,
> FlowingSplitProcess<SplitterProducesForOtherCases>
for FlowingCaseOfFinalizedSplitProcess<
  ThisTag,
  SplitterProducesForThisCase,
  SplitterProducesForOtherCases,
  ProcessBefore,
  ThisCase,
>
where
  ThisCase::EverProduced: Union<ProcessBefore::EverProduced>,
  <ThisCase::EverProduced as Union<ProcessBefore::EverProduced>>::Union: ParamList,
{
  type ProcessBeforeSplitProduces = ProcessBefore::ProcessBeforeSplitProduces;
  type SplitterProducesForThisCase = SplitterProducesForThisCase;
  type EveryFlowingCaseProduces = ThisCase::Produces;
  type SubprocessConsumes = ProcessBefore::SubprocessConsumes;
  type Messages = ProcessBefore::Messages;
  type EverProduced = <ThisCase::EverProduced as Union<ProcessBefore::EverProduced>>::Union;

  async fn resume_run(
    &self,
    previous_run_produced: SessionContext,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
    form_context: MaybeFormContext,
    back_token: Option<BackToken>,
  ) -> IntermediateFlowingSplitResult<Self::ProcessBeforeSplitProduces, SplitterProducesForOtherCases,
    Self::EveryFlowingCaseProduces, Self::Messages> {
    if previous_run_yielded_at.0 < self.case_index {
      let process_before_output = self
        .split_process_before
        .resume_run(previous_run_produced, previous_run_yielded_at, user_input, form_context, back_token)
        .await?;
      match process_before_output {
        IntermediateFinalizedSplitOutcome::GoToCase {
          process_before_split_produced,
          splitter_produces_to_other_cases,
        } => {
          let produced = match splitter_produces_to_other_cases {
            Coproduct::Inl((_pd, params)) => Coproduct::Inl(params),
            Coproduct::Inr(inr_value) => Coproduct::Inr(inr_value),
          };
          self.continue_run(process_before_split_produced, produced, back_token).await
        }
        IntermediateFinalizedSplitOutcome::Yield(a, b, c, d) => Ok(IntermediateFlowingSplitOutcome::Yield(a, b, c, d)),
        IntermediateFinalizedSplitOutcome::Finish(a) => Ok(IntermediateFlowingSplitOutcome::Finish(a)),
        IntermediateFinalizedSplitOutcome::RetryUserInput(a, b) => Ok(IntermediateFlowingSplitOutcome::RetryUserInput(a, b)),
        IntermediateFinalizedSplitOutcome::Back => Ok(IntermediateFlowingSplitOutcome::Back),
      }
    } else {
      match self.this_case.resume_run(
        previous_run_produced,
        previous_run_yielded_at,
        user_input,
        form_context,
        back_token,
      ).await? {
        IntermediateRunOutcome::Continue(a) => Ok(IntermediateFlowingSplitOutcome::Continue(a)),
        IntermediateRunOutcome::Yield(a, b, c, d) => Ok(IntermediateFlowingSplitOutcome::Yield(a, b, c, d)),
        IntermediateRunOutcome::Finish(a) => Ok(IntermediateFlowingSplitOutcome::Finish(a)),
        IntermediateRunOutcome::RetryUserInput(a, b) => Ok(IntermediateFlowingSplitOutcome::RetryUserInput(a, b)),
        IntermediateRunOutcome::Back => Ok(IntermediateFlowingSplitOutcome::Back),
      }
    }
  }

  async fn continue_run(
    &self,
    process_before_split_produced: Self::ProcessBeforeSplitProduces,
    splitter_produces_for_this_case_or_other_cases_consumes: Coproduct<
      Self::SplitterProducesForThisCase,
      SplitterProducesForOtherCases,
    >,
    back_token: Option<BackToken>,
  ) -> IntermediateFlowingSplitResult<Self::ProcessBeforeSplitProduces, SplitterProducesForOtherCases,
    Self::EveryFlowingCaseProduces, Self::Messages> {
    match splitter_produces_for_this_case_or_other_cases_consumes {
      Coproduct::Inl(splitter_produces_for_this_case) => {
        let this_case_consumes = splitter_produces_for_this_case.concat(process_before_split_produced);
        match self.this_case.run_subprocess(this_case_consumes, back_token).await? {
          IntermediateRunOutcome::Continue(a) => Ok(IntermediateFlowingSplitOutcome::Continue(a)),
          IntermediateRunOutcome::Yield(a, b, c, d) => Ok(IntermediateFlowingSplitOutcome::Yield(a, b, c, d)),
          IntermediateRunOutcome::Finish(a) => Ok(IntermediateFlowingSplitOutcome::Finish(a)),
          IntermediateRunOutcome::RetryUserInput(a, b) => Ok(IntermediateFlowingSplitOutcome::RetryUserInput(a, b)),
          IntermediateRunOutcome::Back => Ok(IntermediateFlowingSplitOutcome::Back),
        }
      }
      Coproduct::Inr(splitter_produces_to_other_cases) => Ok(IntermediateFlowingSplitOutcome::GoToCase {
        process_before_split_produced,
        splitter_produces_to_other_cases,
      }),
    }
  }

  async fn run_split_subprocess(&self, subprocess_consumes: Self::SubprocessConsumes, back_token: Option<BackToken>) ->
  IntermediateFlowingSplitResult<Self::ProcessBeforeSplitProduces, SplitterProducesForOtherCases, Self::EveryFlowingCaseProduces, Self::Messages> {
    let process_before_output = self.split_process_before.run_split_subprocess(subprocess_consumes, back_token).await?;
    match process_before_output {
      IntermediateFinalizedSplitOutcome::GoToCase {
        process_before_split_produced,
        splitter_produces_to_other_cases,
      } => {
        let produced = match splitter_produces_to_other_cases {
          Coproduct::Inl((_pd, params)) => Coproduct::Inl(params),
          Coproduct::Inr(inr_value) => Coproduct::Inr(inr_value),
        };
        self.continue_run(process_before_split_produced, produced, back_token).await
      }
      IntermediateFinalizedSplitOutcome::Yield(a, b, c, d) => Ok(IntermediateFlowingSplitOutcome::Yield(a, b, c, d)),
      IntermediateFinalizedSplitOutcome::Finish(a) => Ok(IntermediateFlowingSplitOutcome::Finish(a)),
      IntermediateFinalizedSplitOutcome::RetryUserInput(a, b) => Ok(IntermediateFlowingSplitOutcome::RetryUserInput(a, b)),
      IntermediateFinalizedSplitOutcome::Back => Ok(IntermediateFlowingSplitOutcome::Back),
    }
  }

  fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex {
    let used_index = self.split_process_before.enumerate_steps(last_used_index);
    self.case_index = used_index + 1;
    self.this_case.enumerate_steps(self.case_index)
  }
}

/// last case
impl<
  ThisTag: Send + Sync,
  SplitterProducesForThisCase: ParamList + Concat<ProcessBefore::ProcessBeforeSplitProduces>,
  ProcessBefore: FinalizedSplitProcess<Coproduct<(ThisTag, SplitterProducesForThisCase), CNil>>,
  ThisCase: FlowingProcess<
    SubprocessConsumes=<SplitterProducesForThisCase as Concat<ProcessBefore::ProcessBeforeSplitProduces>>::Concatenated,
    Messages=ProcessBefore::Messages,
  >,
> FlowingProcess
for FlowingCaseOfFinalizedSplitProcess<
  ThisTag,
  SplitterProducesForThisCase,
  CNil,
  ProcessBefore,
  ThisCase,
>
where
  ThisCase::EverProduced: Union<ProcessBefore::EverProduced>,
  <ThisCase::EverProduced as Union<ProcessBefore::EverProduced>>::Union: ParamList,
{
  type ProcessBeforeProduces = ProcessBefore::ProcessBeforeSplitProduces;
  type Produces = ThisCase::Produces;
  type SubprocessConsumes = ProcessBefore::SubprocessConsumes;
  type Messages = ProcessBefore::Messages;
  type EverProduced = <ThisCase::EverProduced as Union<ProcessBefore::EverProduced>>::Union;

  async fn resume_run(
    &self,
    previous_run_produced: SessionContext,
    previous_run_yielded_at: PreviousRunYieldedAt,
    user_input: String,
    form_context: MaybeFormContext,
    back_token: Option<BackToken>,
  ) -> IntermediateRunResult<Self::Produces, Self::Messages> {
    if previous_run_yielded_at.0 < self.case_index {
      let process_before_output = self
        .split_process_before
        .resume_run(previous_run_produced, previous_run_yielded_at, user_input, form_context, back_token)
        .await?;
      match process_before_output {
        IntermediateFinalizedSplitOutcome::GoToCase {
          process_before_split_produced,
          splitter_produces_to_other_cases,
        } => match splitter_produces_to_other_cases {
          Coproduct::Inl((_pd, produces_to_this_case)) => {
            let this_case_consumes = produces_to_this_case.concat(process_before_split_produced);
            match self.this_case.run_subprocess(this_case_consumes, back_token).await? {
              IntermediateRunOutcome::Continue(this_case_produced) => Ok(IntermediateRunOutcome::Continue(this_case_produced)),
              IntermediateRunOutcome::Yield(a, b, c, d) => Ok(IntermediateRunOutcome::Yield(a, b, c, d)),
              IntermediateRunOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
              IntermediateRunOutcome::RetryUserInput(a, b) => Ok(IntermediateRunOutcome::RetryUserInput(a, b)),
              IntermediateRunOutcome::Back => Ok(IntermediateRunOutcome::Back),
            }
          }
          Coproduct::Inr(c_nil) => match c_nil {},
        },
        IntermediateFinalizedSplitOutcome::Yield(a, b, c, d) => Ok(IntermediateRunOutcome::Yield(a, b, c, d)),
        IntermediateFinalizedSplitOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
        IntermediateFinalizedSplitOutcome::RetryUserInput(a, b) => Ok(IntermediateRunOutcome::RetryUserInput(a, b)),
        IntermediateFinalizedSplitOutcome::Back => Ok(IntermediateRunOutcome::Back),
      }
    } else {
      match self.this_case.resume_run(
        previous_run_produced,
        previous_run_yielded_at,
        user_input,
        form_context,
        back_token,
      ).await? {
        IntermediateRunOutcome::Continue(a) => Ok(IntermediateRunOutcome::Continue(a)),
        IntermediateRunOutcome::Yield(a, b, c, d) => Ok(IntermediateRunOutcome::Yield(a, b, c, d)),
        IntermediateRunOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
        IntermediateRunOutcome::RetryUserInput(a, b) => Ok(IntermediateRunOutcome::RetryUserInput(a, b)),
        IntermediateRunOutcome::Back => Ok(IntermediateRunOutcome::Back),
      }
    }
  }

  /// The process execution will call [`crate::builder::SplitProcess::continue_run`] instead of this one.
  /// I implemented it like this to avoid:
  /// ```ignore
  /// let _ = EntryA
  ///   .show_split(SplitA, |subprocess|
  ///     subprocess
  ///       .case_via(Case1, |x| x)
  ///       .case_via(Case2, |x| x.show(FormA))
  ///   )
  ///   .end(FinalA);
  /// ```
  /// and use the builder like this instead:
  /// ```ignore
  /// let _ = EntryA
  ///   .show_split(SplitA)
  ///   .case_via(Case1, |x| x)
  ///   .case_via(Case2, |x| x.show(FormA))
  ///   .end(FinalA);
  /// ```
  async fn continue_run(&self, _process_before_produces: Self::ProcessBeforeProduces, _back_token: Option<BackToken>) -> IntermediateRunResult<Self::Produces, Self::Messages> {
    unreachable!("continue_run from last case is unreachable. The process is always continued from SplitProcess")
  }

  async fn run_subprocess(&self, subprocess_consumes: Self::SubprocessConsumes, back_token: Option<BackToken>) -> IntermediateRunResult<Self::Produces, Self::Messages> {
    let process_before_output = self.split_process_before.run_split_subprocess(subprocess_consumes, back_token).await?;
    match process_before_output {
      IntermediateFinalizedSplitOutcome::GoToCase {
        process_before_split_produced,
        splitter_produces_to_other_cases,
      } => match splitter_produces_to_other_cases {
        Coproduct::Inl((_pd, produces_to_this_case)) => {
          let this_case_consumes = produces_to_this_case.concat(process_before_split_produced);
          match self.this_case.run_subprocess(this_case_consumes, back_token).await? {
            IntermediateRunOutcome::Continue(this_case_produced) => Ok(IntermediateRunOutcome::Continue(this_case_produced)),
            IntermediateRunOutcome::Yield(a, b, c, d) => Ok(IntermediateRunOutcome::Yield(a, b, c, d)),
            IntermediateRunOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
            IntermediateRunOutcome::RetryUserInput(a, b) => Ok(IntermediateRunOutcome::RetryUserInput(a, b)),
            IntermediateRunOutcome::Back => Ok(IntermediateRunOutcome::Back),
          }
        }
        Coproduct::Inr(c_nil) => match c_nil {},
      },
      IntermediateFinalizedSplitOutcome::Yield(a, b, c, d) => Ok(IntermediateRunOutcome::Yield(a, b, c, d)),
      IntermediateFinalizedSplitOutcome::Finish(a) => Ok(IntermediateRunOutcome::Finish(a)),
      IntermediateFinalizedSplitOutcome::RetryUserInput(a, b) => Ok(IntermediateRunOutcome::RetryUserInput(a, b)),
      IntermediateFinalizedSplitOutcome::Back => Ok(IntermediateRunOutcome::Back),
    }
  }

  fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex {
    let used_index = self.split_process_before.enumerate_steps(last_used_index);
    self.case_index = used_index + 1;
    self.this_case.enumerate_steps(self.case_index)
  }
}
