use crate::frunk::hlist::{HCons, HNil};
use crate::frunk::plucker::Plucker;

/// Trait for pulling out some subset of an `HList`, using type inference.
/// Like `Sculptor`, but ignores the remainder.
pub trait Extract<Target, Indices>: crate::sealed::Sealed {
  type Remainder;
  fn extract(self) -> Target;
  fn extract_with_remainder(self) -> (Target, Self::Remainder);
}

/// Implementation for when the target is an empty `HList` (`HNil`)
///
/// Index type is `HNil` because we don't need an index for finding `HNil`
impl<Source: crate::sealed::Sealed> Extract<HNil, HNil> for Source {
  type Remainder = Source;

  #[inline(always)]
  fn extract(self) -> HNil {
    HNil
  }

  #[inline(always)]
  fn extract_with_remainder(self) -> (HNil, Source) {
    (HNil, self)
  }
}

/// Implementation for when we have a non-empty `HCons` target
///
/// Indices are `HCons`<`IndexHead`, `IndexTail`> here because the compiler is being asked to figure out the
/// Index for Plucking the first item of type `THead` out of Self and the rest (`IndexTail`) is for the
/// Plucker's remainder induce.
impl<TargetHead, TargetTail, SourceHead, SourceTail, IndexHead, IndexTail>
  Extract<HCons<TargetHead, TargetTail>, HCons<IndexHead, IndexTail>> for HCons<SourceHead, SourceTail>
where
  HCons<SourceHead, SourceTail>: Plucker<TargetHead, IndexHead>,
  <HCons<SourceHead, SourceTail> as Plucker<TargetHead, IndexHead>>::Remainder: Extract<TargetTail, IndexTail>,
{
  type Remainder = <<HCons<SourceHead, SourceTail> as Plucker<TargetHead, IndexHead>>::Remainder as Extract<
    TargetTail,
    IndexTail,
  >>::Remainder;

  #[inline(always)]
  fn extract(self) -> HCons<TargetHead, TargetTail> {
    self.extract_with_remainder().0
  }

  #[inline(always)]
  fn extract_with_remainder(self) -> (HCons<TargetHead, TargetTail>, Self::Remainder) {
    let (head, remainder): (TargetHead, <HCons<SourceHead, SourceTail> as Plucker<TargetHead, IndexHead>>::Remainder) =
      self.pluck();
    let (tail, final_remainder): (TargetTail, Self::Remainder) = remainder.extract_with_remainder();
    (HCons { tail, head }, final_remainder)
  }
}
