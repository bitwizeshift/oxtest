//! This internal-module defines the set of const parameter inputs.
use syn::parse::{Parse, ParseStream};
use syn::{ExprArray, Result};

use super::ParameterInput;

/// A struct containing const parameter inputs that can be specified for a test.
///
/// `ConstParameterInput` inputs contain the identifier of the const-generic
/// parameter being substituted, along with an array of each possibly valid input.
///
/// This input is formed from the the `const_parameter` argument in the
/// [`neotest`] attribute:
///
/// ```ignore
/// #[neotest(
///   /* ... */
///   const_parameter = VALUE as [1, 2, 3],
///   /* ... */
/// )]
/// fn test_value<const VALUE: usize>() { /* ... */ }
/// ```
///
/// [`neotest`]: crate::neotest
#[allow(dead_code)]
#[derive(Clone)]
pub struct ConstParameterInput {
  pub ident: syn::Ident,
  pub inputs: ExprArray,
}

impl Parse for ConstParameterInput {
  /// Parses the input from the parse stream
  ///
  /// Expected input is in the form `<ident> as [<expr>, <expr>, ...]`.
  ///
  /// # Example
  ///
  /// ```ignore
  /// V as [0xdeadbeef, 0xbadf00d, 0xc0ffee]
  /// ```
  fn parse(input: ParseStream) -> Result<Self> {
    let result: ParameterInput = input.parse()?;

    Ok(ConstParameterInput {
      ident: result.ident,
      inputs: result.inputs,
    })
  }
}
