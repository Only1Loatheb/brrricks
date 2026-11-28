pub mod finalized_case_of_flowing_split_process;
pub mod first_case_of_flowing_split_process;
pub mod flowing_case_of_finalized_split_process;
pub mod flowing_case_of_flowing_split_process;

pub(crate) mod sealed {
  use crate::builder::{
    IntermediateFlowingSplitResult, IntermediateFlowingSplitStartResult, PreviousRunYieldedAt, StepIndex,
  };
  use crate::error_coproduct::ErrorCoproduct;
  use crate::param_list::ParamList;
  use crate::param_list::concat::Concat;
  use crate::step::{BackToken, ProcessMessages};
  use std::future::Future;

  pub trait FlowingSplitProcess<SplitterProducesForOtherCases>: Sized + Send + Sync {
    // Please specify all associated types at the impl FlowingSplitProcess side for inference to work.
    type EntryConsumes: ParamList;
    type ProcessBeforeSplitProduces: ParamList;
    type SplitterProducesForThisCase: ParamList + Concat<Self::ProcessBeforeSplitProduces>;
    type EveryFlowingCaseProduces: ParamList; // already includes ProcessBeforeSplitProduces;
    type Messages: ProcessMessages;
    type ProcessBeforeSplitEverProduced: ParamList;
    type EverProduced: ParamList;
    type Errors: ErrorCoproduct;

    fn resume_run(
      &self,
      previous_run_produced: &[u8],
      previous_run_yielded_at: PreviousRunYieldedAt,
      user_input: &str,
      form_context: Option<&[u8]>,
      back_token: Option<BackToken>,
    ) -> impl Future<
      Output = IntermediateFlowingSplitResult<
        Self::ProcessBeforeSplitProduces,
        SplitterProducesForOtherCases,
        Self::EveryFlowingCaseProduces,
        Self::Messages,
        Self::Errors,
      >,
    > + Send;

    fn run_split_from_entry(
      &self,
      entry_consumes: Self::EntryConsumes,
      back_token: Option<BackToken>,
    ) -> impl Future<
      Output = IntermediateFlowingSplitStartResult<
        Self::ProcessBeforeSplitProduces,
        SplitterProducesForOtherCases,
        Self::EveryFlowingCaseProduces,
        Self::Messages,
        Self::Errors,
      >,
    > + Send;

    fn enumerate_steps(&mut self, last_used_index: StepIndex) -> StepIndex;
  }
}

pub(crate) use sealed::FlowingSplitProcess;
