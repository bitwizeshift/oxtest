/// The undocumented [`__Context`] object is an internal neotest utility that
/// tells the test driver which sections to execute.
///
/// This enables subsections of tests to each be executed independently.
#[doc(hidden)]
pub struct __Context {
  section_path: Vec<usize>,
}

#[allow(dead_code)]
impl __Context {
  /// Creates a new [`__Context`] that has the specified section path
  ///
  /// # Arguments
  ///
  /// * section_path - the path of sections to take to test
  #[doc(hidden)]
  pub fn path(section_path: &[usize]) -> Self {
    Self {
      section_path: section_path.into(),
    }
  }

  /// Creates a new [`__Context`] that is corresponds to executing _all_ tests.
  ///
  /// This is a special context that is supplied for the top-level test-runners.
  #[doc(hidden)]
  pub fn all_tests() -> Self {
    Self {
      section_path: Default::default(),
    }
  }

  /// Queries whether the specified section `path` is currently enabled in a
  /// given test.
  ///
  /// This will return `true` in one of two conditions:
  ///
  /// 1. The [`__Context`]'s path is shorter than `path`. This is used to determine
  ///    "root" tests. This allows a base section to execute all sub-tests.
  #[doc(hidden)]
  pub fn section_enabled(&self, path: &[usize]) -> bool {
    for (i, path_segment) in path
      .iter()
      .enumerate()
      .take(self.section_path.len().min(path.len()))
    {
      if self.section_path[i] != *path_segment {
        return false;
      }
    }
    true
  }
}
