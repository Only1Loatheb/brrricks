use std::marker::PhantomData;
use typenum::*;

use crate::brick::*;
use crate::internal_brick::*;

// accept different types in builder (with additional type params) and do the checking, and build with non-generic types

pub type ALL = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>;

pub type EMPTY = U0;

const _: () = assert!(ALL::U128 == u128::MAX);


enum FlowingLinearProcess {
  NoOp,
  Flowing {
    brick: InternalLinearBrick,
    process_before_brick: Box<FlowingLinearProcess>,
  },
}

struct FlowingProcess<
  CONSUMES: Unsigned,
  REQUIRES: Unsigned,
  FORBIDS: Unsigned,
  PRODUCES: Unsigned,
  ACCOMPLISHES: Unsigned,
> {
  process: FlowingLinearProcess,
  consumes: PhantomData<CONSUMES>,
  requires: PhantomData<REQUIRES>,
  forbids: PhantomData<FORBIDS>,
  produces: PhantomData<PRODUCES>,
  accomplishes: PhantomData<ACCOMPLISHES>,
}

pub fn empty_process() -> FlowingProcess<EMPTY, EMPTY, EMPTY, EMPTY, EMPTY> {
   FlowingProcess {
     process: FlowingLinearProcess::NoOp,
     consumes: Default::default(),
     requires: Default::default(),
     forbids: Default::default(),
     produces: Default::default(),
     accomplishes: Default::default(),
   }
}

pub fn process<
  BRICK_FORBIDS: Unsigned,
  BRICK_PRODUCES: Unsigned,
  BRICK_ACCOMPLISHES: Unsigned,
>(
  brick: LinearBrick<EMPTY, EMPTY, BRICK_FORBIDS, BRICK_PRODUCES, BRICK_ACCOMPLISHES>,
) -> FlowingProcess<EMPTY, EMPTY, BRICK_FORBIDS, BRICK_PRODUCES, BRICK_ACCOMPLISHES> {
  FlowingProcess {
    process: FlowingLinearProcess::Flowing {
      brick: InternalLinearBrick::new(brick),
      process_before_brick: Box::new(FlowingLinearProcess::NoOp),
    },
    consumes: Default::default(),
    requires: Default::default(),
    forbids: Default::default(),
    produces: Default::default(),
    accomplishes: Default::default(),
  }
}

pub fn finnish<
  BRICK_FORBIDS: Unsigned,
  BRICK_ACCOMPLISHES: Unsigned
>(
  brick: FinalBrick<EMPTY, EMPTY, BRICK_FORBIDS, BRICK_ACCOMPLISHES>,
) -> FinalizedProcess {
  FinalizedProcess::Linear {
    0: FinalizedLinearProcess {
      brick: InternalFinalBrick::new(brick),
      process_before_brick: FlowingLinearProcess::NoOp,
    },
  }
}

// use
// impl<
//   CONSUMES: Unsigned,
//   REQUIRES: Unsigned,
//   FORBIDS: Unsigned,
//   PRODUCES: Unsigned,
//   ACCOMPLISHES: Unsigned,
// > FlowingLinearProcess<CONSUMES, REQUIRES, FORBIDS, PRODUCES, ACCOMPLISHES>
// {
//   pub fn finnish<
//     BRICK_CONSUMES: Unsigned + IsEqual<And<PRODUCES, BRICK_CONSUMES>>, // a_includes_b(a & b == b)
//     BRICK_REQUIRES: Unsigned + IsEqual<And<ACCOMPLISHES, BRICK_REQUIRES>>, // a_includes_b(a & b == b)
//     BRICK_FORBIDS: Unsigned,
//     BRICK_ACCOMPLISHES: Unsigned,
//   >(
//     self,
//     brick: FinalBrick<BRICK_CONSUMES, BRICK_REQUIRES, BRICK_FORBIDS, BRICK_ACCOMPLISHES>,
//   ) -> FinalizedProcess<
//     Or<FORBIDS, BRICK_FORBIDS>,
//     PRODUCES,
//     Or<ACCOMPLISHES, BRICK_ACCOMPLISHES>,
//   >
//   where
//     BRICK_CONSUMES::Output: NonZero,
//     BRICK_REQUIRES::Output: NonZero,
//     And<BRICK_FORBIDS, ACCOMPLISHES>: Zero,
//     Or<ACCOMPLISHES, BRICK_FORBIDS>: Zero,
//   {
//     FinalizedProcess::Linear {
//       0: FinalizedLinearProcess {
//         brick: InternalFinalBrick::new(brick),
//         process_before_brick: FlowingProcess::Linear(self),
//       },
//     }
//   }
// }

