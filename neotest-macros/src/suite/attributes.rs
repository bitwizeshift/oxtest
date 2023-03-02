#![allow(unused)]
use std::rc::Rc;

use quote::{ToTokens, TokenStreamExt};
use syn::Attribute;

use crate::common::attribute;

#[derive(Clone)]
pub struct TestAttributes {
  attrs: Rc<Vec<Attribute>>,
}

impl TestAttributes {
  /// Constructs a new [`TestAttributes`] set from the given `attrs`
  ///
  /// # Arguments
  ///
  /// * `attrs` - the attributes to share to all test executors
  pub fn new(mut attrs: Vec<Attribute>) -> Self {
    if !Self::has_test_attribute(&attrs) {
      attrs.push(attribute::test())
    }
    Self {
      attrs: Rc::new(attrs),
    }
  }

  /// Constructs a new [`TestAttributes`] set from the given `attrs` slice
  ///
  /// # Arguments
  ///
  /// * `attrs` - the attributes to share to all test executors
  pub fn from_slice(attrs: &[Attribute]) -> Self {
    Self::new(attrs.into())
  }

  /// Returns an iterator over all attributes
  pub fn iter(&self) -> impl Iterator<Item = &Attribute> {
    self.attrs.iter()
  }

  /// A utility for testing whether the attributes contains the `#[test]`
  /// attribute.
  ///
  /// # Arguments
  ///
  /// * `attrs` - the attributes
  fn has_test_attribute(attrs: &[Attribute]) -> bool {
    attrs
      .iter()
      .map(|v| v.path.get_ident())
      .filter_map(|v| v)
      .any(|v| *v == "test")
  }
}

impl ToTokens for TestAttributes {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    tokens.append_all(self.attrs.iter())
  }
}
