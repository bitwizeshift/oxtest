use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn derive(input: TokenStream) -> TokenStream {
  let DeriveInput { ident, .. } = parse_macro_input!(input);

  let output = quote! {
    impl ::neotest::Fixture for #ident {
      fn prepare() -> ::neotest::Result<Self> {
        Ok(Default::default())
      }
    }
  };
  output.into()
}
