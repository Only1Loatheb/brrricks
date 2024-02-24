use std::marker::PhantomData;
use typenum::*;
use private::*;
use std::ops::*;

use process::internal_process::*;
use crate::brick;
use crate::brick::*;

pub type EMPTY = U0;

pub struct FlowingProcess<
  CONSUMES: ParamBitSet,
  REQUIRES: Unsigned,
  FORBIDS: Unsigned,
  PRODUCES: ParamBitSet,
  ACCOMPLISHES: Unsigned,
> {
  pub(crate) process: InternalFlowingProcess,
  pub(crate) consumes: PhantomData<CONSUMES>,
  pub(crate) requires: PhantomData<REQUIRES>,
  pub(crate) forbids: PhantomData<FORBIDS>,
  pub(crate) produces: PhantomData<PRODUCES>,
  pub(crate) accomplishes: PhantomData<ACCOMPLISHES>,
}

impl<
  CONSUMES: ParamBitSet,
  REQUIRES: Unsigned,
  FORBIDS: Unsigned,
  PRODUCES: ParamBitSet,
  ACCOMPLISHES: Unsigned,
> FlowingProcess<CONSUMES, REQUIRES, FORBIDS, PRODUCES, ACCOMPLISHES> {
  pub fn finnish<
    BRICK_CONSUMES: ParamBitSet + BitOr<CONSUMES> + BitAnd<PRODUCES> + Cmp<<BRICK_CONSUMES as BitAnd<PRODUCES>>::Output> + IsEqualPrivate<<BRICK_CONSUMES as BitAnd<PRODUCES>>::Output, <BRICK_CONSUMES as Cmp<<BRICK_CONSUMES as BitAnd<PRODUCES>>::Output>>::Output>,
    BRICK_REQUIRES: Unsigned + BitAnd<ACCOMPLISHES> + BitAnd<PRODUCES> + BitOr<REQUIRES> + Cmp<<BRICK_REQUIRES as BitAnd<ACCOMPLISHES>>::Output> + IsEqualPrivate<<BRICK_REQUIRES as BitAnd<ACCOMPLISHES>>::Output, <BRICK_REQUIRES as Cmp<<BRICK_REQUIRES as BitAnd<ACCOMPLISHES>>::Output>>::Output>,
    BRICK_FORBIDS: Unsigned + BitOr<ACCOMPLISHES> + BitAnd<ACCOMPLISHES> + BitOr<FORBIDS>,
    BRICK_ACCOMPLISHES: Unsigned + BitOr<ACCOMPLISHES> + BitAnd<ACCOMPLISHES>,
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
    <BRICK_CONSUMES as BitOr<CONSUMES>>::Output: ParamBitSet, // result
    <BRICK_REQUIRES as BitOr<REQUIRES>>::Output: Unsigned, // result
    <BRICK_FORBIDS as BitOr<FORBIDS>>::Output: Unsigned, // result
    <BRICK_ACCOMPLISHES as BitOr<ACCOMPLISHES>>::Output: Unsigned, // result
    Eq<BRICK_CONSUMES, And<BRICK_CONSUMES, PRODUCES>>: NonZero, // PRODUCES contain BRICK_CONSUMES
    Eq<BRICK_REQUIRES, And<BRICK_REQUIRES, ACCOMPLISHES>>: NonZero, // ACCOMPLISHES contain BRICK_REQUIRES
    And<BRICK_FORBIDS, ACCOMPLISHES>: Zero, // BRICK_FORBIDS are not in ACCOMPLISHES
    And<BRICK_ACCOMPLISHES, ACCOMPLISHES>: Zero, // doesn't accomplish what was already accomplished
  {
    FinalizedProcess {
      process: InternalFinalizedProcess::Flowing(brick.to_internal(), self.process),
      consumes: Default::default(),
      requires: Default::default(),
      forbids: Default::default(),
      produces: Default::default(),
      accomplishes: Default::default(),
    }
  }

  pub fn split<
    BRICK_SPLITS: Unsigned + Sub<U1> + Cmp<U1> + IsGreaterPrivate<U1, <BRICK_SPLITS as Cmp<U1>>::Output>,
    BRICK_CONSUMES: ParamBitSet + BitOr<CONSUMES>,
    BRICK_REQUIRES: Unsigned + BitAnd<<<BRICK_ACCOMPLISHES as CaseActionSetArray>::HEAD as BitOr<ACCOMPLISHES>>::Output> + Cmp<<BRICK_REQUIRES as BitAnd<<<BRICK_ACCOMPLISHES as CaseActionSetArray>::HEAD as BitOr<ACCOMPLISHES>>::Output>>::Output> + IsEqualPrivate<<BRICK_REQUIRES as BitAnd<<<BRICK_ACCOMPLISHES as CaseActionSetArray>::HEAD as BitOr<ACCOMPLISHES>>::Output>>::Output, <BRICK_REQUIRES as Cmp<<BRICK_REQUIRES as BitAnd<<<BRICK_ACCOMPLISHES as CaseActionSetArray>::HEAD as BitOr<ACCOMPLISHES>>::Output>>::Output>>::Output> + BitOr<REQUIRES>,
    BRICK_FORBIDS: Unsigned + BitAnd<<<BRICK_ACCOMPLISHES as CaseActionSetArray>::HEAD as BitOr<ACCOMPLISHES>>::Output> + BitOr<FORBIDS>,
    BRICK_PRODUCES: CaseParamSetArray,
    BRICK_ACCOMPLISHES: CaseActionSetArray,
    FIRST_CONSUMES: ParamBitSet + BitAnd<<<BRICK_PRODUCES as CaseParamSetArray>::HEAD as BitOr<PRODUCES>>::Output> + Cmp<<FIRST_CONSUMES as BitAnd<<<BRICK_PRODUCES as CaseParamSetArray>::HEAD as BitOr<PRODUCES>>::Output>>::Output> + Cmp<<FIRST_CONSUMES as BitAnd<<<BRICK_PRODUCES as CaseParamSetArray>::HEAD as BitOr<PRODUCES>>::Output>>::Output> + IsEqualPrivate<<FIRST_CONSUMES as BitAnd<<<BRICK_PRODUCES as brick::CaseParamSetArray>::HEAD as BitOr<PRODUCES>>::Output>>::Output, <FIRST_CONSUMES as Cmp<<FIRST_CONSUMES as BitAnd<<<BRICK_PRODUCES as brick::CaseParamSetArray>::HEAD as BitOr<PRODUCES>>::Output>>::Output>>::Output>,
    FIRST_REQUIRES: Unsigned,
    FIRST_FORBIDS: Unsigned,
    FIRST_PRODUCES: ParamBitSet,
    FIRST_ACCOMPLISHES: Unsigned,
  >(
    self,
    brick: SplitterBrick<BRICK_SPLITS, BRICK_CONSUMES, BRICK_REQUIRES, BRICK_FORBIDS, BRICK_PRODUCES, BRICK_ACCOMPLISHES>,
    first_case: FlowingProcess<FIRST_CONSUMES, FIRST_REQUIRES, FIRST_FORBIDS, FIRST_PRODUCES, FIRST_ACCOMPLISHES>,
  ) -> FlowingSplitterProcess<
      <BRICK_SPLITS as Sub<U1>>::Output,
      Or<BRICK_CONSUMES, CONSUMES>,
      Or<BRICK_REQUIRES, REQUIRES>,
      Or<BRICK_FORBIDS, FORBIDS>,
      Or<BRICK_PRODUCES::INTERSECTION, PRODUCES>,
      Or<BRICK_ACCOMPLISHES::INTERSECTION, ACCOMPLISHES>,
      FIRST_CONSUMES,
      FIRST_REQUIRES,
      FIRST_FORBIDS,
      FIRST_PRODUCES,
      FIRST_ACCOMPLISHES,
  >
  where
    <BRICK_SPLITS as Sub<U1>>::Output: Unsigned,
    <BRICK_CONSUMES as BitOr<CONSUMES>>::Output: ParamBitSet,
    <BRICK_REQUIRES as BitOr<REQUIRES>>::Output: Unsigned,
    <BRICK_REQUIRES as IsEqualPrivate<<BRICK_REQUIRES as BitAnd<<<BRICK_ACCOMPLISHES as CaseActionSetArray>::HEAD as BitOr<ACCOMPLISHES>>::Output>>::Output, <BRICK_REQUIRES as Cmp<<BRICK_REQUIRES as BitAnd<<<BRICK_ACCOMPLISHES as CaseActionSetArray>::HEAD as BitOr<ACCOMPLISHES>>::Output>>::Output>>::Output>>::Output: NonZero,
    <BRICK_FORBIDS as BitOr<FORBIDS>>::Output: Unsigned,
    <BRICK_PRODUCES as CaseParamSetArray>::UNION: BitAnd<PRODUCES>,
    <BRICK_PRODUCES as CaseParamSetArray>::HEAD: BitOr<PRODUCES>,
    <BRICK_PRODUCES as CaseParamSetArray>::INTERSECTION: BitOr<PRODUCES>,
    <<BRICK_PRODUCES as CaseParamSetArray>::INTERSECTION as BitOr<PRODUCES>>::Output: ParamBitSet,
    <BRICK_ACCOMPLISHES as CaseActionSetArray>::UNION: BitAnd<ACCOMPLISHES>,
    <BRICK_ACCOMPLISHES as CaseActionSetArray>::HEAD: BitOr<ACCOMPLISHES>,
    <BRICK_ACCOMPLISHES as CaseActionSetArray>::INTERSECTION: BitOr<ACCOMPLISHES>,
    <<BRICK_ACCOMPLISHES as CaseActionSetArray>::INTERSECTION as BitOr<ACCOMPLISHES>>::Output: Unsigned,
    Gr<BRICK_SPLITS, U1>: NonZero,
    Eq<FIRST_CONSUMES, And<FIRST_CONSUMES, Or<BRICK_PRODUCES::HEAD, PRODUCES>>>: NonZero, // (PRODUCES union BRICK_PRODUCES) contain FIRST_CONSUMES
    Eq<BRICK_REQUIRES, And<BRICK_REQUIRES, Or<BRICK_ACCOMPLISHES::HEAD, ACCOMPLISHES>>>: NonZero, // ACCOMPLISHES contain BRICK_REQUIRES
    And<BRICK_FORBIDS, Or<BRICK_ACCOMPLISHES::HEAD, ACCOMPLISHES>>: Zero, // BRICK_FORBIDS are not in ACCOMPLISHES
    And<BRICK_PRODUCES::UNION, PRODUCES>: Zero, // splitter doesn't produce what was already produced
    And<BRICK_ACCOMPLISHES::UNION, ACCOMPLISHES>: Zero, // splitter doesn't accomplish what was already accomplished
  {
    FlowingSplitterProcess {
      process: InternalFlowingSplitProcess::FirstCase {
        brick: brick.to_internal(),
        first_case: first_case.process,
        process_before: self.process,
      },
      splits_left: Default::default(),
      root_consumes: Default::default(),
      root_requires: Default::default(),
      root_forbids: Default::default(),
      root_produces: Default::default(),
      root_accomplishes: Default::default(),
      split_consumes: Default::default(),
      split_requires: Default::default(),
      split_forbids: Default::default(),
      split_produces: Default::default(),
      split_accomplishes: Default::default(),
    }
  }
}

