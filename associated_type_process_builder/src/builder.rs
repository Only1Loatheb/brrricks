use std::marker::PhantomData;
use typenum::*;
use typenum::private::*;
use std::ops::*;

use process::internal_process::*;
use crate::brick;
use crate::brick::*;

pub type EMPTY = U0;

pub struct FlowingProcess<
  USES: ParamBitSet,
  REQUIRES: Unsigned,
  FORBIDS: Unsigned,
  PRODUCES: ParamBitSet,
  ACCOMPLISHES: Unsigned,
> {
  pub(crate) process: InternalFlowingProcess,
  pub(crate) uses: PhantomData<USES>,
  pub(crate) requires: PhantomData<REQUIRES>,
  pub(crate) forbids: PhantomData<FORBIDS>,
  pub(crate) produces: PhantomData<PRODUCES>,
  pub(crate) accomplishes: PhantomData<ACCOMPLISHES>,
}

impl<
  USES: ParamBitSet,
  REQUIRES: Unsigned,
  FORBIDS: Unsigned,
  PRODUCES: ParamBitSet,
  ACCOMPLISHES: Unsigned,
> FlowingProcess<USES, REQUIRES, FORBIDS, PRODUCES, ACCOMPLISHES> {
  pub fn finnish<
    BRICK_USES: ParamBitSet + BitOr<USES> + BitAnd<PRODUCES> + Cmp<<BRICK_USES as BitAnd<PRODUCES>>::Output> + IsEqualPrivate<<BRICK_USES as BitAnd<PRODUCES>>::Output, <BRICK_USES as Cmp<<BRICK_USES as BitAnd<PRODUCES>>::Output>>::Output>,
    BRICK_REQUIRES: Unsigned + BitAnd<ACCOMPLISHES> + BitAnd<PRODUCES> + BitOr<REQUIRES> + Cmp<<BRICK_REQUIRES as BitAnd<ACCOMPLISHES>>::Output> + IsEqualPrivate<<BRICK_REQUIRES as BitAnd<ACCOMPLISHES>>::Output, <BRICK_REQUIRES as Cmp<<BRICK_REQUIRES as BitAnd<ACCOMPLISHES>>::Output>>::Output>,
    BRICK_FORBIDS: Unsigned + BitOr<ACCOMPLISHES> + BitAnd<ACCOMPLISHES> + BitOr<FORBIDS>,
    BRICK_ACCOMPLISHES: Unsigned + BitOr<ACCOMPLISHES> + BitAnd<ACCOMPLISHES>,
  >(
    self,
    brick: FinalBrick<BRICK_USES, BRICK_REQUIRES, BRICK_FORBIDS, BRICK_ACCOMPLISHES>,
  ) -> FinalizedProcess<
    Or<BRICK_USES, USES>,
    Or<BRICK_REQUIRES, REQUIRES>,
    Or<BRICK_FORBIDS, FORBIDS>,
    PRODUCES,
    Or<BRICK_ACCOMPLISHES, ACCOMPLISHES>,
  >
  where
  // outputs
    <BRICK_USES as BitOr<USES>>::Output: ParamBitSet,
    <BRICK_REQUIRES as BitOr<REQUIRES>>::Output: Unsigned,
    <BRICK_FORBIDS as BitOr<FORBIDS>>::Output: Unsigned,
    <BRICK_ACCOMPLISHES as BitOr<ACCOMPLISHES>>::Output: Unsigned,
  // constraint
    Eq<BRICK_USES, And<BRICK_USES, PRODUCES>>: NonZero, // PRODUCES contain BRICK_USES
    Eq<BRICK_REQUIRES, And<BRICK_REQUIRES, ACCOMPLISHES>>: NonZero, // ACCOMPLISHES contain BRICK_REQUIRES
    And<BRICK_FORBIDS, ACCOMPLISHES>: Zero, // BRICK_FORBIDS are not in ACCOMPLISHES
    And<BRICK_ACCOMPLISHES, ACCOMPLISHES>: Zero, // doesn't accomplish what was already accomplished
  {
    FinalizedProcess {
      process: InternalFinalizedProcess::Flowing(brick.to_internal(), self.process),
      uses: Default::default(),
      requires: Default::default(),
      forbids: Default::default(),
      produces: Default::default(),
      accomplishes: Default::default(),
    }
  }

  pub fn split<
    PARAM_HEAD: ParamBitSet + BitOr<PRODUCES> + BitOr<<PRODUCES_AND_ACCOMPLISHES_TAIL as CaseArray>::PARAM_UNION> + BitAnd<<PRODUCES_AND_ACCOMPLISHES_TAIL as CaseArray>::PARAM_INTERSECTION>,
    ACTION_HEAD: Unsigned + BitOr<ACCOMPLISHES> + BitOr<<PRODUCES_AND_ACCOMPLISHES_TAIL as CaseArray>::ACTION_UNION> + BitAnd<<PRODUCES_AND_ACCOMPLISHES_TAIL as CaseArray>::ACTION_INTERSECTION>,
    PRODUCES_AND_ACCOMPLISHES_TAIL: CaseArray + Len,
    BRICK_USES: ParamBitSet + BitOr<USES>,
    BRICK_REQUIRES: Unsigned + BitAnd<<ACTION_HEAD as BitOr<ACCOMPLISHES>>::Output> + Cmp<<BRICK_REQUIRES as BitAnd<<ACTION_HEAD as BitOr<ACCOMPLISHES>>::Output>>::Output> + IsEqualPrivate<<BRICK_REQUIRES as BitAnd<<ACTION_HEAD as BitOr<ACCOMPLISHES>>::Output>>::Output, <BRICK_REQUIRES as Cmp<<BRICK_REQUIRES as BitAnd<<ACTION_HEAD as BitOr<ACCOMPLISHES>>::Output>>::Output>>::Output> + BitOr<REQUIRES>,
    BRICK_FORBIDS: Unsigned + BitAnd<<ACTION_HEAD as BitOr<ACCOMPLISHES>>::Output> + BitOr<FORBIDS>,
    CASE_USES: ParamBitSet + BitAnd<<PARAM_HEAD as BitOr<PRODUCES>>::Output> + Cmp<<CASE_USES as BitAnd<<PARAM_HEAD as BitOr<PRODUCES>>::Output>>::Output> + Cmp<<CASE_USES as BitAnd<<PARAM_HEAD as BitOr<PRODUCES>>::Output>>::Output> + IsEqualPrivate<<CASE_USES as BitAnd<<PARAM_HEAD as BitOr<PRODUCES>>::Output>>::Output, <CASE_USES as Cmp<<CASE_USES as BitAnd<<PARAM_HEAD as BitOr<PRODUCES>>::Output>>::Output>>::Output>,
    CASE_REQUIRES: Unsigned,
    CASE_FORBIDS: Unsigned,
    CASE_PRODUCES: ParamBitSet,
    CASE_ACCOMPLISHES: Unsigned,
  >(
    self,
    brick: SplitterBrick<BRICK_USES, BRICK_REQUIRES, BRICK_FORBIDS, TArr<(PARAM_HEAD, ACTION_HEAD), PRODUCES_AND_ACCOMPLISHES_TAIL>>,
    the_case: FlowingProcess<CASE_USES, CASE_REQUIRES, CASE_FORBIDS, CASE_PRODUCES, CASE_ACCOMPLISHES>,
  ) -> FlowingSplitterProcess<
    PRODUCES_AND_ACCOMPLISHES_TAIL,
    Or<BRICK_USES, USES>,
    Or<BRICK_REQUIRES, REQUIRES>,
    Or<BRICK_FORBIDS, FORBIDS>,
    Or<And<PARAM_HEAD, PRODUCES_AND_ACCOMPLISHES_TAIL::PARAM_INTERSECTION>, PRODUCES>,
    Or<And<ACTION_HEAD, PRODUCES_AND_ACCOMPLISHES_TAIL::ACTION_INTERSECTION>, ACCOMPLISHES>,
    CASE_USES,
    CASE_REQUIRES,
    CASE_FORBIDS,
    CASE_PRODUCES,
    CASE_ACCOMPLISHES,
  >
  where
    // compiler generated types
    <ACTION_HEAD as BitOr<<PRODUCES_AND_ACCOMPLISHES_TAIL as CaseArray>::ACTION_UNION>>::Output: BitAnd<ACCOMPLISHES>,
    <PARAM_HEAD as BitOr<<PRODUCES_AND_ACCOMPLISHES_TAIL as CaseArray>::PARAM_UNION>>::Output: BitAnd<PRODUCES>,
    <PRODUCES_AND_ACCOMPLISHES_TAIL as Len>::Output: Add<B1>,
    <PRODUCES_AND_ACCOMPLISHES_TAIL as Len>::Output: Add<U1>,
    <<PRODUCES_AND_ACCOMPLISHES_TAIL as Len>::Output as Add<B1>>::Output: Unsigned,
    <PARAM_HEAD as BitOr<<PRODUCES_AND_ACCOMPLISHES_TAIL as CaseArray>::PARAM_UNION>>::Output: ParamBitSet,
    <<PRODUCES_AND_ACCOMPLISHES_TAIL as Len>::Output as Add<B1>>::Output: Cmp<U1>,
    <<PRODUCES_AND_ACCOMPLISHES_TAIL as Len>::Output as Add<B1>>::Output: IsGreaterPrivate<U1, <<<PRODUCES_AND_ACCOMPLISHES_TAIL as Len>::Output as Add<B1>>::Output as Cmp<U1>>::Output>,
    <<<PRODUCES_AND_ACCOMPLISHES_TAIL as Len>::Output as Add<B1>>::Output as IsGreaterPrivate<U1, <<<PRODUCES_AND_ACCOMPLISHES_TAIL as Len>::Output as Add<B1>>::Output as Cmp<U1>>::Output>>::Output: NonZero,
    <ACTION_HEAD as BitOr<<PRODUCES_AND_ACCOMPLISHES_TAIL as CaseArray>::ACTION_UNION>>::Output: ParamBitSet,
    <PARAM_HEAD as BitAnd<<PRODUCES_AND_ACCOMPLISHES_TAIL as CaseArray>::PARAM_INTERSECTION>>::Output: BitOr<PRODUCES>,
    <ACTION_HEAD as BitAnd<<PRODUCES_AND_ACCOMPLISHES_TAIL as CaseArray>::ACTION_INTERSECTION>>::Output: BitOr<ACCOMPLISHES>,
    <PARAM_HEAD as BitAnd<<PRODUCES_AND_ACCOMPLISHES_TAIL as CaseArray>::PARAM_INTERSECTION>>::Output: ParamBitSet,
    <ACTION_HEAD as BitAnd<<PRODUCES_AND_ACCOMPLISHES_TAIL as CaseArray>::ACTION_INTERSECTION>>::Output: Unsigned,
  // outputs
    <BRICK_USES as BitOr<USES>>::Output: ParamBitSet,
    <BRICK_REQUIRES as BitOr<REQUIRES>>::Output: Unsigned,
    <BRICK_FORBIDS as BitOr<FORBIDS>>::Output: Unsigned,
    <<PARAM_HEAD as BitAnd<<PRODUCES_AND_ACCOMPLISHES_TAIL as CaseArray>::PARAM_INTERSECTION>>::Output as BitOr<PRODUCES>>::Output: ParamBitSet,
    <<ACTION_HEAD as BitAnd<<PRODUCES_AND_ACCOMPLISHES_TAIL as CaseArray>::ACTION_INTERSECTION>>::Output as BitOr<ACCOMPLISHES>>::Output: Unsigned,
  // constraint
    Eq<CASE_USES, And<CASE_USES, Or<PARAM_HEAD, PRODUCES>>>: NonZero, // (PRODUCES union BRICK_PRODUCES) contain CASE_USES
    Eq<BRICK_REQUIRES, And<BRICK_REQUIRES, Or<ACTION_HEAD, ACCOMPLISHES>>>: NonZero, // ACCOMPLISHES contain BRICK_REQUIRES
    And<BRICK_FORBIDS, Or<ACTION_HEAD, ACCOMPLISHES>>: Zero, // BRICK_FORBIDS are not in ACCOMPLISHES
    And<Or<PARAM_HEAD, PRODUCES_AND_ACCOMPLISHES_TAIL::PARAM_UNION>, PRODUCES>: Zero, // splitter doesn't produce what was already produced
    And<Or<ACTION_HEAD, PRODUCES_AND_ACCOMPLISHES_TAIL::ACTION_UNION>, ACCOMPLISHES>: Zero, // splitter doesn't accomplish what was already accomplished
  {
    FlowingSplitterProcess {
      process: InternalFlowingSplitProcess::FirstCase {
        splitter_brick: brick.to_internal(),
        first_case: the_case.process,
        process_before: self.process,
      },
      produces_and_accomplishes: Default::default(),
      root_uses: Default::default(),
      root_requires: Default::default(),
      root_forbids: Default::default(),
      root_produces: Default::default(),
      root_accomplishes: Default::default(),
      split_uses: Default::default(),
      split_requires: Default::default(),
      split_forbids: Default::default(),
      split_produces: Default::default(),
      split_accomplishes: Default::default(),
    }
  }
}

