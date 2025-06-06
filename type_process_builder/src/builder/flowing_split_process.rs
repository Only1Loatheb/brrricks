use crate::builder::finalized_process::FinalizedProcess;
use crate::builder::finalized_split_process::FinalizedSplitProcess;
use crate::builder::flowing_process::FlowingProcess;
use crate::param_list::ParamList;
use crate::step::splitter_output_repr::SplitterOutput;
use crate::step::step::Splitter;
use std::marker::PhantomData;

pub trait FlowingSplitProcess {}

pub struct FirstCaseOfFlowingSplitProcess<
  ProcessBefore: FlowingProcess,
  SplitterConsumes: ParamList,
  SplitterProduces: SplitterOutput,
  SplitterStep: Splitter<SplitterConsumes, SplitterProduces>,
  FirstCase: FlowingProcess,
> {
  pub process_before: ProcessBefore,
  pub splitter: SplitterStep,
  pub first_case: FirstCase,
  pub phantom_data: PhantomData<(SplitterConsumes, SplitterProduces)>,
}

impl<
    ProcessBefore: FlowingProcess,
    SplitterConsumes: ParamList,
    SplitterProduces: SplitterOutput,
    SplitterStep: Splitter<SplitterConsumes, SplitterProduces>,
    FirstCase: FlowingProcess,
  > FlowingSplitProcess
  for FirstCaseOfFlowingSplitProcess<ProcessBefore, SplitterConsumes, SplitterProduces, SplitterStep, FirstCase>
{
}

pub struct NextCaseFlowingOfFlowingSplitProcess<ProcessBefore: FlowingSplitProcess, NextCase: FlowingProcess> {
  pub split_process_before: ProcessBefore,
  pub next_case: NextCase,
}
impl<ProcessBefore: FlowingSplitProcess, NextCase: FlowingProcess> FlowingSplitProcess
  for NextCaseFlowingOfFlowingSplitProcess<ProcessBefore, NextCase>
{
}

pub struct NextCaseFinalizedOfFlowingSplitProcess<ProcessBefore: FlowingSplitProcess, NextCase: FinalizedProcess> {
  pub split_process_before: ProcessBefore,
  pub next_case: NextCase,
}
impl<ProcessBefore: FlowingSplitProcess, NextCase: FinalizedProcess> FlowingSplitProcess
  for NextCaseFinalizedOfFlowingSplitProcess<ProcessBefore, NextCase>
{
}

pub struct NextCaseFromFinalizedOfFlowingSplitProcess<ProcessBefore: FinalizedSplitProcess, NextCase: FlowingProcess> {
  pub split_process_before: ProcessBefore,
  pub next_case: NextCase,
}
impl<ProcessBefore: FinalizedSplitProcess, NextCase: FlowingProcess> FlowingSplitProcess
  for NextCaseFromFinalizedOfFlowingSplitProcess<ProcessBefore, NextCase>
{
}
