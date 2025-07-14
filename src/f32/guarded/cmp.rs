//! This module implements the `PartialEq` and `PartialOrd` traits for `GuardedF32`.
//!
//! The `PartialEq` trait allows for equality comparisons between `GuardedF32` instances and `f32`
//! values, while the `PartialOrd` trait enables ordering comparisons.
use super::GuardedF32;
use std::cmp::{Ordering, PartialEq, PartialOrd};

impl PartialEq for GuardedF32 {
    /// Compares two `GuardedF32` values for equality.
    ///
    /// # Returns
    ///
    /// Returns `true` if both values are valid (finite) and equal, otherwise returns `false`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use floatguard::GuardedF32;
    ///
    /// let a = GuardedF32::new(2.0).unwrap();
    /// let b = GuardedF32::new(2.0).unwrap();
    /// assert_eq!(a, b);
    ///
    /// let a_invalid = GuardedF32::new(2.0);
    /// let b_invalid = GuardedF32::new(f32::NAN);
    /// assert_ne!(a_invalid, b_invalid);
    /// ```
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for GuardedF32 {}

impl PartialEq<f32> for GuardedF32 {
    /// Compares `GuardedF32` with `f32` for equality.
    ///
    /// # Returns
    ///
    /// Returns `true` if `GuardedF32` is valid (finite) and equal to `f32`, otherwise returns `false`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use floatguard::GuardedF32;
    ///
    /// let a = GuardedF32::new(2.0).unwrap();
    /// let b = 2.0;
    /// assert_eq!(a, b);
    ///
    /// let invalid = f32::NAN;
    /// assert_ne!(a, invalid);
    /// ```
    fn eq(&self, other: &f32) -> bool {
        other.is_finite() && self.0 == *other
    }
}

impl PartialEq<GuardedF32> for f32 {
    /// Compares `f32` with `GuardedF32` for equality.
    ///
    /// # Returns
    ///
    /// Returns `true` if `f32` is finite and equal to `GuardedF32`, otherwise returns `false`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use floatguard::GuardedF32;
    ///
    /// let a = 2.0;
    /// let b = GuardedF32::new(2.0).unwrap();
    /// assert_eq!(a, b);
    ///
    /// let a_invalid = f32::NAN;
    /// let b_invalid = GuardedF32::new(2.0).unwrap();
    /// assert_ne!(a_invalid, b_invalid);
    /// ```
    fn eq(&self, other: &GuardedF32) -> bool {
        self.is_finite() && *self == other.0
    }
}

impl PartialOrd for GuardedF32 {
    /// Compares two `GuardedF32` values.
    ///
    /// # Returns
    ///
    /// Returns `Some(Ordering)` if both values are valid (finite), otherwise returns `None`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use floatguard::GuardedF32;
    ///
    /// let a = GuardedF32::new(2.0).unwrap();
    /// let b = GuardedF32::new(3.0).unwrap();
    /// assert_eq!(a < b, true);
    /// assert_eq!(a <= b, true);
    /// assert_eq!(a > b, false);
    /// assert_eq!(a >= b, false);
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GuardedF32 {
    /// Compares two `GuardedF32` values.
    ///
    /// # Returns
    ///
    /// Returns `Ordering` if both values are valid (finite), otherwise panics.
    ///
    /// # Example
    ///
    /// ```rust
    /// use floatguard::GuardedF32;
    ///
    /// let a = GuardedF32::new(2.0).unwrap();
    /// let b = GuardedF32::new(3.0).unwrap();
    /// assert_eq!(a.cmp(&b), std::cmp::Ordering::Less);
    /// assert_eq!(b.cmp(&a), std::cmp::Ordering::Greater);
    /// assert_eq!(a.cmp(&a), std::cmp::Ordering::Equal);
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

impl PartialOrd<f32> for GuardedF32 {
    /// Compares `GuardedF32` with `f32`.
    ///
    /// # Returns
    ///
    /// Returns `Some(Ordering)` if both values are valid (finite), otherwise returns `None`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use floatguard::GuardedF32;
    ///
    /// let a = GuardedF32::new(2.0).unwrap();
    /// let b = 3.0;
    /// assert_eq!(a < b, true);
    /// assert_eq!(a <= b, true);
    /// assert_eq!(a > b, false);
    /// assert_eq!(a >= b, false);
    ///
    /// let b_invalid = f32::NAN;
    /// assert_eq!(a.partial_cmp(&b_invalid), None);
    /// ```
    fn partial_cmp(&self, other: &f32) -> Option<Ordering> {
        if other.is_finite() {
            self.0.partial_cmp(other)
        } else {
            None
        }
    }
}

impl PartialOrd<GuardedF32> for f32 {
    /// Compares `f32` with `GuardedF32`.
    ///
    /// # Returns
    ///
    /// Returns `Some(Ordering)` if both values are finite, otherwise returns `None`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use floatguard::GuardedF32;
    /// use std::cmp::Ordering;
    ///
    /// let a = GuardedF32::new(2.0).unwrap();
    /// let b = 3.0;
    /// assert_eq!(a > b, false);
    /// assert_eq!(a >= b, false);
    /// assert_eq!(a < b, true);
    /// assert_eq!(a <= b, true);
    ///
    /// assert_eq!(f32::NAN.partial_cmp(&b), None);
    /// ```
    fn partial_cmp(&self, other: &GuardedF32) -> Option<Ordering> {
        if self.is_finite() {
            self.partial_cmp(&other.0)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{GuardedF32, f32::tests::valid_f32};
    use proptest::prelude::*;

    proptest! {
        // Ordering
        #[test]
        fn test_valid_cmp_valid(a in valid_f32(), b in valid_f32()) {
            let checked_a = GuardedF32::new(a).unwrap();
            let checked_b = GuardedF32::new(b).unwrap();

            prop_assert_eq!(checked_a > checked_b, a > b);
            prop_assert_eq!(checked_a > b, a > b);
            prop_assert_eq!(a > checked_b, a > b);
            prop_assert_eq!(checked_a >= checked_b, a >= b);
            prop_assert_eq!(checked_a < checked_b, a < b);
            prop_assert_eq!(checked_a <= checked_b, a <= b);
            prop_assert_eq!(checked_a.partial_cmp(&checked_b), a.partial_cmp(&b));
        }

        #[test]
        fn test_valid_cmp_invalid(a in valid_f32(), b in valid_f32()) {
            prop_assert_eq!(GuardedF32::new(a).unwrap() > GuardedF32::new(b).unwrap(), a > b);
            prop_assert_eq!(GuardedF32::new(a).unwrap() >= GuardedF32::new(b).unwrap(), a >= b);
            prop_assert_eq!(GuardedF32::new(a).unwrap() < GuardedF32::new(b).unwrap(), a < b);
            prop_assert_eq!(GuardedF32::new(a).unwrap() <= GuardedF32::new(b).unwrap(), a <= b);
        }

        // Equality Operator
        #[allow(clippy::float_cmp)]
        #[test]
        fn test_valid_eq_valid(a in valid_f32()) {
            let checked_a = GuardedF32::new(a).unwrap();

            prop_assert_eq!(checked_a, a);
            prop_assert_eq!(a, checked_a);
            prop_assert_eq!(checked_a, checked_a);
        }
    }
}
