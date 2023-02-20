//! This internal module defines a series of utilities for working with [`syn`].
//!
//! This utilities enable better access to certain symbol types, as well as
//! easier mechanisms for quoting.
mod contains_ident;
mod ident;
mod inner;
mod try_ident;
mod type_sequence;

pub use contains_ident::*;
pub use ident::*;
pub use inner::*;
pub use try_ident::*;
pub use type_sequence::*;