pub struct FinalizedSplitterProcess<
  PRODUCES_AND_ACCOMPLISHES: CaseArray,
  ROOT_USES: ParamBitSet,
  ROOT_REQUIRES: Unsigned,
  ROOT_FORBIDS: Unsigned,
  ROOT_PRODUCES: ParamBitSet,
  ROOT_ACCOMPLISHES: Unsigned,
  ACCUM_USES: ParamBitSet,
  ACCUM_REQUIRES: Unsigned,
  ACCUM_FORBIDS: Unsigned,
  ACCUM_PRODUCES: ParamBitSet,
  ACCUM_ACCOMPLISHES: Unsigned,
> {
  pub(crate) process: InternalFinalizedSplitProcess,
  pub(crate) produces_and_accomplishes: PhantomData<PRODUCES_AND_ACCOMPLISHES>,
  pub(crate) root_uses: PhantomData<ROOT_USES>,
  pub(crate) root_requires: PhantomData<ROOT_REQUIRES>,
  pub(crate) root_forbids: PhantomData<ROOT_FORBIDS>,
  pub(crate) root_produces: PhantomData<ROOT_PRODUCES>,
  pub(crate) root_accomplishes: PhantomData<ROOT_ACCOMPLISHES>,
  pub(crate) split_uses: PhantomData<ACCUM_USES>,
  pub(crate) split_requires: PhantomData<ACCUM_REQUIRES>,
  pub(crate) split_forbids: PhantomData<ACCUM_FORBIDS>,
  pub(crate) split_produces: PhantomData<ACCUM_PRODUCES>,
  pub(crate) split_accomplishes: PhantomData<ACCUM_ACCOMPLISHES>,
}

