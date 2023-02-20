/// This
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::Type;

/// A sequence of types, each with potential attribute specifications.
///
/// This is a custom structure that follows [`syn`]'s [`ExprArray`] type, except
/// rather than parsing [`Expr`] types, it parses [`Type`]s. This is needed for
/// the handling of generic inputs.
///
/// [`Expr`]: syn::Expr
/// [`ExprArray`]: syn::ExprArray
#[derive(Clone)]
pub struct TypeSequence {
  pub bracket_token: syn::token::Bracket,
  pub elems: syn::punctuated::Punctuated<Type, Comma>,
}

impl syn::parse::Parse for TypeSequence {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let content;
    let bracket_token = syn::bracketed!(content in input);
    let mut elems = Punctuated::new();

    while !content.is_empty() {
      let first: syn::Type = content.parse()?;
      elems.push_value(first);
      if content.is_empty() {
        break;
      }
      let punct = content.parse()?;
      elems.push_punct(punct);
    }

    Ok(TypeSequence {
      bracket_token,
      elems,
    })
  }
}
