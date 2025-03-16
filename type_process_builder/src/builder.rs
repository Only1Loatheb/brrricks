use crate::step::param_list::ParamList;
use crate::step::step::Linear;
use crate::step::*;
use frunk_core::coproduct::{CNil, Coproduct};
use frunk_core::hlist::{HList, Selector};
use process_builder_common::process_domain::Message;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::marker::PhantomData;

pub struct PreviousInterpretationYielded(usize);
pub struct CurrentInterpretationYields(usize);

pub enum InterpretationOutcome<T: ParamList> {
  Continue(T),
  Yield(Message, Value, CurrentInterpretationYields),
  Finish(Message),
}

type TYPE_TO_DESERIALIZE_FROM = read::IoRead<R>;

const WILL_BE_RENUMBERED: usize = 0;

type InterpretationResult<T: ParamList> = anyhow::Result<InterpretationOutcome<T>>;

pub mod flowing_process {
  use std::future::Future;
  use std::io;
  use crate::builder::finalized_process::{FinalizedProcess, FlowingFinalizedProcess};
  use crate::builder::flowing_split_process::FlowingSplitProcess;
  use crate::builder::InterpretationOutcome::*;
  use crate::builder::{
    CurrentInterpretationYields, InterpretationOutcome, InterpretationResult, PreviousInterpretationYielded, WILL_BE_RENUMBERED,
  };
  use crate::hlist_concat::Concat;
  use crate::hlist_empty::Empty;
  use crate::hlist_transformer::TransformTo;
  use crate::step::param_list::ParamList;
  use crate::step::step::{Final, Linear};
  use frunk_core::hlist::{HNil, Sculptor, Selector};
  use process_builder_common::process_domain::Message;
  use serde::de::DeserializeOwned;
  use serde::Deserializer;
  use serde_json::Value;
  use std::marker::PhantomData;
  use std::process::Output;

  pub trait FlowingProcess: Sized {
    type LastStepConsumes: ParamList;
    type Produces: ParamList;

    fn then<CONSUMES: ParamList, PRODUCES: ParamList + Concat<Self::Produces>, LINEAR_STEP: Linear<CONSUMES, PRODUCES>>(
      self,
      step: LINEAR_STEP,
    ) -> impl FlowingProcess {
      LinearFlowingProcess {
        process_before: self,
        last_step: step,
        step_index: WILL_BE_RENUMBERED,
        phantom_data: Default::default(),
      }
    }

    async fn interpret_resume(
      &self,
      consumes: impl io::Read,
      previous_interpretation_yielded: PreviousInterpretationYielded,
    ) -> InterpretationResult<Self::Produces>;

    async fn interpret(&self, last_step_consumes: Self::LastStepConsumes) -> InterpretationResult<Self::Produces>;

    // // should be the only interpret_resume method, but it is a bit too hard to implement right now
    // async fn interpret_resume(
    //   &self,
    //   previous_interpretation_produced: Value,
    //   last_interpreted: LastInterpreted,
    // ) -> InterpretationResult<Self::Produces>;
  }

  pub struct EmptyProcess;
  impl FlowingProcess for EmptyProcess {
    type LastStepConsumes = HNil;
    type Produces = HNil;

    async fn interpret_resume(
      &self,
      consumes: impl io::Read,
      previous_interpretation_yielded: PreviousInterpretationYielded,
    ) -> InterpretationResult<Self::Produces> {
      Ok(Continue(HNil))
    }

    async fn interpret(&self, last_step_consumes: Self::LastStepConsumes) -> InterpretationResult<Self::Produces> {
      Ok(Continue(HNil))
    }

    // async fn interpret_resume(
    //   &self,
    //   previous_interpretation_result: Value,
    //   last_interpreted: LastInterpreted,
    // ) -> InterpretationResult<Self::Produces> {
    //   Ok(Continue(HNil))
    // }
  }

  pub struct LinearFlowingProcess<
    PROCESS_BEFORE: FlowingProcess,
    LINEAR_CONSUMES: ParamList,
    LINEAR_PRODUCES: ParamList,
    LINEAR_STEP: Linear<LINEAR_CONSUMES, LINEAR_PRODUCES>,
  > {
    pub process_before: PROCESS_BEFORE,
    pub last_step: LINEAR_STEP,
    pub step_index: usize,
    pub phantom_data: PhantomData<(LINEAR_CONSUMES, LINEAR_PRODUCES)>,
  }

