use crate::invariant::Invariant;
use crate::step::step::Linear;
use crate::step::*;
use frunk_core::coproduct::{CNil, Coproduct};
use frunk_core::hlist::{HList, Selector};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use process_builder_common::process_domain::Message;

trait Process<SERIALIZED> {
    async fn interpret(&self, input: SERIALIZED) -> anyhow::Result<Option<Message>>;
}

pub mod flowing_process {
  use crate::builder::finalized_process::{FinalizedProcess, FlowingFinalizedProcess};
  use crate::builder::flowing_split_process::FlowingSplitProcess;
  use crate::hlist_concat::Concat;
  use crate::step::param_list::ParamList;
  use crate::step::step::{Final, Linear};
  use frunk_core::hlist::{HNil, Selector};
  use process_builder_common::process_domain::Message;
  use crate::builder::Process;

  pub trait FlowingProcess: Process {
    type Produced: ParamList;
  }

  pub struct EmptyProcess;
  impl Process for EmptyProcess {
    async fn interpret(&self, input: ()) -> anyhow::Result<Option<Message>> {
      anyhow::Ok(None)
    }
  }
  impl FlowingProcess for EmptyProcess {
    type Produced = HNil;
  }

  pub struct LinearFlowingProcess<
    PROCESS_BEFORE: FlowingProcess,
    LINEAR_CONSUMES: ParamList,
    LINEAR_PRODUCES: ParamList,
  > {
    pub process_before: PROCESS_BEFORE,
    pub final_step: dyn Linear<LINEAR_CONSUMES, LINEAR_PRODUCES>,
  }

  impl<
      PROCESS_BEFORE: FlowingProcess,
      LINEAR_CONSUMES: ParamList,
      LINEAR_PRODUCES: ParamList + Concat<PROCESS_BEFORE>,
    > FlowingProcess for LinearFlowingProcess<PROCESS_BEFORE, LINEAR_CONSUMES, LINEAR_PRODUCES>
  {
    type Produced = <LINEAR_PRODUCES as Concat<PROCESS_BEFORE>>::Concatenated;

    async fn interpret(&self, input: ()) -> anyhow::Result<Option<Message>> {
      todo!()
    }
  }

  // pub struct SplitFlowingProcess<FLOWING_SPLIT_PROCESS: FlowingSplitProcess> {
  //   pub process: FLOWING_SPLIT_PROCESS,
  // }
  // impl<FLOWING_SPLIT_PROCESS: FlowingSplitProcess> FlowingProcess for SplitFlowingProcess<FLOWING_SPLIT_PROCESS> {
  //   type Produced = ();
  // }

  // methods
  impl EmptyProcess {
    fn finnish<FINAL_STEP: Final<HNil>>(&self, step: FINAL_STEP) -> impl FinalizedProcess {
      FlowingFinalizedProcess {
        process_before: EmptyProcess,
        final_step: step,
      }
    }
  }

  impl<PROCESS_BEFORE: FlowingProcess, LINEAR_CONSUMES: ParamList, LINEAR_PRODUCES: ParamList>
    LinearFlowingProcess<PROCESS_BEFORE, LINEAR_CONSUMES, LINEAR_PRODUCES>
  {
    fn finnish<SEL, FINAL_CONSUMES: ParamList + Selector<LINEAR_PRODUCES, SEL>, FINAL_STEP: Final<FINAL_CONSUMES>>(
      &self,
      step: FINAL_STEP,
    ) -> impl FinalizedProcess {
      FlowingFinalizedProcess {
        process_before: self,
        final_step: step,
      }
    }
  }
}

pub mod finalized_process {
  use crate::builder::finalized_split_process::FinalizedSplitProcess;
  use crate::builder::flowing_process::FlowingProcess;
  use crate::step::param_list::ParamList;
  use crate::step::step::Final;

  pub trait FinalizedProcess {}

  pub struct FlowingFinalizedProcess<PROCESS_BEFORE: FlowingProcess, FINAL_CONSUMES: ParamList> {
    pub process_before: PROCESS_BEFORE,
    pub final_step: dyn Final<FINAL_CONSUMES>,
  }
  impl<PROCESS_BEFORE: FlowingProcess, FINAL_CONSUMES: ParamList> FinalizedProcess
    for FlowingFinalizedProcess<PROCESS_BEFORE, FINAL_CONSUMES>
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
    PROCESS_BEFORE: FlowingProcess,
    SPLITTER_CONSUMES: ParamList,
    SPLITTER_PRODUCES: SplitterOutput,
    FIRST_CASE: FinalizedProcess,
  > {
    pub process_before: PROCESS_BEFORE,
    pub splitter: dyn Splitter<SPLITTER_CONSUMES, SPLITTER_PRODUCES>,
    pub first_case: FIRST_CASE,
  }
  impl<
      PROCESS_BEFORE: FlowingProcess,
      SPLITTER_CONSUMES: ParamList,
      SPLITTER_PRODUCES: SplitterOutput,
      FIRST_CASE: FinalizedProcess,
    > FinalizedSplitProcess
    for FirstCaseOfFinalizedSplitProcess<PROCESS_BEFORE, SPLITTER_CONSUMES, SPLITTER_PRODUCES, FIRST_CASE>
  {
  }

  pub struct NextCaseOfFinalizedSplitProcess<PROCESS_BEFORE: FinalizedProcess, NEXT_CASE: FinalizedProcess> {
    pub split_process_before: PROCESS_BEFORE,
    pub next_case: NEXT_CASE,
  }
  impl<PROCESS_BEFORE: FinalizedProcess, NEXT_CASE: FinalizedProcess> FinalizedSplitProcess
    for NextCaseOfFinalizedSplitProcess<PROCESS_BEFORE, NEXT_CASE>
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
    PROCESS_BEFORE: FlowingProcess,
    SPLITTER_CONSUMES: ParamList,
    SPLITTER_PRODUCES: SplitterOutput,
    FIRST_CASE: FlowingSplitProcess,
  > {
    pub process_before: PROCESS_BEFORE,
    pub splitter: dyn Splitter<SPLITTER_CONSUMES, SPLITTER_PRODUCES>,
    pub first_case: FIRST_CASE,
  }
  impl<
      PROCESS_BEFORE: FlowingProcess,
      SPLITTER_CONSUMES: ParamList,
      SPLITTER_PRODUCES: SplitterOutput,
      FIRST_CASE: FinalizedProcess,
    > FlowingSplitProcess
    for FirstCaseOfFlowingSplitProcess<PROCESS_BEFORE, SPLITTER_CONSUMES, SPLITTER_PRODUCES, FIRST_CASE>
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

  pub struct NextCaseFromFinalizedOfFlowingSplitProcess<
    PROCESS_BEFORE: FinalizedSplitProcess,
    NEXT_CASE: FlowingProcess,
  > {
    pub split_process_before: PROCESS_BEFORE,
    pub next_case: NEXT_CASE,
  }
  impl<PROCESS_BEFORE: FinalizedSplitProcess, NEXT_CASE: FlowingProcess> FlowingSplitProcess
    for NextCaseFromFinalizedOfFlowingSplitProcess<PROCESS_BEFORE, NEXT_CASE>
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
