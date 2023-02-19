//! This internal module defines a simple utility wrapper for tokenizing
//! slices of types that implement [`ToTokens`].
use quote::{ToTokens, TokenStreamExt};

/// A wrapper utility around any slice. This enables using containers like
/// [`Vec`] when converting to tokens, which allows it to be used in [`quote`]
/// or [`parse_quote`].
///
/// # Example
///
/// Basic use:
///
/// ```ignore
/// use crate::syn_utils::Inner;
/// use quote::quote;
///
/// let vec: Vec<syn::Expr> = get_exprs();
/// let inner = Inner(&vec);
///
/// quote!{
///     #inner
/// }
/// ```
///
/// [`parse_quote`]: syn::parse_quote
/// [`quote`]: quote::quote
pub struct Inner<'a, T: 'a>(pub &'a [T]);

impl<'a, T: 'a + ToTokens> ToTokens for Inner<'a, T> {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    tokens.append_all(self.0)
  }
}
