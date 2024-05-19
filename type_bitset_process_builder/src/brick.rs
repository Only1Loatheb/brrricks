use std::marker::PhantomData;
use std::ops::*;

use typenum::*;
use typenum::private::IsGreaterPrivate;
use async_trait::async_trait;

use process::brick_domain::*;
use process::internal_brick::*;
use crate::split_index::TypeSplitIndex;

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
  type PARAM_UNION: ParamBitSet;
  type ACTION_UNION: Unsigned;
  type PARAM_INTERSECTION: ParamBitSet;
  type ACTION_INTERSECTION: Unsigned;
  fn get() -> Vec<Vec<ParamId>>;
}

impl<PARAM_HEAD: ParamBitSet, ACTION_HEAD: Unsigned> CaseArray for TArr<(PARAM_HEAD, ACTION_HEAD), ATerm> {
  type PARAM_UNION = PARAM_HEAD;
  type ACTION_UNION = ACTION_HEAD;
  type PARAM_INTERSECTION = PARAM_HEAD;
  type ACTION_INTERSECTION = ACTION_HEAD;

  fn get() -> Vec<Vec<ParamId>> {
    vec![]
  }
}

impl<
  TAIL: CaseArray,
  PARAM_HEAD: ParamBitSet + BitOr<TAIL::PARAM_UNION> + BitAnd<TAIL::PARAM_INTERSECTION>,
  ACTION_HEAD: Unsigned + BitOr<TAIL::ACTION_UNION> + BitAnd<TAIL::ACTION_INTERSECTION>,
> CaseArray for TArr<(PARAM_HEAD, ACTION_HEAD), TAIL>
where
  <PARAM_HEAD as BitOr<TAIL::PARAM_UNION>>::Output: ParamBitSet,
  <ACTION_HEAD as BitOr<TAIL::ACTION_UNION>>::Output: Unsigned,
  <PARAM_HEAD as BitAnd<TAIL::PARAM_INTERSECTION>>::Output: ParamBitSet,
  <ACTION_HEAD as BitAnd<TAIL::ACTION_INTERSECTION>>::Output: Unsigned,
{
  type PARAM_UNION = Or<PARAM_HEAD, TAIL::PARAM_UNION>;
  type ACTION_UNION = Or<ACTION_HEAD, TAIL::ACTION_UNION>;
  type PARAM_INTERSECTION = And<PARAM_HEAD, TAIL::PARAM_INTERSECTION>;
  type ACTION_INTERSECTION = And<ACTION_HEAD, TAIL::ACTION_INTERSECTION>;

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

#[derive(Clone)]
pub struct TypeSplitterOutput<CASES_LEN: Unsigned>(pub TypeSplitIndex<CASES_LEN>, pub OutputParams);

#[async_trait]
pub trait TypeSplitterBrickHandler<CASES_LEN: Unsigned>: Send + Sync {
    async fn handle(&self, input: InputParams) -> anyhow::Result<TypeSplitterOutput<CASES_LEN>>;
}

// consider https://github.com/rust-phf/rust-phf for SplitIndex
pub struct SplitterBrick<
  CONSUMES: ParamBitSet,
  REQUIRES: Unsigned,
  FORBIDS: Unsigned,
  PRODUCES_AND_ACCOMPLISHES: CaseArray + Len,
> where
  <PRODUCES_AND_ACCOMPLISHES as Len>::Output: Cmp<U1>,
  <PRODUCES_AND_ACCOMPLISHES as Len>::Output: IsGreaterPrivate<U1, <<PRODUCES_AND_ACCOMPLISHES as Len>::Output as Cmp<U1>>::Output>,
  Gr<Length<PRODUCES_AND_ACCOMPLISHES>, U1>: NonZero,                                                 // split has more than one case
{
  pub name: String,
  pub consumes: PhantomData<CONSUMES>,
  pub requires_prior_completion: PhantomData<REQUIRES>,
  pub forbids_prior_completion: PhantomData<FORBIDS>,
  pub produces_and_accomplishes: PhantomData<PRODUCES_AND_ACCOMPLISHES>,
  pub handler: Box<dyn TypeSplitterBrickHandler<Length<PRODUCES_AND_ACCOMPLISHES>>>,
}

#[async_trait]
impl<CASES_LEN: Unsigned> SplitterBrickHandler for dyn TypeSplitterBrickHandler<CASES_LEN>
where
{
  async fn handle(&self, input: InputParams) -> anyhow::Result<SplitterOutput> {
    let result = self.handle(input).await?;
    anyhow::Ok(SplitterOutput(SplitIndex(result.0.get()), result.1))
  }
}

impl<
  CONSUMES: ParamBitSet,
  REQUIRES: Unsigned,
  FORBIDS: Unsigned,
  PRODUCES_AND_ACCOMPLISHES: CaseArray + Len,
> SplitterBrick<CONSUMES, REQUIRES, FORBIDS, PRODUCES_AND_ACCOMPLISHES>
where
  Length<PRODUCES_AND_ACCOMPLISHES>: Cmp<U1>,
  Length<PRODUCES_AND_ACCOMPLISHES>: IsGreaterPrivate<U1, <Length<PRODUCES_AND_ACCOMPLISHES> as Cmp<U1>>::Output>,
  Gr<Length<PRODUCES_AND_ACCOMPLISHES>, U1>: NonZero,
{
  pub(crate) fn to_internal(self) -> InternalSplitterBrick {
    InternalSplitterBrick {
      name: self.name,
      consumes: CONSUMES::get().0,
      produces: PRODUCES_AND_ACCOMPLISHES::get(),
      handler: Box::new(self.handler) as Box<dyn SplitterBrickHandler>,
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
