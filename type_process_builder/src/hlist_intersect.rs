// use frunk_core::hlist::{HCons, HList, HNil, Selector};
//
// // Trait to compute type-level intersection of two HLists
// pub trait TypeIntersection<Other> {
//   type Output;
// }
//
// // Base case: HNil intersected with anything is HNil
// impl<Other> TypeIntersection<Other> for HNil {
//   type Output = HNil;
// }
//
// // Recursive case: check if `Head` is in `Other`
// impl<Head, Tail, Idx, Other: Selector<Head, Idx>> TypeIntersection<Other> for HCons<Head, Tail>
// where
//   Tail: TypeIntersection<Other>,
//   IfContains<
//     Head,
//     Other,
//     HCons<Head, <Tail as TypeIntersection<Other>>::Output>,
//     <Tail as TypeIntersection<Other>>::Output,
//   >: HList,
// {
//   type Output = IfContains<
//     Head,
//     Other,
//     HCons<Head, <Tail as TypeIntersection<Other>>::Output>,
//     <Tail as TypeIntersection<Other>>::Output,
//   >;
// }
//
// // Helper trait to conditionally include type if it exists in Other
// pub type IfContains<Head, Other, Yes, No> = <SelectorHelper<Head, Other, Yes, No> as Select>::Result;
//
// pub trait Select {
//   type Result: HList;
// }
//
// pub struct SelectorHelper<Head, Other, Yes, No>(std::marker::PhantomData<(Head, Other, Yes, No)>);
//
// // If `Other: Selector<Head>` succeeds, use `Yes`
// impl<Head, Idx, Other: Selector<Head, Idx>, Yes: HList, No: HList> Select for SelectorHelper<Head, Other, Yes, No> {
//   type Result = Yes;
// }
