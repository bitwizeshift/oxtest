use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

mod derive_fixture;
mod input;
mod model;
mod syn_utils;

/// A derive-macro for automatically implementing the [`Fixture`] trait.
///
/// When deriving from [`Fixture`], it's necessary to also implement [`Default`].
/// It's recommended to do this through the derive-macro the same way; otherwise
/// it'd be clearer to just implement [`Fixture`] directly and define the `prepare`
/// function for setup.
///
/// # Examples
///
/// Basic use:
///
/// ```
/// # use crate::Fixture
/// #[derive(Default, Fixture)]
/// struct MyFixture {
///     // ...
/// }
/// ```
///
/// [`Fixture`]: neotest_common::Fixture
#[proc_macro_derive(Fixture)]
pub fn fixture(input: TokenStream) -> TokenStream {
  derive_fixture::derive(input)
}

/// An attribute that indicates that an impl block is a test fixture.
///
/// This macro enables type fixtures to be written using `self` arguments rather
/// than through argument injection. For example, take this typical
/// injected fixture:
///
/// ```
/// # use neotest::Fixture;
/// #[derive(Default, Fixture)]
/// struct TestFixture {};
///
/// #[neotest(
///     fixture = testFixture,
///     parameter = a as [42, 32]
/// )]
/// fn test_some_thing(f: TestFixture, a: u32) {
///     // some test
/// }
/// ```
///
/// This can be rewritten as:
///
/// ```
/// # use neotest::neotest_fixture;
/// # use neotest::Fixture;
/// #[derive(Default, Fixture)]
/// struct TestFixture {};
///
/// #[neotest_fixture]
/// impl TestFixture {
///     #[neotest(parameter = a as [42, 32])]
///     fn test_some_thing(self, a: u32) {
///         // some test
///     }
/// }
/// ```
///
/// Functionally, these two will be the exact same; so which is preferred is
/// just a matter of preference.
#[proc_macro_attribute]
pub fn neotest_fixture(attribute: TokenStream, item: TokenStream) -> TokenStream {
  let _ = parse_macro_input!(attribute as syn::parse::Nothing);

  item
}

/// A helper macro that will invoke an expression and generate a compile-error
/// if it generates an error.
macro_rules! syn_try {
  ($Expr:expr) => {
    match $Expr {
      Ok(data) => data,
      Err(err) => {
        return TokenStream::from(err.to_compile_error());
      }
    }
  };
}

