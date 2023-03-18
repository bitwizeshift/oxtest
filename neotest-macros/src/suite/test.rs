use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt};
use syn::{parse_quote, Attribute, Block, ItemFn, ReturnType, Signature};

use crate::common::{ident, ty};

///
pub struct Test {
  attrs: Vec<Attribute>,
  sig: Signature,
  block: Box<Block>,
}

impl Test {
  pub fn new(item: ItemFn) -> Self {
    let block = match &item.sig.output {
      ReturnType::Default => {
        let block = item.block;
        parse_quote!(
          {
            #block
            Ok(())
          }
        )
      }
      ReturnType::Type(_, _) => item.block,
    };
    Self {
      attrs: item.attrs,
      sig: Self::test_signature(item.sig),
      block,
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

  fn test_signature(mut sig: Signature) -> Signature {
    let context_ident = ident::context();
    let context_ty = ty::context();
    sig.ident = ident::new_test_impl(&sig.ident);
    sig
      .inputs
      .push(parse_quote!(mut #context_ident: #context_ty));
    sig.output = ty::test_result();
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
