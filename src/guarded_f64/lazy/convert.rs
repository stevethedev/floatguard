use crate::FloatError;

use super::UnguardedF64;

/// Implementing the ability to convert `GuardedF64` to `f64` safely.
///
/// This conversion will return an error if the value is NaN or infinite.
impl TryFrom<UnguardedF64> for f64 {
    type Error = FloatError;

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
    /// use floatguard::{UnguardedF64, FloatError};
    ///
    /// let valid_value = UnguardedF64::new(2.0);
    /// assert_eq!(valid_value.try_into(), Ok(2.0));
    ///
    /// let invalid_value = UnguardedF64::new(f64::NAN);
    /// assert_eq!(f64::try_from(invalid_value), Err(FloatError::NaN));
    ///
    /// let inf_value = UnguardedF64::new(f64::INFINITY);
    /// assert_eq!(f64::try_from(inf_value), Err(FloatError::Infinity));
    /// ```
    fn try_from(value: UnguardedF64) -> Result<Self, Self::Error> {
        value.check().map(Self::from)
    }
}

impl From<f64> for UnguardedF64 {
    /// Converts an `f64` into a `GuardedF64`.
    ///
    /// # Returns
    ///
    /// Returns a `GuardedF64` if the value is valid (finite), otherwise returns an error.
    ///
    /// # Example
    ///
    /// ```rust
    /// use floatguard::{UnguardedF64, GuardedF64, FloatError};
    ///
    /// assert_eq!(UnguardedF64::from(3.14).check(), GuardedF64::new(3.14));
    ///
    /// assert_eq!(UnguardedF64::from(f64::NAN).check(), Err(FloatError::NaN));
    /// ```
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GuardedF64;
    use crate::guarded_f64::tests::{invalid_f64, valid_f64};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_from_valid(a in valid_f64()) {
            let checked_a = UnguardedF64::new(a);

            prop_assert_eq!(checked_a.check(), GuardedF64::new(a));
            prop_assert_eq!(f64::try_from(checked_a), Ok(a));
        }

        #[test]
        fn test_from_invalid(a in invalid_f64()) {
            let checked_a = UnguardedF64::new(a);
            let float_error = if a.is_nan() {
                FloatError::NaN
            } else if a.is_infinite() {
                FloatError::Infinity
            } else {
                unreachable!()
            };

            prop_assert_eq!(checked_a.check(), Err(float_error));
            prop_assert_eq!(f64::try_from(checked_a), Err(float_error));
        }
    }
}
