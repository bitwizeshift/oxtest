use proc_macro::{self, TokenStream};
use proc_macro2::Span;
use quote::quote;
use syn::parse_macro_input;

pub(crate) fn fixture(attribute: TokenStream, item: TokenStream) -> TokenStream {
  let item_fn = parse_macro_input!(item as syn::ItemFn);
  let sig = item_fn.sig;

  assert!(sig.inputs.len() == 1, "Expects one fixture argument");
  let fixture_arg = sig.inputs.first().unwrap();
  let fixture_arg_pattern = match fixture_arg {
    syn::FnArg::Typed(val) => val,
    _ => panic!("Expected value"),
  };
  let fixture_arg_type_ident = &fixture_arg_pattern.ty;
  let fixture_arg_value_ident = match &*fixture_arg_pattern.pat {
    syn::Pat::Ident(ref ident) => ident.clone(),
    _ => panic!(""),
  };

  let test_fn_ident = sig.ident;
  let test_fn_block = item_fn.block;

  let fixture_ident = parse_macro_input!(attribute as syn::Ident);
  let fixture_fn_name: String =
    "__".to_string() + &test_fn_ident.to_string() + &"_fixture_dispatch".to_string();
  let fixture_fn_ident = syn::Ident::new(&fixture_fn_name, Span::call_site());

  let output = quote! {
    #[doc(hidden)]
    mod #test_fn_ident {
      use super::*;

      // Not used... yet
    }

    #[doc(hidden)]
    fn #fixture_fn_ident(#fixture_arg_value_ident: #fixture_arg_type_ident) {
      #test_fn_block
    }

    #[test]
    fn #test_fn_ident() {
      let mut fixture = #fixture_ident::default();

      fixture.set_up();
      #fixture_fn_ident(&mut fixture);
      fixture.tear_down();
    }
  };

  output.into()
}
