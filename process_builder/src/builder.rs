use std::marker::PhantomData;
use typenum::*;
use std::ops::*;

use crate::brick::*;
use crate::internal_brick::*;
use crate::internal_process::*;

pub type EMPTY = U0;

pub struct FlowingProcess<
  CONSUMES: TypeLevelSet,
  REQUIRES: TypeLevelSet,
  FORBIDS: TypeLevelSet,
  PRODUCES: TypeLevelSet,
  ACCOMPLISHES: TypeLevelSet,
> {
  pub(crate) process: InternalFlowingProcess,
  pub(crate) consumes: PhantomData<CONSUMES>,
  pub(crate) requires: PhantomData<REQUIRES>,
  pub(crate) forbids: PhantomData<FORBIDS>,
  pub(crate) produces: PhantomData<PRODUCES>,
  pub(crate) accomplishes: PhantomData<ACCOMPLISHES>,
}

impl<
  CONSUMES: TypeLevelSet,
  REQUIRES: TypeLevelSet,
  FORBIDS: TypeLevelSet,
  PRODUCES: TypeLevelSet,
  ACCOMPLISHES: TypeLevelSet,
> FlowingProcess<CONSUMES, REQUIRES, FORBIDS, PRODUCES, ACCOMPLISHES>
{
  pub fn finnish<
    BRICK_CONSUMES: TypeLevelSet + IsEqual<And<BRICK_CONSUMES, PRODUCES>> + BitOr<CONSUMES> + BitAnd<PRODUCES>, // a_includes_b(a & b == b)
    BRICK_REQUIRES: TypeLevelSet + IsEqual<And<BRICK_REQUIRES, ACCOMPLISHES>> + BitAnd<ACCOMPLISHES> + BitAnd<PRODUCES> + BitOr<REQUIRES>, // a_includes_b(a & b == b)
    BRICK_FORBIDS: TypeLevelSet + BitOr<ACCOMPLISHES> + BitAnd<ACCOMPLISHES> + BitOr<FORBIDS>,
    BRICK_ACCOMPLISHES: TypeLevelSet + BitOr<ACCOMPLISHES>,
  >(
    self,
    brick: FinalBrick<BRICK_CONSUMES, BRICK_REQUIRES, BRICK_FORBIDS, BRICK_ACCOMPLISHES>,
  ) -> FinalizedProcess<
    Or<BRICK_CONSUMES, CONSUMES>,
    Or<BRICK_REQUIRES, REQUIRES>,
    Or<BRICK_FORBIDS, FORBIDS>,
    PRODUCES,
    Or<BRICK_ACCOMPLISHES, ACCOMPLISHES>,
  >
  where
  <BRICK_CONSUMES as BitOr<CONSUMES>>::Output: TypeLevelSet,
  <BRICK_REQUIRES as BitOr<REQUIRES>>::Output: TypeLevelSet,
  <BRICK_FORBIDS as BitOr<FORBIDS>>::Output: TypeLevelSet,
  <BRICK_ACCOMPLISHES as BitOr<ACCOMPLISHES>>::Output: TypeLevelSet,
    // BRICK_CONSUMES::Output: NonZero,
    // BRICK_REQUIRES::Output: NonZero,
    // False: IsEqual<And<BRICK_FORBIDS, ACCOMPLISHES>>,
    // False: IsEqual<Or<BRICK_FORBIDS, ACCOMPLISHES>>,
  {
    FinalizedProcess {
      process: InternalFinalizedProcess::Flowing(
        InternalFinalBrick::new(brick),
        self.process,
      ),
      consumes: Default::default(),
      requires: Default::default(),
      forbids: Default::default(),
      produces: Default::default(),
      accomplishes: Default::default(),
    }
  }
}

pub struct SplitterProcess<
  SPLITS_LEFT: TypeLevelSet,
  ROOT_CONSUMES: TypeLevelSet,
  ROOT_REQUIRES: TypeLevelSet,
  ROOT_FORBIDS: TypeLevelSet,
  ROOT_PRODUCES: TypeLevelSet,
  ROOT_ACCOMPLISHES: TypeLevelSet,
  SPLIT_CONSUMES: TypeLevelSet,
  SPLIT_REQUIRES: TypeLevelSet,
  SPLIT_FORBIDS: TypeLevelSet,
  SPLIT_PRODUCES: TypeLevelSet,
  SPLIT_ACCOMPLISHES: TypeLevelSet,
