//! This internal-module defines the set of parameter inputs.
use syn::parse::Parse;
use syn::token::As;
use syn::ExprArray;

/// A struct containing parameter inputs that can be specified for a test.
///
/// Parameter inputs contain the identifier of the parameter being substituted,
/// along with an array of each possibly valid input.
///
/// This input is formed from the the `parameter` argument in the
/// [`neotest`] attribute:
///
/// ```ignore
/// #[neotest(
///     /* ... */
///     parameter = a as [0xdead, 0xbeef, 0xc0ffee],
///     parameter = b as ["hello", "world"],
///     /* ... */
/// )]
/// fn test_value(a: u32, b: &str) { /* ... */ }
/// ```
///
/// [`neotest`]: crate::neotest
#[derive(Clone)]
pub struct ParameterInput {
  pub ident: syn::Ident,
  pub inputs: ExprArray,
}

impl Parse for ParameterInput {
  /// Parses the input from the parse stream
  ///
  /// Expected input is in the form `<ident> as [<expr0>, <expr1>, ...]`.
  ///
  /// # Example
  ///
  /// ```ignore
  /// v as [1, 2, 3]
  /// ```
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let ident: syn::Ident = input.parse()?;
    input.parse::<As>()?;
    let inputs: syn::ExprArray = input.parse()?;

    Ok(ParameterInput { ident, inputs })
  }
}