pub struct FlowingSplitterProcess<
  PRODUCES_AND_ACCOMPLISHES: CaseArray,
  ROOT_USES: ParamBitSet,
  ROOT_REQUIRES: Unsigned,
  ROOT_FORBIDS: Unsigned,
  ROOT_PRODUCES: ParamBitSet,
  ROOT_ACCOMPLISHES: Unsigned,
  ACCUM_USES: ParamBitSet,
  ACCUM_REQUIRES: Unsigned,
  ACCUM_FORBIDS: Unsigned,
  ACCUM_PRODUCES: ParamBitSet,
  ACCUM_ACCOMPLISHES: Unsigned,
> {
  pub(crate) process: InternalFlowingSplitProcess,
  pub(crate) produces_and_accomplishes: PhantomData<PRODUCES_AND_ACCOMPLISHES>,
  pub(crate) root_uses: PhantomData<ROOT_USES>,
  pub(crate) root_requires: PhantomData<ROOT_REQUIRES>,
  pub(crate) root_forbids: PhantomData<ROOT_FORBIDS>,
  pub(crate) root_produces: PhantomData<ROOT_PRODUCES>,
  pub(crate) root_accomplishes: PhantomData<ROOT_ACCOMPLISHES>,
  pub(crate) split_uses: PhantomData<ACCUM_USES>,
  pub(crate) split_requires: PhantomData<ACCUM_REQUIRES>,
  pub(crate) split_forbids: PhantomData<ACCUM_FORBIDS>,
  pub(crate) split_produces: PhantomData<ACCUM_PRODUCES>,
  pub(crate) split_accomplishes: PhantomData<ACCUM_ACCOMPLISHES>,
}

