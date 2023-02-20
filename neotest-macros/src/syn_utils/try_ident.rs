//! This internal module defines a trait mechanism for accessing [`syn::Ident`]
//! values from parse types if they support it.
use syn::{FnArg, Pat};

use super::Ident;

/// A small utility for attempting to get [`syn::Ident`] values from parseable
/// objects.
///
/// If an ident is not available, this returns [`None`].
pub trait TryIdent {
  /// Return the reference to the underlying [`syn::Ident`], if possible.
  ///
  /// Returns [`None`] if not.
  fn try_ident(&self) -> Option<&syn::Ident>;
}

/// Any type that implements [`Ident`] must also logically implement
/// [`TryIdent`] in a manner that cannot fail.
impl<T> TryIdent for T
where
  T: Ident,
{
  fn try_ident(&self) -> Option<&syn::Ident> {
    Some(self.ident())
  }
}

/// [`Pat`] objects only have a [`syn::Ident`] value if the pattern is a [`Pat::Ident`].
impl TryIdent for Pat {
  fn try_ident(&self) -> Option<&syn::Ident> {
    match self {
      Pat::Ident(ident) => Some(&ident.ident),
      _ => None,
    }
  }
}

/// [`FnArg`] objects only have a [`syn::Ident`] value if the argument is a
/// [`FnArg::Typed`] pattern that contains one.
impl TryIdent for FnArg {
  fn try_ident(&self) -> Option<&syn::Ident> {
    match self {
      FnArg::Typed(v) => v.pat.try_ident(),
      _ => None,
    }
  }
}
