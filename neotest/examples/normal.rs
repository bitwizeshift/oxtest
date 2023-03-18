#[cfg(test)]
mod test {
  use neotest::neotest;
  use neotest_common::TestResult;

  // Test without a fixture
  #[neotest]
  fn test_normal() {}

  #[neotest]
  fn test_normal_with_return() -> TestResult {
    Ok(())
  }
}

fn main() {}
