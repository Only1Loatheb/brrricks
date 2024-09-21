use std::marker::PhantomData;
use typenum::*;
use typenum::private::*;
use std::ops::*;
use frunk_core::hlist::Selector;
use process_builder_common::internal_process::*;
use crate::brick;
use crate::brick::*;
use crate::invariant::Invariant;

pub type EMPTY = U0;

pub struct FlowingProcess<'same_process,> {
  pub(crate) process: InternalFlowingProcess,
  // pub(crate) consumes: PhantomData<CONSUMES>,
  pub(crate) next_param_id: usize,
  pub(crate) same_process_invariant: Invariant<'same_process>,
}

impl<'same_process> FlowingProcess<'same_process> {

  pub fn finnish<FINAL_BRICK_CONSUMES: ParamReprList<'same_process>>(
    mut self,
    _consumes: FINAL_BRICK_CONSUMES,
    brick: FinalBrick<'same_process, FINAL_BRICK_CONSUMES>,
  ) -> FinalizedProcess<'same_process>
  {
    FinalizedProcess {
      process: InternalFinalizedProcess::Flowing(brick.to_internal(), self.process),
      next_param_id: self.next_param_id,
      same_process_invariant: Default::default(),
    }
  }

  pub fn split<
    ROOT_CONSUMES: ParamReprList<'same_process>,
    SEL,
    CONSUMES_CASE_THIS: ParamReprList<'same_process> + Selector<ROOT_CONSUMES, SEL>,
    PRODUCES_CASE_THIS: ParamReprList<'same_process>,
    PRODUCES_CASE_OTHER: SplitterReprCase<'same_process>
  >(
    self,
    _consumes: ROOT_CONSUMES,
    splitter_brick: SplitterBrick<'same_process, PRODUCES_CASE_THIS, PRODUCES_CASE_OTHER>,
    this_case_process: FlowingProcess<'same_process>,
  ) -> FlowingSplitterProcess<'same_process, ROOT_CONSUMES>
  {
    FlowingSplitterProcess {
      process: InternalFlowingSplitProcess::FirstCase {
        splitter_brick: splitter_brick.to_internal(),
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
  'same_process,
    ROOT_CONSUMES: ParamReprList<'same_process>,
    PRODUCES_CASE_OTHER: SplitterReprCase<'same_process>
> {
  pub(crate) process: InternalFlowingSplitProcess,
  pub(crate) root_consumes: PhantomData<ROOT_CONSUMES>,
  pub(crate) produces_case_other: PhantomData<PRODUCES_CASE_OTHER>,
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


pub struct FinalizedProcess<'same_process> {
  pub(crate) process: InternalFinalizedProcess,
  pub(crate) next_param_id: usize,
  pub(crate) same_process_invariant: Invariant<'same_process>,
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
