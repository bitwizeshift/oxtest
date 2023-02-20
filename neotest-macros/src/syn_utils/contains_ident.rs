//! This internal module defines a trait for determining whether collections
//! contain the desired [`syn::Ident`].
use syn::{punctuated::Punctuated, token::Comma, FnArg, GenericParam};

use super::TryIdent;

/// A small trait to help detecting arguments containing the specified identifier
/// name.
pub trait ContainsIdent {
  /// Tests whether `self` contains the specified [`syn::Ident`], returning `true`
  /// if its found.
  ///
  /// # Arguments
  ///
  /// * `ident` - the [`syn::Ident`] to test for existence of.
  fn contains_ident(&self, ident: &syn::Ident) -> bool;
}

impl ContainsIdent for Punctuated<FnArg, Comma> {
  fn contains_ident(&self, ident: &syn::Ident) -> bool {
    self
      .iter()
      .any(|v| v.try_ident().map(ToString::to_string) == Some(ident.to_string()))
  }
}

impl ContainsIdent for Punctuated<GenericParam, Comma> {
  fn contains_ident(&self, ident: &syn::Ident) -> bool {
    self
      .iter()
      .any(|v| v.try_ident().map(ToString::to_string) == Some(ident.to_string()))
  }
}
