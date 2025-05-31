  use crate::builder::finalized_process::FinalizedProcess;
  use crate::builder::finalized_split_process::FinalizedSplitProcess;
  use crate::builder::flowing_process::FlowingProcess;
  use crate::step::param_list::ParamList;
  use crate::step::splitter_output_repr::SplitterOutput;
  use crate::step::step::Splitter;
  use std::marker::PhantomData;

  pub trait FlowingSplitProcess {}

  pub struct FirstCaseOfFlowingSplitProcess<
    PROCESS_BEFORE: FlowingProcess,
    SPLITTER_CONSUMES: ParamList,
    SPLITTER_PRODUCES: SplitterOutput,
    SPLITTER_STEP: Splitter<SPLITTER_CONSUMES, SPLITTER_PRODUCES>,
    FIRST_CASE: FlowingProcess,
  > {
    pub process_before: PROCESS_BEFORE,
    pub splitter: SPLITTER_STEP,
    pub first_case: FIRST_CASE,
    pub phantom_data: PhantomData<(SPLITTER_CONSUMES, SPLITTER_PRODUCES)>,
  }

  impl<
      PROCESS_BEFORE: FlowingProcess,
      SPLITTER_CONSUMES: ParamList,
      SPLITTER_PRODUCES: SplitterOutput,
      SPLITTER_STEP: Splitter<SPLITTER_CONSUMES, SPLITTER_PRODUCES>,
      FIRST_CASE: FlowingProcess,
    > FlowingSplitProcess
    for FirstCaseOfFlowingSplitProcess<PROCESS_BEFORE, SPLITTER_CONSUMES, SPLITTER_PRODUCES, SPLITTER_STEP, FIRST_CASE>
  {
  }

  pub struct NextCaseFlowingOfFlowingSplitProcess<PROCESS_BEFORE: FlowingSplitProcess, NEXT_CASE: FlowingProcess> {
    pub split_process_before: PROCESS_BEFORE,
    pub next_case: NEXT_CASE,
  }
  impl<PROCESS_BEFORE: FlowingSplitProcess, NEXT_CASE: FlowingProcess> FlowingSplitProcess
    for NextCaseFlowingOfFlowingSplitProcess<PROCESS_BEFORE, NEXT_CASE>
  {
  }

  pub struct NextCaseFinalizedOfFlowingSplitProcess<PROCESS_BEFORE: FlowingSplitProcess, NEXT_CASE: FinalizedProcess> {
    pub split_process_before: PROCESS_BEFORE,
    pub next_case: NEXT_CASE,
  }
  impl<PROCESS_BEFORE: FlowingSplitProcess, NEXT_CASE: FinalizedProcess> FlowingSplitProcess
    for NextCaseFinalizedOfFlowingSplitProcess<PROCESS_BEFORE, NEXT_CASE>
  {
  }

  pub struct NextCaseFromFinalizedOfFlowingSplitProcess<PROCESS_BEFORE: FinalizedSplitProcess, NEXT_CASE: FlowingProcess> {
    pub split_process_before: PROCESS_BEFORE,
    pub next_case: NEXT_CASE,
  }
  impl<PROCESS_BEFORE: FinalizedSplitProcess, NEXT_CASE: FlowingProcess> FlowingSplitProcess
    for NextCaseFromFinalizedOfFlowingSplitProcess<PROCESS_BEFORE, NEXT_CASE>
  {
  }