impl<
  PARAM_HEAD: ParamBitSet + BitAnd<ROOT_PRODUCES> + BitOr<ROOT_PRODUCES>,
  ACTION_HEAD: Unsigned + BitAnd<ROOT_ACCOMPLISHES> + BitOr<ROOT_ACCOMPLISHES>,
  ROOT_USES: ParamBitSet,
  ROOT_REQUIRES: Unsigned,
  ROOT_FORBIDS: Unsigned,
  ROOT_PRODUCES: ParamBitSet,
  ROOT_ACCOMPLISHES: Unsigned,
  ACCUM_USES: ParamBitSet + BitOr<ROOT_USES>,
  ACCUM_REQUIRES: Unsigned + BitOr<ROOT_REQUIRES>,
  ACCUM_FORBIDS: Unsigned + BitOr<ROOT_FORBIDS>,
  ACCUM_PRODUCES: ParamBitSet + BitAnd<ROOT_PRODUCES>,
  ACCUM_ACCOMPLISHES: Unsigned + BitAnd<ROOT_ACCOMPLISHES>,
> FlowingSplitterProcess<
  TArr<(PARAM_HEAD, ACTION_HEAD), ATerm>,
  ROOT_USES,
  ROOT_REQUIRES,
  ROOT_FORBIDS,
  ROOT_PRODUCES,
  ROOT_ACCOMPLISHES,
  ACCUM_USES,
  ACCUM_REQUIRES,
  ACCUM_FORBIDS,
  ACCUM_PRODUCES,
  ACCUM_ACCOMPLISHES