pub struct FinalizedSplitterProcess<
  SPLITS_LEFT: Unsigned,
  ROOT_CONSUMES: ParamBitSet,
  ROOT_REQUIRES: Unsigned,
  ROOT_FORBIDS: Unsigned,
  ROOT_PRODUCES: ParamBitSet,
  ROOT_ACCOMPLISHES: Unsigned,
  ACCUM_CONSUMES: ParamBitSet,
  ACCUM_REQUIRES: Unsigned,
  ACCUM_FORBIDS: Unsigned,
  ACCUM_PRODUCES: ParamBitSet,
  ACCUM_ACCOMPLISHES: Unsigned,
> {
  pub(crate) process: InternalFinalizedSplitProcess,
  pub(crate) splits_left: PhantomData<SPLITS_LEFT>,
  pub(crate) root_consumes: PhantomData<ROOT_CONSUMES>,
  pub(crate) root_requires: PhantomData<ROOT_REQUIRES>,
  pub(crate) root_forbids: PhantomData<ROOT_FORBIDS>,
  pub(crate) root_produces: PhantomData<ROOT_PRODUCES>,
  pub(crate) root_accomplishes: PhantomData<ROOT_ACCOMPLISHES>,
  pub(crate) split_consumes: PhantomData<ACCUM_CONSUMES>,
  pub(crate) split_requires: PhantomData<ACCUM_REQUIRES>,
  pub(crate) split_forbids: PhantomData<ACCUM_FORBIDS>,
  pub(crate) split_produces: PhantomData<ACCUM_PRODUCES>,
  pub(crate) split_accomplishes: PhantomData<ACCUM_ACCOMPLISHES>,
}

