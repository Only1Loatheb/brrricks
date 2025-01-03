use crate::invariant::Invariant;
use crate::step::step::Linear;
use crate::step::*;
use frunk_core::coproduct::{CNil, Coproduct};
use frunk_core::hlist::{HList, Selector};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub mod flowing_process {
  use crate::builder::flowing_split_process::FlowingSplitProcess;
  use crate::step::param_list::ParamList;
  use crate::step::step::Linear;

  pub trait FlowingProcess {}

  pub struct EmptyProcess;
  impl FlowingProcess for EmptyProcess {}

  pub struct LinearFlowingProcess<
    LINEAR_CONSUMES: ParamList,
    LINEAR_PRODUCES: ParamList,
    PROCESS_BEFORE: FlowingProcess,
  > {
    pub last_step: dyn Linear<LINEAR_CONSUMES, LINEAR_PRODUCES>,
    pub process_before: PROCESS_BEFORE,
  }
  impl<LINEAR_CONSUMES: ParamList, LINEAR_PRODUCES: ParamList, PROCESS_BEFORE: FlowingProcess> FlowingProcess
    for LinearFlowingProcess<LINEAR_CONSUMES, LINEAR_PRODUCES, PROCESS_BEFORE>
  {
  }

  pub struct SplitFlowingProcess<FLOWING_SPLIT_PROCESS: FlowingSplitProcess> {
    pub process: FLOWING_SPLIT_PROCESS,
  }
  impl<FLOWING_SPLIT_PROCESS: FlowingSplitProcess> FlowingProcess for SplitFlowingProcess<FLOWING_SPLIT_PROCESS> {
  }
}

pub mod finalized_process {
  use crate::builder::finalized_split_process::FinalizedSplitProcess;
  use crate::builder::flowing_process::FlowingProcess;
  use crate::step::param_list::ParamList;
  use crate::step::step::Final;

  pub trait FinalizedProcess {}

  pub struct FlowingFinalizedProcess<FINAL_CONSUMES: ParamList, PROCESS_BEFORE: FlowingProcess> {
    pub last_step: dyn Final<FINAL_CONSUMES>,
    pub process_before: PROCESS_BEFORE,
  }
  impl<FINAL_CONSUMES: ParamList, PROCESS_BEFORE: FlowingProcess> FinalizedProcess
    for FlowingFinalizedProcess<FINAL_CONSUMES, PROCESS_BEFORE>
  {
  }

  pub struct SplitFinalizedProcess<FINALIZED_SPLIT_PROCESS: FinalizedSplitProcess> {
    process: FINALIZED_SPLIT_PROCESS, // maybe box?
  }
  impl<FINALIZED_SPLIT_PROCESS: FinalizedSplitProcess> FinalizedProcess
    for SplitFinalizedProcess<FINALIZED_SPLIT_PROCESS>
  {
  }
}

pub mod finalized_split_process {
  use crate::builder::finalized_process::FinalizedProcess;
  use crate::builder::flowing_process::FlowingProcess;
  use crate::step::param_list::ParamList;
  use crate::step::splitter_output_repr::SplitterOutput;
  use crate::step::step::Splitter;

  pub trait FinalizedSplitProcess {}

  pub struct FirstCaseOfFinalizedSplitProcess<
    SPLITTER_CONSUMES: ParamList,
    SPLITTER_PRODUCES: SplitterOutput,
    FIRST_CASE: FinalizedProcess,
    PROCESS_BEFORE: FlowingProcess,
  > {
    pub splitter: dyn Splitter<SPLITTER_CONSUMES, SPLITTER_PRODUCES>,
    pub first_case: FIRST_CASE,
    pub process_before: PROCESS_BEFORE,
  }
  impl<
      SPLITTER_CONSUMES: ParamList,
      SPLITTER_PRODUCES: SplitterOutput,
      FIRST_CASE: FinalizedProcess,
      PROCESS_BEFORE: FlowingProcess,
    > FinalizedSplitProcess
    for FirstCaseOfFinalizedSplitProcess<SPLITTER_CONSUMES, SPLITTER_PRODUCES, FIRST_CASE, PROCESS_BEFORE>
  {
  }

