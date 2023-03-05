use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, TokenStreamExt};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_quote, Attribute, FnArg, Signature, Stmt};

use crate::common::ident;
use crate::syn_utils::{
  FunctionCall, FunctionDefinition, ResolveFnArg, ResolveFnArgDecl, TryIdent,
};

use super::test::Test;

struct FixtureData {
  ident: syn::Ident,
  arg: FnArg,
}

pub struct TestDispatcher {
  attrs: Vec<Attribute>,
  sig: Signature,
  fixture: Option<FixtureData>,
}

impl TestDispatcher {
  pub fn new(test: &Test, fixture: Option<syn::Ident>) -> Self {
    Self {
      attrs: test.attrs().into(),
      sig: Self::dispatcher_signature(test.signature().clone(), fixture.as_ref()),
      fixture: fixture.map(|v| FixtureData {
        ident: v,
        arg: test.signature().inputs.first().unwrap().clone(),
      }),
    }
  }

  pub fn dispatcher_name(&self) -> &syn::Ident {
    &self.sig.ident
  }

  fn dispatcher_signature(mut sig: Signature, fixture: Option<&syn::Ident>) -> Signature {
    sig.ident = ident::new_test_dispatch(&sig.ident);
    sig.inputs = Self::dispatcher_arguments(sig.inputs, fixture);
    sig
  }

  fn dispatcher_arguments(
    inputs: Punctuated<FnArg, Comma>,
    fixture: Option<&syn::Ident>,
  ) -> Punctuated<FnArg, Comma> {
    if fixture.is_some() {
      // TODO(mrodusek): can this be done more easily? It feels like we should be
      // able to just swap a FnArg from the Punctuated.
      let mut new_inputs: Punctuated<FnArg, Comma> = Default::default();
      new_inputs.extend(inputs.into_iter().skip(1));
      new_inputs
    } else {
      inputs
    }
  }
}

impl TestDispatcher {
  pub fn to_tokens_with_call(&self, tokens: &mut TokenStream, test: &Test) {
    const FIXTURE_NAME: &str = "__fixture";
    let fixture_ident = syn::Ident::new(FIXTURE_NAME, Span::call_site());

    // Define the test function
    tokens.append_all(self.attrs.iter());
    FunctionDefinition(&self.sig).surround(tokens, |tokens| {
      self.prepend_fixture(&fixture_ident, tokens);
      FunctionCall(test.test_name()).arguments(tokens, |tokens| {
        self.prepend_fixture_arg(&fixture_ident, tokens);
        for arg in self.sig.inputs.iter() {
          arg.try_ident().to_tokens(tokens);
          Comma::default().to_tokens(tokens);
        }
      })
    });
  }

  fn prepend_fixture(&self, fixture_ident: &syn::Ident, tokens: &mut TokenStream) {
    match &self.fixture {
      Some(v) => {
        let ident = &v.ident;
        let fixture_arg = &v.arg;
        let resolve_arg_decl = ResolveFnArgDecl::new(fixture_ident, fixture_arg);

        let block: Stmt = parse_quote! {
          let #resolve_arg_decl = #ident::prepare().unwrap();
        };
        block.to_tokens(tokens);
      }
      None => {}
    }
  }

  fn prepend_fixture_arg(&self, fixture_ident: &syn::Ident, tokens: &mut TokenStream) {
    if let Some(fixture) = &self.fixture {
      let fixture_arg = &fixture.arg;
      let resolve_arg = ResolveFnArg::new(fixture_ident, fixture_arg);
      resolve_arg.to_tokens(tokens);
      Comma::default().to_tokens(tokens);
    }
  }
}
