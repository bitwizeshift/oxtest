#[derive(Default, PartialEq, PartialOrd, Debug)]
pub struct Error {
  message: String,
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.message.fmt(f)
  }
}

impl std::error::Error for Error {}

impl Error {
  pub fn from_error<T: std::error::Error>(e: T) -> Self {
    Self {
      message: e.to_string(),
    }
  }

  pub fn new() -> Self {
    Self::default()
  }
}

impl<T> From<T> for Error
where
  String: From<T>,
{
  fn from(value: T) -> Self {
    Self {
      message: String::from(value),
    }
  }
}

/// A result type for unit tests.
pub type Result<T> = ::std::result::Result<T, Error>;

/// The result that is either implicitly or explicitly returned from tests.
pub type TestResult = crate::Result<()>;
