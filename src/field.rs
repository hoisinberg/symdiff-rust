use std::ops::{Add, Div, Mul, Neg, Sub};

/// A type whose instances and operations are assumed to satisfy the axioms that define a [field](
/// https://en.wikipedia.org/wiki/Field_(mathematics)) in the mathematical sense.
pub trait Field<T = Self>:
  Add<T, Output = Self>
  + Sub<T, Output = Self>
  + Mul<T, Output = Self>
  + Div<T, Output = Self>
  + Neg<Output = Self>
  + Copy
{
  /// The additive identity element of the field, satisfying for all `t: T` equations like
  /// `t + zero<T>() == t`, etc.
  fn zero() -> Self;

  /// The multiplicative identity element of the field, satisfying for all `t: T` equations like
  /// `t * one<T>() == t`, etc.
  fn one() -> Self;
}

impl Field for f32 {
  fn zero() -> f32 {
    return 0.0;
  }
  fn one() -> f32 {
    return 1.0;
  }
}

impl Field for f64 {
  fn zero() -> f64 {
    return 0.0;
  }
  fn one() -> f64 {
    return 1.0;
  }
}

#[cfg(test)]
mod tests {
  use std::fmt::Debug;

  use crate::field::Field;

  fn assert_additive_identity<T: Field + PartialEq<T> + Debug>(ts: &[T]) {
    for &t in ts {
      assert_eq!(t + T::zero(), t);
      assert_eq!(T::zero() + t, t);
      assert_eq!(t * T::zero(), T::zero());
      assert_eq!(T::zero() * t, T::zero());
      assert_eq!(-T::zero(), T::zero());
    }
  }

  fn assert_multiplicative_identity<T: Field + PartialEq<T> + Debug>(ts: &[T]) {
    for &t in ts {
      assert_eq!(t * T::one(), t);
      assert_eq!(T::one() * t, t);
      assert_eq!(t / T::one(), t);
    }
  }

  static F32_TEST_VALUES:&[f32] = &[-1.0, 0.0, 1.0, std::f32::consts::PI, std::f32::consts::E];

  #[test]
  fn test_f32_zero_is_additive_identity() {
    assert_additive_identity::<f32>(F32_TEST_VALUES);
  }

  #[test]
  fn test_f32_one_is_multiplicative_identity() {
    assert_multiplicative_identity::<f32>(F32_TEST_VALUES);
  }

  static F64_TEST_VALUES:&[f64] = &[-1.0, 0.0, 1.0, std::f64::consts::PI, std::f64::consts::E];

  #[test]
  fn test_f64_zero_is_additive_identity() {
    assert_additive_identity::<f64>(F64_TEST_VALUES);
  }

  #[test]
  fn test_f64_one_is_multiplicative_identity() {
    assert_multiplicative_identity::<f64>(F64_TEST_VALUES);
  }
}
