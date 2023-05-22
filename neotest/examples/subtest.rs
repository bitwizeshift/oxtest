#[cfg(test)]
mod test {
  use neotest::{neotest, subtest};

  #[neotest]
  fn test_string_default() {
    let sut = String::default();

    subtest!(string_is_empty, {
      assert!(sut.is_empty());
    });
    subtest!(string_has_zero_len, {
      assert_eq!(sut.len(), 0);
    })
  }

  #[neotest(parameter = cap as [10, 42, 64])]
  fn test_vec_with_capacity(cap: usize) {
    let sut: Vec<i32> = Vec::with_capacity(cap);

    subtest!(capacity_is_set_to_input, {
      assert_eq!(sut.capacity(), cap);
    });
    subtest!(vec_is_empty, {
      assert!(sut.is_empty());
    });
    subtest!(vec_size_is_zero, {
      assert_eq!(sut.len(), 0);
    })
  }
}

fn main() {}
