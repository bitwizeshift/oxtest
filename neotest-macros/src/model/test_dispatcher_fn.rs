use quote::{ToTokens, TokenStreamExt};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_quote, Attribute, Block, FnArg, Signature};

use crate::model::{attribute, fn_arg, ident, TestImplFn};
use crate::syn_utils::{ResolveFnArg, ResolveFnArgDecl, TryIdent};

/// This models the fixture dispatch function that will be used to forward
/// a fully-constructed fixture.
pub struct TestDispatcherFn {
  pub attrs: Vec<Attribute>,
  pub sig: Signature,
  pub block: Box<Block>,
}

fn remove_fixture_arg(
  fixture: Option<&syn::Ident>,
  inputs: Punctuated<FnArg, Comma>,
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

impl TestDispatcherFn {
  pub fn new(
    fixture: Option<syn::Ident>,
    attrs: Vec<Attribute>,
    sig: Signature,
    impl_fn: &TestImplFn,
    inputs: Punctuated<syn::Expr, Comma>,
  ) -> Self {
    let maybe_fixture_arg = fixture
      .as_ref()
      .and_then(|_| sig.inputs.first().map(ToOwned::to_owned));

    let mut sig = sig;
    sig.ident = ident::new_test_dispatch(&sig.ident);
    sig.inputs = remove_fixture_arg(fixture.as_ref(), sig.inputs);
    sig.inputs.push(fn_arg::context());

    // Context is the last argument passed to tests
    let mut inputs = inputs;
    for v in sig.inputs.iter().filter_map(TryIdent::try_ident) {
      inputs.push(parse_quote!(#v));
    }

    let mut attrs = attrs;
    attrs.push(attribute::doc_hidden());
    attrs.push(attribute::allow_dead_code());

    let impl_fn_ident = impl_fn.fn_ident();

    let block: Box<Block> = Box::new(match &fixture {
      Some(v) => {
        let fixture_arg = maybe_fixture_arg.unwrap();
        let fixture_ident: syn::Ident = parse_quote! { __fixture };
        let resolve_arg = ResolveFnArg::new(&fixture_ident, &fixture_arg);
        let resolve_arg_decl = ResolveFnArgDecl::new(&fixture_ident, &fixture_arg);

        parse_quote! {
          {
            let #resolve_arg_decl = #v::prepare().unwrap();
            #impl_fn_ident(#resolve_arg, #inputs);
          }
        }
      }
      None => parse_quote! {
        {
          #impl_fn_ident(#inputs);
        }
      },
    });

    Self { attrs, sig, block }
  }

  pub fn fn_ident(&self) -> &syn::Ident {
    &self.sig.ident
  }
}

impl ToTokens for TestDispatcherFn {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    tokens.append_all(self.attrs.iter());
    self.sig.to_tokens(tokens);
    self.block.to_tokens(tokens);
  }
}