pub struct FlowingSplitProcess<
  SPLITS_LEFT: Unsigned,
  ROOT_CONSUMES: Unsigned,
  ROOT_REQUIRES: Unsigned,
  ROOT_FORBIDS: Unsigned,
  ROOT_PRODUCES: Unsigned,
  ROOT_ACCOMPLISHES: Unsigned,
  SPLIT_CONSUMES: Unsigned,
  SPLIT_REQUIRES: Unsigned,
  SPLIT_FORBIDS: Unsigned,
  SPLIT_PRODUCES: Unsigned,
  SPLIT_ACCOMPLISHES: Unsigned,
> {
  pub(crate) splits_left: PhantomData<SPLITS_LEFT>,
  pub(crate) split_consumes: PhantomData<SPLIT_CONSUMES>,
  pub(crate) split_requires: PhantomData<SPLIT_REQUIRES>,
  pub(crate) split_forbids: PhantomData<SPLIT_FORBIDS>,
  pub(crate) split_produces: PhantomData<SPLIT_PRODUCES>,
  pub(crate) split_accomplishes: PhantomData<SPLIT_ACCOMPLISHES>,
  pub(crate) brick: InternalSplitterBrick,
  pub(crate) process_before_brick: FlowingProcess<
    ROOT_CONSUMES,
    ROOT_REQUIRES,
    ROOT_FORBIDS,
    ROOT_PRODUCES,
    ROOT_ACCOMPLISHES
  >,
}

// impl<
//   CONSUMES: Unsigned,
//   REQUIRES: Unsigned,
//   FORBIDS: Unsigned,
//   PRODUCES: Unsigned,
//   ACCOMPLISHES: Unsigned,
// > FlowingSplitProcess<CONSUMES, REQUIRES, FORBIDS, PRODUCES, ACCOMPLISHES> {
//   pub fn finnish<
//     BRICK_CONSUMES: Unsigned + IsEqual<And<PRODUCES, BRICK_CONSUMES>>, // a_includes_b(a & b == b)
//     BRICK_REQUIRES: Unsigned + IsEqual<And<ACCOMPLISHES, BRICK_REQUIRES>>, // a_includes_b(a & b == b)
//     BRICK_FORBIDS: Unsigned,
//     BRICK_ACCOMPLISHES: Unsigned,
//   >(self, brick: FinalBrick<BRICK_CONSUMES, BRICK_REQUIRES, BRICK_FORBIDS, BRICK_ACCOMPLISHES>) -> FinalizedProcess<
//     Or<FORBIDS, BRICK_FORBIDS>::Output,
//     PRODUCES,
//     Or<ACCOMPLISHES, BRICK_ACCOMPLISHES>::Output,
//   > where
//     BRICK_CONSUMES::Output: NonZero,
//     BRICK_REQUIRES::Output: NonZero,
//     And<BRICK_FORBIDS, ACCOMPLISHES>: Zero,
//     Or<ACCOMPLISHES, BRICK_FORBIDS>: Zero,
//   {
//     FinalizedProcess::Linear {
//       0: FinalizedLinearProcess {
//         brick: InternalFinalBrick::new(brick),
//         process_before_brick: FlowingProcess::Split(self),
//       },
//     }
//   }
// }

pub struct FinalizedLinearProcess {
  pub(crate) brick: InternalFinalBrick,
  pub(crate) process_before_brick: FlowingLinearProcess,
}

pub struct FinalizedSplitProcess {
  pub(crate) brick: Box<dyn SplitterBrickHandler>,
  // pub(crate) cases: Vec<FinalizedProcess>,
  pub(crate) process_before_brick:                                 u8,
  // FlowingProcess<EMPTY, EMPTY, FORBIDS, PRODUCES, ACCOMPLISHES>,
}

pub enum FinalizedProcess {
  Linear(FinalizedLinearProcess),
  Split(FinalizedSplitProcess),
}

