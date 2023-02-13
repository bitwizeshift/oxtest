use proc_macro::TokenStream;

mod derive_fixture;
mod fixture;

#[proc_macro_derive(Fixture)]
pub fn fixture(input: TokenStream) -> TokenStream {
  derive_fixture::derive(input)
}

#[proc_macro_attribute]
pub fn test_fixture(attribute: TokenStream, item: TokenStream) -> TokenStream {
  fixture::fixture(attribute, item)
}
