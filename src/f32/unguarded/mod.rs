mod convert;
mod ops_assign;

/// Represents a checked floating-point number that ensures it is neither NaN nor infinite.
///
/// # Example
///
/// ```rust
/// use floatguard::{UnguardedF32, FloatError, GuardedF32};
///
/// let unchecked_f32 = UnguardedF32::new(1.0);
/// assert_eq!((unchecked_f32 + 1.0).check(), GuardedF32::new(2.0));
///
/// assert_eq!(unchecked_f32.check(), GuardedF32::new(1.0));
///
/// assert_eq!((unchecked_f32 - f32::INFINITY).check(), Err(FloatError::Infinity));
/// ```
#[derive(Debug, Default, Clone, Copy)]
pub struct UnguardedF32(pub(crate) f32);

impl UnguardedF32 {
    /// Creates a new `UnguardedF32` instance.
    ///
    /// # Returns
    ///
    /// Returns a new `UnguardedF32` instance containing the provided `f32` value.
    #[must_use = "This function creates a new UnguardedF32 instance, but does not perform any checks on the value."]
    pub const fn new(value: f32) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for UnguardedF32 {
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
    use super::*;
    use crate::f32::tests::{invalid_f32, valid_f32};
    use crate::{FloatError, GuardedF32};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_new_valid(a in valid_f32()) {
            let unchecked_a = UnguardedF32::new(a);
            prop_assert_eq!(unchecked_a.check(), GuardedF32::new(a));
        }

        #[test]
        fn test_new_invalid(a in invalid_f32()) {
            let unchecked_a = UnguardedF32::new(a);
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
        fn test_display(a in any::<f32>()) {
            let unchecked_a = UnguardedF32::new(a);
            prop_assert_eq!(unchecked_a.to_string(), a.to_string());
        }
    }
}