// impl<
//   CONSUMES: Unsigned,
//   REQUIRES: Unsigned,
//   FORBIDS: Unsigned,
//   PRODUCES: Unsigned,
//   ACCOMPLISHES: Unsigned,
// > FlowingProcess<CONSUMES, REQUIRES, FORBIDS, PRODUCES, ACCOMPLISHES>
// {
//   pub fn finnish<
//     BRICK_CONSUMES: Unsigned + IsEqual<And<PRODUCES, BRICK_CONSUMES>>, // a_includes_b(a & b == b)
//     BRICK_REQUIRES: Unsigned + IsEqual<And<ACCOMPLISHES, BRICK_REQUIRES>>, // a_includes_b(a & b == b)
//     BRICK_FORBIDS: Unsigned,
//     BRICK_ACCOMPLISHES: Unsigned,
//   >(
//     self,
//     brick: FinalBrick<BRICK_CONSUMES, BRICK_REQUIRES, BRICK_FORBIDS, BRICK_ACCOMPLISHES>,
//   ) -> FinalizedProcess
//   where
//     BRICK_CONSUMES::Output: NonZero,
//     BRICK_REQUIRES::Output: NonZero,
//     And<BRICK_FORBIDS, ACCOMPLISHES>: Zero,
//   {
//     match self {
//       FlowingProcess::NoOp => finnish(brick),
//       FlowingProcess::Linear(process, _, _, _, _, _) => process.finnish(brick),
//     }
//   }
//
//   pub fn and_then<
//     BRICK_CONSUMES: Unsigned + IsEqual<And<PRODUCES, BRICK_CONSUMES>>, // a_includes_b(a & b == b)
//     BRICK_REQUIRES: Unsigned + IsEqual<And<ACCOMPLISHES, BRICK_REQUIRES>>, // a_includes_b(a & b == b)
//     BRICK_FORBIDS: Unsigned,
//     BRICK_PRODUCES: Unsigned,
//     BRICK_ACCOMPLISHES: Unsigned,
//   >(
//     self,
//     brick: LinearBrick<
//       BRICK_CONSUMES,
//       BRICK_REQUIRES,
//       BRICK_FORBIDS,
//       BRICK_PRODUCES,
//       BRICK_ACCOMPLISHES,
//     >,
//   ) -> FlowingProcess<
//     Or<CONSUMES, BRICK_CONSUMES>,
//     Or<REQUIRES, BRICK_REQUIRES>,
//     Or<FORBIDS, BRICK_FORBIDS>,
//     Or<PRODUCES, BRICK_PRODUCES>,
//     Or<ACCOMPLISHES, BRICK_ACCOMPLISHES>,
//   >
//   where
//     BRICK_CONSUMES::Output: NonZero,
//     BRICK_REQUIRES::Output: NonZero,
//     And<BRICK_FORBIDS, ACCOMPLISHES>: Zero,
//     Or<ACCOMPLISHES, BRICK_FORBIDS>: Zero,
//   {
//     FlowingProcess {
//       process: FlowingLinearProcess::Flowing {
//         brick: InternalLinearBrick::new(brick),
//         process_before_brick: Box::new(self.process),
//       },
//       consumes: Default::default(),
//       requires: Default::default(),
//       forbids: Default::default(),
//       produces: Default::default(),
//       accomplishes: Default::default(),
//     }
//   }
//
//   pub fn split<
//     SPLITS: Unsigned,
//     BRICK_CONSUMES: Unsigned + IsEqual<And<PRODUCES, BRICK_CONSUMES>>, // a_includes_b(a & b == b)
//     BRICK_REQUIRES: Unsigned + IsEqual<And<ACCOMPLISHES, BRICK_REQUIRES>>, // a_includes_b(a & b == b)
//     BRICK_FORBIDS: Unsigned,
//   >(
//     self,
//     brick: SplitterBrick<SPLITS, BRICK_CONSUMES, BRICK_REQUIRES, BRICK_FORBIDS>,
//   ) -> FlowingSplitProcess<
//     SPLITS,
//     Or<CONSUMES, BRICK_CONSUMES>,
//     Or<REQUIRES, BRICK_REQUIRES>,
//     Or<FORBIDS, BRICK_FORBIDS>,
//     PRODUCES,
//     ACCOMPLISHES,
//     EMPTY,
//     EMPTY,
//     EMPTY,
//     EMPTY,
//     EMPTY,
//   > {
//     FlowingSplitProcess {
//       splits_left: PhantomData::default(),
//       split_consumes: Default::default(),
//       split_requires: Default::default(),
//       split_forbids: Default::default(),
//       split_produces: Default::default(),
//       split_accomplishes: Default::default(),
//       brick: InternalSplitterBrick::new(brick),
//       process_before_brick: self,
//     }
//   }
// }

pub struct NamedProcess {
  pub(crate) path: &'static str,
  pub(crate) process: FinalizedProcess,
}

impl FinalizedProcess {
  pub fn close(self, path: &'static str) -> NamedProcess {
    NamedProcess {
      path,
      process: self,
    }
  }
}
