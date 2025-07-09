//! This module implements the `PartialEq` and `PartialOrd` traits for `CheckedF64`.
//!
//! The `PartialEq` trait allows for equality comparisons between `CheckedF64` instances and `f64`
//! values, while the `PartialOrd` trait enables ordering comparisons.
use super::CheckedF64;
use std::cmp::{Ordering, PartialEq, PartialOrd};

impl PartialEq for CheckedF64 {
    /// Compares two `CheckedF64` values for equality.
    ///
    /// # Returns
    ///
    /// Returns `true` if both values are valid (finite) and equal, otherwise returns `false`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use checked_float::CheckedF64;
    ///
    /// let a = CheckedF64::new(2.0).unwrap();
    /// let b = CheckedF64::new(2.0).unwrap();
    /// assert_eq!(a, b);
    ///
    /// let a_invalid = CheckedF64::new(2.0);
    /// let b_invalid = CheckedF64::new(f64::NAN);
    /// assert_ne!(a_invalid, b_invalid);
    /// ```
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<f64> for CheckedF64 {
    /// Compares `CheckedF64` with `f64` for equality.
    ///
    /// # Returns
    ///
    /// Returns `true` if `CheckedF64` is valid (finite) and equal to `f64`, otherwise returns `false`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use checked_float::CheckedF64;
    ///
    /// let a = CheckedF64::new(2.0).unwrap();
    /// let b = 2.0;
    /// assert_eq!(a, b);
    ///
    /// let invalid = f64::NAN;
    /// assert_ne!(a, invalid);
    /// ```
    fn eq(&self, other: &f64) -> bool {
        other.is_finite() && self.0 == *other
    }
}

impl PartialEq<CheckedF64> for f64 {
    /// Compares `f64` with `CheckedF64` for equality.
    ///
    /// # Returns
    ///
    /// Returns `true` if `f64` is finite and equal to `CheckedF64`, otherwise returns `false`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use checked_float::CheckedF64;
    ///
    /// let a = 2.0;
    /// let b = CheckedF64::new(2.0).unwrap();
    /// assert_eq!(a, b);
    ///
    /// let a_invalid = f64::NAN;
    /// let b_invalid = CheckedF64::new(2.0).unwrap();
    /// assert_ne!(a_invalid, b_invalid);
    /// ```
    fn eq(&self, other: &CheckedF64) -> bool {
        self.is_finite() && *self == other.0
    }
}

impl PartialOrd for CheckedF64 {
    /// Compares two `CheckedF64` values.
    ///
    /// # Returns
    ///
    /// Returns `Some(Ordering)` if both values are valid (finite), otherwise returns `None`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use checked_float::CheckedF64;
    ///
    /// let a = CheckedF64::new(2.0).unwrap();
    /// let b = CheckedF64::new(3.0).unwrap();
    /// assert_eq!(a < b, true);
    /// assert_eq!(a <= b, true);
    /// assert_eq!(a > b, false);
    /// assert_eq!(a >= b, false);
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.0, &other.0)
    }
}

impl PartialOrd<f64> for CheckedF64 {
    /// Compares `CheckedF64` with `f64`.
    ///
    /// # Returns
    ///
    /// Returns `Some(Ordering)` if both values are valid (finite), otherwise returns `None`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use checked_float::CheckedF64;
    ///
    /// let a = CheckedF64::new(2.0).unwrap();
    /// let b = 3.0;
    /// assert_eq!(a < b, true);
    /// assert_eq!(a <= b, true);
    /// assert_eq!(a > b, false);
    /// assert_eq!(a >= b, false);
    ///
    /// let b_invalid = f64::NAN;
    /// assert_eq!(a.partial_cmp(&b_invalid), None);
    /// ```
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        if other.is_finite() {
            self.0.partial_cmp(other)
        } else {
            None
        }
    }
}

impl PartialOrd<CheckedF64> for f64 {
    /// Compares `f64` with `CheckedF64`.
    ///
    /// # Returns
    ///
    /// Returns `Some(Ordering)` if both values are finite, otherwise returns `None`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use checked_float::CheckedF64;
    /// use std::cmp::Ordering;
    ///
    /// let a = CheckedF64::new(2.0).unwrap();
    /// let b = 3.0;
    /// assert_eq!(a > b, false);
    /// assert_eq!(a >= b, false);
    /// assert_eq!(a < b, true);
    /// assert_eq!(a <= b, true);
    ///
    /// assert_eq!(f64::NAN.partial_cmp(&b), None);
    /// ```
    fn partial_cmp(&self, other: &CheckedF64) -> Option<Ordering> {
        if self.is_finite() {
            self.partial_cmp(&other.0)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        CheckedF64,
        checked_f64::tests::valid_f64,
    };
    use proptest::prelude::*;

    proptest! {
        // Ordering
        #[test]
        fn test_valid_cmp_valid(a in valid_f64(), b in valid_f64()) {
            let checked_a = CheckedF64::new(a).unwrap();
            let checked_b = CheckedF64::new(b).unwrap();

            prop_assert_eq!(checked_a > checked_b, a > b);
            prop_assert_eq!(checked_a > b, a > b);
            prop_assert_eq!(a > checked_b, a > b);
            prop_assert_eq!(checked_a >= checked_b, a >= b);
            prop_assert_eq!(checked_a < checked_b, a < b);
            prop_assert_eq!(checked_a <= checked_b, a <= b);
            prop_assert_eq!(checked_a.partial_cmp(&checked_b), a.partial_cmp(&b));
        }

        #[test]
        fn test_valid_cmp_invalid(a in valid_f64(), b in valid_f64()) {
            prop_assert_eq!(CheckedF64::new(a).unwrap() > CheckedF64::new(b).unwrap(), a > b);
            prop_assert_eq!(CheckedF64::new(a).unwrap() >= CheckedF64::new(b).unwrap(), a >= b);
            prop_assert_eq!(CheckedF64::new(a).unwrap() < CheckedF64::new(b).unwrap(), a < b);
            prop_assert_eq!(CheckedF64::new(a).unwrap() <= CheckedF64::new(b).unwrap(), a <= b);
        }

        // Equality Operator
        #[test]
        fn test_valid_eq_valid(a in valid_f64()) {
            let checked_a = CheckedF64::new(a).unwrap();

            prop_assert_eq!(checked_a, a);
            prop_assert_eq!(a, checked_a);
            prop_assert_eq!(checked_a, checked_a);
        }
    }
}
