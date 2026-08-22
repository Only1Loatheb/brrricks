use serde::{Deserialize, Serialize};
use type_process_builder::param_list::ParamList;
use type_process_builder::{hlist, impl_param_value};

use typenum::*;

#[derive(Deserialize, Serialize)]
struct ParamA;

impl_param_value!(ParamA => U0);

fn assert_param_list<T: ParamList>(_val: T) {}

fn main() {
    let list = hlist!(ParamA, ParamA);
    assert_param_list(list);
}