pub struct FlowingSplitterProcess<
  SPLITS_LEFT: Unsigned,
  ROOT_CONSUMES: ParamBitSet,
  ROOT_REQUIRES: Unsigned,
  ROOT_FORBIDS: Unsigned,
  ROOT_PRODUCES: ParamBitSet,
  ROOT_ACCOMPLISHES: Unsigned,
  ACCUM_CONSUMES: ParamBitSet,
  ACCUM_REQUIRES: Unsigned,
  ACCUM_FORBIDS: Unsigned,
  ACCUM_PRODUCES: ParamBitSet,
  ACCUM_ACCOMPLISHES: Unsigned,
> {
  pub(crate) process: InternalFlowingSplitProcess,
  pub(crate) splits_left: PhantomData<SPLITS_LEFT>,
  pub(crate) root_consumes: PhantomData<ROOT_CONSUMES>,
  pub(crate) root_requires: PhantomData<ROOT_REQUIRES>,
  pub(crate) root_forbids: PhantomData<ROOT_FORBIDS>,
  pub(crate) root_produces: PhantomData<ROOT_PRODUCES>,
  pub(crate) root_accomplishes: PhantomData<ROOT_ACCOMPLISHES>,
  pub(crate) split_consumes: PhantomData<ACCUM_CONSUMES>,
  pub(crate) split_requires: PhantomData<ACCUM_REQUIRES>,
  pub(crate) split_forbids: PhantomData<ACCUM_FORBIDS>,
  pub(crate) split_produces: PhantomData<ACCUM_PRODUCES>,
  pub(crate) split_accomplishes: PhantomData<ACCUM_ACCOMPLISHES>,
}

