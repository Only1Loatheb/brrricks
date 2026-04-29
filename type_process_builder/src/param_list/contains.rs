use crate::param_list::ParamValue;
use frunk_core::hlist::{HCons, HNil};
use std::ops::BitOr;
use typenum::{B0, Bit, IsEqual};

pub trait Contains<Needle: ParamValue> {
  type IsContained: Bit;
}

impl<Needle: ParamValue> Contains<Needle> for HNil {
  type IsContained = B0;
}

impl<Needle: ParamValue, Head: ParamValue, Tail: Contains<Needle>> Contains<Needle> for HCons<Head, Tail>
where
  Needle::UID: IsEqual<Head::UID>,
  <Needle::UID as IsEqual<Head::UID>>::Output: BitOr<<Tail as Contains<Needle>>::IsContained>,
  <<Needle::UID as IsEqual<Head::UID>>::Output as BitOr<<Tail as Contains<Needle>>::IsContained>>::Output: Bit,
{
  type IsContained =
    <<Needle::UID as IsEqual<Head::UID>>::Output as BitOr<<Tail as Contains<Needle>>::IsContained>>::Output;
}
