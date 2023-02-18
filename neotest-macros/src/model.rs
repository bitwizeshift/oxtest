//! This module contains the various types parsed
//!

use crate::input::FixtureInput;
use crate::input::TestInputs;
use quote::ToTokens;

mod common;
mod test_dispatcher_fn;
mod test_impl_fn;
mod test_main_fn;

pub(crate) use common::*;
pub(crate) use test_dispatcher_fn::*;
pub(crate) use test_impl_fn::*;
pub(crate) use test_main_fn::*;

/*
pub(crate) struct TestSection {
  pub section_name: syn::Ident,
  pub section_path: Punctuated<syn::Expr, syn::Token![,]>,
  pub subsection: Vec<TestSection>,
}

impl ToTokens for TestSection {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {}
}

/// Represents a test condition that gets executed.
pub(crate) struct TestCondition {
  pub test_name: syn::Ident,
  pub test_dispatch_name: syn::Ident,
  pub parameters: Punctuated<syn::Expr, syn::Token![,]>,
  pub subsections: Vec<TestSection>,
}

impl ToTokens for TestCondition {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let test_name = &self.test_name;
    let test_dispatch_name = &self.test_dispatch_name;
    let parameters = &self.parameters;

    let test_fn_definition: syn::ItemFn = parse_quote! {
      #[test]
      #[doc(hidden)]
      #[ignore(dead_code)]
      fn #test_name() -> ::neotest::TestResult {
        let __context = ::neotest::__Context::default();
        #test_dispatch_name(__context, #parameters)
      }
    };

    test_fn_definition.to_tokens(tokens)
  }
}

pub(crate) enum Subtest {
  Condition(TestCondition),
  Section(TestSection),
}

impl ToTokens for Subtest {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    match &self {
      Subtest::Condition(v) => v.to_tokens(tokens),
      Subtest::Section(v) => v.to_tokens(tokens),
    }
  }
}
*/

pub(crate) struct TestModel {
  test_main_fn: TestMainFn,
  test_fixture_fn: TestDispatcherFn,
  test_impl_fn: TestImplFn,
}

impl TestModel {
  pub fn from_inputs(inputs: TestInputs, test: syn::ItemFn) -> syn::Result<Self> {
    // Perform basic validation
    Self::validate_attributes(&test)?;
    Self::validate_parameters(&inputs, &test)?;
    Self::validate_generic_parameters(&inputs, &test)?;

    // Form the model for the tests
    let test_impl_fn = TestImplFn::new(test.attrs.clone(), test.sig.clone(), test.block);
    let test_fixture_fn = TestDispatcherFn::new(
      inputs.fixture.map(|v| v.ident),
      test.attrs.clone(),
      test.sig.clone(),
      &test_impl_fn,
      Default::default(),
    );
    let test_main_fn = TestMainFn::new(test.attrs.clone(), test.sig.clone(), &test_fixture_fn);

    Ok(Self {
      test_main_fn,
      test_fixture_fn,
      test_impl_fn,
    })
  }

  fn validate_fixture_input(input: &FixtureInput, args: &[syn::FnArg]) -> syn::Result<()> {
    let first = args.first();

    if first.is_none() {
      let type_str = input.ident.to_string();
      return Err(syn::Error::new(
        input.ident.span(),
        format!("test fixture function missing {type_str} fixture as first argument"),
      ));
    }
    Ok(())
  }

  fn validate_attributes(test: &syn::ItemFn) -> syn::Result<()> {
    for attr in test.attrs.iter() {
      if let Some(ident) = attr.path.get_ident() {
        if ident.to_string() == "test" {
          return Err(syn::Error::new(
            ident.span(),
            "#[test] attribute cannot be specified for tests when using #[neotest]",
          ));
        }
      }
    }
    Ok(())
  }

  fn validate_parameters(inputs: &TestInputs, test: &syn::ItemFn) -> syn::Result<()> {
    // If we have a fixture, verify that it's the first argument
    let mut args: Vec<syn::FnArg> = Vec::from_iter(test.sig.inputs.iter().cloned());

    if let Some(fixture) = &inputs.fixture {
      Self::validate_fixture_input(&fixture, &args)?;
      if !args.is_empty() {
        args.remove(0);
      }
    }

    // TODO(mrodusek): Check that all other arguments correspond to some named parameters
    Ok(())
  }

  fn validate_generic_parameters(inputs: &TestInputs, test: &syn::ItemFn) -> syn::Result<()> {
    let (_, _) = (inputs, test);
    // TODO(mrodusek): Check that all generic arguments correspond to name parameters
    Ok(())
  }
}

impl ToTokens for TestModel {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    self.test_impl_fn.to_tokens(tokens);
    self.test_fixture_fn.to_tokens(tokens);
    self.test_main_fn.to_tokens(tokens);
  }
}
