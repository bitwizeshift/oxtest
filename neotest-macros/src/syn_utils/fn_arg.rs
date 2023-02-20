//! This internal module defines some utilities for "resolving" function arguments
//! with a given identifier.
use quote::ToTokens;
use syn::token::{And, Mut};
use syn::{FnArg, Type};

/// A helper function for getting how to resolve the specified [`FnArg`]
///
/// This extracts the ref-ness and mutability of the underlying type, and
/// returns it.
///
/// This is an implementation-function that is used for [`ResolveFnArg`] and
/// [`ResolveFnArgDecl`].
///
/// # Arguments
///
/// * `arg` - the [`FnArg`] to match the call structure of
fn resolve_arg(arg: &FnArg) -> ResolveType {
  match &arg {
    FnArg::Receiver(ref rec) => {
      let and = rec.reference.as_ref().map(|ref v| v.0);
      let mutability = rec.mutability.as_ref();
      if and.is_some() {
        ResolveType::Reference(and.unwrap(), mutability.copied())
      } else {
        ResolveType::Owner
      }
    }
    FnArg::Typed(ref pat_type) => match &*pat_type.ty {
      Type::Reference(ref type_ref) => {
        let and = type_ref.and_token;
        let mutability = type_ref.mutability.as_ref();
        ResolveType::Reference(and, mutability.copied())
      }
      _ => ResolveType::Owner,
    },
  }
}

/// The type of a function argument that can be resolved.
///
/// This is an implementation datatype returned from [`resolve_arg`], which is
/// used for extracting the refness and mutability so that function arguments
/// can be called correctly in [`ResolveFnArg`] and [`ResolveFnArgDecl`].
enum ResolveType {
  Reference(And, Option<Mut>),
  Owner,
}

/// This is a utility type used to allow a given [`syn::Ident`] to be tokenized
/// as an expression that would resolve a given function arg.
///
/// This effectively enables matching the mutability and refness of the function.
///
/// [`syn::Ident`]: struct@syn::Ident
pub struct ResolveFnArg<'a> {
  and: Option<And>,
  mutability: Option<Mut>,
  ident: &'a syn::Ident,
}

impl<'a> ResolveFnArg<'a> {
  /// Creates a new [`ResolveFnArg`] by matching the ref and mutability of the
  /// specified [`FnArg`]
  ///
  /// # Arguments
  ///
  /// * `ident` - the ident of the variable that will resolve `arg`
  /// * `arg` - the argument to resolve
  pub fn new(ident: &'a syn::Ident, arg: &FnArg) -> Self {
    let resolve_type = resolve_arg(&arg);

    match &resolve_type {
      ResolveType::Reference(and, mutability) => Self {
        and: Some(*and),
        mutability: *mutability,
        ident: &ident,
      },
      ResolveType::Owner => Self {
        and: None,
        mutability: None,
        ident: &ident,
      },
    }
  }
}

impl<'a> ToTokens for ResolveFnArg<'a> {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    self.and.to_tokens(tokens);
    self.mutability.to_tokens(tokens);
    self.ident.to_tokens(tokens);
  }
}

/// This is a utility type used to allow a given [`syn::Ident`] to be tokenized
/// as binding declaration.
///
/// This will prepend `mut` for mutable ref args to the given ident. Note that
/// this is _not_ a binding expression (e.g. this is just `[mut] <ident>` -- not
/// `let [mut] <ident>`).
///
/// This is primarily used as a helper to ensure that fixtures will be called
/// with the appropriate refness as needed in a way that will not generate
/// warnings for unnecessary mutability.
///
/// [`syn::Ident`]: struct@syn::Ident
pub struct ResolveFnArgDecl<'a> {
  mutability: Option<Mut>,
  ident: &'a syn::Ident,
}

impl<'a> ResolveFnArgDecl<'a> {
  /// Creates a new [`ResolveFnArgDecl`] given the `ident` of the variable being
  /// defined, and the `arg` that it will eventually resolve.
  ///
  /// # Arguments
  ///
  /// * `ident` - the ident of the variable that will resolve `arg`
  /// * `arg` - the argument to resolve
  pub fn new(ident: &'a syn::Ident, arg: &FnArg) -> Self {
    let resolve_type = resolve_arg(&arg);

    match &resolve_type {
      ResolveType::Reference(_, mutability) => Self {
        mutability: *mutability,
        ident: &ident,
      },
      ResolveType::Owner => Self {
        mutability: None,
        ident: &ident,
      },
    }
  }
}

impl<'a> ToTokens for ResolveFnArgDecl<'a> {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    self.mutability.to_tokens(tokens);
    self.ident.to_tokens(tokens);
  }
}
