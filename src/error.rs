/// The types of errors that can occur when retrieving a floating-point number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// The floating-point value was poisoned in a way that resulted in an infinite value.
    InfiniteValue,

    /// The floating-point value was poisoned in a way that resulted in a NaN (Not a Number) value.
    NanValue,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InfiniteValue => write!(f, "The floating-point value is infinite."),
            Error::NanValue => write!(f, "The floating-point value is NaN (Not a Number)."),
        }
    }
}
