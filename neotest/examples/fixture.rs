#[cfg(test)]
mod test {
  use neotest::{neotest, Fixture};
  #[derive(Default, Fixture)]
  struct SomeFixture {}

  impl SomeFixture {
    fn do_something(&self) {
      println!("Doing something");
    }

    fn do_something_mut(&mut self) {
      println!("Doing something mut");
    }

    fn do_something_truthy(&self) -> bool {
      true
    }
  }

  // Test with a fixture
  #[neotest(fixture = SomeFixture)]
  fn test_fixture(f: SomeFixture) {
    f.do_something();
    assert!(f.do_something_truthy());
  }

  #[neotest(fixture = SomeFixture)]
  fn test_ref_fixture(f: &SomeFixture) {
    f.do_something();
    assert!(f.do_something_truthy());
  }

  #[neotest(fixture = SomeFixture)]
  fn test_mut_fixture(mut f: SomeFixture) {
    f.do_something_mut();
    assert!(f.do_something_truthy());
  }

  #[neotest(fixture = SomeFixture)]
  fn test_mut_ref_fixture(f: &mut SomeFixture) {
    f.do_something_mut();
    assert!(f.do_something_truthy());
  }
}

fn main() {}
