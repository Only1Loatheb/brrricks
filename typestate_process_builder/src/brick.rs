use std::marker::PhantomData;
use std::ops::*;

use typenum::*;

use process::brick_domain::*;
use process::internal_brick::*;
use crate::brick;


// #[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]

pub trait ParamBitSet: Unsigned {
  fn get() -> (Vec<ParamId>, usize);
}

impl ParamBitSet for UTerm {
  fn get() -> (Vec<ParamId>, usize) {
    (vec![], 0)
  }
}

impl<MORE_SIGNIFICANT_BITS: ParamBitSet, LEAST_SIGNIFICANT_BIT: Bit> ParamBitSet for UInt<MORE_SIGNIFICANT_BITS, LEAST_SIGNIFICANT_BIT> {
  fn get() -> (Vec<ParamId>, usize) {
    let (mut vector, mut index) = MORE_SIGNIFICANT_BITS::get();
    if LEAST_SIGNIFICANT_BIT::to_bool() {
      vector.push(ParamId(index));
    }
    index += 1;
    (vector, index)
  }
}

pub trait CaseArray: TypeArray {
  type PARAM_HEAD: ParamBitSet;
  type PARAM_UNION: ParamBitSet;
  type ACTION_HEAD: Unsigned;
  type ACTION_UNION: Unsigned;
  type TAIL: TypeArray;
  fn get() -> Vec<Vec<ParamId>>;
}

impl<PARAM_HEAD: ParamBitSet, ACTION_HEAD: Unsigned> CaseArray for TArr<(PARAM_HEAD, ACTION_HEAD), ATerm> {
  type PARAM_HEAD = PARAM_HEAD;
  type PARAM_UNION = PARAM_HEAD;
  type ACTION_HEAD = ACTION_HEAD;
  type ACTION_UNION = ACTION_HEAD;
  type TAIL = ATerm;

  fn get() -> Vec<Vec<ParamId>> {
    vec![]
  }
}

impl<
  TAIL: CaseArray,
  PARAM_HEAD: ParamBitSet + BitOr<TAIL::PARAM_UNION>,
  ACTION_HEAD: Unsigned + BitOr<TAIL::ACTION_UNION>,
> CaseArray for TArr<(PARAM_HEAD, ACTION_HEAD), TAIL>
  where
    <PARAM_HEAD as BitOr<TAIL::PARAM_UNION>>::Output: ParamBitSet,
    <ACTION_HEAD as BitOr<TAIL::ACTION_UNION>>::Output: ParamBitSet,
{
  type PARAM_HEAD = PARAM_HEAD;
  type PARAM_UNION = Or<PARAM_HEAD, TAIL::PARAM_UNION>;
  type ACTION_HEAD = ACTION_HEAD;
  type ACTION_UNION = Or<ACTION_HEAD, TAIL::ACTION_UNION>;
  type TAIL = TAIL;

  fn get() -> Vec<Vec<ParamId>> {
    let mut vector = TAIL::get();
    vector.push(PARAM_HEAD::get().0);
    vector
  }
}

pub struct LinearBrick<
  CONSUMES: ParamBitSet,
  REQUIRES: Unsigned,
  FORBIDS: Unsigned,
  PRODUCES: ParamBitSet,
  ACCOMPLISHES: Unsigned,
> {
  pub name: String,
  pub consumes: PhantomData<CONSUMES>,
  pub requires_prior_completion: PhantomData<REQUIRES>,
  pub forbids_prior_completion: PhantomData<FORBIDS>,
  pub produces: PhantomData<PRODUCES>,
  pub accomplishes: PhantomData<ACCOMPLISHES>,
  pub handler: Box<dyn LinearBrickHandler>,
}

impl<
  CONSUMES: ParamBitSet,
  REQUIRES: Unsigned,
  FORBIDS: Unsigned,
  PRODUCES: ParamBitSet,
  ACCOMPLISHES: Unsigned,
> LinearBrick<CONSUMES, REQUIRES, FORBIDS, PRODUCES, ACCOMPLISHES> {
  pub(crate) fn to_internal(self) -> InternalLinearBrick {
    InternalLinearBrick {
      name: self.name,
      consumes: CONSUMES::get().0,
      produces: PRODUCES::get().0,
      handler: self.handler,
    }
  }
}

// consider https://github.com/rust-phf/rust-phf for SplitIndex
pub struct SplitterBrick<
  CONSUMES: ParamBitSet,
  REQUIRES: Unsigned,
  FORBIDS: Unsigned,
  PRODUCES_AND_ACCOMPLISHES: CaseArray,
> {
  pub name: String,
  pub consumes: PhantomData<CONSUMES>,
  pub requires_prior_completion: PhantomData<REQUIRES>,
  pub forbids_prior_completion: PhantomData<FORBIDS>,
  pub produces_and_accomplishes: PhantomData<PRODUCES_AND_ACCOMPLISHES>,
  pub handler: Box<dyn SplitterBrickHandler>,
}

impl<
  CONSUMES: ParamBitSet,
  REQUIRES: Unsigned,
  FORBIDS: Unsigned,
  PRODUCES_AND_ACCOMPLISHES: CaseArray,
> SplitterBrick<CONSUMES, REQUIRES, FORBIDS, PRODUCES_AND_ACCOMPLISHES> {
  pub(crate) fn to_internal(self) -> InternalSplitterBrick {
    InternalSplitterBrick {
      name: self.name,
      consumes: CONSUMES::get().0,
      produces: PRODUCES_AND_ACCOMPLISHES::get(),
      handler: self.handler,
    }
  }
}

pub struct FinalBrick<
  CONSUMES: ParamBitSet,
  REQUIRES: Unsigned,
  FORBIDS: Unsigned,
  ACCOMPLISHES: Unsigned,
> {
  pub name: String,
  pub consumes: PhantomData<CONSUMES>,
  pub requires_prior_completion: PhantomData<REQUIRES>,
  pub forbids_prior_completion: PhantomData<FORBIDS>,
  pub accomplishes: PhantomData<ACCOMPLISHES>,
  pub handler: Box<dyn FinalBrickHandler>,
}

impl<
  CONSUMES: ParamBitSet,
  REQUIRES: Unsigned,
  FORBIDS: Unsigned,
  ACCOMPLISHES: Unsigned,
> FinalBrick<CONSUMES, REQUIRES, FORBIDS, ACCOMPLISHES> {
  pub(crate) fn to_internal(self) -> InternalFinalBrick {
    InternalFinalBrick {
      name: self.name,
      consumes: CONSUMES::get().0,
      handler: self.handler,
    }
  }
}
