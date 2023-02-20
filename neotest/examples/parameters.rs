#[cfg(test)]
mod test {
  use neotest::neotest;

  #[neotest(
    // Order does not need to match input order
    parameter = b as ["3", "4"],
    parameter = a as [1, 2, 5],
  )]
  fn test_parameter(a: u32, b: &str) {
    let b_int = u32::from_str_radix(b, 10).expect("this is an int");
    assert_ne!(a, b_int);
  }

  #[neotest(
    // To avoid combinatorics, tuples can be used for inputs
    parameter = a as [(1,1), (2,2), (3,3)]
  )]
  fn test_tuple_parameter(a: (u32, u32)) {
    let expect = a.0 * a.1;
    let value = a.0 * a.0;

    assert_eq!(value, expect);
  }
}

fn main() {}
