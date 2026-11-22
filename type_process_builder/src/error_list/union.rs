use crate::error_list::contains::CoprodContains;
use crate::error_list::{ErrorCoproduct, PreventDuplicateErrorUidInCoproduct, StepError};
use crate::{CNil, CoprodInjector, Coproduct};
use typenum::{B0, B1};

mod sealed {
  pub trait Sealed {}
  impl Sealed for typenum::B0 {}
  impl Sealed for typenum::B1 {}
}

pub trait IfNotKeepError<Head, Tail, Index>: sealed::Sealed {
  type Union: ErrorCoproduct;

  fn inject_head(head: Head) -> Self::Union;
  fn lift_tail(tail: Tail) -> Self::Union;
}

impl<Head: StepError, Tail: ErrorCoproduct + CoprodContains<Head>> IfNotKeepError<Head, Tail, ()> for B0
where
  <Tail as CoprodContains<Head>>::IsContained: PreventDuplicateErrorUidInCoproduct<Head>,
{
  type Union = Coproduct<Head, Tail>;

  #[inline(always)]
  fn inject_head(head: Head) -> Self::Union {
    Coproduct::Inl(head)
  }

  #[inline(always)]
  fn lift_tail(tail: Tail) -> Self::Union {
    Coproduct::Inr(tail)
  }
}

impl<Head: StepError, Tail: ErrorCoproduct + CoprodInjector<Head, Index>, Index> IfNotKeepError<Head, Tail, Index>
  for B1
{
  type Union = Tail;

  #[inline(always)]
  fn inject_head(head: Head) -> Self::Union {
    Tail::inject(head)
  }

  #[inline(always)]
  fn lift_tail(tail: Tail) -> Self::Union {
    tail
  }
}

pub trait CoprodUnion<RHS: ErrorCoproduct, Indices>: ErrorCoproduct {
  type Union: ErrorCoproduct;

  fn merge_lhs(self) -> Self::Union;
  fn merge_rhs(rhs: RHS) -> Self::Union;
}

impl<RHS: ErrorCoproduct> CoprodUnion<RHS, CNil> for CNil {
  type Union = RHS;

  #[inline(always)]
  fn merge_lhs(self) -> Self::Union {
    match self {}
  }

  #[inline(always)]
  fn merge_rhs(rhs: RHS) -> Self::Union {
    rhs
  }
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

  #[inline(always)]
  fn merge_lhs(self) -> Self::Union {
    match self {
      Coproduct::Inl(head) => {
        <<<Tail as CoprodUnion<RHS, TailIndices>>::Union as CoprodContains<Head>>::IsContained as IfNotKeepError<
          Head,
          <Tail as CoprodUnion<RHS, TailIndices>>::Union,
          HeadIndex,
        >>::inject_head(head)
      },
      Coproduct::Inr(tail) => {
        <<<Tail as CoprodUnion<RHS, TailIndices>>::Union as CoprodContains<Head>>::IsContained as IfNotKeepError<
          Head,
          <Tail as CoprodUnion<RHS, TailIndices>>::Union,
          HeadIndex,
        >>::lift_tail(tail.merge_lhs())
      },
    }
  }

  #[inline(always)]
  fn merge_rhs(rhs: RHS) -> Self::Union {
    <<<Tail as CoprodUnion<RHS, TailIndices>>::Union as CoprodContains<Head>>::IsContained as IfNotKeepError<
      Head,
      <Tail as CoprodUnion<RHS, TailIndices>>::Union,
      HeadIndex,
    >>::lift_tail(Tail::merge_rhs(rhs))
  }
}