> {
  pub(crate) brick: InternalSplitterBrick,
  pub(crate) process_before: FlowingProcess<
    ROOT_CONSUMES,
    ROOT_REQUIRES,
    ROOT_FORBIDS,
    ROOT_PRODUCES,
    ROOT_ACCOMPLISHES
  >,
  pub(crate) splits_left: PhantomData<SPLITS_LEFT>,
  pub(crate) split_consumes: PhantomData<SPLIT_CONSUMES>,
  pub(crate) split_requires: PhantomData<SPLIT_REQUIRES>,
  pub(crate) split_forbids: PhantomData<SPLIT_FORBIDS>,
  pub(crate) split_produces: PhantomData<SPLIT_PRODUCES>,
  pub(crate) split_accomplishes: PhantomData<SPLIT_ACCOMPLISHES>,
}

pub struct FinalizedProcess<
  CONSUMES: TypeLevelSet,
  REQUIRES: TypeLevelSet,
  FORBIDS: TypeLevelSet,
  PRODUCES: TypeLevelSet,
  ACCOMPLISHES: TypeLevelSet,
> {
  pub(crate) process: InternalFinalizedProcess,
  pub(crate) consumes: PhantomData<CONSUMES>,
  pub(crate) requires: PhantomData<REQUIRES>,
  pub(crate) forbids: PhantomData<FORBIDS>,
  pub(crate) produces: PhantomData<PRODUCES>,
  pub(crate) accomplishes: PhantomData<ACCOMPLISHES>,
}

impl<
  CONSUMES: TypeLevelSet,
  REQUIRES: TypeLevelSet,
  FORBIDS: TypeLevelSet,
  PRODUCES: TypeLevelSet,
  ACCOMPLISHES: TypeLevelSet,
> FinalizedProcess<CONSUMES, REQUIRES, FORBIDS, PRODUCES, ACCOMPLISHES> {
  pub fn close(self, path: &'static str) -> NamedProcess {
    NamedProcess {
      path,
      process: self.process,
    }
  }
}


// impl<
//   CONSUMES: TypeLevelSet,
//   REQUIRES: TypeLevelSet,
//   FORBIDS: TypeLevelSet,
//   PRODUCES: TypeLevelSet,
//   ACCOMPLISHES: TypeLevelSet,
// > FlowingSplitProcess<CONSUMES, REQUIRES, FORBIDS, PRODUCES, ACCOMPLISHES> {
//   pub fn finnish<
//     BRICK_CONSUMES: TypeLevelSet + IsEqual<And<PRODUCES, BRICK_CONSUMES>>, // a_includes_b(a & b == b)
//     BRICK_REQUIRES: TypeLevelSet + IsEqual<And<ACCOMPLISHES, BRICK_REQUIRES>>, // a_includes_b(a & b == b)
//     BRICK_FORBIDS: TypeLevelSet,
//     BRICK_ACCOMPLISHES: TypeLevelSet,
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

// impl<
//   CONSUMES: TypeLevelSet,
//   REQUIRES: TypeLevelSet,
//   FORBIDS: TypeLevelSet,
//   PRODUCES: TypeLevelSet,
//   ACCOMPLISHES: TypeLevelSet,
// > FlowingProcess<CONSUMES, REQUIRES, FORBIDS, PRODUCES, ACCOMPLISHES>
// {
//   pub fn finnish<
//     BRICK_CONSUMES: TypeLevelSet + IsEqual<And<PRODUCES, BRICK_CONSUMES>>, // a_includes_b(a & b == b)
//     BRICK_REQUIRES: TypeLevelSet + IsEqual<And<ACCOMPLISHES, BRICK_REQUIRES>>, // a_includes_b(a & b == b)
//     BRICK_FORBIDS: TypeLevelSet,
//     BRICK_ACCOMPLISHES: TypeLevelSet,
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
//     BRICK_CONSUMES: TypeLevelSet + IsEqual<And<PRODUCES, BRICK_CONSUMES>>, // a_includes_b(a & b == b)
//     BRICK_REQUIRES: TypeLevelSet + IsEqual<And<ACCOMPLISHES, BRICK_REQUIRES>>, // a_includes_b(a & b == b)
//     BRICK_FORBIDS: TypeLevelSet,
//     BRICK_PRODUCES: TypeLevelSet,
//     BRICK_ACCOMPLISHES: TypeLevelSet,
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
//     SPLITS: TypeLevelSet,
//     BRICK_CONSUMES: TypeLevelSet + IsEqual<And<PRODUCES, BRICK_CONSUMES>>, // a_includes_b(a & b == b)
//     BRICK_REQUIRES: TypeLevelSet + IsEqual<And<ACCOMPLISHES, BRICK_REQUIRES>>, // a_includes_b(a & b == b)
//     BRICK_FORBIDS: TypeLevelSet,
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
  pub(crate) process: InternalFinalizedProcess,
}
