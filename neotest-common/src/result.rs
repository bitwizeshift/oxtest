/// A generic error type that is returned from all tests.
///
/// This type can polymorphically take the form of any type that implements
/// [`Error`], which allows any and all generic errors to be a valid return type
/// from tests.
///
/// [`Error`]: std::error::Error
pub type Error = Box<dyn std::error::Error>;

/// A [`Result`] object returned from utilities in this test framework.
///
/// This uses the generic [`Error`] implementation to ensure that it can return
/// any and all error objects back from test-cases.
///
/// [`Result`]: std::result::Result
pub type Result<T> = ::std::result::Result<T, Error>;

/// A [`Result`] object returned from all tests, either implicitly or explicitly.
///
/// All test results can only successfully return unit `()` objects, and so this
/// `TestResult` type pins the return type.
pub type TestResult = crate::Result<()>;
