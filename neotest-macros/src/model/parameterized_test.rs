//! This module defines the model type for managing parameterized testing.
use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_quote, Attribute, Block, Expr, ItemMod, Signature, Stmt, Visibility};

use crate::input::{ParameterInput, TestInputs};
use crate::model::{attribute, ident, ty, TestDispatcherFn};
use crate::syn_utils::Inner;

/// A structure that holds the input values to pass to parameterized tests.
///
/// This is an internal-only type used for book-keeping during test-case
/// generation.
#[derive(Default, Clone)]
struct ParameterizedTestInputs {
  input_indices: Vec<usize>,
  parameters: Punctuated<Expr, Comma>,
}

impl ParameterizedTestInputs {
  /// Creates a vector of [`ParameterizedTestInputs`] by combinatorially
  /// producing all combination of parameter inputs.
  ///
  /// # Arguments
  ///
  /// * `inputs` - the parameter inputs to test with
  pub fn combine_inputs(inputs: &TestInputs) -> Vec<Self> {
    let mut result: Vec<Self> = Vec::with_capacity(Self::input_size(&inputs));

    Self::populate_into(&mut result, &inputs.parameters);

    let context_ident = ident::context();
    let context_expr: Expr = parse_quote! { #context_ident };

    // Add the __context parameter into each parameter set. This is passed to
    // all test drivers.
    result
      .iter_mut()
      .for_each(|v| v.parameters.push(context_expr.clone()));

    result
  }

  /// Recursively populates `result` with all combinations of parameter
  /// inputs.
  ///
  /// # Arguments
  ///
  /// * `result` - the vector to populate
  /// * `parameters` - the input parameters to use for producing combinations
  fn populate_into(result: &mut Vec<Self>, parameters: &[ParameterInput]) {
    if parameters.is_empty() {
      return;
    }
    let current = Self {
      input_indices: Vec::with_capacity(parameters.len()),
      parameters: Punctuated::default(),
    };
    Self::populate_into_aux(result, current, parameters)
  }

  /// An auxiliary function used for the recursion of parameter inputs.
  ///
  /// This passes the current state of the test input down into the next
  /// set of parameter values.
  ///
  /// # Arguments
  ///
  /// * `result` - the vector to populate
  /// * `current` - the current state of inputs
  /// * `parameters` - the input parameters to use for producing combinations
  fn populate_into_aux(result: &mut Vec<Self>, current: Self, parameters: &[ParameterInput]) {
    // Recursive base-case: we have finished iterating
    if parameters.is_empty() {
      result.push(current)
    } else {
      let param = parameters.first().unwrap();
      for (i, v) in param.inputs.elems.iter().enumerate() {
        let mut current = current.clone();
        current.input_indices.push(i);
        current.parameters.push(v.clone());
        Self::populate_into_aux(result, current, &parameters[1..])
      }
    }
  }

  /// Computes the number of input combinations that are possible given
  /// the specified test inputs.
  ///
  /// Since parameterized tests produce the complete-graph of combinations,
  /// the number of possible inputs is multiplicative. Thus, if a test takes
  /// two parameters `(a, b)`, and `a` is given `3` possible inputs while `b`
  /// is given `2` possible inputs, the result will be `6` -- which corresponds
  /// to the following combinations:
  ///
  /// * `(a_0, b_0)
  /// * `(a_0, b_1)
  /// * `(a_1, b_0)
  /// * `(a_1, b_1)
  /// * `(a_2, b_0)
  /// * `(a_2, b_1)
  ///
  /// # Arguments
  ///
  /// * `inputs` - the test inputs to provide
  fn input_size(inputs: &TestInputs) -> usize {
    inputs
      .parameters
      .iter()
      .map(|v| v.inputs.elems.len())
      .fold(1, |acc, v| acc * v)
  }
}

/// A test function that performs parameter substitution for parameterized-tests.
///
/// One instance of [`ParameterizedTestFn`] will correspond to a given parameter
/// set.
pub struct ParameterizedTestFn {
  pub attrs: Vec<Attribute>,
  pub sig: Signature,
  pub block: Box<Block>,
}

impl ParameterizedTestFn {
  /// Creates a single [`ParameterizedTestFn`] from test input information.
  ///
  /// # Arguments
  ///
  /// * `attrs` - the attributes to apply to the test case
  /// * `sig` - the signature of the base test function
  /// * `params` - the parameters to pass to the test function
  /// * `dispatch` - the dispatcher function to call
  fn new(
    attrs: Vec<Attribute>,
    sig: Signature,
    params: &ParameterizedTestInputs,
    dispatch: &TestDispatcherFn,
  ) -> Self {
    // Add test attributes, and hide this from the caller
    let mut attrs = attrs;
    attrs.push(attribute::doc_hidden());
    attrs.push(attribute::test());

    // Change the signature to be a proper test, and update the test-name.
    let mut sig = sig;
    sig.inputs.clear();
    sig.ident = ident::new_test_input(&params.input_indices, sig.ident.span());
    let dispatcher_ident = dispatch.fn_ident();
    let context_type = ty::context();
    let context_ident = ident::context();
    let inputs = &params.parameters;

    let block: Box<Block> = parse_quote! {
      {
        let #context_ident = #context_type::all_tests();
        super::#dispatcher_ident(#inputs);
      }
    };

    Self { attrs, block, sig }
  }
}

impl ToTokens for ParameterizedTestFn {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let visibility: Visibility = parse_quote!(pub(super));

