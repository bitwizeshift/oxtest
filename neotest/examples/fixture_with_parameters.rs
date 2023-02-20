#[cfg(test)]
mod test {
  use neotest::{neotest, Fixture};
  #[derive(Default, Fixture)]
  struct SomeFixture {}

  impl SomeFixture {
    fn test_something(&self, a: u32, b: u32) {
      assert_ne!(a, b);
    }
  }

  #[neotest(
    fixture = SomeFixture,
    // Order does not need to match input order
    parameter = b as ["3", "4"],
    parameter = a as [1, 2, 5],
  )]
  fn test_fixture_with_parameter(fixture: SomeFixture, a: u32, b: &str) {
    let b_int = u32::from_str_radix(b, 10).expect("this is an int");
    fixture.test_something(a, b_int);
  }
}

fn main() {}
