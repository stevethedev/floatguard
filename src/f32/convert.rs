use crate::FloatError;

use super::{GuardedF32, UnguardedF32};

impl UnguardedF32 {
    /// Checks if the `UnguardedF32` value is valid (finite).
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
    /// use floatguard::{UnguardedF32, FloatError, GuardedF32};
    ///
    /// let unchecked_f32 = UnguardedF32::new(1.0);
    /// assert_eq!(unchecked_f32.check(), GuardedF32::new(1.0));
    ///
    /// let invalid_f32 = UnguardedF32::new(f32::NAN);
    /// assert_eq!(invalid_f32.check(), Err(FloatError::NaN));
    ///
    /// let inf_f32 = UnguardedF32::new(f32::INFINITY);
    /// assert_eq!(inf_f32.check(), Err(FloatError::Infinity));
    ///
    /// let neg_inf_f32 = UnguardedF32::new(f32::NEG_INFINITY);
    /// assert_eq!(neg_inf_f32.check(), Err(FloatError::Infinity));
    /// ```
    pub const fn check(self) -> Result<GuardedF32, FloatError> {
        GuardedF32::new(self.0)
    }
}

impl TryFrom<UnguardedF32> for GuardedF32 {
    type Error = FloatError;

    /// Converts an `UnguardedF32` to `GuardedF32`.
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
    /// use floatguard::{UnguardedF32, FloatError, GuardedF32};
    ///
    /// let valid_value = UnguardedF32::new(2.0);
    /// assert_eq!(valid_value.try_into(), GuardedF32::new(2.0));
    ///
    /// let invalid_value = UnguardedF32::new(f32::NAN);
    /// assert_eq!(GuardedF32::try_from(invalid_value), Err(FloatError::NaN));
    ///
    /// let inf_value = UnguardedF32::new(f32::INFINITY);
    /// assert_eq!(GuardedF32::try_from(inf_value), Err(FloatError::Infinity));
    /// ```
    fn try_from(value: UnguardedF32) -> Result<Self, Self::Error> {
        value.check()
    }
}

impl From<GuardedF32> for UnguardedF32 {
    /// Converts a `GuardedF32` into an `UnguardedF32`.
    ///
    /// # Returns
    ///
    /// Returns a new `UnguardedF32` instance containing the inner `f32` value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use floatguard::{UnguardedF32, GuardedF32};
    ///
    /// let checked_f32 = GuardedF32::new(3.14).unwrap();
    /// let unchecked_f32 = UnguardedF32::from(checked_f32);
    /// assert_eq!(unchecked_f32.check(), GuardedF32::new(3.14));
    /// ```
    fn from(value: GuardedF32) -> Self {
        Self(value.0)
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

        #[test]
        fn test_from_guarded(guarded in valid_f32()) {
            let guarded_f32 = GuardedF32::new(guarded).unwrap();
            let unchecked_f32: UnguardedF32 = UnguardedF32::from(guarded_f32);

            prop_assert_eq!(GuardedF32::try_from(unchecked_f32), Ok(guarded_f32));
        }
    }
}
