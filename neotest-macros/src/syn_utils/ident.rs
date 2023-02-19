//! This internal module defines a simple trait for accessing [`syn::Ident`]
//! from different symbols.
use syn::{GenericParam, Signature};

/// A small utility trait for helping to extract references to [`syn::Ident`] values
/// from parseable objects.
pub trait Ident {
  fn ident(&self) -> &syn::Ident;
}

impl Ident for syn::Ident {
  fn ident(&self) -> &syn::Ident {
    &self
  }
}

impl<'a> Ident for &'a syn::Ident {
  fn ident(&self) -> &syn::Ident {
    self
  }
}

/// [`Signature`] objects define idents for the function name.
impl Ident for Signature {
  fn ident(&self) -> &syn::Ident {
    &self.ident
  }
}

/// [`GenericParam`] objects define idents in either the Type, Const, or
/// Lifetime arguments.
impl Ident for GenericParam {
  fn ident(&self) -> &syn::Ident {
    match self {
      GenericParam::Type(v) => &v.ident,
      GenericParam::Const(v) => &v.ident,
      GenericParam::Lifetime(v) => &v.lifetime.ident,
    }
  }
}
