//! This module provides a checked floating-point number type, `GuardedF64`, which ensures that the
//! value is neither NaN nor infinite.
mod cmp;

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
/// assert_eq!((checked_f64 / 0.0).check(), Err(FloatError));
///
/// assert_eq!((checked_f64 - f64::INFINITY).check(), Err(FloatError));
///
/// assert_eq!((checked_f64 % f64::NAN).check(), Err(FloatError));
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
    /// assert_eq!(invalid_value, Err(FloatError));
    ///
    /// let inf_value = GuardedF64::new(f64::INFINITY);
    /// assert_eq!(inf_value, Err(FloatError));
    /// ```
    pub const fn new(value: f64) -> Result<Self, FloatError> {
        if value.is_finite() {
            Ok(Self(value))
        } else {
            Err(FloatError)
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

impl TryFrom<f64> for GuardedF64 {
    type Error = FloatError;

    /// Converts a `f64` to `GuardedF64`.
    ///
    /// # Returns
    ///
    /// Returns a `GuardedF64` if the value is valid (finite), otherwise returns an error.
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
    /// let valid_value = GuardedF64::new(2.0);
    /// assert!(valid_value.is_ok());
    ///
    /// let invalid_value = GuardedF64::new(f64::NAN);
    /// assert!(invalid_value.is_err());
    ///
    /// let inf_value = GuardedF64::new(f64::INFINITY);
    /// assert!(inf_value.is_err());
    /// ```
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// Implementing the ability to convert `GuardedF64` to `f64` safely.
///
/// This conversion will return an error if the value is NaN or infinite.
impl From<GuardedF64> for f64 {
    /// Converts a `GuardedF64` to `f64`.
    ///
    /// # Returns
    ///
    /// Returns the inner `f64` value if it is valid (finite), otherwise returns an error.
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
    /// assert_eq!(valid_value.try_into(), Ok(2.0));
    ///
    /// let invalid_value = GuardedF64::try_from(f64::NAN);
    /// assert_eq!(invalid_value, Err(FloatError));
    ///
    /// let inf_value = GuardedF64::try_from(f64::INFINITY);
    /// assert_eq!(inf_value, Err(FloatError));
    /// ```
    fn from(value: GuardedF64) -> Self {
        value.0
    }
}

impl std::ops::Deref for GuardedF64 {
    type Target = f64;

    /// Dereferences `GuardedF64` to its inner `f64` value.
    ///
    /// # Returns
    ///
    /// Returns a reference to the inner `f64` value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use floatguard::GuardedF64;
    ///
    /// let value = GuardedF64::new(2.0).unwrap();
    /// assert_eq!(*value, 2.0);
    /// ```
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use crate::guarded_f64::tests::{invalid_f64, valid_f64};

    proptest! {
        #[test]
        fn test_from_valid(a in valid_f64()) {
            prop_assert_eq!(GuardedF64::new(a), Ok(GuardedF64(a)));
        }

        #[test]
        fn test_from_invalid(a in invalid_f64()) {
            prop_assert_eq!(GuardedF64::new(a), Err(FloatError));
        }
    }
}
