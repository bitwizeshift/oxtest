//! This module exposes various [`Matcher`]s for the [`crate::neotest`] library.
//!
//! The [`Matcher`] abstraction is a way to semantically convey a specific
//! criteria to match on in a higher abstraction in a potentially stateful way.
//!
//! Since matchers can be stateful, this can be a useful way to convey more
//! complicated condition-criterias for testing purposes, which can then be
//! distilled to easier assertions.

/// An abstraction for handling "Matching" when used in expectations, mocking,
/// and assertions.
///
/// A "Matcher" is conceptually a unary function which evaluates its operand
/// for whether it passes a given test. This primarily exists for supplying up
/// expectations for mocking operations.
pub trait Matcher<Rhs = Self> {
  fn matches(&self, v: Rhs) -> bool;
}

/// A [`Matcher`] that always returns `true`
///
/// This is effectively an "identity" matcher.
#[derive(Copy, Clone)]
pub struct Any;

impl<T> Matcher<T> for Any {
  #[inline]
  fn matches(&self, _: T) -> bool {
    true
  }
}

macro_rules! implement_order_matchers {
  ($($Name:ident($Trait:ident::$Fn:ident);)+) => {
    $(
      #[derive(Default, Copy, Clone)]
      pub struct $Name<T>(pub T)
      where
        T: $Trait;

      impl<T, U> Matcher<U> for $Name<T>
      where
        T: $Trait,
        U: $Trait<T>,
      {
        #[inline]
        fn matches(&self, v: U) -> bool {
          v.$Fn(&self.0)
        }
      }
    )+
  }
}

implement_order_matchers! {
  Le(PartialOrd::le);
  Ge(PartialOrd::ge);
  Lt(PartialOrd::lt);
  Gt(PartialOrd::gt);
  Eq(PartialEq::eq);
  Ne(PartialEq::ne);
}

/// A [`Matcher`] that inverts the result of another matcher.
///
/// This is a simple composition object so that named matchers are be used in
/// larger constructions.
pub struct Not<T>(pub T);

impl<T, U> Matcher<U> for Not<T>
where
  T: Matcher<U>,
{
  #[inline]
  fn matches(&self, v: U) -> bool {
    !self.0.matches(v)
  }
}

/// A [`Matcher`] that expects the tested value to simply be `false`.
pub struct IsFalse;

impl Matcher<bool> for IsFalse {
  #[inline]
  fn matches(&self, v: bool) -> bool {
    !v
  }
}

/// A [`Matcher`] that expects the tested value to simply be `true`.
pub struct IsTrue;

impl Matcher<bool> for IsTrue {
  #[inline]
  fn matches(&self, v: bool) -> bool {
    v
  }
}

/// A [`Matcher`] that expects the tested value to be convertible to `false`
/// (e.g. is "falsey").
pub struct IsFalsey;

impl<T> Matcher<T> for IsFalsey
where
  bool: From<T>,
{
  #[inline]
  fn matches(&self, v: T) -> bool {
    !bool::from(v)
  }
}

/// A [`Matcher`] that expects the tested value to be convertible to `true`
/// (e.g. is "truthy").
pub struct IsTruthy;

impl<T> Matcher<T> for IsTruthy
where
  bool: From<T>,
{
  #[inline]
  fn matches(&self, v: T) -> bool {
    bool::from(v)
  }
}

/// A [`Matcher`] that expects the result to be a [`Some`] value for an optional.
pub struct IsSome;

impl<T> Matcher<Option<T>> for IsSome {
  #[inline]
  fn matches(&self, v: Option<T>) -> bool {
    v.is_some()
  }
}

/// A [`Matcher`] that expects the result to be a [`None`].
pub struct IsNone;

impl<T> Matcher<Option<T>> for IsNone {
  #[inline]
  fn matches(&self, v: Option<T>) -> bool {
    v.is_none()
  }
}

impl<T: PartialEq> Matcher<T> for T {
  #[inline]
  fn matches(&self, v: T) -> bool {
    self.eq(&v)
  }
}

#[cfg(test)]
mod test {
  use neotest_macros::subtest;

  use super::*;

  #[crate::neotest]
  fn test_le() {
    const VALUE: u32 = 5;
    let matcher = Le(VALUE);

    subtest! {|matches_less|
      assert!(matcher.matches(VALUE - 1));
    }
    subtest! {|matches_equal|
      assert!(matcher.matches(VALUE));
    }
    subtest! {|does_not_match_greater|
      assert!(!matcher.matches(VALUE + 1));
    }
  }

