//! This internal-module defines an intermediate format used for parsing subtest
//! data from the `subtest` macro.
use syn::parse::{Parse, ParseStream};
use syn::Token;

pub struct SubtestInput {
  pub ident: syn::Ident,
  pub block: syn::Block,
}

impl Parse for SubtestInput {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let ident: syn::Ident = input.parse()?;
    input.parse::<Token![,]>()?;
    let block: syn::Block = input.parse()?;

    Ok(Self { ident, block })
  }
}
