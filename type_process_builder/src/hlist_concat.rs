use frunk_core::hlist::{HCons, HList, HNil};

pub trait Concat<RHS: HList>: HList {
    type Concatenated: HList;

    fn concat(self, rhs: RHS) -> Self::Concatenated;
}

// Base case: Concatenating HNil with another HList results in the other HList
impl<RHS: HList> Concat<RHS> for HNil {
    type Concatenated = RHS;

    fn concat(self, rhs: RHS) -> Self::Concatenated {
        rhs
    }
}

// Recursive case: Concatenate HCons
impl<Head, Tail, RHS> Concat<RHS> for HCons<Head, Tail>
where
    Tail: Concat<RHS>,
    RHS: HList,
{
    type Concatenated = HCons<Head, <Tail as Concat<RHS>>::Concatenated>;

    fn concat(self, rhs: RHS) -> Self::Concatenated {
        HCons {
            head: self.head,
            tail: self.tail.concat(rhs),
        }
    }
}
