/// The additive identity element for the implementing type, satisfying for all `t: T` equations
/// like `t + zero<T>() == t`, etc.
pub trait Zero {
  fn zero() -> Self;
}

/// The multiplicative identity element for the implementing type, satisfying for all `t: T`
/// equations like `t * one<T>() == t`, etc.
pub trait One {
  fn one() -> Self;
}

impl Zero for f32 {
  fn zero() -> f32 {
    return 0.0;
  }
}
impl One for f32 {
  fn one() -> f32 {
    return 1.0;
  }
}

impl Zero for f64 {
  fn zero() -> f64 {
    return 0.0;
  }
}
impl One for f64 {
  fn one() -> f64 {
    return 1.0;
  }
}

#[cfg(test)]
mod tests {
  use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Neg, Sub},
  };

  use super::{One, Zero};

  trait Arithmetic<T = Self>:
    Add<T, Output = T>
    + Sub<T, Output = T>
    + Mul<T, Output = T>
    + Div<T, Output = T>
    + Neg<Output = T>
    + Zero
    + One
  {
  }
  impl Arithmetic for f32 {}
  impl Arithmetic for f64 {}

  fn assert_additive_identity<T: Arithmetic<T> + PartialEq<T> + Copy + Debug>(ts: &[T]) {
    for &t in ts {
      assert_eq!(t + T::zero(), t);
      assert_eq!(T::zero() + t, t);
      assert_eq!(t - T::zero(), t);
      assert_eq!(T::zero() - t, -t);
      assert_eq!(t * T::zero(), T::zero());
      assert_eq!(T::zero() * t, T::zero());
      assert_eq!(-T::zero(), T::zero());
    }
  }

  fn assert_multiplicative_identity<T: Arithmetic<T> + PartialEq<T> + Copy + Debug>(ts: &[T]) {
    for &t in ts {
      assert_eq!(t * T::one(), t);
      assert_eq!(T::one() * t, t);
      assert_eq!(t / T::one(), t);
    }
  }

  static F32_TEST_VALUES: &[f32] = &[-1.0, 0.0, 1.0, std::f32::consts::PI, std::f32::consts::E];

  #[test]
  fn test_f32_zero_is_additive_identity() {
    assert_additive_identity::<f32>(F32_TEST_VALUES);
  }

  #[test]
  fn test_f32_one_is_multiplicative_identity() {
    assert_multiplicative_identity::<f32>(F32_TEST_VALUES);
  }

  static F64_TEST_VALUES: &[f64] = &[-1.0, 0.0, 1.0, std::f64::consts::PI, std::f64::consts::E];

  #[test]
  fn test_f64_zero_is_additive_identity() {
    assert_additive_identity::<f64>(F64_TEST_VALUES);
  }

  #[test]
  fn test_f64_one_is_multiplicative_identity() {
    assert_multiplicative_identity::<f64>(F64_TEST_VALUES);
  }
}
