mod convert;
mod ops_assign;

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
/// assert_eq!((unchecked_f64 - f64::INFINITY).check(), Err(FloatError::Infinity));
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::guarded_f64::tests::{invalid_f64, valid_f64};
    use crate::{FloatError, GuardedF64};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_new_valid(a in valid_f64()) {
            let unchecked_a = UnguardedF64::new(a);
            prop_assert_eq!(unchecked_a.check(), GuardedF64::new(a));
        }

        #[test]
        fn test_new_invalid(a in invalid_f64()) {
            let unchecked_a = UnguardedF64::new(a);
            let float_error = if a.is_nan() {
                FloatError::NaN
            } else if a.is_infinite() {
                FloatError::Infinity
            } else {
                unreachable!()
            };

            prop_assert_eq!(unchecked_a.check(), Err(float_error));
        }

        #[test]
        fn test_display(a in any::<f64>()) {
            let unchecked_a = UnguardedF64::new(a);
            prop_assert_eq!(unchecked_a.to_string(), a.to_string());
        }
    }
}
