use crate::FloatError;

use super::UnguardedF32;

/// Implementing the ability to convert `GuardedF32` to `f32` safely.
///
/// This conversion will return an error if the value is NaN or infinite.
impl TryFrom<UnguardedF32> for f32 {
    type Error = FloatError;

    /// Converts a `GuardedF32` to `f32`.
    ///
    /// # Returns
    ///
    /// Returns the inner `f32` value if it is valid (finite), otherwise returns an error.
    ///
    /// # Errors
    ///
    /// Returns `FloatError` if the value is NaN or infinite.
    ///
    /// # Example
    ///
    /// ```rust
    /// use floatguard::{UnguardedF32, FloatError};
    ///
    /// let valid_value = UnguardedF32::new(2.0);
    /// assert_eq!(valid_value.try_into(), Ok(2.0));
    ///
    /// let invalid_value = UnguardedF32::new(f32::NAN);
    /// assert_eq!(f32::try_from(invalid_value), Err(FloatError::NaN));
    ///
    /// let inf_value = UnguardedF32::new(f32::INFINITY);
    /// assert_eq!(f32::try_from(inf_value), Err(FloatError::Infinity));
    /// ```
    fn try_from(value: UnguardedF32) -> Result<Self, Self::Error> {
        value.check().map(Self::from)
    }
}

impl From<f32> for UnguardedF32 {
    /// Converts an `f32` into a `GuardedF32`.
    ///
    /// # Returns
    ///
    /// Returns a `GuardedF32` if the value is valid (finite), otherwise returns an error.
    ///
    /// # Example
    ///
    /// ```rust
    /// use floatguard::{UnguardedF32, GuardedF32, FloatError};
    ///
    /// assert_eq!(UnguardedF32::from(3.14).check(), GuardedF32::new(3.14));
    ///
    /// assert_eq!(UnguardedF32::from(f32::NAN).check(), Err(FloatError::NaN));
    /// ```
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GuardedF32;
    use crate::f32::tests::{invalid_f32, valid_f32};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_from_valid(a in valid_f32()) {
            let checked_a = UnguardedF32::new(a);

            prop_assert_eq!(checked_a.check(), GuardedF32::new(a));
            prop_assert_eq!(f32::try_from(checked_a), Ok(a));
        }

        #[test]
        fn test_from_invalid(a in invalid_f32()) {
            let checked_a = UnguardedF32::new(a);
            let float_error = if a.is_nan() {
                FloatError::NaN
            } else if a.is_infinite() {
                FloatError::Infinity
            } else {
                unreachable!()
            };

            prop_assert_eq!(checked_a.check(), Err(float_error));
            prop_assert_eq!(f32::try_from(checked_a), Err(float_error));
        }
    }
}