pub struct FinalizedProcess<
  CONSUMES: ParamBitSet,
  REQUIRES: Unsigned,
  FORBIDS: Unsigned,
  PRODUCES: ParamBitSet,
  ACCOMPLISHES: Unsigned,
> {
  pub(crate) process: InternalFinalizedProcess,
  pub(crate) consumes: PhantomData<CONSUMES>,
  pub(crate) requires: PhantomData<REQUIRES>,
  pub(crate) forbids: PhantomData<FORBIDS>,
  pub(crate) produces: PhantomData<PRODUCES>,
  pub(crate) accomplishes: PhantomData<ACCOMPLISHES>,
}

impl<
  CONSUMES: ParamBitSet,
  REQUIRES: Unsigned,
  FORBIDS: Unsigned,
  PRODUCES: ParamBitSet,
  ACCOMPLISHES: Unsigned,
> FinalizedProcess<CONSUMES, REQUIRES, FORBIDS, PRODUCES, ACCOMPLISHES> {
  pub fn close(self, path: String) -> NamedProcess {
    NamedProcess {
      path,
      process: self.process,
    }
  }
}


// impl<
//   CONSUMES: ParamBitSet,
//   REQUIRES: Unsigned,
//   FORBIDS: Unsigned,
//   PRODUCES: ParamBitSet,
//   ACCOMPLISHES: Unsigned,
// > FlowingSplitProcess<CONSUMES, REQUIRES, FORBIDS, PRODUCES, ACCOMPLISHES> {
//   pub fn finnish<
//     BRICK_CONSUMES: ParamBitSet + IsEqual<And<PRODUCES, BRICK_CONSUMES>>, // a_includes_b(a & b == b)
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

// impl<
//   CONSUMES: ParamBitSet,
//   REQUIRES: Unsigned,
//   FORBIDS: Unsigned,
//   PRODUCES: ParamBitSet,
//   ACCOMPLISHES: Unsigned,
// > FlowingProcess<CONSUMES, REQUIRES, FORBIDS, PRODUCES, ACCOMPLISHES>
// {
//   pub fn finnish<
//     BRICK_CONSUMES: ParamBitSet + IsEqual<And<PRODUCES, BRICK_CONSUMES>>, // a_includes_b(a & b == b)
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
//     BRICK_CONSUMES: ParamBitSet + IsEqual<And<PRODUCES, BRICK_CONSUMES>>, // a_includes_b(a & b == b)
//     BRICK_REQUIRES: Unsigned + IsEqual<And<ACCOMPLISHES, BRICK_REQUIRES>>, // a_includes_b(a & b == b)
//     BRICK_FORBIDS: Unsigned,
//     BRICK_PRODUCES: ParamBitSet,
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
//     SPLITS: ParamBitSet,
//     BRICK_CONSUMES: ParamBitSet + IsEqual<And<PRODUCES, BRICK_CONSUMES>>, // a_includes_b(a & b == b)
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
