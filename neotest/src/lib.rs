//! Neotest is an enhanced Rust testing framework
//!
//! This project builds off of Rust's built-in `test` framework by providing
//! various quality-of-life improvements.
//!
//! # Features
//!
//! * Test-fixtures with customized [`Fixture::set_up`] and [`Fixture::tear_down`]
//!   functionality.
//! * Parameterized test-cases created with combinatorial inputs, which creates
//!   an easy mechanisms to set up multiple inputs
//! * Parameterized generic test-cases from either types or const inputs
//! * Sub-tests that each execute independently, allowing for multiple individual
//!   test-cases that can be run per individual test.
//!
//! # Motivation
//!
//! The current state of testing frameworks is... interesting.
#[doc(inline)]
pub use neotest_macros::{neotest, Fixture};

#[doc(inline)]
pub use neotest_common::{Fixture, Result};

// Internal stuff

#[doc(hidden)]
pub mod __internal {
  #[doc(hidden)]
  pub use neotest_common::__Context;
}
