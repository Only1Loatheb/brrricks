use std::marker::PhantomData;

// https://www.reddit.com/r/rust/comments/108z8pl/comment/j3vrsp1/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
pub(crate) type Invariant<'a> = PhantomData<fn(&'a ()) -> &'a ()>;
