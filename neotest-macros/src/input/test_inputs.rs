//! This internal-module defines the set of possible test options that can be
//! specified in attribute parameters.
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{FnArg, Generics, Signature};

use crate::syn_utils::TryIdent;

use super::{ConstParameterInput, FixtureInput, ParameterInput, TypeParameterInput};

#[derive(Clone)]
pub struct TestInputs {
  pub fixture: Option<FixtureInput>,
  pub parameters: Vec<ParameterInput>,
  pub const_parameters: Vec<ConstParameterInput>,
  pub type_parameters: Vec<TypeParameterInput>,
}

impl TestInputs {
  /// Reorders all test inputs to be in the same order as parameters defined in
  /// the [`Signature`].
  ///
  /// # Arguments
  ///
  /// * `sig` - the signature to follow the order of.
  pub fn reorder(&mut self, sig: &Signature) {
    self.parameters.sort_by(|a, b| {
      Self::index_of_arg(&sig.inputs, &a.ident).cmp(&Self::index_of_arg(&sig.inputs, &b.ident))
    })
  }

  /// Finds the index of the specified ident in the list of function args
  ///
  /// # Arguments
  ///
  /// * `args` - the list of arguments
  /// * `ident` - the ident to find
  fn index_of_arg(args: &Punctuated<FnArg, Comma>, ident: &syn::Ident) -> usize {
    args
      .iter()
      .filter_map(TryIdent::try_ident)
      .position(|v| *ident == *v)
      .unwrap()
  }

  /// Finds the generic index of the specified ident in the list of function args
  ///
  /// # Arguments
  ///
  /// * `args` - the list of arguments
  /// * `ident` - the ident to find
  #[allow(unused)]
  fn index_of_generic(args: &Generics, ident: &syn::Ident) -> usize {
    args
      .params
      .iter()
      .filter_map(TryIdent::try_ident)
      .position(|v| *ident == *v)
      .unwrap()
  }
}
