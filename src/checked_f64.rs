use crate::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CheckedF64(f64);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checked_f64_infinite() {
        let inf_value = CheckedF64(f64::INFINITY);
        let result: Result<f64, Error> = inf_value.try_into();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Error::InfiniteValue);
    }

    #[test]
    fn test_checked_f64_nan() {
        let nan_value = CheckedF64(f64::NAN);
        let result: Result<f64, Error> = nan_value.try_into();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Error::NanValue);
    }
}
