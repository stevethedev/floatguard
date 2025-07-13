use super::GuardedF64;
use crate::FloatError;

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
    #![allow(clippy::float_cmp)]

    use super::*;
    use crate::guarded_f64::tests::{invalid_f64, valid_f64};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_from_valid(a in valid_f64()) {
            prop_assert_eq!(GuardedF64::new(a), Ok(GuardedF64(a)));
            prop_assert_eq!(GuardedF64::new(a).map(f64::from), Ok(a));
            prop_assert_eq!(*GuardedF64::new(a).unwrap(), a);

            prop_assert_eq!(GuardedF64::try_from(a), Ok(GuardedF64(a)));
        }

        #[test]
        fn test_from_invalid(a in invalid_f64()) {
            prop_assert_eq!(GuardedF64::new(a), Err(FloatError));

            prop_assert_eq!(GuardedF64::try_from(a), Err(FloatError));
        }
    }
}
