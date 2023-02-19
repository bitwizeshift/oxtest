//! This internal module defines a series of utilities for working with [`syn`].
//!
//! This utilities enable better access to certain symbol types, as well as
//! easier mechanisms for quoting.
mod ident;
mod inner;
mod try_ident;

pub(crate) use ident::*;
pub(crate) use inner::*;
pub(crate) use try_ident::*;
