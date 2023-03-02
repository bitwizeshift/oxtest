use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt};
use syn::{parse_quote, Attribute, Block, ItemFn, Signature};

use crate::common::{ident, ty};

///
pub struct Test {
  attrs: Vec<Attribute>,
  sig: Signature,
  block: Box<Block>,
}

impl Test {
  pub fn new(item: ItemFn) -> Self {
    Self {
      attrs: item.attrs,
      sig: Self::test_signature(item.sig),
      block: Self::translate_sections(item.block),
    }
  }

  /// Returns a reference to the identifier of this test
  pub fn test_name(&self) -> &syn::Ident {
    &self.sig.ident
  }

  pub fn signature(&self) -> &Signature {
    &self.sig
  }

  pub fn attrs(&self) -> &[Attribute] {
    &self.attrs
  }

  /// Translates `#[section]` attributes in the given Block into context checks
  ///
  /// This is used to enable individual sub-tests.
  ///
  /// # Arguments
  ///
  /// * `block` - the block to translate
  fn translate_sections(block: Box<Block>) -> Box<Block> {
    // TODO(mrodusek): Translate #[section] attributes to context subtest checks
    block
  }

  fn test_signature(mut sig: Signature) -> Signature {
    let context_ident = ident::context();
    let context_ty = ty::context();
    sig.ident = ident::new_test_impl(&sig.ident);
    sig.inputs.push(parse_quote!(#context_ident: #context_ty));
    sig
  }
}

impl ToTokens for Test {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    tokens.append_all(self.attrs.iter());
    self.sig.to_tokens(tokens);
    self.block.to_tokens(tokens);
  }
}
