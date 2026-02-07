use frunk_core::hlist::{HCons, HNil, Plucker};

pub trait CloneJust<Target, Indices> {
  fn clone_just(self) -> Target;
}

impl<Source> CloneJust<HNil, HNil> for &Source {
  #[inline(always)]
  fn clone_just(self) -> HNil {
    HNil
  }
}

impl<'a, TargetHead: Clone + 'a, TargetTail, SourceHead: 'a, SourceTail: 'a, IndexHead, IndexTail>
  CloneJust<HCons<TargetHead, TargetTail>, HCons<IndexHead, IndexTail>> for &'a HCons<SourceHead, SourceTail>
where
  &'a HCons<SourceHead, SourceTail>: Plucker<&'a TargetHead, IndexHead>,
  <&'a HCons<SourceHead, SourceTail> as Plucker<&'a TargetHead, IndexHead>>::Remainder:
    CloneJust<TargetTail, IndexTail>,
{
  #[inline(always)]
  fn clone_just(self) -> HCons<TargetHead, TargetTail> {
    let (head, remainder): (
      &'a TargetHead,
      <&'a HCons<SourceHead, SourceTail> as Plucker<&'a TargetHead, IndexHead>>::Remainder,
    ) = self.pluck();
    let tail = remainder.clone_just();
    HCons {
      head: head.clone(),
      tail,
    }
  }
}