  // unique LAST_STEP_CONSUMES + COMMON_PART + unique PROCESS_BEFORE::Consumes + COMMON_PART = LAST_STEP_CONSUMES + PROCESS_BEFORE::Consumes
  // COMMON_PART = (LAST_STEP_CONSUMES + PROCESS_BEFORE::Consumes) - PROCESS_BEFORE::Consumes
  // NEW_CONSUMES = (Consumes + COMMON_PART + COMMON_PART) - COMMON_PART
  // Consumes = NEW_CONSUMES + PROCESS_BEFORE::Consumes
  impl<
    PROCESS_BEFORE: FlowingProcess,
    LAST_STEP_CONSUMES: ParamList,
    LAST_STEP_PRODUCES: ParamList + Concat<PROCESS_BEFORE::Produces>,
    LAST_STEP: Linear<LAST_STEP_CONSUMES, LAST_STEP_PRODUCES>,
  > FlowingProcess for LinearFlowingProcess<PROCESS_BEFORE, LAST_STEP_CONSUMES, LAST_STEP_PRODUCES, LAST_STEP>
  {
    type LastStepConsumes = LAST_STEP_CONSUMES;
    // fixme guarantee no overlap in LAST_STEP_PRODUCES and PROCESS_BEFORE::Produces
    type Produces = <LAST_STEP_PRODUCES as Concat<PROCESS_BEFORE::Produces>>::Concatenated;

    async fn interpret_resume(
      &self,
      consumes: impl io::Read,
      previous_interpretation_yielded: PreviousInterpretationYielded,
    ) -> InterpretationResult<Self::Produces> {
      if previous_interpretation_yielded.0 < self.step_index {
        let process_before_output = self.process_before.interpret_resume(consumes, previous_interpretation_yielded).await?;
        match process_before_output {
          Continue(process_before_produces) => {
            let last_step_consumes: LAST_STEP_CONSUMES = process_before_produces.clone().transform();
            match self.interpret(last_step_consumes).await? {
              Continue(last_step_produces) => last_step_produces.concat(process_before_produces),
              Yield(a, b, c) => Ok(Yield(a, b, c)),
              Finish(a) => Ok(Finish(a)),
            }
          }
          Yield(a, b, c) => Ok(Yield(a, b, c)),
          Finish(a) => Ok(Finish(a)),
        }
      } else {
        // fixme deserialize only values required only up to the next interaction
        let process_before_produces: PROCESS_BEFORE::Produces = serde_json::from_reader(consumes);
        let last_step_consumes: LAST_STEP_CONSUMES = process_before_produces.clone().transform();
        match self.interpret(last_step_consumes).await? {
          Continue(last_step_produces) => last_step_produces.concat(process_before_produces),
          Yield(a, b, c) => Ok(Yield(a, b, c)),
          Finish(a) => Ok(Finish(a)),
        }
      }
    }

    async fn interpret(&self, last_step_consumes: Self::LastStepConsumes) -> InterpretationResult<LAST_STEP_PRODUCES> {
      let last_step_output = self.last_step.handle(last_step_consumes).await?;
      match last_step_output {
        (Some(msg), last_step_produces) =>
        // Should only pass params required in further part of the process, but I don't know what they are.
        // todo Make all the methods generic over Serializer
          Ok(Yield(
            msg,
            last_step_produces.serialize(serde_json::value::Serializer)?, // fixme make it generic over format i.e. json
            CurrentInterpretationYields(self.step_index),
          )),
        (None, last_step_produces) => Ok(Continue(last_step_produces)),
      }
    }
  }
}

