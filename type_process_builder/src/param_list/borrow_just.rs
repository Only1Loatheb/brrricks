use crate::frunk::to_ref::ToRef;
use crate::param_list::extract::Extract;

pub trait BorrowJust<'a, Target: ToRef<'a>, Indices> {
  fn borrow_just(self) -> <Target as ToRef<'a>>::Ref;
}

impl<'a, Source: ToRef<'a>, Target: ToRef<'a>, Indices> BorrowJust<'a, Target, Indices> for &'a Source
where
  <Source as ToRef<'a>>::Ref: Extract<<Target as ToRef<'a>>::Ref, Indices>,
{
  #[inline(always)]
  fn borrow_just(self) -> <Target as ToRef<'a>>::Ref {
    self.to_ref().extract()
  }
}
