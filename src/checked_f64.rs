use crate::Error;

/// Represents a checked floating-point number that ensures it is neither NaN nor infinite.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CheckedF64(f64);

/// Implementing the ability to convert `CheckedF64` to `f64` safely.
/// 
/// This conversion will return an error if the value is NaN or infinite.
/// 
/// ## Example
/// 
/// ```rust
/// use checked_float::{CheckedF64, Error};
/// 
/// assert_eq!(f64::try_from(CheckedF64::from(42.0)), Ok(42.0));
/// assert_eq!(f64::try_from(CheckedF64::from(f64::NAN)), Err(Error::NanValue));
/// assert_eq!(f64::try_from(CheckedF64::from(f64::INFINITY)), Err(Error::InfiniteValue));
/// ```
impl TryFrom<CheckedF64> for f64 {
    type Error = Error;

    fn try_from(value: CheckedF64) -> Result<Self, Self::Error> {
        match (value.0.is_nan(), value.0.is_infinite()) {
            (true, _) => Err(Error::NanValue),
            (_, true) => Err(Error::InfiniteValue),
            _ => Ok(value.0),
        }
    }
}

/// Implementing the ability to convert `f64` to `CheckedF64`.
/// 
/// This conversion will create a `CheckedF64` instance, but it does not check for NaN or
/// infinite values.
/// 
/// ## Example
/// 
/// ```rust
/// use checked_float::CheckedF64;
/// 
/// let checked_value = CheckedF64::from(42.0);
/// 
/// assert_eq!(f64::try_from(checked_value).unwrap(), 42.0);
/// ```
impl From<f64> for CheckedF64 {
    fn from(value: f64) -> Self {
        CheckedF64(value)
    }
}

/// Implementing the unary `-` operator for `CheckedF64`.
/// 
/// This allows negation of the `CheckedF64` value, returning a new `CheckedF64` instance.
/// 
/// ## Example
/// 
/// ```rust
/// use checked_float::CheckedF64;
/// 
/// let value = CheckedF64::from(42.0);
/// let negated_value = -value;
/// 
/// assert_eq!(f64::try_from(negated_value).unwrap(), -42.0);
/// ```
impl std::ops::Neg for CheckedF64 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        CheckedF64(-self.0)
    }
}
