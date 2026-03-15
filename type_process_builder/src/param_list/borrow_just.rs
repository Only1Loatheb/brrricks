use frunk_core::hlist::{HCons, HNil, Plucker};
use frunk_core::traits::ToRef;

pub trait BorrowJust<'a, Target: ToRef<'a>, Indices> {
  fn borrow_just(&'a self) -> <Target as ToRef<'a>>::Output;
}

impl<'a, Source> BorrowJust<'a, HNil, HNil> for Source {
  #[inline(always)]
  fn borrow_just(&'a self) -> HNil {
    HNil
  }
}

impl<'a, TargetHead: 'a, TargetTail: ToRef<'a>, SourceHead: 'a, SourceTail: 'a, IndexHead: 'a, IndexTail>
BorrowJust<'a, HCons<TargetHead, TargetTail>, HCons<IndexHead, IndexTail>> for HCons<SourceHead, SourceTail>
where
  &'a HCons<SourceHead, SourceTail>: Plucker<&'a TargetHead, IndexHead>,
  <&'a HCons<SourceHead, SourceTail> as Plucker<&'a TargetHead, IndexHead>>::Remainder:
  BorrowJust<'a, TargetTail, IndexTail>,
{
  #[inline(always)]
  fn borrow_just(&'a self) -> <HCons<TargetHead, TargetTail> as ToRef<'a>>::Output {
    let (head, remainder) = self.pluck();
    HCons { head, tail: remainder.borrow_just() }
  }
}
