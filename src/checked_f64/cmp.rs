use crate::{CheckedF64, CheckedF64Result, FloatError};
use std::cmp::Ordering;

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

impl PartialEq<CheckedF64Result> for CheckedF64 {
    /// Compares `CheckedF64` with `CheckedF64Result` for equality.
    ///
    /// # Returns
    ///
    /// Returns `true` if `CheckedF64` is valid (finite) and equal to `CheckedF64Result`, otherwise returns `false`.
    fn eq(&self, other: &CheckedF64Result) -> bool {
        other.as_ref().is_ok_and(|value| self.0 == *value)
    }
}

impl PartialEq for CheckedF64Result {
    /// Compares `CheckedF64Result` with `CheckedF64` for equality.
    ///
    /// # Returns
    ///
    /// Returns `true` if `CheckedF64Result` is valid (finite) and equal to `CheckedF64`, otherwise returns `false`.
    fn eq(&self, other: &Self) -> bool {
        self.as_ref().is_ok_and(|value| *value == *other)
    }
}

impl PartialEq<CheckedF64> for CheckedF64Result {
    /// Compares `CheckedF64Result` with `CheckedF64` for equality.
    ///
    /// # Returns
    ///
    /// Returns `true` if `CheckedF64Result` is valid (finite) and equal to `CheckedF64`, otherwise returns `false`.
    fn eq(&self, other: &CheckedF64) -> bool {
        self.as_ref().is_ok_and(|value| *value == other.0)
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
    /// let a = CheckedF64::new(2.0);
    /// let b = 2.0;
    ///
    /// assert_eq!(a, b);
    ///
    /// let a_invalid = CheckedF64::new(2.0);
    /// let b_invalid = f64::NAN;
    ///
    /// assert_ne!(a_invalid, b_invalid);
    ///
    /// let inf = CheckedF64::new(f64::INFINITY);
    /// assert_ne!(inf, b);
    ///
    /// let nan = CheckedF64::new(f64::NAN);
    /// assert_ne!(nan, b);
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
    /// let b = CheckedF64::new(2.0);
    /// assert_eq!(a, b);
    ///
    /// let a_invalid = f64::NAN;
    /// let b_invalid = CheckedF64::new(2.0);
    /// assert_ne!(a_invalid, b_invalid);
    ///
    /// let inf = f64::INFINITY;
    /// let b_inf = CheckedF64::new(f64::INFINITY);
    /// assert_ne!(inf, b_inf);
    ///
    /// let nan = f64::NAN;
    /// let b_nan = CheckedF64::new(f64::NAN);
    /// assert_ne!(nan, b_nan);
    /// ```
    fn eq(&self, other: &CheckedF64) -> bool {
        self.is_finite() && *self == other.0
    }
}

impl PartialEq<Result<f64, FloatError>> for CheckedF64Result {
    /// Compares `CheckedF64` with `Result<CheckedF64, FloatError>` for equality.
    ///
    /// # Returns
    ///
    /// Returns `true` if `CheckedF64` is valid (finite) and equal to the `Ok` variant of the result, otherwise returns `false`.
    fn eq(&self, other: &Result<f64, FloatError>) -> bool {
        other
            .as_ref()
            .is_ok_and(|value| self.as_ref().is_ok_and(|checked| *value == checked.0))
    }
}

impl PartialEq<CheckedF64Result> for Result<f64, FloatError> {
    /// Compares `Result<CheckedF64, FloatError>` with `CheckedF64` for equality.
    ///
    /// # Returns
    ///
    /// Returns `true` if the result is `Ok` and the value is valid (finite) and equal to `CheckedF64`, otherwise returns `false`.
    fn eq(&self, other: &CheckedF64Result) -> bool {
        self.as_ref()
            .is_ok_and(|value| other.as_ref().is_ok_and(|checked| *value == checked.0))
    }
}

impl PartialEq<f64> for CheckedF64Result {
    /// Compares `CheckedF64Result` with `f64` for equality.
    ///
    /// # Returns
    ///
    /// Returns `true` if the result is `Ok` and the value is valid (finite) and equal to `f64`, otherwise returns `false`.
    fn eq(&self, other: &f64) -> bool {
        self.as_ref().is_ok_and(|value| *value == *other)
    }
}

impl PartialEq<CheckedF64Result> for f64 {
    /// Compares `f64` with `CheckedF64Result` for equality.
    ///
    /// # Returns
    ///
    /// Returns `true` if the result is `Ok` and the value is valid (finite) and equal to `f64`, otherwise returns `false`.
    fn eq(&self, other: &CheckedF64Result) -> bool {
        other.as_ref().is_ok_and(|value| *value == *self)
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
    /// let a = CheckedF64::new(2.0);
    /// let b = CheckedF64::new(3.0);
    /// assert_eq!(a < b, true);
    /// assert_eq!(a <= b, true);
    /// assert_eq!(a > b, false);
    /// assert_eq!(a >= b, false);
    ///
    /// let invalid = CheckedF64::new(f64::NAN);
    /// assert_eq!(invalid.partial_cmp(&b), None);
    ///
    /// let inf = CheckedF64::new(f64::INFINITY);
    /// assert_eq!(inf.partial_cmp(&b), None);
    ///
    /// let a_invalid = CheckedF64::new(2.0);
    /// let b_invalid = CheckedF64::new(f64::NAN);
    /// assert_eq!(a_invalid.partial_cmp(&b_invalid), None);
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
    /// let a = CheckedF64::new(2.0);
    /// let b = 3.0;
    /// assert_eq!(a < b, true);
    /// assert_eq!(a <= b, true);
    /// assert_eq!(a > b, false);
    /// assert_eq!(a >= b, false);
    ///
    /// let invalid = CheckedF64::new(f64::NAN);
    /// assert_eq!(invalid.partial_cmp(&b), None);
    ///
    /// let inf = CheckedF64::new(f64::INFINITY);
    /// assert_eq!(inf.partial_cmp(&b), None);
    ///
    /// let a_invalid = CheckedF64::new(2.0);
    /// let b_invalid = f64::NAN;
    /// assert_eq!(a_invalid.partial_cmp(&b_invalid), None);
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
    /// let a = CheckedF64::new(2.0);
    /// let b = 3.0;
    /// assert_eq!(a > b, false);
    /// assert_eq!(a >= b, false);
    /// assert_eq!(a < b, true);
    /// assert_eq!(a <= b, true);
    ///
    /// let invalid = CheckedF64::new(f64::NAN);
    /// assert_eq!(invalid.partial_cmp(&b), None);
    ///
    /// let inf = CheckedF64::new(f64::INFINITY);
    /// assert_eq!(inf.partial_cmp(&b), None);
    /// ```
    fn partial_cmp(&self, other: &CheckedF64) -> Option<Ordering> {
        if self.is_finite() {
            self.partial_cmp(&other.0)
        } else {
            None
        }
    }
}

impl PartialOrd for CheckedF64Result {
    /// Compares `CheckedF64Result` with `CheckedF64Result`.
    ///
    /// # Returns
    ///
    /// Returns `Some(Ordering)` if both values are valid (finite), otherwise returns `None`.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.as_ref(), other.as_ref()) {
            (Ok(a), Ok(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

impl PartialOrd<CheckedF64Result> for CheckedF64 {
    /// Compares `CheckedF64` with `CheckedF64Result`.
    ///
    /// # Returns
    ///
    /// Returns `Some(Ordering)` if both values are valid (finite), otherwise returns `None`.
    fn partial_cmp(&self, other: &CheckedF64Result) -> Option<Ordering> {
        other
            .as_ref()
            .ok()
            .and_then(|value| self.0.partial_cmp(value))
    }
}

impl PartialOrd<CheckedF64> for CheckedF64Result {
    /// Compares `CheckedF64Result` with `CheckedF64`.
    ///
    /// # Returns
    ///
    /// Returns `Some(Ordering)` if both values are valid (finite), otherwise returns `None`.
    fn partial_cmp(&self, other: &CheckedF64) -> Option<Ordering> {
        self.as_ref()
            .ok()
            .and_then(|value| value.partial_cmp(&other.0))
    }
}

impl PartialOrd<Result<f64, FloatError>> for CheckedF64Result {
    /// Compares `CheckedF64Result` with `Result<f64, FloatError>`.
    ///
    /// # Returns
    ///
    /// Returns `Some(Ordering)` if the result is `Ok` and the value is valid (finite), otherwise returns `None`.
    fn partial_cmp(&self, other: &Result<f64, FloatError>) -> Option<Ordering> {
        match other {
            Ok(value) if value.is_finite() => self
                .as_ref()
                .ok()
                .and_then(|checked| checked.0.partial_cmp(value)),
            _ => None,
        }
    }
}

impl PartialOrd<f64> for CheckedF64Result {
    /// Compares `CheckedF64Result` with `f64`.
    ///
    /// # Returns
    ///
    /// Returns `Some(Ordering)` if the result is `Ok` and the value is valid (finite), otherwise returns `None`.
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        if other.is_finite() {
            self.as_ref()
                .ok()
                .and_then(|checked| checked.0.partial_cmp(other))
        } else {
            None
        }
    }
}

impl PartialOrd<CheckedF64Result> for f64 {
    /// Compares `f64` with `CheckedF64`.
    ///
    /// # Returns
    ///
    /// Returns `Some(Ordering)` if the value is valid (finite), otherwise returns `None`.
    fn partial_cmp(&self, other: &CheckedF64Result) -> Option<Ordering> {
        if self.is_finite() {
            other
                .as_ref()
                .ok()
                .and_then(|checked| checked.0.partial_cmp(self))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        CheckedF64,
        checked_f64::tests::{invalid_f64, valid_f64},
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
        fn test_valid_cmp_invalid(a in valid_f64(), b in invalid_f64()) {
            prop_assert_eq!(CheckedF64::new(a) > CheckedF64::new(b), false);
            prop_assert_eq!(CheckedF64::new(a) >= CheckedF64::new(b), false);
            prop_assert_eq!(CheckedF64::new(a) < CheckedF64::new(b), false);
            prop_assert_eq!(CheckedF64::new(a) <= CheckedF64::new(b), false);
        }

        #[test]
        fn test_invalid_cmp_valid(a in invalid_f64(), b in valid_f64()) {
            prop_assert_eq!(CheckedF64::new(a) > CheckedF64::new(b), false);
            prop_assert_eq!(CheckedF64::new(a) >= CheckedF64::new(b), false);
            prop_assert_eq!(CheckedF64::new(a) < CheckedF64::new(b), false);
            prop_assert_eq!(CheckedF64::new(a) <= CheckedF64::new(b), false);
        }

        #[test]
        fn test_invalid_cmp_invalid(a in invalid_f64(), b in invalid_f64()) {
            prop_assert_eq!(CheckedF64::new(a) > CheckedF64::new(b), false);
            prop_assert_eq!(CheckedF64::new(a) >= CheckedF64::new(b), false);
            prop_assert_eq!(CheckedF64::new(a) < CheckedF64::new(b), false);
            prop_assert_eq!(CheckedF64::new(a) <= CheckedF64::new(b), false);
        }

        // Equality Operator
        #[test]
        fn test_valid_eq_valid(a in valid_f64()) {
            let checked_a = CheckedF64::new(a).unwrap();

            prop_assert_eq!(checked_a, a);
            prop_assert_eq!(a, checked_a);
            prop_assert_eq!(checked_a, checked_a);

            prop_assert_eq!(CheckedF64::new(a), checked_a);
            prop_assert_eq!(checked_a, CheckedF64::new(a));
            prop_assert_eq!(CheckedF64::new(a), CheckedF64::new(a));
        }

        #[test]
        fn test_valid_eq_invalid(a in valid_f64(), b in invalid_f64()) {
            prop_assert_ne!(CheckedF64::new(a), CheckedF64::new(b));
        }

        #[test]
        fn test_invalid_eq_valid(a in invalid_f64(), b in valid_f64()) {
            prop_assert_ne!(CheckedF64::new(a), CheckedF64::new(b));
        }

        #[test]
        fn test_invalid_eq_invalid(a in invalid_f64(), b in invalid_f64()) {
            prop_assert_ne!(CheckedF64::new(a), CheckedF64::new(b));
        }
    }
}
