#![allow(unused)]
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::token::{Bracket, Comma};

/// A small utility that provides a index-based path to determine which sections
/// are enabled for sub-tests.
///
/// This type implements [`ToTokens`] so that it can be serialized directly
/// a slice of a literal array. E.g. for a section-path of `0, 1, 2`, this will
/// serialize as `&[0, 1, 2]`.
#[derive(Clone, Default)]
pub struct SectionPath(Vec<usize>);

impl SectionPath {
  /// Creates a new subsection `path`
  ///
  /// # Arguments
  ///
  /// * `path` - the path of section indices
  pub fn new(path: &[usize]) -> Self {
    Self(path.into())
  }

  /// Creates a SubsectionPath with the next specified `section` index
  ///
  /// # Arguments
  ///
  /// * `section` - the section index
  pub fn subsection(&self, section: usize) -> Self {
    let mut path = self.0.clone();
    path.push(section);

    Self(path)
  }
}

impl ToTokens for SectionPath {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    syn::token::And::default().to_tokens(tokens);
    Bracket::default().surround(tokens, |ts| {
      let comma = Comma::default();
      for v in self.0.iter() {
        v.to_tokens(ts);
        comma.to_tokens(ts);
      }
    });
  }
}