  #[crate::neotest]
  fn test_ge() {
    const VALUE: u32 = 5;
    let matcher = Ge(VALUE);

    subtest! {|matches_greater|
      assert!(matcher.matches(VALUE + 1));
    }
    subtest! {|matches_equal|
      assert!(matcher.matches(VALUE));
    }
    subtest! {|does_not_match_less|
      assert!(!matcher.matches(VALUE - 1));
    }
  }

  #[crate::neotest]
  fn test_lt() {
    const VALUE: u32 = 5;
    let matcher = Lt(VALUE);

    subtest! {|matches_less|
      assert!(matcher.matches(VALUE - 1));
    }
    subtest! {|does_not_match_equal|
      assert!(!matcher.matches(VALUE));
    }
    subtest! {|does_not_match_greater|
      assert!(!matcher.matches(VALUE + 1));
    }
  }

  #[crate::neotest]
  fn test_gt() {
    const VALUE: u32 = 5;
    let matcher = Gt(VALUE);

    subtest! {|matches_greater|
      assert!(matcher.matches(VALUE + 1));
    }
    subtest! {|does_not_match_equal|
      assert!(!matcher.matches(VALUE));
    }
    subtest! {|does_not_match_less|
      assert!(!matcher.matches(VALUE - 1));
    }
  }

  #[crate::neotest]
  fn test_eq() {
    const VALUE: u32 = 5;
    let matcher = Eq(VALUE);

    subtest! {|does_not_match_greater|
      assert!(!matcher.matches(VALUE + 1));
    }
    subtest! {|matches_equal|
      assert!(matcher.matches(VALUE));
    }
    subtest! {|does_not_match_less|
      assert!(!matcher.matches(VALUE - 1));
    }
  }

  #[crate::neotest]
  fn test_ne() {
    const VALUE: u32 = 5;
    let matcher = Ne(VALUE);

    subtest! {|matches_greater|
      assert!(matcher.matches(VALUE + 1));
    }
    subtest! {|does_not_match_equal|
      assert!(!matcher.matches(VALUE));
    }
    subtest! {|matches_less|
      assert!(matcher.matches(VALUE - 1));
    }
  }

  #[crate::neotest]
  fn test_any() {
    let matcher = Any;

    subtest! {|matches_int|
      assert!(matcher.matches(5));
    }
    subtest! {|matches_string|
      assert!(matcher.matches("hello world"));
    }
  }

  #[crate::neotest]
  fn test_not() {
    const VALUE: u32 = 5;
    let matcher = Not(Ge(VALUE)); // lt

    subtest! {|negates_input|
      subtest! {|does_not_match_greater|
        assert!(!matcher.matches(VALUE + 1));
      }
      subtest! {|does_not_match_equal|
        assert!(!matcher.matches(VALUE));
      }
      subtest! {|matches_less|
        assert!(matcher.matches(VALUE - 1));
      }
    }
  }

  #[crate::neotest]
  fn test_is_true() {
    let matcher = IsTrue;

    subtest! {|matches_true|
      assert!(matcher.matches(true));
    }
    subtest! {|does_not_match_false|
      assert!(!matcher.matches(false));
    }
  }

  #[crate::neotest]
  fn test_is_false() {
    let matcher = IsFalse;

    subtest! {|matches_false|
      assert!(matcher.matches(false));
    }
    subtest! {|does_not_match_true|
      assert!(!matcher.matches(true));
    }
  }

  struct BoolLike(bool);

  impl From<BoolLike> for bool {
    fn from(value: BoolLike) -> Self {
      value.0
    }
  }

  #[crate::neotest]
  fn test_is_truthy() {
    let matcher = IsTruthy;

    subtest! {|matches_truthy_value|
      assert!(matcher.matches(BoolLike(true)));
    }
    subtest! {|does_not_match_falsey_value|
      assert!(!matcher.matches(BoolLike(false)));
    }
  }

  #[crate::neotest]
  fn test_is_falsey() {
    let matcher = IsFalsey;

    subtest! {|matches_falsey_value|
      assert!(matcher.matches(BoolLike(false)));
    }
    subtest! {|does_not_match_truthy_value|
      assert!(!matcher.matches(BoolLike(true)));
    }
  }

  #[crate::neotest]
  fn test_is_none() {
    let matcher = IsNone;

    subtest! {|matches_none|
      assert!(matcher.matches(None::<()>));
    }
    subtest! {|does_not_match_some|
      assert!(!matcher.matches(Some(42)));
    }
  }

  #[crate::neotest]
  fn test_is_some() {
    let matcher = IsSome;

    subtest! {|matches_some|
      assert!(matcher.matches(Some(42)));
    }
    subtest! {|does_not_match_none|
      assert!(!matcher.matches(None::<()>));
    }
  }
}
