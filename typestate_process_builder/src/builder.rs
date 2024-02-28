use std::marker::PhantomData;
use typenum::*;
use typenum::private::*;
use std::ops::*;

use process::internal_process::*;
use crate::brick;
use crate::brick::*;
use crate::builder_helpers::process;

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
  // outputs
    <BRICK_CONSUMES as BitOr<CONSUMES>>::Output: ParamBitSet,
    <BRICK_REQUIRES as BitOr<REQUIRES>>::Output: Unsigned,
    <BRICK_FORBIDS as BitOr<FORBIDS>>::Output: Unsigned,
    <BRICK_ACCOMPLISHES as BitOr<ACCOMPLISHES>>::Output: Unsigned,
  // constraint
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
    PARAM_HEAD: ParamBitSet + BitOr<<PRODUCES_AND_ACCOMPLISHES_TAIL as CaseArray>::PARAM_UNION> + BitOr<PRODUCES>,
    ACTION_HEAD: Unsigned + BitOr<<PRODUCES_AND_ACCOMPLISHES_TAIL as CaseArray>::ACTION_UNION> + BitOr<ACCOMPLISHES>,
    PRODUCES_AND_ACCOMPLISHES_TAIL: CaseArray + Len,
    BRICK_CONSUMES: ParamBitSet + BitOr<CONSUMES>,
    BRICK_REQUIRES: Unsigned + BitAnd<<ACTION_HEAD as BitOr<ACCOMPLISHES>>::Output> + Cmp<<BRICK_REQUIRES as BitAnd<<ACTION_HEAD as BitOr<ACCOMPLISHES>>::Output>>::Output> + IsEqualPrivate<<BRICK_REQUIRES as BitAnd<<ACTION_HEAD as BitOr<ACCOMPLISHES>>::Output>>::Output, <BRICK_REQUIRES as Cmp<<BRICK_REQUIRES as BitAnd<<ACTION_HEAD as BitOr<ACCOMPLISHES>>::Output>>::Output>>::Output> + BitOr<REQUIRES>,
    BRICK_FORBIDS: Unsigned + BitAnd<<ACTION_HEAD as BitOr<ACCOMPLISHES>>::Output> + BitOr<FORBIDS>,
    FIRST_CONSUMES: ParamBitSet + BitAnd<<PARAM_HEAD as BitOr<PRODUCES>>::Output> + Cmp<<FIRST_CONSUMES as BitAnd<<PARAM_HEAD as BitOr<PRODUCES>>::Output>>::Output> + Cmp<<FIRST_CONSUMES as BitAnd<<PARAM_HEAD as BitOr<PRODUCES>>::Output>>::Output> + IsEqualPrivate<<FIRST_CONSUMES as BitAnd<<PARAM_HEAD as BitOr<PRODUCES>>::Output>>::Output, <FIRST_CONSUMES as Cmp<<FIRST_CONSUMES as BitAnd<<PARAM_HEAD as BitOr<PRODUCES>>::Output>>::Output>>::Output>,
    FIRST_REQUIRES: Unsigned,
    FIRST_FORBIDS: Unsigned,
    FIRST_PRODUCES: ParamBitSet,
    FIRST_ACCOMPLISHES: Unsigned,
  >(
    self,
    brick: SplitterBrick<BRICK_CONSUMES, BRICK_REQUIRES, BRICK_FORBIDS, TArr<(PARAM_HEAD, ACTION_HEAD), PRODUCES_AND_ACCOMPLISHES_TAIL>>,
    first_case: FlowingProcess<FIRST_CONSUMES, FIRST_REQUIRES, FIRST_FORBIDS, FIRST_PRODUCES, FIRST_ACCOMPLISHES>,
  ) -> FlowingSplitterProcess<
    PRODUCES_AND_ACCOMPLISHES_TAIL,
    Or<BRICK_CONSUMES, CONSUMES>,
    Or<BRICK_REQUIRES, REQUIRES>,
    Or<BRICK_FORBIDS, FORBIDS>,
    PRODUCES, // And<BRICK_PRODUCES::INTERSECTION, PRODUCES>, todo
    ACCOMPLISHES, // And<BRICK_ACCOMPLISHES::INTERSECTION, ACCOMPLISHES>, up
    FIRST_CONSUMES,
    FIRST_REQUIRES,
    FIRST_FORBIDS,
    FIRST_PRODUCES,
    FIRST_ACCOMPLISHES,
  >
  where
  // outputs
    <BRICK_CONSUMES as BitOr<CONSUMES>>::Output: ParamBitSet,
    <BRICK_REQUIRES as BitOr<REQUIRES>>::Output: Unsigned,
    <BRICK_FORBIDS as BitOr<FORBIDS>>::Output: Unsigned,
  // compiler generated types
    <ACTION_HEAD as std::ops::BitOr<<PRODUCES_AND_ACCOMPLISHES_TAIL as brick::CaseArray>::ACTION_UNION>>::Output: std::ops::BitAnd<ACCOMPLISHES>,
    <PARAM_HEAD as std::ops::BitOr<<PRODUCES_AND_ACCOMPLISHES_TAIL as brick::CaseArray>::PARAM_UNION>>::Output: std::ops::BitAnd<PRODUCES>,
    <PRODUCES_AND_ACCOMPLISHES_TAIL as typenum::Len>::Output: std::ops::Add<typenum::B1>,
    <PRODUCES_AND_ACCOMPLISHES_TAIL as typenum::Len>::Output: Add<typenum::UInt<typenum::UTerm, B1>>,
    <<PRODUCES_AND_ACCOMPLISHES_TAIL as typenum::Len>::Output as std::ops::Add<typenum::B1>>::Output: typenum::Unsigned,
    <PARAM_HEAD as std::ops::BitOr<<PRODUCES_AND_ACCOMPLISHES_TAIL as brick::CaseArray>::PARAM_UNION>>::Output: brick::ParamBitSet,
    <<PRODUCES_AND_ACCOMPLISHES_TAIL as typenum::Len>::Output as std::ops::Add<typenum::B1>>::Output: typenum::Cmp<typenum::UInt<typenum::UTerm, typenum::B1>>,
    <<PRODUCES_AND_ACCOMPLISHES_TAIL as typenum::Len>::Output as std::ops::Add<typenum::B1>>::Output: IsGreaterPrivate<typenum::UInt<typenum::UTerm, typenum::B1>, <<<PRODUCES_AND_ACCOMPLISHES_TAIL as typenum::Len>::Output as std::ops::Add<typenum::B1>>::Output as typenum::Cmp<typenum::UInt<typenum::UTerm, typenum::B1>>>::Output>,
    <<<PRODUCES_AND_ACCOMPLISHES_TAIL as typenum::Len>::Output as std::ops::Add<typenum::B1>>::Output as typenum::private::IsGreaterPrivate<typenum::UInt<typenum::UTerm, typenum::B1>, <<<PRODUCES_AND_ACCOMPLISHES_TAIL as typenum::Len>::Output as std::ops::Add<typenum::B1>>::Output as typenum::Cmp<typenum::UInt<typenum::UTerm, typenum::B1>>>::Output>>::Output: typenum::NonZero,
    <ACTION_HEAD as std::ops::BitOr<<PRODUCES_AND_ACCOMPLISHES_TAIL as brick::CaseArray>::ACTION_UNION>>::Output: brick::ParamBitSet,
  // constraint
    Eq<FIRST_CONSUMES, And<FIRST_CONSUMES, Or<PARAM_HEAD, PRODUCES>>>: NonZero, // (PRODUCES union BRICK_PRODUCES) contain FIRST_CONSUMES
    Eq<BRICK_REQUIRES, And<BRICK_REQUIRES, Or<ACTION_HEAD, ACCOMPLISHES>>>: NonZero, // ACCOMPLISHES contain BRICK_REQUIRES
    And<BRICK_FORBIDS, Or<ACTION_HEAD, ACCOMPLISHES>>: Zero, // BRICK_FORBIDS are not in ACCOMPLISHES
    And<Or<PARAM_HEAD, PRODUCES_AND_ACCOMPLISHES_TAIL::PARAM_UNION>, PRODUCES>: Zero, // splitter doesn't produce what was already produced
    And<Or<ACTION_HEAD, PRODUCES_AND_ACCOMPLISHES_TAIL::ACTION_UNION>, ACCOMPLISHES>: Zero, // splitter doesn't accomplish what was already accomplished
  {
    FlowingSplitterProcess {
      process: InternalFlowingSplitProcess::FirstCase {
        brick: brick.to_internal(),
        first_case: first_case.process,
        process_before: self.process,
      },
      produces_and_accomplishes: Default::default(),
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
  PRODUCES_AND_ACCOMPLISHES: CaseArray,
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
  pub(crate) produces_and_accomplishes: PhantomData<PRODUCES_AND_ACCOMPLISHES>,
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
  PRODUCES_AND_ACCOMPLISHES: CaseArray,
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
  pub(crate) produces_and_accomplishes: PhantomData<PRODUCES_AND_ACCOMPLISHES>,
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

impl<
  PARAM_HEAD: ParamBitSet,
  ACTION_HEAD: Unsigned,
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
> FlowingSplitterProcess<
  TArr<(PARAM_HEAD, ACTION_HEAD), ATerm>,
  ROOT_CONSUMES,
  ROOT_REQUIRES,
  ROOT_FORBIDS,
  ROOT_PRODUCES,
  ROOT_ACCOMPLISHES,
  ACCUM_CONSUMES,
  ACCUM_REQUIRES,
  ACCUM_FORBIDS,
  ACCUM_PRODUCES,
  ACCUM_ACCOMPLISHES
> {
  pub fn last_case<
    CASE_CONSUMES: ParamBitSet,
    CASE_REQUIRES: Unsigned,
    CASE_FORBIDS: Unsigned,
    CASE_PRODUCES: ParamBitSet,
    CASE_ACCOMPLISHES: Unsigned,
  >(
    self,
    last_case: FlowingProcess<CASE_CONSUMES, CASE_REQUIRES, CASE_FORBIDS, CASE_PRODUCES, CASE_ACCOMPLISHES>,
  ) -> FlowingProcess<
    ROOT_CONSUMES,
    ROOT_REQUIRES,
    ROOT_FORBIDS,
    ROOT_PRODUCES,
    ROOT_ACCOMPLISHES,
  > {
    FlowingProcess {
      process: InternalFlowingProcess::Split {
        0: Box::new(
          InternalFlowingSplitProcess::NextCaseFlowing {
            next_case: last_case.process,
            process_before: Box::new(self.process),
          },
        ),
      },
      consumes: Default::default(),
      requires: Default::default(),
      forbids: Default::default(),
      produces: Default::default(),
      accomplishes: Default::default(),
    }
  }
}

impl<
  PARAM_HEAD: ParamBitSet + BitOr<<PRODUCES_AND_ACCOMPLISHES_TAIL as brick::CaseArray>::PARAM_UNION>,
  ACTION_HEAD: Unsigned + BitOr<<PRODUCES_AND_ACCOMPLISHES_TAIL as brick::CaseArray>::ACTION_UNION>,
  PRODUCES_AND_ACCOMPLISHES_TAIL: CaseArray + Len,
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
> FlowingSplitterProcess<
  TArr<(PARAM_HEAD, ACTION_HEAD), PRODUCES_AND_ACCOMPLISHES_TAIL>,
  ROOT_CONSUMES,
  ROOT_REQUIRES,
  ROOT_FORBIDS,
  ROOT_PRODUCES,
  ROOT_ACCOMPLISHES,
  ACCUM_CONSUMES,
  ACCUM_REQUIRES,
  ACCUM_FORBIDS,
  ACCUM_PRODUCES,
  ACCUM_ACCOMPLISHES,
> where
  <PARAM_HEAD as std::ops::BitOr<<PRODUCES_AND_ACCOMPLISHES_TAIL as brick::CaseArray>::PARAM_UNION>>::Output: brick::ParamBitSet,
  <ACTION_HEAD as std::ops::BitOr<<PRODUCES_AND_ACCOMPLISHES_TAIL as brick::CaseArray>::ACTION_UNION>>::Output: brick::ParamBitSet,
{
  pub fn case<
    CASE_CONSUMES: ParamBitSet,
    CASE_REQUIRES: Unsigned,
    CASE_FORBIDS: Unsigned,
    CASE_PRODUCES: ParamBitSet,
    CASE_ACCOMPLISHES: Unsigned,
  >(
    self,
    next_case: FlowingProcess<CASE_CONSUMES, CASE_REQUIRES, CASE_FORBIDS, CASE_PRODUCES, CASE_ACCOMPLISHES>,
  ) -> FlowingSplitterProcess<
    PRODUCES_AND_ACCOMPLISHES_TAIL,
    ROOT_CONSUMES,
    ROOT_REQUIRES,
    ROOT_FORBIDS,
    ROOT_PRODUCES,
    ROOT_ACCOMPLISHES,
    ACCUM_CONSUMES, // + CASE_CONSUMES: ParamBitSet,
    ACCUM_REQUIRES, // + CASE_REQUIRES: Unsigned,
    ACCUM_FORBIDS, // + CASE_FORBIDS: Unsigned,
    ACCUM_PRODUCES, // + CASE_PRODUCES: ParamBitSet,
    ACCUM_ACCOMPLISHES, // + CASE_ACCOMPLISHES: Unsigned,
  > {
    FlowingSplitterProcess {
      process: InternalFlowingSplitProcess::NextCaseFlowing {
        next_case: next_case.process,
        process_before: Box::new(self.process),
      },
      produces_and_accomplishes: Default::default(),
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
