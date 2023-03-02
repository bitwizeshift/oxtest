#![allow(unused)]
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse_quote, Block, ItemFn, Signature, Stmt};

use crate::input::{FixtureInput, TestInputs};
use crate::suite::{Test, TestAttributes};
use crate::syn_utils::{ContainsIdent, FunctionDefinition, ModuleDefinition, TryIdent};

use super::{TestDispatcher, TestExecutor, TestParameters};

pub struct ParameterizedTestSuite {
  test: Test,
  main: TestDispatcher,

  attrs: TestAttributes,
  sig: Signature,
  subtests: Vec<TestExecutor>,
}

impl ParameterizedTestSuite {
  fn push_subtest<F>(&mut self, name: syn::Ident, params: TestParameters, test: &Test, f: F)
  where
    F: FnOnce(&mut TestExecutor),
  {
    let mut executor = TestExecutor::new(name, self.attrs.clone(), params, &test);
    f(&mut executor);
    self.subtests.push(executor);
  }
}

impl ToTokens for ParameterizedTestSuite {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    self.test.to_tokens(tokens);
    self.main.to_tokens_with_call(tokens, &self.test);

    self.attrs.to_tokens(tokens);
    FunctionDefinition(&self.sig).surround(tokens, |tokens| {
      for test in self.subtests.iter() {
        let scope = &self.sig.ident;
        let test_name = test.executor_name();

        let invoke_stmt: Stmt = parse_quote! { #scope::#test_name(); };
        invoke_stmt.to_tokens(tokens);
      }
    });

    ModuleDefinition(&self.sig.ident).surround(tokens, |tokens| {
      let use_stmt: Stmt = parse_quote! { use super::*; };
      use_stmt.to_tokens(tokens);
      for subtest in self.subtests.iter() {
        subtest.to_tokens_with_call(tokens, &self.main);
      }
    })
  }
}

pub struct StandardTestSuite {
  test: Test,
  main: TestDispatcher,
  root: TestExecutor,
}

impl ToTokens for StandardTestSuite {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    self.test.to_tokens(tokens);
    self.main.to_tokens_with_call(tokens, &self.test);
    self.root.to_tokens_with_call(tokens, &self.main);
  }
}

pub enum TestSuite {
  /// A test executor for dispatching to parameterized-tests
  Parameterized(ParameterizedTestSuite),

  /// A test executor for dispatching basic non-parameterized tests that may
  /// contain subtests.
  Standard(StandardTestSuite),
}

impl ToTokens for TestSuite {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    match &self {
      TestSuite::Parameterized(executor) => executor.to_tokens(tokens),
      TestSuite::Standard(executor) => executor.to_tokens(tokens),
    }
  }
}

impl TestSuite {
  /// Constructs a [`TestSuite`] from the specified test inputs.
  ///
  /// If the provided [`TestInputs`] contains any parameterized tests, this will
  /// produce a [`TestSuite::Parameterized`], otherwise this will produce a
  /// [`TestSuite::Standard`].
  ///
  /// # Arguments
  ///
  /// * `inputs` - the test input arguments supplied in the test attribute
  /// * `test` - the definition of the test function
  pub fn from_inputs(mut inputs: TestInputs, test: ItemFn) -> syn::Result<Self> {
    Self::validate(&inputs, &test)?;
    inputs.reorder(&test.sig);

    if inputs.parameters.is_empty() {
      Ok(Self::standard_from_inputs(inputs, test))
    } else {
      Ok(Self::parameterized_from_inputs(inputs, test))
    }
  }

  fn parameterized_from_inputs(inputs: TestInputs, mut test_fn: ItemFn) -> Self {
    let sig = test_fn.sig.clone();
    let graph = Self::translate_sections(&mut test_fn.block);

    let test = Test::new(test_fn);
    let attrs = TestAttributes::new(test.attrs().into());
    let main = TestDispatcher::new(&test, inputs.fixture.as_ref().cloned().map(|v| v.ident));
    let mut subtests = Self::multiplex_subtests(attrs.clone(), &inputs, &test);
    for subtest in &mut subtests {
      Self::apply_subsections(subtest, &graph.subsections);
    }

    Self::Parameterized(ParameterizedTestSuite {
      attrs,
      main,
      sig: Self::suite_signature(sig),
      test,
      subtests,
    })
  }

  fn suite_signature(mut sig: Signature) -> Signature {
    sig.inputs.clear();
    sig
  }

  /// Constructs a [`TestSuite::Standard`] object from the specified inputs
  ///
  /// A standard test-suite is a non-parameterized test-suite that may, or may not,
  /// contain subtests.
  ///
  /// # Arguments
  ///
  /// * `inputs` - the test input arguments supplied in the test attribute
  /// * `test` - the definition of the test function
  fn standard_from_inputs(inputs: TestInputs, mut test_fn: ItemFn) -> Self {
    let name = test_fn.sig.ident.clone();
    let graph = Self::translate_sections(&mut test_fn.block);

    let test = Test::new(test_fn);
    let attrs = TestAttributes::new(test.attrs().into());
    let main = TestDispatcher::new(&test, inputs.fixture.map(|v| v.ident));
    let mut root = TestExecutor::new(name, attrs, Default::default(), &test);

    Self::apply_subsections(&mut root, &graph.subsections);

    Self::Standard(StandardTestSuite { test, main, root })
  }

  /// Translates `#[section]` attributes within the test function into
  /// context-section-path checks, and returns a graph of all discovered
  /// sections.
  fn translate_sections(block: &mut Box<Block>) -> SectionGraph {
    // TODO(mrodusek): Implement translation for sections
    Default::default()
  }

  fn apply_subsections(executor: &mut TestExecutor, sections: &[Section]) {
    if sections.is_empty() {
      return;
    }
    for section in sections.iter() {
      executor.push_subtest(section.index, Some(section.name.clone()), |executor| {
        Self::apply_subsections(executor, &sections[1..]);
      });
    }
  }

  fn multiplex_subtests(
    attrs: TestAttributes,
    inputs: &TestInputs,
    test: &Test,
  ) -> Vec<TestExecutor> {
    let test_cases = TestParameters::multiplex(&inputs);
    let mut result: Vec<TestExecutor> = Vec::with_capacity(test_cases.len());
    for (ident, params) in test_cases.into_iter() {
      result.push(TestExecutor::new(ident, attrs.clone(), params, &test));
    }
    result
  }
}

#[allow(dead_code)]
struct Section {
  index: usize,
  name: syn::Ident,
  subsection: Vec<Section>,
}

#[derive(Default)]
#[allow(dead_code)]
struct SectionGraph {
  subsections: Vec<Section>,
}

impl TestSuite {
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
