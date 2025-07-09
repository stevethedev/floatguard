use crate::{GuardedF64, FloatError};

/// Represents a checked floating-point number that ensures it is neither NaN nor infinite.
///
/// # Example
///
/// ```rust
/// use floatguard::{UnguardedF64, FloatError, GuardedF64};
///
/// let unchecked_f64 = UnguardedF64::new(1.0);
/// assert_eq!((unchecked_f64 + 1.0).check(), GuardedF64::new(2.0));
///
/// assert_eq!(unchecked_f64.check(), GuardedF64::new(1.0));
///
/// assert_eq!((unchecked_f64 - f64::INFINITY).check(), Err(FloatError));
/// ```
#[derive(Debug, Default, Clone, Copy)]
pub struct UnguardedF64(pub(crate) f64);

impl UnguardedF64 {
    /// Creates a new `UnguardedF64` instance.
    ///
    /// # Returns
    ///
    /// Returns a new `UnguardedF64` instance containing the provided `f64` value.
    #[must_use = "This function creates a new UnguardedF64 instance, but does not perform any checks on the value."]
    pub const fn new(value: f64) -> Self {
        Self(value)
    }

    /// Checks if the `UnguardedF64` value is valid (finite).
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
    /// use floatguard::{UnguardedF64, FloatError, GuardedF64};
    ///
    /// let unchecked_f64 = UnguardedF64::new(1.0);
    /// assert_eq!(unchecked_f64.check(), GuardedF64::new(1.0));
    ///
    /// let invalid_f64 = UnguardedF64::new(f64::NAN);
    /// assert_eq!(invalid_f64.check(), Err(FloatError));
    ///
    /// let inf_f64 = UnguardedF64::new(f64::INFINITY);
    /// assert_eq!(inf_f64.check(), Err(FloatError));
    ///
    /// let neg_inf_f64 = UnguardedF64::new(f64::NEG_INFINITY);
    /// assert_eq!(neg_inf_f64.check(), Err(FloatError));
    /// ```
    pub const fn check(self) -> Result<GuardedF64, FloatError> {
        GuardedF64::new(self.0)
    }
}

impl std::fmt::Display for UnguardedF64 {
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

impl TryFrom<UnguardedF64> for GuardedF64 {
    type Error = FloatError;

    /// Converts an `UnguardedF64` to `GuardedF64`.
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
    /// use floatguard::{UnguardedF64, FloatError, GuardedF64};
    ///
    /// let valid_value = UnguardedF64::new(2.0);
    /// assert_eq!(valid_value.try_into(), GuardedF64::new(2.0));
    ///
    /// let invalid_value = UnguardedF64::new(f64::NAN);
    /// assert_eq!(GuardedF64::try_from(invalid_value), Err(FloatError));
    ///
    /// let inf_value = UnguardedF64::new(f64::INFINITY);
    /// assert_eq!(GuardedF64::try_from(inf_value), Err(FloatError));
    /// ```
    fn try_from(value: UnguardedF64) -> Result<Self, Self::Error> {
        value.check()
    }
}

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
    /// assert_eq!(f64::try_from(invalid_value), Err(FloatError));
    ///
    /// let inf_value = UnguardedF64::new(f64::INFINITY);
    /// assert_eq!(f64::try_from(inf_value), Err(FloatError));
    /// ```
    fn try_from(value: UnguardedF64) -> Result<Self, Self::Error> {
        value.check().map(Self::from)
    }
}

impl From<GuardedF64> for UnguardedF64 {
    /// Converts a `GuardedF64` into an `UnguardedF64`.
    ///
    /// # Returns
    ///
    /// Returns a new `UnguardedF64` instance containing the inner `f64` value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use floatguard::{UnguardedF64, GuardedF64};
    ///
    /// let checked_f64 = GuardedF64::new(3.14).unwrap();
    /// let unchecked_f64 = UnguardedF64::from(checked_f64);
    /// assert_eq!(unchecked_f64.check(), GuardedF64::new(3.14));
    /// ```
    fn from(value: GuardedF64) -> Self {
        Self(value.0)
    }
}

impl From<f64> for UnguardedF64 {
    /// Converts a `f64` into a `GuardedF64`.
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
    /// assert_eq!(UnguardedF64::from(f64::NAN).check(), Err(FloatError));
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

            prop_assert_eq!(checked_a.check(), Err(FloatError));
            prop_assert_eq!(f64::try_from(checked_a), Err(FloatError));
        }
    }
}
