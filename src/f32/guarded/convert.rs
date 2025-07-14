use super::GuardedF32;
use crate::FloatError;

impl TryFrom<f32> for GuardedF32 {
    type Error = FloatError;

    /// Converts a `f32` to `GuardedF32`.
    ///
    /// # Returns
    ///
    /// Returns a `GuardedF32` if the value is valid (finite), otherwise returns an error.
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
    /// let valid_value = GuardedF32::new(2.0);
    /// assert!(valid_value.is_ok());
    ///
    /// let invalid_value = GuardedF32::new(f32::NAN);
    /// assert!(invalid_value.is_err());
    ///
    /// let inf_value = GuardedF32::new(f32::INFINITY);
    /// assert!(inf_value.is_err());
    /// ```
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// Implementing the ability to convert `GuardedF32` to `f32` safely.
///
/// This conversion will return an error if the value is NaN or infinite.
impl From<GuardedF32> for f32 {
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
    /// use floatguard::{GuardedF32, FloatError};
    ///
    /// let valid_value = GuardedF32::new(2.0).unwrap();
    /// assert_eq!(valid_value.try_into(), Ok(2.0));
    ///
    /// let invalid_value = GuardedF32::try_from(f32::NAN);
    /// assert_eq!(invalid_value, Err(FloatError::NaN));
    ///
    /// let inf_value = GuardedF32::try_from(f32::INFINITY);
    /// assert_eq!(inf_value, Err(FloatError::Infinity));
    /// ```
    fn from(value: GuardedF32) -> Self {
        value.0
    }
}

impl std::ops::Deref for GuardedF32 {
    type Target = f32;

    /// Dereferences `GuardedF32` to its inner `f32` value.
    ///
    /// # Returns
    ///
    /// Returns a reference to the inner `f32` value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use floatguard::GuardedF32;
    ///
    /// let value = GuardedF32::new(2.0).unwrap();
    /// assert_eq!(*value, 2.0);
    /// ```
    fn deref(&self) -> &Self::Target {
        &self.0
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
        fn test_from_valid(a in valid_f32()) {
            prop_assert_eq!(GuardedF32::new(a), Ok(GuardedF32(a)));
            prop_assert_eq!(GuardedF32::new(a).map(f32::from), Ok(a));
            prop_assert_eq!(*GuardedF32::new(a).unwrap(), a);

            prop_assert_eq!(GuardedF32::try_from(a), Ok(GuardedF32(a)));
        }

        #[test]
        fn test_from_invalid(a in invalid_f32()) {
            let float_error = if a.is_nan() {
                FloatError::NaN
            } else if a.is_infinite() {
                FloatError::Infinity
            } else {
                unreachable!()
            };
            prop_assert_eq!(GuardedF32::new(a), Err(float_error));

            prop_assert_eq!(GuardedF32::try_from(a), Err(float_error));
        }
    }
}
