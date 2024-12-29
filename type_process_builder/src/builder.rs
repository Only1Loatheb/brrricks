use crate::invariant::Invariant;
use crate::step::param_repr_list::ParamReprList;
use crate::step::step::Linear;
use crate::step::*;
use frunk_core::coproduct::{CNil, Coproduct};
use frunk_core::hlist::{HList, Selector};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
//
// // We can allow one case for to support accept forms and confirm forms
// pub enum InternalFinalizedSplitProcess {
//     FirstCase {
//         splitter_brick: InternalSplitterBrick,
//         first_cases: InternalFinalizedProcess,
//         process_before: InternalFlowingProcess,
//     },
//     NextCase {
//         next_case: InternalFinalizedProcess,
//         split_process_before: Box<InternalFinalizedSplitProcess>,
//     },
// }
//
// // builder has to take care of having at least 2 cases
// pub enum InternalFlowingSplitProcess {
//     FirstCase {
//         splitter_brick: InternalSplitterBrick,
//         first_case: InternalFlowingProcess,
//         process_before: InternalFlowingProcess,
//     },
//     NextCase {
//         next_case: InternalFlowingProcess,
//         split_process_before: Box<InternalFinalizedSplitProcess>,
//     },
//     NextCaseFlowing {
//         next_case: InternalFlowingProcess,
//         split_process_before: Box<InternalFlowingSplitProcess>,
//     },
//     NextCaseFinalized {
//         next_case: InternalFinalizedProcess,
//         split_process_before: Box<InternalFlowingSplitProcess>,
//     },
// }
// pub enum InternalFlowingProcess {
//     Empty,
//     Linear(InternalLinearBrick, Box<InternalFlowingProcess>),
//     Split(Box<InternalFlowingSplitProcess>),
// }
//
// pub enum InternalFinalizedProcess {
//     Linear(InternalFinalBrick, InternalFlowingProcess),
//     Split(Box<InternalFinalizedSplitProcess>),
// }

pub mod flowing_process {
  use crate::step::param_repr_list::ParamReprList;
  use crate::step::step::Linear;
  use frunk_core::hlist::{HCons, HNil};

  pub trait FlowingProcess {}

  impl FlowingProcess for HNil {}

  impl<LAST_STEP_CONSUMES: ParamReprList, LAST_STEP_PRODUCES: ParamReprList, PROCESS_BEFORE: FlowingProcess>
    FlowingProcess for HCons<dyn Linear<CONSUMES = LAST_STEP_CONSUMES, PRODUCES = LAST_STEP_PRODUCES>, PROCESS_BEFORE>
  {
  }

  pub fn empty() -> impl FlowingProcess {
    HNil
  }
}

pub mod finalized_process {
  use crate::builder::flowing_process;
  use crate::builder::flowing_process::FlowingProcess;
  use crate::step::param_repr_list::ParamReprList;
  use crate::step::step::Final;
  use frunk_core::hlist::HCons;

  // shouldn't be implemented for HNil
  pub trait FinalizedProcess {}

  impl<LAST_STEP_CONSUMES: ParamReprList, PROCESS_BEFORE: FlowingProcess> FinalizedProcess
    for HCons<dyn Final<CONSUMES = LAST_STEP_CONSUMES>, PROCESS_BEFORE>
  {
  }

  fn finnish(final_step: impl Final) -> impl FinalizedProcess {
    HCons {
      head: final_step,
      tail: flowing_process::empty(),
    }
  }
}

pub struct FlowingProcess<'same_process> {
  pub(crate) process: InternalFlowingProcess,
  pub(crate) next_param_id: usize,
  pub(crate) same_process_invariant: Invariant<'same_process>,
}

