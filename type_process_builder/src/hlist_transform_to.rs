use frunk_core::hlist::{HCons, HNil, Plucker};

/// Trait for pulling out some subset of an HList, using type inference.
/// Like Sculptor, but ignores the remainder.
pub trait TransformTo<Target, Indices> {
  fn transform(self) -> Target;
}

/// Implementation for when the target is an empty HList (HNil)
///
/// Index type is HNil because we don't need an index for finding HNil
impl<Source> TransformTo<HNil, HNil> for Source {
  #[inline(always)]
  fn transform(self) -> HNil {
    HNil
  }
}

/// Implementation for when we have a non-empty HCons target
///
/// Indices are HCons<IndexHead, IndexTail> here because the compiler is being asked to figure out the
/// Index for Plucking the first item of type THead out of Self and the rest (IndexTail) is for the
/// Plucker's remainder induce.
impl<TargetHead, TargetTail, SourceHead, SourceTail, IndexHead, IndexTail>
  TransformTo<HCons<TargetHead, TargetTail>, HCons<IndexHead, IndexTail>> for HCons<SourceHead, SourceTail>
where
  HCons<SourceHead, SourceTail>: Plucker<TargetHead, IndexHead>,
  <HCons<SourceHead, SourceTail> as Plucker<TargetHead, IndexHead>>::Remainder: TransformTo<TargetTail, IndexTail>,
{
  #[inline(always)]
  fn transform(self) -> HCons<TargetHead, TargetTail> {
    let (head, remainder): (
      TargetHead,
      <HCons<SourceHead, SourceTail> as Plucker<TargetHead, IndexHead>>::Remainder,
    ) = self.pluck();
    let tail: TargetTail = remainder.transform();
    HCons { head, tail }
  }
}