// async fn interpret_resume(
//   &self,
//   previous_interpretation_produced: Value,
//   last_interpreted: LastInterpreted,
// ) -> InterpretationResult<Self::Produces> {
//   if last_interpreted.0 < self.step_index {
//     let process_before_output = self
//       .process_before
//       .interpret_resume(previous_interpretation_produced, last_interpreted)
//       .await?;
//     match process_before_output {
//       Continue(process_before_produces) => {
//         let last_step_output = self.last_step.handle(process_before_produces).await?; //
//         // process_before_produces most likely will need to be adapted with a selector
//         match last_step_output {
//           (Some(msg), last_step_produces) => Ok(Yield(
//             msg,
//             serde_json::to_value(last_step_produces.concat(process_before_produces))?,
//             LastInterpreted(self.step_index),
//           )),
//           (None, last_step_produces) => Ok(Continue(last_step_produces.concat(process_before_produces))),
//         }
//       }
//       Yield(a, b, c) => Ok(Yield(a, b, c)),
//       Finish(a) => Ok(Finish(a)),
//     }
//   } else {
//     let params = serde_json::from_value::<Self::Produces>(previous_interpretation_produced)?;
//     Ok(Continue(params))
//   }
// }
}

// pub struct SplitFlowingProcess<FLOWING_SPLIT_PROCESS: FlowingSplitProcess> {
//   pub process: FLOWING_SPLIT_PROCESS,
// }
// impl<FLOWING_SPLIT_PROCESS: FlowingSplitProcess> FlowingProcess for SplitFlowingProcess<FLOWING_SPLIT_PROCESS> {
//   type Produced = ();
// }

// builder methods
impl EmptyProcess {
  fn finnish<FINAL_STEP: Final<HNil>>(self, step: FINAL_STEP) -> impl FinalizedProcess {
    FlowingFinalizedProcess {
      process_before: EmptyProcess,
      final_step: step,
      phantom_data: Default::default(),
    }
  }
}

impl<
  PROCESS_BEFORE: FlowingProcess,
  LINEAR_CONSUMES: ParamList + Concat<<PROCESS_BEFORE as FlowingProcess>::LastStepConsumes>,
  LINEAR_PRODUCES: ParamList + Concat<<PROCESS_BEFORE as FlowingProcess>::Produces>,
  LINEAR_STEP: Linear<LINEAR_CONSUMES, LINEAR_PRODUCES>,
> LinearFlowingProcess<PROCESS_BEFORE, LINEAR_CONSUMES, LINEAR_PRODUCES, LINEAR_STEP>
{
  fn finnish<FINAL_CONSUMES: ParamList, FINAL_STEP: Final<FINAL_CONSUMES>>(self, step: FINAL_STEP) -> impl FinalizedProcess {
    FlowingFinalizedProcess {
      process_before: self,
      final_step: step,
      phantom_data: Default::default(),
    }
  }
}
}

pub mod finalized_process {
  use crate::builder::finalized_split_process::FinalizedSplitProcess;
  use crate::builder::flowing_process::FlowingProcess;
  use crate::builder::{CurrentInterpretationYields, InterpretationResult, PreviousInterpretationYielded};
  use crate::step::param_list::ParamList;
  use crate::step::step::Final;
  use frunk_core::hlist::HNil;
  use serde_json::Value;
  use std::marker::PhantomData;

  pub trait FinalizedProcess: Sized {
    async fn interpret_resume(
      &self,
      previous_interpretation_produced: Value,
      previous_interpretation_yielded: PreviousInterpretationYielded,
    ) -> InterpretationResult<HNil>; // fixme create result type for finalised process, or undo changes
  }

  pub struct FlowingFinalizedProcess<PROCESS_BEFORE: FlowingProcess, FINAL_CONSUMES: ParamList, FINAL_STEP: Final<FINAL_CONSUMES>> {
    pub process_before: PROCESS_BEFORE,
    pub final_step: FINAL_STEP,
    pub phantom_data: PhantomData<FINAL_CONSUMES>,
  }

  impl<PROCESS_BEFORE: FlowingProcess, FINAL_CONSUMES: ParamList, FINAL_STEP: Final<FINAL_CONSUMES>> FinalizedProcess
  for FlowingFinalizedProcess<PROCESS_BEFORE, FINAL_CONSUMES, FINAL_STEP>
  {
    async fn interpret_resume(
      &self,
      previous_interpretation_produced: Value,
      previous_interpretation_yielded: PreviousInterpretationYielded,
    ) -> InterpretationResult<HNil> {
      todo!()
    }
  }