  pub struct NextCaseOfFinalizedSplitProcess<NEXT_CASE: FinalizedProcess, PROCESS_BEFORE: FinalizedProcess> {
    pub next_case: NEXT_CASE,
    pub split_process_before: PROCESS_BEFORE,
  }
  impl<NEXT_CASE: FinalizedProcess, PROCESS_BEFORE: FinalizedProcess> FinalizedSplitProcess
    for NextCaseOfFinalizedSplitProcess<NEXT_CASE, PROCESS_BEFORE>
  {
  }
}

pub mod flowing_split_process {
  use crate::builder::finalized_process::FinalizedProcess;
  use crate::builder::finalized_split_process::FinalizedSplitProcess;
  use crate::builder::flowing_process::FlowingProcess;
  use crate::step::param_list::ParamList;
  use crate::step::splitter_output_repr::SplitterOutput;
  use crate::step::step::Splitter;

  pub trait FlowingSplitProcess {}

  pub struct FirstCaseOfFlowingSplitProcess<
    SPLITTER_CONSUMES: ParamList,
    SPLITTER_PRODUCES: SplitterOutput,
    FIRST_CASE: FlowingSplitProcess,
    PROCESS_BEFORE: FlowingProcess,
  > {
    pub splitter: dyn Splitter<SPLITTER_CONSUMES, SPLITTER_PRODUCES>,
    pub first_case: FIRST_CASE,
    pub process_before: PROCESS_BEFORE,
  }
  impl<
      SPLITTER_CONSUMES: ParamList,
      SPLITTER_PRODUCES: SplitterOutput,
      FIRST_CASE: FinalizedProcess,
      PROCESS_BEFORE: FlowingProcess,
    > FlowingSplitProcess
    for FirstCaseOfFlowingSplitProcess<SPLITTER_CONSUMES, SPLITTER_PRODUCES, FIRST_CASE, PROCESS_BEFORE>
  {
  }

  pub struct NextCaseFlowingOfFlowingSplitProcess<NEXT_CASE: FlowingProcess, PROCESS_BEFORE: FlowingSplitProcess> {
    pub next_case: NEXT_CASE,
    pub split_process_before: PROCESS_BEFORE,
  }
  impl<NEXT_CASE: FlowingProcess, PROCESS_BEFORE: FlowingSplitProcess> FlowingSplitProcess
    for NextCaseFlowingOfFlowingSplitProcess<NEXT_CASE, PROCESS_BEFORE>
  {
  }

  pub struct NextCaseFinalizedOfFlowingSplitProcess<NEXT_CASE: FinalizedProcess, PROCESS_BEFORE: FlowingSplitProcess> {
    pub next_case: NEXT_CASE,
    pub split_process_before: PROCESS_BEFORE,
  }
  impl<NEXT_CASE: FinalizedProcess, PROCESS_BEFORE: FlowingSplitProcess> FlowingSplitProcess
    for NextCaseFinalizedOfFlowingSplitProcess<NEXT_CASE, PROCESS_BEFORE>
  {
  }

  pub struct NextCaseFromFinalizedOfFlowingSplitProcess<NEXT_CASE: FlowingProcess, PROCESS_BEFORE: FinalizedSplitProcess> {
    pub next_case: NEXT_CASE,
    pub split_process_before: PROCESS_BEFORE,
  }
  impl<NEXT_CASE: FlowingProcess, PROCESS_BEFORE: FinalizedSplitProcess> FlowingSplitProcess
    for NextCaseFromFinalizedOfFlowingSplitProcess<NEXT_CASE, PROCESS_BEFORE>
  {
  }
}

