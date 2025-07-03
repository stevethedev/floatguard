mod cmp;
mod consts;
mod ops_binary;
mod ops_unary;
mod math;

use crate::FloatError;

/// A result type for operations on `CheckedF64` that can return a `FloatError`.
pub type Result = crate::FloatResult<CheckedF64>;

/// Represents a checked floating-point number that ensures it is neither NaN nor infinite.
///
/// # Example
///
/// ```rust
/// use checked_float::{CheckedF64, FloatError};
///
/// let checked_f64 = CheckedF64::new(1.0).expect("1.0 is a valid f64 value");
/// assert_eq!(checked_f64 + 1.0, Ok(2.0));
///
/// assert_eq!(*(checked_f64 / 0.0), Err(FloatError));
///
/// assert_eq!(*(checked_f64 - f64::INFINITY), Err(FloatError));
///
/// assert_eq!(*(checked_f64 % f64::NAN), Err(FloatError));
/// ```
#[derive(Debug, Default, Clone, Copy)]
pub struct CheckedF64(f64);

impl CheckedF64 {
    #[must_use = "this function returns a new CheckedF64 instance"]
    pub const fn new(value: f64) -> Result {
        Result::new(if value.is_finite() {
            Ok(Self(value))
        } else {
            Err(FloatError)
        })
    }
}

impl std::fmt::Display for CheckedF64 {
    /// Formats the `CheckedF64` as a string.
    ///
    /// # Returns
    ///
    /// Returns a string representation of the inner `f64` value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use checked_float::CheckedF64;
    ///
    /// let value = CheckedF64::new(2.0).unwrap();
    /// assert_eq!(value.to_string(), "2");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Implementing the ability to convert `CheckedF64` to `f64` safely.
///
/// This conversion will return an error if the value is NaN or infinite.
impl From<CheckedF64> for f64 {
    /// Converts a `CheckedF64` to `f64`.
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
    /// use checked_float::{CheckedF64, FloatError};
    ///
    /// let valid_value = CheckedF64::new(2.0);
    /// assert_eq!(valid_value, 2.0);
    ///
    /// let invalid_value = CheckedF64::new(f64::NAN);
    /// assert!(invalid_value.is_err());
    ///
    /// let inf_value = CheckedF64::new(f64::INFINITY);
    /// assert!(inf_value.is_err());
    /// ```
    fn from(value: CheckedF64) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    const INVALID_VALUES: &[f64; 3] = &[f64::NAN, f64::INFINITY, f64::NEG_INFINITY];

    pub fn valid_f64() -> impl Strategy<Value = f64> {
        // Avoid NaN, ±∞, and subnormals.
        // This gives good coverage while staying in safe computation territory.
        (f64::MIN..=f64::MAX).prop_filter("Reject NaN and infinities", |v| {
            v.is_finite() && !v.is_nan()
        })
    }

    pub fn invalid_f64() -> impl Strategy<Value = f64> {
        prop::sample::select(INVALID_VALUES)
    }

    proptest! {
        #[test]
        fn test_from_valid(a in valid_f64()) {
            prop_assert_eq!(CheckedF64::new(a), Ok(a));
        }

        #[test]
        fn test_from_invalid(a in invalid_f64()) {
            prop_assert_eq!(*CheckedF64::new(a), Err(FloatError));
        }
    }
}
