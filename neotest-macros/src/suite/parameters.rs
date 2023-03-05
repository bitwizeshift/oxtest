use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::{token::Comma, Expr};

use std::rc::Rc;

use crate::common::ident;
use crate::input::{ParameterInput, TestInputs};

/// A collection of test parameters to provide to a given [`TestDispatcher`]
/// invocation.
#[derive(Clone)]
pub struct TestParameters {
  params: Rc<Punctuated<Expr, Comma>>,
}

impl Default for TestParameters {
  fn default() -> Self {
    Self::new(Default::default())
  }
}

impl TestParameters {
  /// Creates a new [`TestParameters`] from the set of expressions.
  ///
  /// # Arguments
  ///
  /// * `params` - the parameters to construct this with
  pub fn new(params: Punctuated<Expr, Comma>) -> Self {
    Self {
      params: Rc::new(params),
    }
  }

  pub fn multiplex(inputs: &TestInputs) -> Vec<(syn::Ident, TestParameters)> {
    let test_cases = MultiplexedTestParameters::multiplex(inputs);

    let mut result: Vec<(syn::Ident, TestParameters)> = Vec::with_capacity(test_cases.len());
    for param in test_cases.into_iter() {
      let ident = ident::new_test_input(&param.input_indices, Span::call_site());

      result.push((ident, TestParameters::new(param.parameters)));
    }
    result
  }

  pub fn is_empty(&self) -> bool {
    self.params.is_empty()
  }

  /// Serializes the test-parameters to a comma-separated sequence, while also
  /// including the `context` identifier.
  ///
  /// This is used for serializing arguments to the dispatcher-call.
  ///
  /// # Arguments
  ///
  /// * `tokens` - the [`TokenStream`] to serialize to
  /// * `context` - the context identifier
  pub fn to_tokens_with_context(&self, tokens: &mut TokenStream, context: &syn::Ident) {
    if self.params.is_empty() {
      context.to_tokens(tokens);
    } else {
      self.params.to_tokens(tokens);
      Comma::default().to_tokens(tokens);
      context.to_tokens(tokens);
    }
  }
}

#[derive(Default, Clone)]
struct MultiplexedTestParameters {
  input_indices: Vec<usize>,
  parameters: Punctuated<Expr, Comma>,
}

impl MultiplexedTestParameters {
  /// Creates a vector of [`MultiplexedTestParameters`] by combinatorially
  /// producing all combination of parameter inputs.
  ///
  /// # Arguments
  ///
  /// * `inputs` - the parameter inputs to test with
  pub fn multiplex(inputs: &TestInputs) -> Vec<Self> {
    let mut result: Vec<Self> = Vec::with_capacity(Self::input_size(inputs));

    Self::multiplex_into(&mut result, &inputs.parameters);

    result
  }

  /// Recursively populates `result` with all combinations of parameter
  /// inputs.
  ///
  /// # Arguments
  ///
  /// * `result` - the vector to populate
  /// * `parameters` - the input parameters to use for producing combinations
  fn multiplex_into(result: &mut Vec<Self>, parameters: &[ParameterInput]) {
    if parameters.is_empty() {
      return;
    }
    let current = Self {
      input_indices: Vec::with_capacity(parameters.len()),
      parameters: Punctuated::default(),
    };
    Self::multiplex_into_aux(result, current, parameters)
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
  fn multiplex_into_aux(result: &mut Vec<Self>, current: Self, parameters: &[ParameterInput]) {
    // Recursive base-case: we have finished iterating
    if parameters.is_empty() {
      result.push(current)
    } else {
      let param = parameters.first().unwrap();
      for (i, v) in param.inputs.elems.iter().enumerate() {
        let mut current = current.clone();
        current.input_indices.push(i);
        current.parameters.push(v.clone());
        Self::multiplex_into_aux(result, current, &parameters[1..])
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
      .product()
  }
}
