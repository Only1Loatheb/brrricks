use crate::error_coproduct::contains::CoprodContains;
use crate::error_coproduct::{ErrorCoproduct, PreventDuplicateErrorUidInCoproduct, StepError};
use crate::{CNil, CoprodInjector, Coproduct};
use typenum::{B0, B1};

mod sealed {
  pub trait Sealed {}
  impl Sealed for typenum::B0 {}
  impl Sealed for typenum::B1 {}
}

pub trait IfNotKeepError<Head, Tail, Index>: sealed::Sealed {
  type Union: ErrorCoproduct;
}

impl<Head: StepError, Tail: ErrorCoproduct + CoprodContains<Head>> IfNotKeepError<Head, Tail, ()> for B0
where
  <Tail as CoprodContains<Head>>::IsContained: PreventDuplicateErrorUidInCoproduct<Head>,
{
  type Union = Coproduct<Head, Tail>;
}

impl<Head: StepError, Tail: ErrorCoproduct + CoprodInjector<Head, Index>, Index> IfNotKeepError<Head, Tail, Index>
  for B1
{
  type Union = Tail;
}

pub trait CoprodUnion<RHS: ErrorCoproduct, Indices>: ErrorCoproduct {
  type Union: ErrorCoproduct;
}

impl<RHS: ErrorCoproduct> CoprodUnion<RHS, CNil> for CNil {
  type Union = RHS;
}

impl<
  Head: StepError,
  Tail: CoprodUnion<RHS, TailIndices> + ErrorCoproduct + CoprodContains<Head>,
  RHS: ErrorCoproduct,
  HeadIndex,
  TailIndices,
> CoprodUnion<RHS, Coproduct<HeadIndex, TailIndices>> for Coproduct<Head, Tail>
where
  <Tail as CoprodContains<Head>>::IsContained: PreventDuplicateErrorUidInCoproduct<Head>,
  <Tail as CoprodUnion<RHS, TailIndices>>::Union: CoprodContains<Head>,
  <<Tail as CoprodUnion<RHS, TailIndices>>::Union as CoprodContains<Head>>::IsContained:
    IfNotKeepError<Head, <Tail as CoprodUnion<RHS, TailIndices>>::Union, HeadIndex>,
{
  type Union =
    <<<Tail as CoprodUnion<RHS, TailIndices>>::Union as CoprodContains<Head>>::IsContained as IfNotKeepError<
      Head,
      <Tail as CoprodUnion<RHS, TailIndices>>::Union,
      HeadIndex,
    >>::Union;
}
