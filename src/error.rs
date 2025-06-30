/// The types of errors that can occur when retrieving a floating-point number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// The floating-point value was poisoned in a way that resulted in an infinite value.
    InfiniteValue,

    /// The floating-point value was poisoned in a way that resulted in a NaN (Not a Number) value.
    NanValue,
}

impl std::error::Error for Error {}

/// Implements the `Display` trait for the `Error` enum, providing a user-friendly
/// description of the error.
/// 
/// This implementation formats the error messages for `InfiniteValue` and `NanValue`
/// 
/// # Example
/// 
/// ```rust
/// use checked_float::Error;
/// 
/// let error = Error::InfiniteValue;
/// assert_eq!(error.to_string(), "The floating-point value is infinite.");
/// 
/// let error = Error::NanValue;
/// assert_eq!(error.to_string(), "The floating-point value is NaN (Not a Number).");
/// ```
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InfiniteValue => write!(f, "The floating-point value is infinite."),
            Self::NanValue => write!(f, "The floating-point value is NaN (Not a Number)."),
        }
    }
}
