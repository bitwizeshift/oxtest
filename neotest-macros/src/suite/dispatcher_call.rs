use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
  parse_quote,
  token::{Colon2, Super},
  Stmt,
};

use crate::common::{ident, ty};
use crate::syn_utils::FunctionCall;

use crate::suite::{SectionPath, TestDispatcher, TestParameters};

/// A representation of a call to the underlying test dispatch function
///
/// This will tokenize with [`ToTokens`] into a stream of:
///
/// ```ignore
/// let __context = ::neotest_common::__internal::__Context::path([&[p0,p1]]);
/// super::__neotest_test_dispatch(v0, v1, __context)
/// ```
///
/// Where `super::` is prepended as many times as its needed for the proper scope.
pub struct DispatcherCall {
  parameters: TestParameters,
  section_path: SectionPath,
  depth: usize,
}

impl Default for DispatcherCall {
  fn default() -> Self {
    Self {
      parameters: TestParameters::new(Default::default()),
      section_path: Default::default(),
      depth: 0,
    }
  }
}

impl DispatcherCall {
  pub fn new(parameters: TestParameters) -> Self {
    Self {
      parameters,
      section_path: Default::default(),
      depth: 1, // Parameters always start at depth 1
    }
  }

  pub fn subsection(&self, subtest: usize) -> Self {
    Self {
      parameters: self.parameters.clone(),
      section_path: self.section_path.subsection(subtest),
      depth: self.depth + 1,
    }
  }
}

impl DispatcherCall {
  pub fn to_tokens_with_call(&self, tokens: &mut TokenStream, dispatch: &TestDispatcher) {
    let context_ident = ident::context();
    let context_ty = ty::context();
    let section_path = &self.section_path;

    let define: Stmt = parse_quote! {
      let #context_ident = #context_ty::path(#section_path);
    };
    define.to_tokens(tokens);

    for _ in 0..self.depth {
      Super::default().to_tokens(tokens);
      Colon2::default().to_tokens(tokens);
    }
    FunctionCall(&dispatch.dispatcher_name()).arguments(tokens, |tokens| {
      self
        .parameters
        .to_tokens_with_context(tokens, &context_ident);
    });
  }
}
