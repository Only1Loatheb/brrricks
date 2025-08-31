use frunk_core::hlist::{HCons, HList, HNil, Plucker};
use frunk_core::{hlist, HList};

// ---------- Type-level booleans ----------
pub trait Bool {}
pub struct True;
pub struct False;
impl Bool for True {}
impl Bool for False {}

// ---------- Compute Contains<M, H> at the type level ----------
pub trait Contains<M: HList, H> {
  type Out: Bool;
}

// Base: empty M => not found
impl<H> Contains<HNil, H> for () {
  type Out = False;
}

// If head == H, found
impl<T, H> Contains<HCons<H, T>, H> for ()
where
  T: HList,
{
  type Out = True;
}

// Otherwise, search tail
impl<HH, T, H, Flag> Contains<HCons<HH, T>, H> for ()
where
  T: HList,
  (): Contains<T, H, Out = Flag>,
  Flag: Bool,
  // Prevent overlap with the "head == H" impl:
  // This impl only applies when HH != H, which we encode
  // structurally by not matching H in the head position.
{
  type Out = Flag;
}

// ---------- Intersection<L, M> (public facade) ----------
pub trait Intersection<L: HList, M: HList> {
  type Out: HList;
  fn apply(l: L) -> Self::Out;
}

// Delegate to Intersect<L, M, Flag>
impl<L, M, Flag, O> Intersection<L, M> for ()
where
  L: HList,
  M: HList,
  (): Intersect<L, M, Flag, Out = O>,
  O: HList,
  // compute Flag = Contains<M, Head(L)> recursively inside Intersect
{
  type Out = O;
  fn apply(l: L) -> Self::Out {
    <() as Intersect<_, _, Flag>>::apply(l)
  }
}

// ---------- Internal worker with an explicit Flag ----------
pub trait Intersect<L: HList, M: HList, Flag: Bool> {
  type Out: HList;
  fn apply(l: L) -> Self::Out;
}

// Base: ∅ ∩ M = ∅
impl<M: HList> Intersect<HNil, M, False> for () {
  type Out = HNil;
  fn apply(_: HNil) -> Self::Out {
    HNil
  }
}
impl<M: HList> Intersect<HNil, M, True> for () {
  type Out = HNil;
  fn apply(_: HNil) -> Self::Out {
    HNil
  }
}

// Case 1: head is NOT in M (Flag = False)
// (H :: T) ∩ M = T ∩ M
impl<H, T, M, NextFlag, O> Intersect<HCons<H, T>, M, False> for ()
where
  T: HList,
  M: HList,
  // recompute membership for next step:
  (): Contains<M, HeadOf<T>, Out = NextFlag>,
  (): Intersect<T, M, NextFlag, Out = O>,
  O: HList,
  // utility:
  HeadOf<T>: ?Sized,
{
  type Out = O;
  fn apply(l: HCons<H, T>) -> Self::Out {
    // drop head, recurse on tail
    <() as Intersect<_, _, NextFlag>>::apply(l.tail)
  }
}

// Case 2: head IS in M (Flag = True)
// (H :: T) ∩ M = H :: (T ∩ (M - H))
// We use Plucker to "consume" one H from M.
impl<H, T, M, MR, NextFlag, O> Intersect<HCons<H, T>, M, True> for ()
where
  T: HList,
  M: HList + Plucker<H, MR>,
  MR: HList,
  (): Contains<MR, HeadOf<T>, Out = NextFlag>,
  (): Intersect<T, MR, NextFlag, Out = O>,
  O: HList,
  HeadOf<T>: ?Sized,
{
  type Out = HCons<H, O>;
  fn apply(l: HCons<H, T>) -> Self::Out {
    HCons {
      head: l.head,
      tail: <() as Intersect<_, _, NextFlag>>::apply(l.tail),
    }
  }
}

// ---------- Small helper: HeadOf<T> at the type level ----------
pub trait HeadTy {
  type Head;
}
impl HeadTy for HNil {
  type Head = ();
} // dummy
impl<H, T> HeadTy for HCons<H, T> {
  type Head = H;
}
type HeadOf<T> = <T as HeadTy>::Head;

// ---------- Demo ----------
pub type IntersectOut<L, M> = <() as Intersection<L, M>>::Out;

fn main() {
  let l = hlist![42u32, "hello", true];
  type L = HList!(u32, &'static str, bool);
  type M = HList!(bool, f64, u32);

  type R = IntersectOut<L, M>; // Hlist!(u32, bool)
  let out: R = <() as Intersection<_, _>>::apply(l);

  assert_eq!(out.head, 42u32);
  assert_eq!(out.tail.head, true);
}