/// The primary macro used for creating neotests
///
/// The minimal version of this macro will evaluate into just a normal Rust
/// `#[test]`. Additional features are also supported:
///
/// * `fixture` which creates test-fixtures.
///   This is in the form:
///   ```text
///   fixture = <Fixture name>
///   ```
///   This parameter can only be specified at most once per test.
///
/// * `parameter` which produces parameter-based inputs.
///   This is in the form:
///   ```text
///   parameter = <param name> as [<expr0>, <expr1>, ...]
///   ```
///   This parameter can be specified multiple times per test, but only once per
///   `<param name>`.
///
/// * `type_parameter` which produces generic type inputs
///   This is in the form:
///   ```text
///   type_parameter = <generic type input name> as [<type0>, <type1> ...]
///   ```
///   This parameter can be specified multiple times per test, but only once per
///   `<generic type input name>`.
///
/// * `const_parameter` which produces generic const inputs
///   This is in the form:
///   ```text
///   const_parameter = <generic const input name> as [<expr0>, <expr1>, ...]
///   ```
///   This parameter can be specified multiple times per test, but only once per
///   `<generic const input name>`.
///
/// When executing tests with multiple parameter inputs, this will produce the
/// complete graph of all possible inputs -- e.g. for parameters `(a, b)` with
/// inputs `a as [0, 1]`  and `b as [2, 3]`, this will call the test 4 times
/// with the following input combinations:
///
/// * `a = 0`, `b = 2`,
/// * `a = 0`, `b = 3`,
/// * `a = 1`, `b = 2`,
/// * `a = 1`, `b = 3`,
///
/// To execute tests with the same sets of inputs, use tuples instead:
///
/// ```
/// #[neotest(parameter = a as [(1, 1), (2, 2), /* etc */])]
/// fn test_something(a: (u32, u32)){ /* ... */ }
/// ```
///
/// # Examples
///
/// Basic use:
///
/// ```
/// use neotest::neotest;
///
/// // Equivalent to just #[test]
/// #[neotest]
/// fn test_something() {
///     /* ... */
/// }
/// ```
///
/// Using fixtures:
///
/// ```
/// use neotest::neotest;
/// use neotest::Fixture;
///
/// // Derive macro can be used to create fixture, or can be implemented
/// // directly with `impl Fixture for TestFixture`.
/// #[derive(Default, Fixture)]
/// struct TestFixture {
///     /* ... */
/// }
///
/// // calls TestFixture::set_up on begin, and TestFixture::tear_down on end
/// #[neotest(fixture = TestFixture)]
/// fn test_something_with_fixture(f: &TestFixture) {
///     /* ... */
/// }
/// ```
///
/// Test parameter inputs:
///
/// ```
/// use neotest::neotest;
///
/// // Calls test_something_with_parameter with 0xdead and 0xbeef
/// #[neotest(parameter = a as [0xdead, 0xbeef])]
/// fn test_something_with_parameter(a: u32) {
///     /* ... */
/// }
/// ```
///
/// Test generic type-parameter inputs:
///
/// ```
/// use neotest::neotest;
///
/// // Calls test_something_with_generic_type_parameter with T as u32 and u64
/// #[neotest(type_parameter = T as [u32, u64])]
/// fn test_something_with_generic_type_parameter<T>() {
///     /* ... */
/// }
/// ```
///
/// Test generic const-parameter inputs:
///
/// ```
/// use neotest::neotest;
///
/// // Calls test_something_with_generic_const_parameter with VALUE as 0xdead
/// // and 0xbeef
/// #[neotest(const_parameter = VALUE as [0xdead, 0xbeef])]
/// fn test_something_with_generic_const_parameter<const VALUE: u32>() {
///     /* ... */
/// }
/// ```
///
/// Combined with everything:
///
/// ```
/// use neotest::neotest;
/// use neotest::Fixture;
///
/// // Derive macro can be used to create fixture, or can be implemented
/// // directly with `impl Fixture for TestFixture`.
/// #[derive(Default)]
/// struct TestFixture {
///     /* ... */
/// }
///
/// impl Fixture for TestFixture {
///    fn set_up(&mut self) { /* set up logic */ }
///    fn tear_down(&mut self) { /* tear-down logic */ }
/// }
///
/// // Calls test_something_with_everything with all combinations of inputs.
/// //
/// // In other words, calls with the following:
/// // * T = u32
/// //   * VALUE = 0xdead
/// //     * (a, b) = (1, 4), (1, 5), (2, 4), (2, 5), (3, 4), (3, 5)
/// //   * VALUE = 0xbeef
/// //     * (a, b) = (1, 4), (1, 5), (2, 4), (2, 5), (3, 4), (3, 5)
/// // * T = u64
/// //   * VALUE = 0xdead
/// //     * (a, b) = (1, 4), (1, 5), (2, 4), (2, 5), (3, 4), (3, 5)
/// //   * VALUE = 0xbeef
/// //     * (a, b) = (1, 4), (1, 5), (2, 4), (2, 5), (3, 4), (3, 5)
/// #[neotest(
///     fixture = TestFixture,
///     parameter = a as [1, 2, 3],
///     parameter = b as [4, 5],
///     type_parameter = T as [u32, i32]
///     const_parameter = VALUE as [0xdead, 0xbeef]
/// )]
/// fn test_something_with_everything<const VALUE: u32, T>(f: &TestFixture, a: T, b: T) {
///   /* ... */
/// }
/// ```
#[proc_macro_attribute]
pub fn neotest(attribute: TokenStream, item: TokenStream) -> TokenStream {
  // Parse input
  let input = syn::parse_macro_input!(attribute as input::TestInputs);
  let item = syn::parse_macro_input!(item as syn::ItemFn);

  // Form model
  let model = syn_try! { model::TestModel::from_inputs(input, item) };

  // Generate output
  model.to_token_stream().into()
}

/// An exposition-macro for test sections.
///
/// This is used to provide test-names for tests
///
/// # Examples
///
/// ```
/// # use crate::neotest
/// # use crate::section
/// #[neotest]
/// fn test_something() {
///     let sut: Vec<u8> = Vec::default();
///
///     #[section(creates_empty_vector)] {
///         #[section(has_0_capacity)] {
///             assert_eq!(sut.capacity(), 0);
///         }
///         #[section(has_0_size)] {
///             assert_eq!(sut.len(), 0);
///         }
///     }
/// }
/// ```
///
/// # Note
///
/// This macro only exists to give proper intellisense. Real sections will be
/// substituted by the [`neotest`] attribute.
///
/// [`neotest`]: macro@crate::neotest
#[proc_macro_attribute]
pub fn section(attribute: TokenStream, item: TokenStream) -> TokenStream {
  if !attribute.is_empty() {
    let _ = syn::parse_macro_input!(attribute as syn::Ident);
  }
  let item = syn::parse_macro_input!(item as syn::Block);

  item.to_token_stream().into()
}
