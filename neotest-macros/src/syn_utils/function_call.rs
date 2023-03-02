use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::token::Paren;

/// A helper utility for defining a module-definition.
///
/// This follows the API set out by the [`Brace`]/[`Bracket`] utility from
/// [`syn`] by requiring a call to [`Self::surround`] to encase the enclosing
/// definitions.
///
/// [`Bracket`]: syn::token::Bracket
pub struct FunctionCall<'a>(pub &'a syn::Ident);

impl<'a> FunctionCall<'a> {
  /// Outputs anything from `f` into the [`TokenStream`] surrounding the contents
  /// with a module definition of the specified ident.
  ///
  /// # Arguments
  ///
  /// * `tokens` - the token-stream to write to
  /// * `f` - the function that will expand contents within the module definition.
  pub fn arguments<F>(&self, tokens: &mut TokenStream, f: F)
  where
    F: FnOnce(&mut TokenStream),
  {
    self.0.to_tokens(tokens);
    Paren::default().surround(tokens, |v| f(v));
  }
}