> where
      And<PARAM_HEAD, ROOT_PRODUCES>: Zero, // splitter doesn't produce what was already produced
    And<ACTION_HEAD, ROOT_ACCOMPLISHES>: Zero, // splitter doesn't accomplish what was already accomplished
{
  pub fn last_case<
    CASE_USES: ParamBitSet + BitOr<<ACCUM_USES as BitOr<ROOT_USES>>::Output> + BitAnd<<PARAM_HEAD as std::ops::BitOr<ROOT_PRODUCES>>::Output> + typenum::Cmp<<CASE_USES as std::ops::BitAnd<<PARAM_HEAD as std::ops::BitOr<ROOT_PRODUCES>>::Output>>::Output> + typenum::private::IsEqualPrivate<<CASE_USES as std::ops::BitAnd<<PARAM_HEAD as std::ops::BitOr<ROOT_PRODUCES>>::Output>>::Output, <CASE_USES as typenum::Cmp<<CASE_USES as std::ops::BitAnd<<PARAM_HEAD as std::ops::BitOr<ROOT_PRODUCES>>::Output>>::Output>>::Output>,
    CASE_REQUIRES: Unsigned + BitOr<<ACCUM_REQUIRES as BitOr<ROOT_REQUIRES>>::Output> + BitAnd<<ACTION_HEAD as BitOr<ROOT_ACCOMPLISHES>>::Output> + Cmp<<CASE_REQUIRES as BitAnd<<ACTION_HEAD as BitOr<ROOT_ACCOMPLISHES>>::Output>>::Output> + IsEqualPrivate<<CASE_REQUIRES as std::ops::BitAnd<<ACTION_HEAD as std::ops::BitOr<ROOT_ACCOMPLISHES>>::Output>>::Output, <CASE_REQUIRES as typenum::Cmp<<CASE_REQUIRES as std::ops::BitAnd<<ACTION_HEAD as std::ops::BitOr<ROOT_ACCOMPLISHES>>::Output>>::Output>>::Output>,
    CASE_FORBIDS: Unsigned + BitOr<<ACCUM_FORBIDS as BitOr<ROOT_FORBIDS>>::Output> + BitAnd<<ACTION_HEAD as BitOr<ROOT_ACCOMPLISHES>>::Output>,
    CASE_PRODUCES: ParamBitSet + BitAnd<ACCUM_PRODUCES>,
    CASE_ACCOMPLISHES: Unsigned + BitAnd<ACCUM_ACCOMPLISHES>,
  >(
    self,
    the_case: FlowingProcess<CASE_USES, CASE_REQUIRES, CASE_FORBIDS, CASE_PRODUCES, CASE_ACCOMPLISHES>,
  ) -> FlowingProcess<
    Or<CASE_USES, Or<ACCUM_USES, ROOT_USES>>,
    Or<CASE_REQUIRES, Or<ACCUM_REQUIRES, ROOT_REQUIRES>>,
    Or<CASE_FORBIDS, Or<ACCUM_FORBIDS, ROOT_FORBIDS>>,
    Or<And<CASE_PRODUCES, ACCUM_PRODUCES>, ROOT_PRODUCES>,
    Or<And<CASE_ACCOMPLISHES, ACCUM_ACCOMPLISHES>, ROOT_ACCOMPLISHES>,
  > where
    // generated
  <CASE_PRODUCES as BitAnd<ACCUM_PRODUCES>>::Output: BitOr<ROOT_PRODUCES>  ,
  <CASE_ACCOMPLISHES as BitAnd<ACCUM_ACCOMPLISHES>>::Output: BitOr<ROOT_ACCOMPLISHES>,
  // outputs
  <CASE_USES as BitOr<<ACCUM_USES as BitOr<ROOT_USES>>::Output>>::Output: ParamBitSet,
  <CASE_REQUIRES as BitOr<<ACCUM_REQUIRES as BitOr<ROOT_REQUIRES>>::Output>>::Output: Unsigned,
  <CASE_FORBIDS as BitOr<<ACCUM_FORBIDS as BitOr<ROOT_FORBIDS>>::Output>>::Output: Unsigned,
  <<CASE_PRODUCES as BitAnd<ACCUM_PRODUCES>>::Output as BitOr<ROOT_PRODUCES>>::Output: ParamBitSet,
  <<CASE_ACCOMPLISHES as BitAnd<ACCUM_ACCOMPLISHES>>::Output as BitOr<ROOT_ACCOMPLISHES>>::Output: Unsigned,
  // constraint
    Eq<CASE_USES, And<CASE_USES, Or<PARAM_HEAD, ROOT_PRODUCES>>>: NonZero, // (ROOT_PRODUCES union SPLIT_PRODUCES) contain CASE_USES
    Eq<CASE_REQUIRES, And<CASE_REQUIRES, Or<ACTION_HEAD, ROOT_ACCOMPLISHES>>>: NonZero, // (ROOT_ACCOMPLISHES union SPLIT_ACCOMPLISHES) contain CASE_REQUIRES
    And<CASE_FORBIDS, Or<ACTION_HEAD, ROOT_ACCOMPLISHES>>: Zero, // (ACTION_HEAD union ROOT_ACCOMPLISHES) are not in ACCOMPLISHES
  {
    FlowingProcess {
      process: InternalFlowingProcess::Split {
        0: Box::new(
          InternalFlowingSplitProcess::NextCaseFlowing {
            next_case: the_case.process,
            split_process_before: Box::new(self.process),
          },
        ),
      },
      uses: Default::default(),
      requires: Default::default(),
      forbids: Default::default(),
      produces: Default::default(),
      accomplishes: Default::default(),
    }
  }
}