    tokens.append_all(self.attrs.iter());
    visibility.to_tokens(tokens);
    self.sig.to_tokens(tokens);
    self.block.to_tokens(tokens);
  }
}

/// A small utility for representing an expansion of a qualified call to
/// parameterized-test functions.
pub struct ParameterizedTestCall {
  qualified_test: syn::Path,
}

impl ToTokens for ParameterizedTestCall {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let path = &self.qualified_test;
    let block: Stmt = parse_quote! {
      #path();
    };

    block.to_tokens(tokens)
  }
}

/// An aggregate type that collects all parameterized test-cases into a
/// base-module-aware type.
pub struct ParameterizedTests {
  /// The name of the main test being executed
  test_ident: syn::Ident,

  /// The collection of parameterized tests to execute
  tests: Vec<ParameterizedTestFn>,
}

impl ParameterizedTests {
  /// Creates a collection of parameterized tests from the given test inputs,
  /// if the inputs defines any parameters to use.
  ///
  /// # Arguments
  ///
  /// * `attrs` - the attributes to apply to each test case
  /// * `sig` - the signature of the base test case
  /// * `inputs` - the inputs to the test case
  /// * `dispatch` - the dispatch function model
  pub fn new(
    attrs: Vec<Attribute>,
    sig: Signature,
    inputs: TestInputs,
    dispatch: &TestDispatcherFn,
  ) -> Option<Self> {
    let params = ParameterizedTestInputs::combine_inputs(&inputs);
    let len = params.len();
    println!("{len} params specified.");
    if params.is_empty() {
      return None;
    }

    let tests: Vec<ParameterizedTestFn> = params
      .iter()
      .map(|v| ParameterizedTestFn::new(attrs.clone(), sig.clone(), v, &dispatch))
      .collect();

    Some(Self {
      test_ident: sig.ident.clone(),
      tests,
    })
  }

  /// Creates a [`syn::Path`] object representing the qualified name of the
  /// test function being executed.
  ///
  /// # Arguments
  ///
  /// * `test_ident` - the identifier of the test to execute
  fn make_call_path(&self, test_ident: &syn::Ident) -> syn::Path {
    let ident = &self.test_ident;
    parse_quote! { #ident::#test_ident }
  }

  /// Creates a [`Vec`] of [`ParameterizedTestCall`] to represent how to call
  /// all parameterized sub-tests.
  pub fn as_calls(&self) -> Vec<ParameterizedTestCall> {
    self
      .tests
      .iter()
      .map(|v| ParameterizedTestCall {
        qualified_test: self.make_call_path(&v.sig.ident),
      })
      .collect()
  }
}

impl ToTokens for ParameterizedTests {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    if !self.tests.is_empty() {
      let test_ident = &self.test_ident;
      let tests = Inner(&self.tests);
      let item: ItemMod = parse_quote! {
        #[doc(hidden)]
        mod #test_ident {
          use super::*;

          #tests
        }
      };

      item.to_tokens(tokens)
    }
  }
}
