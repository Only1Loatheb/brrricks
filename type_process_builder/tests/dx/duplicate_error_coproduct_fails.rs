use thiserror::Error;
use type_process_builder::error_list::ErrorCoproduct;
use type_process_builder::{Coprod, impl_step_error};
use typenum::*;

#[derive(Error, Debug)]
#[error("error A")]
struct ErrorA;

impl_step_error!(ErrorA => U0);

fn assert_error_coproduct<T: ErrorCoproduct>() {}

fn main() {
  assert_error_coproduct::<Coprod!(ErrorA, ErrorA)>();
}
