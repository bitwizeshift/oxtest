//! A crate contain common contents for the neotest test framework.
//!
//! This provides some of the essential logic, such as:
//!
//! * The [`Result`] and [`TestResult`] types for returning errors from tests,
//! * The [`Error`] type for generic representation of any failures,
//! * The [`Fixture`] trait, required for fixture-based logic, and
//! * Various pieces of internal utilities needed to make this framework operate.
//!
//! This crate should never be depended on directly. The base `neotest` crate
//! should be used instead.
mod context;
mod fixture;
mod result;

#[doc(inline)]
pub use fixture::*;

#[doc(inline)]
pub use result::*;

/// An internal module that contains implementation-details required for
/// creating test suites.
///
/// This should never be used directly by consumers, as the implementation
/// provides no stability guarantees (and never will, since this limits the
/// implementation-freedom).
#[doc(hidden)]
pub mod __internal {
  #[doc(hidden, inline)]
  pub use super::context::*;
}