// pub struct FlowingProcess<'same_process> {
//   pub(crate) process: InternalFlowingProcess,
//   pub(crate) next_param_id: usize,
//   pub(crate) same_process_invariant: Invariant<'same_process>,
// }
//
// impl<'same_process> FlowingProcess<'same_process> {
//   pub fn finnish<FINAL_BRICK_CONSUMES: ParamList>(
//     mut self,
//     _consumes: FINAL_BRICK_CONSUMES,
//     brick: FinalBrick<'same_process, FINAL_BRICK_CONSUMES>,
//   ) -> FinalizedProcess<'same_process> {
//     FinalizedProcess {
//       process: InternalFinalizedProcess::Flowing(brick.to_internal(), self.process),
//       next_param_id: self.next_param_id,
//       same_process_invariant: Default::default(),
//     }
//   }
//
//   pub fn split<
//     ROOT_CONSUMES: ParamList, // could be solved with changing 'same_process lifetime bounds
//     SEL,
//     CONSUMES_CASE_THIS: ParamList + Selector<ROOT_CONSUMES, SEL>,
//     PRODUCES_CASE_THIS: ParamList,
//     PRODUCES_CASE_OTHER: SplitterOutputRepr<'same_process>,
//   >(
//     self,
//     _consumes: ROOT_CONSUMES,
//     splitter_brick: SplitterBrick<'same_process, PRODUCES_CASE_THIS, PRODUCES_CASE_OTHER>,
//     first_case_process: FlowingProcess<'same_process>,
//   ) -> FlowingSplitterProcess<'same_process, ROOT_CONSUMES, PRODUCES_CASE_OTHER> {
//     FlowingSplitterProcess {
//       process: InternalFlowingSplitProcess::FirstCase {
//         splitter_brick: splitter_brick.to_internal(),
//         first_case: first_case_process.process,
//         process_before: self.process,
//       },
//       root_consumes: Default::default(),
//       produces_case_other: Default::default(),
//       next_param_id: first_case_process.next_param_id,
//     }
//   }
// }
//
// pub struct FinalizedSplitterProcess<
//   'same_process,
//   ROOT_CONSUMES: ParamList,
//   PRODUCES_CASE_OTHER: SplitterOutputRepr<'same_process>,
// > {
//   pub(crate) process: InternalFinalizedSplitProcess,
//   pub(crate) root_consumes: PhantomData<ROOT_CONSUMES>,
//   pub(crate) produces_case_other: PhantomData<PRODUCES_CASE_OTHER>,
//   pub(crate) next_param_id: usize,
// }
//
// pub struct FlowingSplitterProcess<
//   'same_process,
//   ROOT_CONSUMES: ParamList,
//   PRODUCES_CASE_OTHER: SplitterOutputRepr<'same_process>,
// > {
//   pub(crate) process: InternalFlowingSplitProcess,
//   pub(crate) root_consumes: PhantomData<ROOT_CONSUMES>,
//   pub(crate) produces_case_other: PhantomData<PRODUCES_CASE_OTHER>,
//   pub(crate) next_param_id: usize,
// }
//
// impl<'same_process, ROOT_CONSUMES: ParamList, PRODUCES_CASE_LAST: ParamList>
//   FlowingSplitterProcess<'same_process, ROOT_CONSUMES, Coproduct<PRODUCES_CASE_LAST, CNil>>
// {
//   pub fn last_case(
//     self,
//     _consumes: PRODUCES_CASE_LAST,
//     last_case_process: FlowingProcess<'same_process>,
//   ) -> FlowingProcess<'same_process> {
//     FlowingProcess {
//       process: InternalFlowingProcess::Split {
//         0: Box::new(InternalFlowingSplitProcess::NextCaseFlowing {
//           next_case: last_case_process.process,
//           split_process_before: Box::new(self.process),
//         }),
//       },
//       next_param_id: last_case_process.next_param_id,
//       same_process_invariant: Default::default(),
//     }
//   }
// }
//
// impl<
//     'same_process,
//     ROOT_CONSUMES: ParamList,
//     PRODUCES_CASE_THIS: ParamList,
//     PRODUCES_CASE_OTHER: SplitterOutputRepr<'same_process>,
//   > FlowingSplitterProcess<'same_process, ROOT_CONSUMES, PRODUCES_CASE_OTHER>
// {
//   pub fn next_case(
//     self,
//     _consumes: PRODUCES_CASE_THIS,
//     next_case_process: FlowingProcess<'same_process>,
//   ) -> FlowingSplitterProcess<'same_process, ROOT_CONSUMES, PRODUCES_CASE_OTHER> {
//     FlowingSplitterProcess {
//       process: InternalFlowingSplitProcess::NextCaseFlowing {
//         next_case: next_case_process.process,
//         split_process_before: Box::new(self.process),
//       },
//       root_consumes: Default::default(),
//       produces_case_other: Default::default(),
//       next_param_id: next_case_process.next_param_id,
//     }
//   }
// }
//
// pub struct FinalizedProcess<'same_process> {
//   pub(crate) process: InternalFinalizedProcess,
//   pub(crate) next_param_id: usize,
//   pub(crate) same_process_invariant: Invariant<'same_process>,
//   // add consumes           ???
// }
//
// impl<'same_process> FinalizedProcess<'same_process> {
//   pub fn close(self, path: String) -> NamedProcess {
//     NamedProcess {
//       path,
//       process: self.process,
//     }
//   }
// }
