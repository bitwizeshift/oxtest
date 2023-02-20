use quote::{ToTokens, TokenStreamExt};
use syn::{parse_quote, Attribute, Block, Signature};

use crate::model::{attribute, TestDispatcherFn};
use crate::syn_utils::Inner;

use super::{ident, ty, ParameterizedTests};

/// This models the function that will actually perform the rust-test.
///
/// This will be the cleanly-named function marked up with the [`test`] attribute.
/// For parameter-based inputs, this will dispatch all sub-tests.
pub(crate) struct TestMainFn {
  attrs: Vec<Attribute>,
  sig: Signature,
  block: Box<Block>,
}

impl TestMainFn {
  pub fn new(
    attrs: Vec<Attribute>,
    sig: Signature,
    param_tests: Option<&ParameterizedTests>,
    fixture_fn: &TestDispatcherFn,
  ) -> Self {
    let mut attrs = attrs;
    attrs.push(attribute::test());

    let dispatch_ident = fixture_fn.fn_ident();

    // We can't have inputs on the actual test-runner.
    let mut sig = sig;
    sig.inputs.clear();

    let context = ty::context();
    let context_ident = ident::context();
    let block: Box<Block> = if let Some(tests) = param_tests {
      let calls = tests.as_calls();
      let calls_inner = Inner(&calls);

      parse_quote! {
        {
          #calls_inner
        }
      }
    } else {
      parse_quote! {
        {
          let #context_ident = #context::all_tests();

          #dispatch_ident(#context_ident);
        }
      }
    };

    Self { attrs, sig, block }
  }
}

impl ToTokens for TestMainFn {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    tokens.append_all(self.attrs.iter());
    self.sig.to_tokens(tokens);
    self.block.to_tokens(tokens);
  }
}
