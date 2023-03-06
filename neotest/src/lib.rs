//! Neotest is an enhanced Rust testing framework
//!
//! This project builds off of Rust's built-in `test` framework by providing
//! various quality-of-life improvements.
//!
//! # Features
//!
//! * Test-fixtures with customized prepartion via [`Fixture::prepare`].
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
pub use neotest_macros::{neotest, subtest, Fixture};

#[doc(inline)]
pub use neotest_common::{Fixture, Result};

#[doc(hidden, inline)]
pub use neotest_common::__internal;
