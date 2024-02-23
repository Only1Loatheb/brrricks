use std::marker::PhantomData;
use typenum::*;
use process::brick_domain::*;
use process::internal_brick::*;

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

pub struct LinearBrick<
  CONSUMES: ParamBitSet,
  REQUIRES: Unsigned,
  FORBIDS: Unsigned,
  PRODUCES: ParamBitSet,
  ACCOMPLISHES: Unsigned,
> {
  pub name: &'static str,
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
  SPLITS: ParamBitSet,
  CONSUMES: ParamBitSet,
  REQUIRES: Unsigned,
  FORBIDS: Unsigned,
> {
  pub name: &'static str,
  pub splits: PhantomData<SPLITS>,
  pub consumes: PhantomData<CONSUMES>,
  pub requires_prior_completion: PhantomData<REQUIRES>,
  pub forbids_prior_completion: PhantomData<FORBIDS>,
  // pub produces_and_accomplishes: Vec<(Vec<ActionId>, Vec<ParamId>)>,
  pub handler: Box<dyn SplitterBrickHandler>,
}

impl<
  SPLITS: ParamBitSet,
  CONSUMES: ParamBitSet,
  REQUIRES: Unsigned,
  FORBIDS: Unsigned,
> SplitterBrick<SPLITS, CONSUMES, REQUIRES, FORBIDS> {

  pub(crate) fn to_internal(self) -> InternalSplitterBrick {
    InternalSplitterBrick {
      name: self.name,
      consumes: CONSUMES::get().0,
      // produces: brick
      //     .produces_and_accomplishes
      //     .into_iter()
      //     .map(|(_, params)| params)
      //     .collect(),
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
  pub name: &'static str,
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