impl<
  PARAM_HEAD: ParamBitSet + BitOr<ROOT_PRODUCES> + BitOr<<PRODUCES_AND_ACCOMPLISHES_TAIL as CaseArray>::PARAM_UNION> + BitAnd<<PRODUCES_AND_ACCOMPLISHES_TAIL as CaseArray>::PARAM_INTERSECTION>,
  ACTION_HEAD: Unsigned + BitOr<ROOT_ACCOMPLISHES> + BitOr<<PRODUCES_AND_ACCOMPLISHES_TAIL as CaseArray>::ACTION_UNION> + BitAnd<<PRODUCES_AND_ACCOMPLISHES_TAIL as CaseArray>::ACTION_INTERSECTION>,
  PRODUCES_AND_ACCOMPLISHES_TAIL: CaseArray,
  ROOT_USES: ParamBitSet,
  ROOT_REQUIRES: Unsigned,
  ROOT_FORBIDS: Unsigned,
  ROOT_PRODUCES: ParamBitSet,
  ROOT_ACCOMPLISHES: Unsigned,
  ACCUM_USES: ParamBitSet,
  ACCUM_REQUIRES: Unsigned,
  ACCUM_FORBIDS: Unsigned,
  ACCUM_PRODUCES: ParamBitSet,
  ACCUM_ACCOMPLISHES: Unsigned,
> FlowingSplitterProcess<
  TArr<(PARAM_HEAD, ACTION_HEAD), PRODUCES_AND_ACCOMPLISHES_TAIL>,
  ROOT_USES,
  ROOT_REQUIRES,
  ROOT_FORBIDS,
  ROOT_PRODUCES,
  ROOT_ACCOMPLISHES,
  ACCUM_USES,
  ACCUM_REQUIRES,
  ACCUM_FORBIDS,
  ACCUM_PRODUCES,
  ACCUM_ACCOMPLISHES,
> where
 // generated
  <PARAM_HEAD as BitAnd<<PRODUCES_AND_ACCOMPLISHES_TAIL as CaseArray>::PARAM_INTERSECTION>>::Output: ParamBitSet,
  <PARAM_HEAD as BitOr<<PRODUCES_AND_ACCOMPLISHES_TAIL as CaseArray>::PARAM_UNION>>::Output: ParamBitSet,
  <PARAM_HEAD as std::ops::BitOr<<PRODUCES_AND_ACCOMPLISHES_TAIL as brick::CaseArray>::PARAM_UNION>>::Output: std::ops::BitAnd<ROOT_PRODUCES>,
  <ACTION_HEAD as BitAnd<<PRODUCES_AND_ACCOMPLISHES_TAIL as CaseArray>::ACTION_INTERSECTION>>::Output: Unsigned,
  <ACTION_HEAD as BitOr<<PRODUCES_AND_ACCOMPLISHES_TAIL as CaseArray>::ACTION_UNION>>::Output: Unsigned,
  <ACTION_HEAD as std::ops::BitOr<<PRODUCES_AND_ACCOMPLISHES_TAIL as brick::CaseArray>::ACTION_UNION>>::Output: std::ops::BitAnd<ROOT_ACCOMPLISHES>,