  pub struct SplitFinalizedProcess<FINALIZED_SPLIT_PROCESS: FinalizedSplitProcess> {
    process: FINALIZED_SPLIT_PROCESS, // maybe box?
  }
  impl<FINALIZED_SPLIT_PROCESS: FinalizedSplitProcess> FinalizedProcess for SplitFinalizedProcess<FINALIZED_SPLIT_PROCESS> {
    async fn interpret_resume(
      &self,
      previous_interpretation_produced: Value,
      previous_interpretation_yielded: PreviousInterpretationYielded,
    ) -> InterpretationResult<HNil> {
      todo!()
    }
  }
}

pub mod finalized_split_process {
  use crate::builder::finalized_process::FinalizedProcess;
  use crate::builder::flowing_process::FlowingProcess;
  use crate::builder::InterpretationOutcome::*;
  use crate::builder::{CurrentInterpretationYields, InterpretationOutcome, InterpretationResult, PreviousInterpretationYielded};
  use crate::step::param_list::ParamList;
  use crate::step::splitter_output_repr::SplitterOutput;
  use crate::step::step::Splitter;
  use frunk_core::coproduct::Coproduct;
  use frunk_core::hlist::HNil;
  use process_builder_common::process_domain::Message;
  use serde_json::Value;
  use std::marker::PhantomData;

  pub trait FinalizedSplitProcess: Sized {
    async fn interpret_resume(
      &self,
      previous_interpretation_produced: Value,
      previous_interpretation_yielded: PreviousInterpretationYielded,
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
  > FinalizedSplitProcess
  for FirstCaseOfFinalizedSplitProcess<PROCESS_BEFORE, SPLITTER_CONSUMES, Coproduct<CASE_THIS, CASE_OTHER>, SPLITTER_STEP, FIRST_CASE>
  {
    // type SplitterOutput = <CASE_THIS as Concat<PROCESS_BEFORE::Produces>>::Concatenated;

    async fn interpret_resume(
      &self,
      previous_interpretation_produced: Value,
      previous_interpretation_yielded: PreviousInterpretationYielded,
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

  pub struct NextCaseOfFinalizedSplitProcess<PROCESS_BEFORE: FinalizedSplitProcess, NEXT_CASE: FinalizedProcess> {
    pub split_process_before: PROCESS_BEFORE,
    pub next_case: NEXT_CASE,
  }

  impl<PROCESS_BEFORE: FinalizedSplitProcess, NEXT_CASE: FinalizedProcess> FinalizedSplitProcess
  for NextCaseOfFinalizedSplitProcess<PROCESS_BEFORE, NEXT_CASE>
  {
    async fn interpret_resume(
      &self,
      previous_interpretation_produced: Value,
      previous_interpretation_yielded: PreviousInterpretationYielded,
    ) -> InterpretationResult<Self::SplitterOutput> {
      todo!()
    }
  }
}

pub mod flowing_split_process {
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
  {}

  pub struct NextCaseFlowingOfFlowingSplitProcess<PROCESS_BEFORE: FlowingSplitProcess, NEXT_CASE: FlowingProcess> {
    pub split_process_before: PROCESS_BEFORE,
    pub next_case: NEXT_CASE,
  }
  impl<PROCESS_BEFORE: FlowingSplitProcess, NEXT_CASE: FlowingProcess> FlowingSplitProcess
  for NextCaseFlowingOfFlowingSplitProcess<PROCESS_BEFORE, NEXT_CASE>
  {}

  pub struct NextCaseFinalizedOfFlowingSplitProcess<PROCESS_BEFORE: FlowingSplitProcess, NEXT_CASE: FinalizedProcess> {
    pub split_process_before: PROCESS_BEFORE,
    pub next_case: NEXT_CASE,
  }
  impl<PROCESS_BEFORE: FlowingSplitProcess, NEXT_CASE: FinalizedProcess> FlowingSplitProcess
  for NextCaseFinalizedOfFlowingSplitProcess<PROCESS_BEFORE, NEXT_CASE>
  {}

  pub struct NextCaseFromFinalizedOfFlowingSplitProcess<PROCESS_BEFORE: FinalizedSplitProcess, NEXT_CASE: FlowingProcess> {
    pub split_process_before: PROCESS_BEFORE,
    pub next_case: NEXT_CASE,
  }
  impl<PROCESS_BEFORE: FinalizedSplitProcess, NEXT_CASE: FlowingProcess> FlowingSplitProcess
  for NextCaseFromFinalizedOfFlowingSplitProcess<PROCESS_BEFORE, NEXT_CASE>
  {}
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