impl<'same_process> FlowingProcess<'same_process> {
  pub fn finnish<FINAL_BRICK_CONSUMES: ParamReprList<'same_process>>(
    mut self,
    _consumes: FINAL_BRICK_CONSUMES,
    brick: FinalBrick<'same_process, FINAL_BRICK_CONSUMES>,
  ) -> FinalizedProcess<'same_process> {
    FinalizedProcess {
      process: InternalFinalizedProcess::Flowing(brick.to_internal(), self.process),
      next_param_id: self.next_param_id,
      same_process_invariant: Default::default(),
    }
  }

  pub fn split<
    ROOT_CONSUMES: ParamReprList<'same_process>, // could be solved with changing 'same_process lifetime bounds
    SEL,
    CONSUMES_CASE_THIS: ParamReprList<'same_process> + Selector<ROOT_CONSUMES, SEL>,
    PRODUCES_CASE_THIS: ParamReprList<'same_process>,
    PRODUCES_CASE_OTHER: SplitterOutputRepr<'same_process>,
  >(
    self,
    _consumes: ROOT_CONSUMES,
    splitter_brick: SplitterBrick<'same_process, PRODUCES_CASE_THIS, PRODUCES_CASE_OTHER>,
    first_case_process: FlowingProcess<'same_process>,
  ) -> FlowingSplitterProcess<'same_process, ROOT_CONSUMES, PRODUCES_CASE_OTHER> {
    FlowingSplitterProcess {
      process: InternalFlowingSplitProcess::FirstCase {
        splitter_brick: splitter_brick.to_internal(),
        first_case: first_case_process.process,
        process_before: self.process,
      },
      root_consumes: Default::default(),
      produces_case_other: Default::default(),
      next_param_id: first_case_process.next_param_id,
    }
  }
}

pub struct FinalizedSplitterProcess<
  'same_process,
  ROOT_CONSUMES: ParamReprList<'same_process>,
  PRODUCES_CASE_OTHER: SplitterOutputRepr<'same_process>,
> {
  pub(crate) process: InternalFinalizedSplitProcess,
  pub(crate) root_consumes: PhantomData<ROOT_CONSUMES>,
  pub(crate) produces_case_other: PhantomData<PRODUCES_CASE_OTHER>,
  pub(crate) next_param_id: usize,
}

pub struct FlowingSplitterProcess<
  'same_process,
  ROOT_CONSUMES: ParamReprList<'same_process>,
  PRODUCES_CASE_OTHER: SplitterOutputRepr<'same_process>,
> {
  pub(crate) process: InternalFlowingSplitProcess,
  pub(crate) root_consumes: PhantomData<ROOT_CONSUMES>,
  pub(crate) produces_case_other: PhantomData<PRODUCES_CASE_OTHER>,
  pub(crate) next_param_id: usize,
}

impl<'same_process, ROOT_CONSUMES: ParamReprList<'same_process>, PRODUCES_CASE_LAST: ParamReprList<'same_process>>
  FlowingSplitterProcess<'same_process, ROOT_CONSUMES, Coproduct<PRODUCES_CASE_LAST, CNil>>
{
  pub fn last_case(
    self,
    _consumes: PRODUCES_CASE_LAST,
    last_case_process: FlowingProcess<'same_process>,
  ) -> FlowingProcess<'same_process> {
    FlowingProcess {
      process: InternalFlowingProcess::Split {
        0: Box::new(InternalFlowingSplitProcess::NextCaseFlowing {
          next_case: last_case_process.process,
          split_process_before: Box::new(self.process),
        }),
      },
      next_param_id: last_case_process.next_param_id,
      same_process_invariant: Default::default(),
    }
  }
}

impl<
    'same_process,
    ROOT_CONSUMES: ParamReprList<'same_process>,
    PRODUCES_CASE_THIS: ParamReprList<'same_process>,
    PRODUCES_CASE_OTHER: SplitterOutputRepr<'same_process>,
  > FlowingSplitterProcess<'same_process, ROOT_CONSUMES, PRODUCES_CASE_OTHER>
{
  pub fn next_case(
    self,
    _consumes: PRODUCES_CASE_THIS,
    next_case_process: FlowingProcess<'same_process>,
  ) -> FlowingSplitterProcess<'same_process, ROOT_CONSUMES, PRODUCES_CASE_OTHER> {
    FlowingSplitterProcess {
      process: InternalFlowingSplitProcess::NextCaseFlowing {
        next_case: next_case_process.process,
        split_process_before: Box::new(self.process),
      },
      root_consumes: Default::default(),
      produces_case_other: Default::default(),
      next_param_id: next_case_process.next_param_id,
    }
  }
}

pub struct FinalizedProcess<'same_process> {
  pub(crate) process: InternalFinalizedProcess,
  pub(crate) next_param_id: usize,
  pub(crate) same_process_invariant: Invariant<'same_process>,
  // add consumes           ???
}

impl<'same_process> FinalizedProcess<'same_process> {
  pub fn close(self, path: String) -> NamedProcess {
    NamedProcess {
      path,
      process: self.process,
    }
  }
}