// constraints
    And<Or<PARAM_HEAD, PRODUCES_AND_ACCOMPLISHES_TAIL::PARAM_UNION>, ROOT_PRODUCES>: Zero, // splitter doesn't produce what was already produced
    And<Or<ACTION_HEAD, PRODUCES_AND_ACCOMPLISHES_TAIL::ACTION_UNION>, ROOT_ACCOMPLISHES>: Zero, // splitter doesn't accomplish what was already accomplished
{
  pub fn next_case<
    CASE_USES: ParamBitSet + BitOr<ACCUM_USES> + BitAnd<<PARAM_HEAD as std::ops::BitOr<ROOT_PRODUCES>>::Output> +typenum::Cmp<<CASE_USES as std::ops::BitAnd<<PARAM_HEAD as std::ops::BitOr<ROOT_PRODUCES>>::Output>>::Output> + typenum::private::IsEqualPrivate<<CASE_USES as std::ops::BitAnd<<PARAM_HEAD as std::ops::BitOr<ROOT_PRODUCES>>::Output>>::Output, <CASE_USES as typenum::Cmp<<CASE_USES as std::ops::BitAnd<<PARAM_HEAD as std::ops::BitOr<ROOT_PRODUCES>>::Output>>::Output>>::Output>,
    CASE_REQUIRES: Unsigned + BitOr<ACCUM_REQUIRES> + BitAnd<<ACTION_HEAD as std::ops::BitOr<ROOT_ACCOMPLISHES>>::Output> + Cmp<<CASE_REQUIRES as std::ops::BitAnd<<ACTION_HEAD as std::ops::BitOr<ROOT_ACCOMPLISHES>>::Output>>::Output> + typenum::private::IsEqualPrivate<<CASE_REQUIRES as std::ops::BitAnd<<ACTION_HEAD as std::ops::BitOr<ROOT_ACCOMPLISHES>>::Output>>::Output, <CASE_REQUIRES as typenum::Cmp<<CASE_REQUIRES as std::ops::BitAnd<<ACTION_HEAD as std::ops::BitOr<ROOT_ACCOMPLISHES>>::Output>>::Output>>::Output>,
    CASE_FORBIDS: Unsigned + BitOr<ACCUM_FORBIDS> + BitAnd<<ACTION_HEAD as std::ops::BitOr<ROOT_ACCOMPLISHES>>::Output>,
    CASE_PRODUCES: ParamBitSet + BitAnd<ACCUM_PRODUCES>,
    CASE_ACCOMPLISHES: Unsigned + BitAnd<ACCUM_ACCOMPLISHES>,
  >(
    self,
    the_case: FlowingProcess<CASE_USES, CASE_REQUIRES, CASE_FORBIDS, CASE_PRODUCES, CASE_ACCOMPLISHES>,
  ) -> FlowingSplitterProcess<
    PRODUCES_AND_ACCOMPLISHES_TAIL,
    ROOT_USES,
    ROOT_REQUIRES,
    ROOT_FORBIDS,
    ROOT_PRODUCES,
    ROOT_ACCOMPLISHES,
    Or<CASE_USES, ACCUM_USES>,
    Or<CASE_REQUIRES, ACCUM_REQUIRES>,
    Or<CASE_FORBIDS, ACCUM_FORBIDS>,
    And<CASE_PRODUCES, ACCUM_PRODUCES>,
    And<CASE_ACCOMPLISHES, ACCUM_ACCOMPLISHES>,
  > where
  // outputs
  <CASE_USES as BitOr<ACCUM_USES>>::Output: ParamBitSet,
  <CASE_REQUIRES as BitOr<ACCUM_REQUIRES>>::Output: Unsigned,
  <CASE_FORBIDS as BitOr<ACCUM_FORBIDS>>::Output: Unsigned,
  <CASE_PRODUCES as BitAnd<ACCUM_PRODUCES>>::Output: ParamBitSet,
  <CASE_ACCOMPLISHES as BitAnd<ACCUM_ACCOMPLISHES>>::Output: Unsigned,
  // constraint
    Eq<CASE_USES, And<CASE_USES, Or<PARAM_HEAD, ROOT_PRODUCES>>>: NonZero, // (ROOT_PRODUCES union SPLIT_PRODUCES) contain CASE_USES
    Eq<CASE_REQUIRES, And<CASE_REQUIRES, Or<ACTION_HEAD, ROOT_ACCOMPLISHES>>>: NonZero, // (ROOT_ACCOMPLISHES union SPLIT_ACCOMPLISHES) contain CASE_REQUIRES
    And<CASE_FORBIDS, Or<ACTION_HEAD, ROOT_ACCOMPLISHES>>: Zero, // (ACTION_HEAD union ROOT_ACCOMPLISHES) are not in ACCOMPLISHES
  {
    FlowingSplitterProcess {
      process: InternalFlowingSplitProcess::NextCaseFlowing {
        next_case: the_case.process,
        split_process_before: Box::new(self.process),
      },
      produces_and_accomplishes: Default::default(),
      root_uses: Default::default(),
      root_requires: Default::default(),
      root_forbids: Default::default(),
      root_produces: Default::default(),
      root_accomplishes: Default::default(),
      split_uses: Default::default(),
      split_requires: Default::default(),
      split_forbids: Default::default(),
      split_produces: Default::default(),
      split_accomplishes: Default::default(),
    }
  }
}


