//! This module implements the `PartialEq` and `PartialOrd` traits for `GuardedF64`.
//!
//! The `PartialEq` trait allows for equality comparisons between `GuardedF64` instances and `f64`
//! values, while the `PartialOrd` trait enables ordering comparisons.
use super::GuardedF64;
use std::cmp::{Ordering, PartialEq, PartialOrd};

impl PartialEq for GuardedF64 {
    /// Compares two `GuardedF64` values for equality.
    ///
    /// # Returns
    ///
    /// Returns `true` if both values are valid (finite) and equal, otherwise returns `false`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use floatguard::GuardedF64;
    ///
    /// let a = GuardedF64::new(2.0).unwrap();
    /// let b = GuardedF64::new(2.0).unwrap();
    /// assert_eq!(a, b);
    ///
    /// let a_invalid = GuardedF64::new(2.0);
    /// let b_invalid = GuardedF64::new(f64::NAN);
    /// assert_ne!(a_invalid, b_invalid);
    /// ```
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for GuardedF64 {}

impl PartialEq<f64> for GuardedF64 {
    /// Compares `GuardedF64` with `f64` for equality.
    ///
    /// # Returns
    ///
    /// Returns `true` if `GuardedF64` is valid (finite) and equal to `f64`, otherwise returns `false`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use floatguard::GuardedF64;
    ///
    /// let a = GuardedF64::new(2.0).unwrap();
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

impl PartialEq<GuardedF64> for f64 {
    /// Compares `f64` with `GuardedF64` for equality.
    ///
    /// # Returns
    ///
    /// Returns `true` if `f64` is finite and equal to `GuardedF64`, otherwise returns `false`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use floatguard::GuardedF64;
    ///
    /// let a = 2.0;
    /// let b = GuardedF64::new(2.0).unwrap();
    /// assert_eq!(a, b);
    ///
    /// let a_invalid = f64::NAN;
    /// let b_invalid = GuardedF64::new(2.0).unwrap();
    /// assert_ne!(a_invalid, b_invalid);
    /// ```
    fn eq(&self, other: &GuardedF64) -> bool {
        self.is_finite() && *self == other.0
    }
}

impl PartialOrd for GuardedF64 {
    /// Compares two `GuardedF64` values.
    ///
    /// # Returns
    ///
    /// Returns `Some(Ordering)` if both values are valid (finite), otherwise returns `None`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use floatguard::GuardedF64;
    ///
    /// let a = GuardedF64::new(2.0).unwrap();
    /// let b = GuardedF64::new(3.0).unwrap();
    /// assert_eq!(a < b, true);
    /// assert_eq!(a <= b, true);
    /// assert_eq!(a > b, false);
    /// assert_eq!(a >= b, false);
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GuardedF64 {
    /// Compares two `GuardedF64` values.
    ///
    /// # Returns
    ///
    /// Returns `Ordering` if both values are valid (finite), otherwise panics.
    ///
    /// # Example
    ///
    /// ```rust
    /// use floatguard::GuardedF64;
    ///
    /// let a = GuardedF64::new(2.0).unwrap();
    /// let b = GuardedF64::new(3.0).unwrap();
    /// assert_eq!(a.cmp(&b), std::cmp::Ordering::Less);
    /// ```
    fn cmp(&self, other: &Self) -> Ordering {
        let lhs = self.0;
        let rhs = other.0;

        match (lhs < rhs, lhs > rhs) {
            (true, _) => Ordering::Less,
            (_, true) => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }
}

impl PartialOrd<f64> for GuardedF64 {
    /// Compares `GuardedF64` with `f64`.
    ///
    /// # Returns
    ///
    /// Returns `Some(Ordering)` if both values are valid (finite), otherwise returns `None`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use floatguard::GuardedF64;
    ///
    /// let a = GuardedF64::new(2.0).unwrap();
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

impl PartialOrd<GuardedF64> for f64 {
    /// Compares `f64` with `GuardedF64`.
    ///
    /// # Returns
    ///
    /// Returns `Some(Ordering)` if both values are finite, otherwise returns `None`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use floatguard::GuardedF64;
    /// use std::cmp::Ordering;
    ///
    /// let a = GuardedF64::new(2.0).unwrap();
    /// let b = 3.0;
    /// assert_eq!(a > b, false);
    /// assert_eq!(a >= b, false);
    /// assert_eq!(a < b, true);
    /// assert_eq!(a <= b, true);
    ///
    /// assert_eq!(f64::NAN.partial_cmp(&b), None);
    /// ```
    fn partial_cmp(&self, other: &GuardedF64) -> Option<Ordering> {
        if self.is_finite() {
            self.partial_cmp(&other.0)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{GuardedF64, f64::tests::valid_f64};
    use proptest::prelude::*;

    proptest! {
        // Ordering
        #[test]
        fn test_valid_cmp_valid(a in valid_f64(), b in valid_f64()) {
            let checked_a = GuardedF64::new(a).unwrap();
            let checked_b = GuardedF64::new(b).unwrap();

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
            prop_assert_eq!(GuardedF64::new(a).unwrap() > GuardedF64::new(b).unwrap(), a > b);
            prop_assert_eq!(GuardedF64::new(a).unwrap() >= GuardedF64::new(b).unwrap(), a >= b);
            prop_assert_eq!(GuardedF64::new(a).unwrap() < GuardedF64::new(b).unwrap(), a < b);
            prop_assert_eq!(GuardedF64::new(a).unwrap() <= GuardedF64::new(b).unwrap(), a <= b);
        }

        // Equality Operator
        #[allow(clippy::float_cmp)]
        #[test]
        fn test_valid_eq_valid(a in valid_f64()) {
            let checked_a = GuardedF64::new(a).unwrap();

            prop_assert_eq!(checked_a, a);
            prop_assert_eq!(a, checked_a);
            prop_assert_eq!(checked_a, checked_a);
        }
    }
}
