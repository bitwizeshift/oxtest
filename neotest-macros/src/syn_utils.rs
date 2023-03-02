//! This internal module defines a series of utilities for working with [`syn`].
//!
//! This utilities enable better access to certain symbol types, as well as
//! easier mechanisms for quoting.
mod contains_ident;
mod fn_arg;
mod function_call;
mod function_definition;
mod ident;
mod inner;
mod module_definition;
mod try_ident;
mod type_sequence;

// Re-export all submodule contents.

#[doc(inline)]
pub use contains_ident::*;
#[doc(inline)]
pub use fn_arg::*;
#[doc(inline)]
pub use function_call::*;
#[doc(inline)]
pub use function_definition::*;
#[doc(inline)]
pub use ident::*;
#[doc(inline)]
pub use inner::*;
#[doc(inline)]
pub use module_definition::*;
#[doc(inline)]
pub use try_ident::*;
#[doc(inline)]
pub use type_sequence::*;
