//! This module contains the various types parsed
//!

use crate::input::FixtureInput;
use crate::input::TestInputs;
use crate::syn_utils::ContainsIdent;
use crate::syn_utils::TryIdent;
use quote::ToTokens;

mod common;
mod parameterized_test;
mod test_dispatcher_fn;
mod test_impl_fn;
mod test_main_fn;

pub use common::*;
pub use parameterized_test::*;
pub use test_dispatcher_fn::*;
pub use test_impl_fn::*;
pub use test_main_fn::*;

pub struct TestModel {
  test_main_fn: TestMainFn,
  test_fixture_fn: TestDispatcherFn,
  test_impl_fn: TestImplFn,
  param_test_fns: Option<ParameterizedTests>,
}

impl TestModel {
  pub fn from_inputs(mut inputs: TestInputs, test: syn::ItemFn) -> syn::Result<Self> {
    Self::validate(&inputs, &test)?;

    inputs.reorder(&test.sig);

    Ok(Self::new_model(inputs, test))
  }
}

// Validation

impl TestModel {
  /// Validates the inputs and function being tested for correctness
  ///
  /// # Arguments
  ///
  /// * `inputs` - the test inputs passed to the attribute
  /// * `test` - the function performing the testing
  fn validate(inputs: &TestInputs, test: &syn::ItemFn) -> syn::Result<()> {
    // Perform basic validation
    Self::validate_attributes(test)?;
    Self::validate_parameters(inputs, test)?;
    Self::validate_generic_parameters(inputs, test)?;
    Ok(())
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
        if *ident == "test" {
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
      Self::validate_fixture_input(fixture, &args)?;
      if !args.is_empty() {
        args.remove(0);
      }
    }

    for ident in inputs.parameters.iter().map(|v| &v.ident) {
      if !test.sig.inputs.contains_ident(ident) {
        let name = ident.to_string();
        return Err(syn::Error::new(
          ident.span(),
          format!("Test input '{name}' is not a valid function parameter."),
        ));
      }
    }

    // TODO(mrodusek): Refactor this into smaller functions
    for arg in args.iter() {
      if let Some(ident) = arg.try_ident() {
        let arg_inputs: Vec<&syn::Ident> = inputs
          .parameters
          .iter()
          .map(|v| &v.ident)
          .filter(|v| *ident == v.to_string())
          .collect();
        let count = arg_inputs.len();

        if count != 1 {
          let param_name = ident.to_string();
          if count == 0 {
            return Err(syn::Error::new(
              ident.span(),
              format!(
                "Test function parameter '{param_name}' is not bound to test input. Use `parameter = {param_name} as ...` to set a parameter.",
              ),
            ));
          } else if count > 1 {
            let ident = arg_inputs[1];

            return Err(syn::Error::new(
              ident.span(),
              format!("Test input '{param_name}' specified more than once."),
            ));
          }
        }
      }
    }
    Ok(())
  }

  fn validate_generic_parameters(inputs: &TestInputs, test: &syn::ItemFn) -> syn::Result<()> {
    let (_, _) = (inputs, test);
    // TODO(mrodusek): Check that all generic arguments correspond to name parameters
    Ok(())
  }
}

// Model Formation

impl TestModel {
  /// Makes a new model from the given test inputs and functions
  ///
  /// This assumes that the model has already been validated
  ///
  /// # Arguments
  ///
  /// * inputs - the inputs to the parameters
  /// * test - the function performing testing
  fn new_model(inputs: TestInputs, test: syn::ItemFn) -> Self {
    // Form the model for the tests
    let test_impl_fn = TestImplFn::new(test.attrs.clone(), test.sig.clone(), test.block);
    let test_fixture_fn = TestDispatcherFn::new(
      inputs.fixture.clone().map(|v| v.ident),
      test.attrs.clone(),
      test.sig.clone(),
      &test_impl_fn,
      Default::default(),
    );
    let param_test_fns = ParameterizedTests::new(
      test.attrs.clone(),
      test.sig.clone(),
      inputs,
      &test_fixture_fn,
    );

    let test_main_fn = TestMainFn::new(
      test.attrs.clone(),
      test.sig,
      param_test_fns.as_ref(),
      &test_fixture_fn,
    );

    Self {
      test_main_fn,
      test_fixture_fn,
      test_impl_fn,
      param_test_fns,
    }
  }
}

impl ToTokens for TestModel {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    self.test_impl_fn.to_tokens(tokens);
    self.test_fixture_fn.to_tokens(tokens);
    self.test_main_fn.to_tokens(tokens);
    self.param_test_fns.to_tokens(tokens);
  }
}
