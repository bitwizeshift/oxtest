use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse_quote, Signature, Stmt, VisPublic, Visibility};

use crate::suite::TestAttributes;
use crate::syn_utils::{FunctionDefinition, ModuleDefinition};

use super::{DispatcherCall, Test, TestDispatcher, TestParameters};

/// The name of a test executor function.
///
/// This is a function that will act as an entry-point into other tests.
/// This will evaluate into a function definition in the form of:
///
/// ```ignore
/// #[test]
/// /* other attributes */
/// fn test_name() {
///   test_name::sub_test_1();
///   /* any sub tests or input tests */
/// }
/// mod test_name {
///   use super::*;
///   fn sub_test_1() {
///      let __context = ::neotest_common::__internal::__Context::path(&[]);
///      __neotest_test_name_dispatch(param0, param1, __context);
///   }
///   mod sub_test_1 {}
/// }
/// ```
pub struct TestExecutor {
  attrs: TestAttributes,
  sig: Signature,
  dispatch_call: DispatcherCall,
  subtests: Vec<TestExecutor>,
}

impl TestExecutor {
  pub fn new(name: syn::Ident, attrs: TestAttributes, params: TestParameters, test: &Test) -> Self {
    Self {
      attrs,
      dispatch_call: if params.is_empty() {
        DispatcherCall::default()
      } else {
        DispatcherCall::new(params)
      },
      sig: Self::executor_signature(test.signature().clone(), name),
      subtests: Default::default(),
    }
  }

  pub fn executor_name(&self) -> &syn::Ident {
    &self.sig.ident
  }

  /// Creates a derived subtest with a new index, and also provides
  pub fn push_subtest<F>(&mut self, subsection: usize, name: syn::Ident, f: F)
  where
    F: FnOnce(&mut TestExecutor),
  {
    let dispatcher = self.dispatch_call.subsection(subsection);
    let mut new_sig = self.sig.clone();
    new_sig.ident = name;

    let mut test = Self {
      attrs: self.attrs.clone(),
      dispatch_call: dispatcher,
      sig: new_sig,
      subtests: Default::default(),
    };

    f(&mut test);

    self.subtests.push(test);
  }

  pub fn to_tokens_with_call(&self, tokens: &mut TokenStream, dispatcher: &TestDispatcher) {
    self.attrs.to_tokens(tokens);

    Visibility::Public(VisPublic {
      pub_token: Default::default(),
    })
    .to_tokens(tokens);
    FunctionDefinition(&self.sig).surround(tokens, |tokens| {
      self.dispatch_call.to_tokens_with_call(tokens, dispatcher);
    });

    // Only define submodules when we have subtests to run
    if self.subtests.is_empty() {
      return;
    }

    // Define all subtests inside of the module definition
    ModuleDefinition(&self.sig.ident).surround(tokens, |tokens| {
      let use_stmt: Stmt = parse_quote! { use super::*; };
      use_stmt.to_tokens(tokens);

      for subtest in self.subtests.iter() {
        subtest.to_tokens_with_call(tokens, dispatcher);
      }
    });
  }

  fn executor_signature(mut sig: Signature, ident: syn::Ident) -> Signature {
    sig.ident = ident;
    sig.inputs.clear();
    sig
  }
}
