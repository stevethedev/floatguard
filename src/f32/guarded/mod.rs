//! This module provides a checked floating-point number type, `GuardedF32`, which ensures that the
//! value is neither NaN nor infinite.
mod cmp;
mod convert;

use crate::FloatError;

/// Represents a checked floating-point number that ensures it is neither NaN nor infinite.
///
/// # Example
///
/// ```rust
/// use floatguard::{GuardedF32, FloatError};
///
/// let checked_f32 = GuardedF32::new(1.0).expect("1.0 is a valid f32 value");
/// assert_eq!((checked_f32 + 1.0).check(), GuardedF32::new(2.0));
///
/// assert_eq!((checked_f32 / 0.0).check(), Err(FloatError::Infinity));
///
/// assert_eq!((checked_f32 - f32::INFINITY).check(), Err(FloatError::Infinity));
///
/// assert_eq!((checked_f32 % f32::NAN).check(), Err(FloatError::NaN));
/// ```
#[derive(Debug, Default, Clone, Copy)]
pub struct GuardedF32(pub(crate) f32);

impl GuardedF32 {
    /// Creates a new `GuardedF32` instance.
    ///
    /// # Returns
    ///
    /// Returns a new `GuardedF32` instance containing the provided `f32` value if it is valid (finite).
    ///
    /// # Errors
    ///
    /// Returns `FloatError` if the value is NaN or infinite.
    ///
    /// # Example
    ///
    /// ```rust
    /// use floatguard::{GuardedF32, FloatError};
    ///
    /// let valid_value = GuardedF32::new(2.0).unwrap();
    /// assert_eq!(valid_value, 2.0f32);
    ///
    /// let invalid_value = GuardedF32::new(f32::NAN);
    /// assert_eq!(invalid_value, Err(FloatError::NaN));
    ///
    /// let inf_value = GuardedF32::new(f32::INFINITY);
    /// assert_eq!(inf_value, Err(FloatError::Infinity));
    /// ```
    pub const fn new(value: f32) -> Result<Self, FloatError> {
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

impl std::fmt::Display for GuardedF32 {
    /// Formats the `GuardedF32` as a string.
    ///
    /// # Returns
    ///
    /// Returns a string representation of the inner `f32` value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use floatguard::GuardedF32;
    ///
    /// let value = GuardedF32::new(2.0).unwrap();
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
    use crate::f32::tests::{invalid_f32, valid_f32};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_new_valid(a in valid_f32()) {
            prop_assert_eq!(GuardedF32::new(a), Ok(GuardedF32(a)));
            prop_assert_eq!(GuardedF32::new(a).map(f32::from), Ok(a));
            prop_assert_eq!(*GuardedF32::new(a).unwrap(), a);
        }

        #[test]
        fn test_new_invalid(a in invalid_f32()) {
            let err = if a.is_nan() {
                FloatError::NaN
            } else {
                FloatError::Infinity
            };
            prop_assert_eq!(GuardedF32::new(a), Err(err));
        }

        #[test]
        fn test_display(a in any::<f32>()) {
            let checked_a = GuardedF32::new(a);
            if let Ok(guarded_a) = checked_a {
                prop_assert_eq!(guarded_a.to_string(), a.to_string());
            } else {
                prop_assert!(a.is_nan() || a.is_infinite());
            }
        }
    }
}
