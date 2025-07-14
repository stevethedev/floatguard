/// An error occurred while processing a floating-point value, indicating that
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum Error {
    /// Indicates that the floating-point value is NaN (Not a Number).
    NaN,

    /// Indicates that the floating-point value is an infinity.
    Infinity,
}

impl std::error::Error for Error {}

/// Implements the `Display` trait for the `Error` enum, providing a user-friendly
/// description of the error.
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The floating-point value is poisoned")
    }
}
