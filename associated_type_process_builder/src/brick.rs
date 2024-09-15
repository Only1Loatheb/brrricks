use std::marker::PhantomData;
use std::ops::*;

use typenum::*;
use typenum::private::IsGreaterPrivate;
use async_trait::async_trait;

use process::brick_domain::*;
use process::internal_brick::*;
use crate::invariant::Invariant;
use crate::split_index::TypeSplitIndex;

// #[derive(PartialEq, Debug, Eq, Clone, Copy, PartialOrd, Ord, Hash)]


pub trait ParamValue {
  fn name() -> ParamId;
}

pub struct ProcessParam<'same_process, PARAM_VALUE: ParamValue> {
  pub(crate) param_value: PARAM_VALUE,
  pub(crate) same_process_invariant: Invariant<'same_process>,
}

pub struct LinearBrick<PARAM_VALUE> {
  pub name: String,
  pub produces: PhantomData<PARAM_VALUE>,
  pub handler: Box<dyn LinearBrickHandler>,
}

impl<PARAM_VALUE> LinearBrick<PARAM_VALUE> {
  pub(crate) fn to_internal(self, uses: ParamId) -> InternalLinearBrick {
    InternalLinearBrick {
      name: self.name,
      uses: vec![uses],
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
  USES: ParamBitSet,
  REQUIRES: Unsigned,
  FORBIDS: Unsigned,
  PRODUCES_AND_ACCOMPLISHES: CaseArray + Len,
> where
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
