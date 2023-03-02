use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::token::{Brace, Mod};

use crate::common::attribute;

/// A helper utility for defining a module-definition.
///
/// This follows the API set out by the [`Brace`]/[`Bracket`] utility from
/// [`syn`] by requiring a call to [`Self::surround`] to encase the enclosing
/// definitions.
///
/// [`Bracket`]: syn::token::Bracket
pub struct ModuleDefinition<'a>(pub &'a syn::Ident);

impl<'a> ModuleDefinition<'a> {
  /// Outputs anything from `f` into the [`TokenStream`] surrounding the contents
  /// with a module definition of the specified ident.
  ///
  /// # Arguments
  ///
  /// * `tokens` - the token-stream to write to
  /// * `f` - the function that will expand contents within the module definition.
  pub fn surround<F>(&self, tokens: &mut TokenStream, f: F)
  where
    F: FnOnce(&mut TokenStream),
  {
    attribute::allow_dead_code().to_tokens(tokens);
    Mod::default().to_tokens(tokens);
    self.0.to_tokens(tokens);
    Brace::default().surround(tokens, |v| f(v));
  }
}
