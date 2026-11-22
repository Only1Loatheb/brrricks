//! Type-safe process modelling library with compile-time flow and parameter invariants.

pub mod builder;
pub mod frunk;
pub mod param_list;
pub mod step;

mod sealed {
  pub trait Sealed {}
}

pub use frunk::coproduct::{CNil, CoprodInjector, Coproduct};
pub use frunk::hlist::{HCons, HList, HNil};
pub use frunk::plucker::{Here, Plucker, There};
pub use frunk::to_ref::ToRef;

// cargo doc --no-deps --package type_process_builder --features docs
/// View the diagrams
#[cfg(feature = "docs")]
#[doc = simple_mermaid::mermaid!("../doc/brrricks_app_session_flow.mmd")]
#[doc = simple_mermaid::mermaid!("../doc/process_builder_states.mmd")]
pub mod documentation_diagrams {}
