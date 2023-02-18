use quote::{ToTokens, TokenStreamExt};
use syn::{Attribute, Block, Signature};

use crate::model::{attribute, fn_arg, ident};

/// This models the function as written by the developer, but is an implementation
/// of the test framework.
///
/// The TestImplFn is very similar to the actual test authored by the developer,
/// except that the test name has been modified to be an internal identifier,
/// and the block of expressions has been updated to be context-aware -- which
/// is necessary for modeling subsections of tests.
pub(crate) struct TestImplFn {
  attrs: Vec<Attribute>,
  sig: Signature,
  block: Box<Block>,
}

impl TestImplFn {
  pub fn new(attrs: Vec<Attribute>, sig: Signature, block: Box<Block>) -> Self {
    let mut attrs = attrs;
    attrs.push(attribute::doc_hidden());
    attrs.push(attribute::allow_dead_code());

    let mut sig = sig;
    sig.ident = ident::new_test_impl(&sig.ident);
    sig.inputs.push(fn_arg::context());
    Self { attrs, sig, block }
  }

  pub fn fn_ident(&self) -> &syn::Ident {
    &self.sig.ident
  }
}

impl ToTokens for TestImplFn {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    tokens.append_all(self.attrs.iter());
    self.sig.to_tokens(tokens);
    self.block.to_tokens(tokens);
  }
}
