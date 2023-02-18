use crate::Result;

/// A trait for defining fixture functionality.
///
/// This can either be directly implemented or automatically done through the
/// [`derive(Fixture)`] macro. When using the derive-macro, this will
/// implement [`Self::prepare`] by using [`Default`].
///
/// # Examples
///
/// Derive macro:
///
/// ```rust
/// # use neotest::Fixture;
/// #[derive(Default, Fixture)]
/// struct TestFixture {
///     some_state: u32,
/// }
/// ```
///
/// Manual Implementation:
///
/// ```rust
/// # use neotest::Fixture;
/// # use neotest::Result;
/// struct TestFixture {
///     some_state: u32,
/// }
///
/// impl Fixture for TestFixture {
///     fn prepare() -> Result<Self> {
///         // ...
///         // some complex preparation logic
///         // ...
///
///         Ok(Self{ some_state: 42 })
///     }
/// }
/// ```
///
/// [`derive(Fixture)`]: crate::fixture
pub trait Fixture
where
  Self: Sized,
{
  /// Prepares this fixture so that it can be used for testing.
  ///
  /// Preparation is able to fail, as indicated by returning a [`Result`].
  /// In such a case, the test will be abandoned and treated as an error.
  ///
  /// # Notes
  ///
  /// Unlike traditional unit-testing frameworks of old, there is no
  /// equivalent `tear_down()` function; rather, the implemented Fixture should
  /// also implement `Drop` if teardown logic is desired.
  fn prepare() -> Result<Self>;
}
