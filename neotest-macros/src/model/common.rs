/// A module containing helper attributes regularly used in the internal-functions
/// within neotest.
pub(crate) mod attribute {
  use syn::parse_quote;
  use syn::Attribute;

  /// A helper function for creating the attribute `#[doc(hidden)]`
  pub(crate) fn doc_hidden() -> Attribute {
    parse_quote!(#[doc(hidden)])
  }

  /// A helper function for creating the attribute `#[allow(dead_code)]`
  pub(crate) fn allow_dead_code() -> Attribute {
    parse_quote!(#[allow(dead_code)])
  }

  /// A helper function for creating the attribute `#[test]`
  pub(crate) fn test() -> Attribute {
    parse_quote!(#[test])
  }
}

/// A module containing helper function argument definitions used in the
/// internal-functions within neotest.
pub(crate) mod fn_arg {
  use syn::parse_quote;
  use syn::FnArg;
  pub(crate) fn context() -> FnArg {
    let context = super::ty::context();
    let context_ident = super::ident::context();

    parse_quote! { #context_ident: #context }
  }
}

pub(crate) mod path {
  use syn::parse_quote;
  use syn::Path;

  pub(crate) fn crate_internal() -> Path {
    parse_quote! { ::neotest::__internal }
  }
}

pub(crate) mod ty {
  use syn::parse_quote;
  use syn::Type;

  pub(crate) fn context() -> Type {
    let path = super::path::crate_internal();

    parse_quote!( #path::__Context )
  }
}

/// A module containing common ident generation functons used within neotest
/// internals.
pub(crate) mod ident {

  use proc_macro2::Span;
  use syn::parse_quote;
  use syn::Ident;

  pub(crate) fn context() -> Ident {
    parse_quote!(__context)
  }

  /// Creates an ident used for performing the actual test itself
  ///
  /// # Arguments
  ///
  /// * `base` - the base name of the test (what is specified by the user)
  pub(crate) fn new_test_impl(base: &Ident) -> Ident {
    let name_str = base.to_string();
    let new_ident_str = format!("__neotest_{name_str}_impl");

    Ident::new(&new_ident_str, base.span())
  }

  /// Creates an ident used for dispatching the fixture object
  ///
  /// # Arguments
  ///
  /// * `base` - the base name of the test (what is specified by the user)
  pub(crate) fn new_test_dispatch(base: &Ident) -> Ident {
    let name_str = base.to_string();
    let new_ident_str = format!("__neotest_{name_str}_dispatcher");

    Ident::new(&new_ident_str, base.span())
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
  pub(crate) fn new_test_input(indices: &[usize], span: Span) -> Ident {
    use std::fmt::Write;
    assert!(!indices.is_empty());

    let mut out = String::from("input");

    // If we have more than one input, make this plural.
    if indices.len() > 1 {
      let _ = write!(&mut out, "{}", "s");
    }

    for n in indices {
      let _ = write!(&mut out, "_{}", n);
    }

    Ident::new(&out, span)
  }
}
