/// A module containing helper attributes regularly used in the internal-functions
/// within neotest.
pub mod attribute {
  use syn::parse_quote;
  use syn::Attribute;

  /// A helper function for creating the attribute `#[allow(dead_code)]`
  pub fn allow_dead_code() -> Attribute {
    parse_quote!(#[allow(dead_code)])
  }

  /// A helper function for creating the attribute `#[test]`
  pub fn test() -> Attribute {
    parse_quote!(#[test])
  }
}

pub mod path {
  use syn::parse_quote;
  use syn::Path;

  pub fn crate_internal() -> Path {
    parse_quote! { ::neotest_common::__internal }
  }
}

pub mod ty {
  use syn::parse_quote;
  use syn::Type;

  pub fn context() -> Type {
    let path = super::path::crate_internal();

    parse_quote!( #path::__Context )
  }
}

/// A module containing common ident generation functons used within neotest
/// internals.
pub mod ident {

  use proc_macro2::Span;
  use quote::format_ident;
  use syn::Ident;

  pub fn context() -> Ident {
    format_ident!("__context")
  }

  /// Creates an ident used for performing the actual test itself
  ///
  /// # Arguments
  ///
  /// * `base` - the base name of the test (what is specified by the user)
  pub fn new_test_impl(base: &Ident) -> Ident {
    format_ident!("__neotest_{base}_impl")
  }

  /// Creates an ident used for dispatching the fixture object
  ///
  /// # Arguments
  ///
  /// * `base` - the base name of the test (what is specified by the user)
  pub fn new_test_dispatch(base: &Ident) -> Ident {
    format_ident!("{base}_dispatcher")
  }

  /// Creates an ident for test input dispatch functions
  ///
  /// The name is produced by concatenating the indices into a string identifier.
  /// The indices each represent which parameter is expanded from the inputs.
  ///
  /// # Arguments
  ///
  /// * `indices` - the indices that represent the input values
  /// * `span` - the span for where this input comes from
  pub fn new_test_input(indices: &[usize], span: Span) -> Ident {
    use std::fmt::Write;
    assert!(!indices.is_empty());

    let mut out = String::from("input");

    // If we have more than one input, make this plural.
    if indices.len() > 1 {
      let _ = write!(&mut out, "s");
    }

    for n in indices {
      let _ = write!(&mut out, "_{}", n);
    }

    Ident::new(&out, span)
  }
}
