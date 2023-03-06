//! This internal-module defines an intermediate format used for parsing subtest
//! data from the `subtest` macro.
use syn::parse::{Parse, ParseStream};
use syn::Token;

pub struct SubtestInput {
  pub ident: syn::Ident,
  pub stmts: Vec<syn::Stmt>,
}

impl Parse for SubtestInput {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    input.parse::<Token![|]>()?;
    let ident: syn::Ident = input.parse()?;
    input.parse::<Token![|]>()?;

    let mut stmts: Vec<syn::Stmt> = Default::default();
    while !input.is_empty() {
      stmts.push(input.parse()?);
    }
    Ok(Self { ident, stmts })
  }
}
