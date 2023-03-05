/// The undocumented [`__Context`] object is an internal neotest utility that
/// tells the test driver which sections to execute.
///
/// This enables subsections of tests to each be executed independently.
#[doc(hidden)]
pub struct __Context {
  section_path: &'static [usize],
  current: usize,
}

#[allow(dead_code)]
impl __Context {
  /// Creates a new [`__Context`] that has the specified section path
  ///
  /// # Arguments
  ///
  /// * section_path - the path of sections to take to test
  #[doc(hidden)]
  pub fn path(section_path: &'static [usize]) -> Self {
    Self {
      section_path,
      current: 0,
    }
  }

  /// Tests whether a context is allowed to execute a subtest
  ///
  /// # Developer Note
  ///
  /// This function is `mut` as this internally counts how many subtests have
  /// been tested to determine whether the current one is executable.
  pub fn can_execute_subtest(&mut self) -> bool {
    let current = self.current;
    self.current += 1;
    self.test_enabled(current)
  }

  /// Produce a sub-context for a subtest
  pub fn subtest(&self) -> Self {
    Self {
      section_path: self.pop_prefix(),
      current: 0,
    }
  }

  /// Tests whether the test at the specified index is enabled
  ///
  /// # Arguments
  ///
  /// * `test` - the index of the subtest to test
  fn test_enabled(&self, test: usize) -> bool {
    self.section_path.is_empty() || self.section_path[0] == test
  }

  /// Removes the first element of the [`__Context`], if possible.
  fn pop_prefix(&self) -> &'static [usize] {
    if self.section_path.is_empty() {
      self.section_path
    } else {
      &self.section_path[1..]
    }
  }
}
