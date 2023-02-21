//! This module contains the definition for all input parseable types used on
//! the base [`neotest`] attribute
//!
//! [`neotest`]: crate::neotest

mod const_parameter_input;
mod fixture_input;
mod parameter_input;
mod test_inputs;
mod test_option;
mod type_parameter_input;

// Re-export all submodule contents.

#[doc(inline)]
pub use const_parameter_input::*;
#[doc(inline)]
pub use fixture_input::*;
#[doc(inline)]
pub use parameter_input::*;
#[doc(inline)]
pub use test_inputs::*;
#[doc(inline)]
pub use test_option::*;
#[doc(inline)]
pub use type_parameter_input::*;