pub struct FinalizedProcess<
  USES: ParamBitSet,
  REQUIRES: Unsigned,
  FORBIDS: Unsigned,
  PRODUCES: ParamBitSet,
  ACCOMPLISHES: Unsigned,
> {
  pub(crate) process: InternalFinalizedProcess,
  pub(crate) uses: PhantomData<USES>,
  pub(crate) requires: PhantomData<REQUIRES>,
  pub(crate) forbids: PhantomData<FORBIDS>,
  pub(crate) produces: PhantomData<PRODUCES>,
  pub(crate) accomplishes: PhantomData<ACCOMPLISHES>,
}

impl<
  USES: ParamBitSet,
  REQUIRES: Unsigned,
  FORBIDS: Unsigned,
  PRODUCES: ParamBitSet,
  ACCOMPLISHES: Unsigned,
> FinalizedProcess<USES, REQUIRES, FORBIDS, PRODUCES, ACCOMPLISHES> {
  pub fn close(self, path: String) -> NamedProcess {
    NamedProcess {
      path,
      process: self.process,
    }
  }
}

//
//   pub fn and_then<
//     BRICK_USES: ParamBitSet + IsEqual<And<PRODUCES, BRICK_USES>>, // a_includes_b(a & b == b)
//     BRICK_REQUIRES: Unsigned + IsEqual<And<ACCOMPLISHES, BRICK_REQUIRES>>, // a_includes_b(a & b == b)
//     BRICK_FORBIDS: Unsigned,
//     BRICK_PRODUCES: ParamBitSet,
//     BRICK_ACCOMPLISHES: Unsigned,
//   >(
//     self,
//     brick: LinearBrick<
//       BRICK_USES,
//       BRICK_REQUIRES,
//       BRICK_FORBIDS,
//       BRICK_PRODUCES,
//       BRICK_ACCOMPLISHES,
//     >,
//   ) -> FlowingProcess<
//     Or<USES, BRICK_USES>,
//     Or<REQUIRES, BRICK_REQUIRES>,
//     Or<FORBIDS, BRICK_FORBIDS>,
//     Or<PRODUCES, BRICK_PRODUCES>,
//     Or<ACCOMPLISHES, BRICK_ACCOMPLISHES>,
//   >
//   where
//     BRICK_USES::Output: NonZero,
//     BRICK_REQUIRES::Output: NonZero,
//     And<BRICK_FORBIDS, ACCOMPLISHES>: Zero,
//     Or<ACCOMPLISHES, BRICK_FORBIDS>: Zero,
//   {
//     FlowingProcess {
//       process: FlowingLinearProcess::Flowing {
//         brick: InternalLinearBrick::new(brick),
//         process_before_brick: Box::new(self.process),
//       },
//       uses: Default::default(),
//       requires: Default::default(),
//       forbids: Default::default(),
//       produces: Default::default(),
//       accomplishes: Default::default(),
//     }
//   }
//
