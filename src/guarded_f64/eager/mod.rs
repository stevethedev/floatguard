//! This module provides a checked floating-point number type, `GuardedF64`, which ensures that the
//! value is neither NaN nor infinite.
mod cmp;
mod convert;

use crate::FloatError;

/// Represents a checked floating-point number that ensures it is neither NaN nor infinite.
///
/// # Example
///
/// ```rust
/// use floatguard::{GuardedF64, FloatError};
///
/// let checked_f64 = GuardedF64::new(1.0).expect("1.0 is a valid f64 value");
/// assert_eq!((checked_f64 + 1.0).check(), GuardedF64::new(2.0));
///
/// assert_eq!((checked_f64 / 0.0).check(), Err(FloatError::Infinity));
///
/// assert_eq!((checked_f64 - f64::INFINITY).check(), Err(FloatError::Infinity));
///
/// assert_eq!((checked_f64 % f64::NAN).check(), Err(FloatError::NaN));
/// ```
#[derive(Debug, Default, Clone, Copy)]
pub struct GuardedF64(pub(crate) f64);

impl GuardedF64 {
    /// Creates a new `GuardedF64` instance.
    ///
    /// # Returns
    ///
    /// Returns a new `GuardedF64` instance containing the provided `f64` value if it is valid (finite).
    ///
    /// # Errors
    ///
    /// Returns `FloatError` if the value is NaN or infinite.
    ///
    /// # Example
    ///
    /// ```rust
    /// use floatguard::{GuardedF64, FloatError};
    ///
    /// let valid_value = GuardedF64::new(2.0).unwrap();
    /// assert_eq!(valid_value, 2.0f64);
    ///
    /// let invalid_value = GuardedF64::new(f64::NAN);
    /// assert_eq!(invalid_value, Err(FloatError::NaN));
    ///
    /// let inf_value = GuardedF64::new(f64::INFINITY);
    /// assert_eq!(inf_value, Err(FloatError::Infinity));
    /// ```
    pub const fn new(value: f64) -> Result<Self, FloatError> {
        if value.is_finite() {
            Ok(Self(value))
        } else {
            Err(if value.is_nan() {
                FloatError::NaN
            } else {
                FloatError::Infinity
            })
        }
    }
}

impl std::fmt::Display for GuardedF64 {
    /// Formats the `GuardedF64` as a string.
    ///
    /// # Returns
    ///
    /// Returns a string representation of the inner `f64` value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use floatguard::GuardedF64;
    ///
    /// let value = GuardedF64::new(2.0).unwrap();
    /// assert_eq!(value.to_string(), "2");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::float_cmp)]

    use super::*;
    use crate::guarded_f64::tests::{invalid_f64, valid_f64};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_new_valid(a in valid_f64()) {
            prop_assert_eq!(GuardedF64::new(a), Ok(GuardedF64(a)));
            prop_assert_eq!(GuardedF64::new(a).map(f64::from), Ok(a));
            prop_assert_eq!(*GuardedF64::new(a).unwrap(), a);
        }

        #[test]
        fn test_new_invalid(a in invalid_f64()) {
            let err = if a.is_nan() {
                FloatError::NaN
            } else {
                FloatError::Infinity
            };
            prop_assert_eq!(GuardedF64::new(a), Err(err));
        }

        #[test]
        fn test_display(a in any::<f64>()) {
            let checked_a = GuardedF64::new(a);
            if let Ok(guarded_a) = checked_a {
                prop_assert_eq!(guarded_a.to_string(), a.to_string());
            } else {
                prop_assert!(a.is_nan() || a.is_infinite());
            }
        }
    }
}
