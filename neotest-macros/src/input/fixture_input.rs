//! This internal-module defines a data-representation of the fixture input.
use syn::parse::ParseStream;
use syn::Result;

/// A struct containing fixture input for a test.
///
/// `FixtureInput` names just the type name of a fixture test.
///
/// This input is formed from the the `const_parameter` argument in the
/// [`neotest`] attribute:
///
/// ```ignore
/// #[neotest(
///   /* ... */
///   fixture = Fixture,
///   /* ... */
/// )]
/// fn test_value(f: &Fixture) { /* ... */ }
/// ```
///
/// [`neotest`]: crate::neotest
#[derive(Clone)]
pub struct FixtureInput {
  pub ident: syn::Ident,
}

impl syn::parse::Parse for FixtureInput {
  /// Parses the fixture name from the parse stream
  ///
  /// Expected input is in the form of just `<ident>`.
  ///
  /// # Example
  ///
  /// ```ignore
  /// TestFixture
  /// ```
  fn parse(input: ParseStream) -> Result<Self> {
    let ident: syn::Ident = input.parse()?;
    Ok(FixtureInput { ident })
  }
}
