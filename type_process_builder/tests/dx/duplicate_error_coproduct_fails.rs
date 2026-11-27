//@require-annotations-for-level: ERROR
use std::fmt;
use type_process_builder::error_coproduct::ErrorCoproduct;
use type_process_builder::{Coprod, impl_step_error};
use typenum::*;

#[derive(Debug)]
struct ErrorA;

impl fmt::Display for ErrorA {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str("error A")
  }
}

impl std::error::Error for ErrorA {}

impl_step_error!(ErrorA => U0);

fn assert_error_coproduct<T: ErrorCoproduct>() {}

fn main() {
  assert_error_coproduct::<Coprod!(ErrorA, ErrorA)>();
  //~^ ERROR: cannot include error: duplicate error UID for `ErrorA` found in ErrorCoproduct
  //~| NOTE: Error `ErrorA` appears multiple times in the error coproduct, or multiple errors share the same UID
}
