/// An error occurred while processing a floating-point value, indicating that
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct Error;

impl std::error::Error for Error {}

/// Implements the `Display` trait for the `Error` enum, providing a user-friendly
/// description of the error.
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The floating-point value is poisoned")
    }
}

#[derive(Copy, Clone)]
pub struct Result<T>(pub(crate) std::result::Result<T, Error>);

impl<T> Result<T> {
    pub(crate) const fn new(result: std::result::Result<T, Error>) -> Self {
        Self(result)
    }

    pub(crate) const fn as_inner(&self) -> &std::result::Result<T, Error> {
        &self.0
    }
}

impl<T> std::fmt::Debug for Result<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<T> PartialEq<std::result::Result<T, Error>> for Result<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &std::result::Result<T, Error>) -> bool {
        self.0 == *other
    }
}

impl<T> std::ops::Deref for Result<T> {
    type Target = std::result::Result<T, Error>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for Result<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<std::result::Result<T, Error>> for Result<T> {
    fn from(result: std::result::Result<T, Error>) -> Self {
        Self(result)
    }
}

impl<T> From<T> for Result<T> {
    fn from(value: T) -> Self {
        Self(Ok(value))
    }
}

impl<T> From<Result<T>> for std::result::Result<T, Error> {
    fn from(result: Result<T>) -> Self {
        result.0
    }
}
