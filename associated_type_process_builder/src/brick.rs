use std::marker::PhantomData;
use std::ops::*;

use typenum::*;
use typenum::private::IsGreaterPrivate;
use async_trait::async_trait;
use frunk_core::coproduct::{CNil, Coproduct};
use frunk_core::hlist::{HList, HNil, HCons};
use process::brick_domain::*;
use process::internal_brick::*;
use crate::invariant::Invariant;
use crate::split_index::TypeSplitIndex;

// #[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]

pub struct ParamRepr<'same_process, PARAM_VALUE> {
  pub(crate) same_process_invariant: Invariant<'same_process>,
  pub(crate) param_value: PhantomData<PARAM_VALUE>,
  pub(crate) param_id: ParamId,
}

pub trait ParamReprList<'same_process>: HList {
  fn get_param_ids(self) -> Vec<ParamId>;
  type VALUE: HList;
}

impl ParamReprList for HNil {
  fn get_param_ids(self) -> Vec<ParamId> {
    vec![]
  }
  type VALUE = HNil;
}

impl<'same_process, PARAM_VALUE, TAIL: ParamReprList> ParamReprList<'same_process>
for HCons<ParamRepr<'same_process, PARAM_VALUE>, TAIL> {
  fn get_param_ids(self) -> Vec<ParamId> {
    let mut param_ids = self.tail.get_param_ids();
    param_ids.push(self.head.param_id);
    param_ids
  }
  type VALUE = HCons<PARAM_VALUE, TAIL::VALUE>;
}

#[async_trait]
pub trait TypeLinearBrickHandler<
  'same_process,
  CONSUMES: ParamReprList<'same_process>,
  PRODUCES: ParamReprList<'same_process>,
>: Send + Sync {
  async fn handle(&self, input: CONSUMES::VALUE) -> anyhow::Result<PRODUCES::VALUE>;
}

/// We can add a list of completed actions later
pub struct LinearBrick<'same_process, CONSUMES: ParamReprList<'same_process>, PRODUCES: ParamReprList<'same_process>> {
  pub name: String,
  pub produces: PRODUCES,
  pub handler: Box<dyn TypeLinearBrickHandler<'same_process, CONSUMES, PRODUCES>>,
}

/// this is bad
/// Let's hope rust will optimise frunk::coproduct::Coproduct just fine :copium:
pub trait SplitterReprCase<'same_process> {
  fn get_param_ids(self) -> Vec<ParamId>;
}

impl<'same_process, INL: ParamReprList<'same_process>> SplitterReprCase<'same_process> for Coproduct<INL, CNil> {
  fn get_param_ids(self) -> Vec<ParamId> {
    match self {
      Coproduct::Inl(inl) => inl.get_param_ids(),
      Coproduct::Inr(_) => panic!(),
    }
  }
}

impl<'same_process, INL: ParamReprList<'same_process>, INR: SplitterReprCase<'same_process>> SplitterReprCase<'same_process>
for Coproduct<INL, INR> {
  fn get_param_ids(self) -> Vec<ParamId> {
    match self {
      Coproduct::Inl(inl) => inl.get_param_ids(),
      Coproduct::Inr(inr) => inr.get_param_ids(),
    }
  }
}

/// consider const generics or Frunk to replace CaseArray
#[async_trait]
pub trait TypeSplitterBrickHandler<'same_process>: Send + Sync {
  async fn handle(&self, input: InputParams) -> anyhow::Result<SplitterReprCase<'same_process>>;
}

/// We can add a list of completed actions later
pub struct SplitterBrick<'same_process, PRODUCES: ParamReprList<'same_process>>
where
  <PRODUCES_AND_ACCOMPLISHES as Len>::Output: Cmp<U1>,
  <PRODUCES_AND_ACCOMPLISHES as Len>::Output: IsGreaterPrivate<U1, <<PRODUCES_AND_ACCOMPLISHES as Len>::Output as Cmp<U1>>::Output>,
  Gr<Length<PRODUCES_AND_ACCOMPLISHES>, U1>: NonZero,                                                 // split has more than one case
{
  pub name: String,
  pub uses: PhantomData<USES>,
  pub requires_prior_completion: PhantomData<REQUIRES>,
  pub forbids_prior_completion: PhantomData<FORBIDS>,
  pub produces_and_accomplishes: PhantomData<PRODUCES_AND_ACCOMPLISHES>,
  pub handler: Box<dyn TypeSplitterBrickHandler<Length<PRODUCES_AND_ACCOMPLISHES>>>,
}

struct TypeSplitterBrickHandlerAdapter<CASES_LEN: Unsigned> {
  inner: Box<dyn TypeSplitterBrickHandler<CASES_LEN>>,
}

impl<CASES_LEN: Unsigned> TypeSplitterBrickHandlerAdapter<CASES_LEN> {
  fn new(inner: Box<dyn TypeSplitterBrickHandler<CASES_LEN>>) -> Self {
    Self { inner }
  }
}

#[async_trait]
impl<CASES_LEN: Unsigned> SplitterBrickHandler for TypeSplitterBrickHandlerAdapter<CASES_LEN> {
  async fn handle(&self, input: InputParams) -> anyhow::Result<SplitterOutput> {
    let result = self.inner.handle(input).await?;
    anyhow::Ok(SplitterOutput(SplitIndex(result.0.get()), result.1))
  }
}

impl<
  USES: ParamBitSet,
  REQUIRES: Unsigned,
  FORBIDS: Unsigned,
  PRODUCES_AND_ACCOMPLISHES: CaseArray + Len,
> SplitterBrick<USES, REQUIRES, FORBIDS, PRODUCES_AND_ACCOMPLISHES>
where
  Length<PRODUCES_AND_ACCOMPLISHES>: Cmp<U1>,
  Length<PRODUCES_AND_ACCOMPLISHES>: IsGreaterPrivate<U1, <Length<PRODUCES_AND_ACCOMPLISHES> as Cmp<U1>>::Output>,
  Gr<Length<PRODUCES_AND_ACCOMPLISHES>, U1>: NonZero,
{
  pub(crate) fn to_internal(self) -> InternalSplitterBrick {
    InternalSplitterBrick {
      name: self.name,
      uses: USES::get().0,
      produces: PRODUCES_AND_ACCOMPLISHES::get(),
      handler: Box::new(TypeSplitterBrickHandlerAdapter::new(self.handler)),
    }
  }
}

pub struct FinalBrick {
  pub name: String,
  pub handler: Box<dyn FinalBrickHandler>,
}

impl FinalBrick {
  pub(crate) fn to_internal(self) -> InternalFinalBrick {
    InternalFinalBrick {
      name: self.name,
      uses: USES::get().0,
      handler: self.handler,
    }
  }
}
