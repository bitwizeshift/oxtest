use neotest::{test_fixture, Fixture};

#[derive(Default, Fixture)]
struct SomeFixture {}

impl SomeFixture {
  fn do_some_setup(&self) {}
}

#[test_fixture(SomeFixture)]
fn test_something(f: &SomeFixture) {
  f.do_some_setup();

  assert!(true);
}

fn main() {}
