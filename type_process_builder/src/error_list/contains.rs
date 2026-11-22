use crate::error_list::StepError;
use crate::{CNil, Coproduct};
use std::ops::BitOr;
use typenum::{B0, Bit, IsEqual};

pub trait CoprodContains<Needle: StepError>: crate::sealed::Sealed {
  type IsContained: Bit;
}

impl<Needle: StepError> CoprodContains<Needle> for CNil {
  type IsContained = B0;
}

impl<Needle: StepError, Head: StepError, Tail: CoprodContains<Needle>> CoprodContains<Needle> for Coproduct<Head, Tail>
where
  Needle::UID: IsEqual<Head::UID>,
  <Needle::UID as IsEqual<Head::UID>>::Output: BitOr<<Tail as CoprodContains<Needle>>::IsContained>,
  <<Needle::UID as IsEqual<Head::UID>>::Output as BitOr<<Tail as CoprodContains<Needle>>::IsContained>>::Output: Bit,
{
  type IsContained =
    <<Needle::UID as IsEqual<Head::UID>>::Output as BitOr<<Tail as CoprodContains<Needle>>::IsContained>>::Output;
}
