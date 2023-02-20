//! This internal-module defines the set of type parameter inputs.
use syn::token::As;

use crate::syn_utils::TypeSequence;

/// A struct containing generic Type parameter inputs that can be specified for
/// a test.
///
/// Type parameter inputs contain the identifier of the type-parameter being
/// substituted, along with a sequence of types to substituted -- modeled in
/// an array.
///
/// This input is formed from the the `type_parameter` argument in the
/// [`neotest`] attribute:
///
/// ```ignore
/// #[neotest(
///   /* ... */
///   type_parameter = T as [u32, u64, usize],
///   type_parameter = U as [std::string::String, f32]
///   /* ... */
/// )]
/// fn test_value<T,U>() { /* ... */ }
/// ```
///
/// [`neotest`]: crate::neotest
#[allow(dead_code)]
#[derive(Clone)]
pub struct TypeParameterInput {
  pub ident: syn::Ident,
  pub inputs: TypeSequence,
}

impl syn::parse::Parse for TypeParameterInput {
  /// Parses the input from the parse stream
  ///
  /// Expected input is in the form `<ident> as [<type>, <type>, ...]`.
  ///
  /// # Example
  ///
  /// ```ignore
  /// T as [u8, String, ::std::vec::Vec]
  /// ```
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let ident: syn::Ident = input.parse()?;
    input.parse::<As>()?;
    let inputs: TypeSequence = input.parse()?;

    Ok(TypeParameterInput { ident, inputs })
  }
}
