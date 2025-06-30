use crate::Error;

/// Represents a checked floating-point number that ensures it is neither NaN nor infinite.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CheckedF64(f64);

/// Implementing the ability to convert `CheckedF64` to `f64` safely.
///
/// This conversion will return an error if the value is NaN or infinite.
///
/// # Example
///
/// ```rust
/// use checked_float::{CheckedF64, Error};
///
/// let valid_value = CheckedF64::from(42.0);
/// assert_eq!(f64::try_from(valid_value), Ok(42.0));
///
/// let nan_value = CheckedF64::from(f64::NAN);
/// assert!(matches!(f64::try_from(nan_value), Err(Error::NanValue)));
///
/// let infinite_value = CheckedF64::from(f64::INFINITY);
/// assert!(matches!(f64::try_from(infinite_value), Err(Error::InfiniteValue)));
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
/// # Example
///
/// ```rust
/// use checked_float::{CheckedF64, Error};
///
/// let checked_value = CheckedF64::from(42.0);
/// assert_eq!(f64::try_from(checked_value).unwrap(), 42.0);
///
/// let invalid_value = CheckedF64::from(f64::NAN);
/// assert!(matches!(f64::try_from(invalid_value), Err(Error::NanValue)));
///
/// let infinite_value = CheckedF64::from(f64::INFINITY);
/// assert!(matches!(f64::try_from(infinite_value), Err(Error::InfiniteValue)));
/// ```
impl From<f64> for CheckedF64 {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

/// Implementing the unary `-` operator for `CheckedF64`.
///
/// This allows negation of the `CheckedF64` value, returning a new `CheckedF64` instance.
///
/// # Example
///
/// ```rust
/// use checked_float::{CheckedF64, Error};
///
/// let value = CheckedF64::from(42.0);
/// let negated_value = -value;
/// assert_eq!(f64::try_from(negated_value).unwrap(), -42.0);
///
/// let invalid_value = CheckedF64::from(f64::NAN);
/// let result = -invalid_value;
/// assert!(matches!(f64::try_from(result), Err(Error::NanValue)));
///
/// let infinite_value = CheckedF64::from(f64::INFINITY);
/// let result = -infinite_value;
/// assert!(matches!(f64::try_from(result), Err(Error::InfiniteValue)));
/// ```
impl std::ops::Neg for CheckedF64 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

/// Implementing the binary `+` operator for `CheckedF64`.
///
/// This allows the addition of two `CheckedF64` values, returning a new `CheckedF64` instance.
///
/// # Example
///
/// ```rust
/// use checked_float::{CheckedF64, Error};
///
/// let value1 = CheckedF64::from(42.0);
/// let value2 = CheckedF64::from(58.0);
/// let sum = value1 + value2;
/// assert_eq!(f64::try_from(sum).unwrap(), 100.0);
///
/// let invalid_value = CheckedF64::from(f64::NAN);
/// let result = invalid_value + CheckedF64::from(58.0);
/// assert!(matches!(f64::try_from(result), Err(Error::NanValue)));
///
/// let infinite_value = CheckedF64::from(f64::INFINITY);
/// let result = infinite_value + CheckedF64::from(58.0);
/// assert!(matches!(f64::try_from(result), Err(Error::InfiniteValue)));
///
/// let invalid_value = CheckedF64::from(f64::NAN);
/// let infinite_value = CheckedF64::from(f64::INFINITY);
/// let result = invalid_value + infinite_value;
/// assert!(matches!(f64::try_from(result), Err(Error::NanValue)));
///
/// let infinite_value = CheckedF64::from(f64::INFINITY);
/// let invalid_value = CheckedF64::from(f64::NAN);
/// let result = infinite_value + invalid_value;
/// assert!(matches!(f64::try_from(result), Err(Error::NanValue)));
/// ```
impl std::ops::Add for CheckedF64 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

/// Implementing the binary `+` operator for `CheckedF64` and `f64`.
///
/// This allows the addition of a `CheckedF64` value and an ` f64 ` value, returning a new `CheckedF64` instance.
///
/// # Example
///
/// ```rust
/// use checked_float::{CheckedF64, Error};
///
/// let checked_value = CheckedF64::from(42.0);
/// let sum = checked_value + 58.0;
/// assert_eq!(f64::try_from(sum).unwrap(), 100.0);
///
/// let invalid_value = CheckedF64::from(f64::NAN);
/// let result = invalid_value + 58.0;
/// assert!(matches!(f64::try_from(result), Err(Error::NanValue)));
///
/// let infinite_value = CheckedF64::from(f64::INFINITY);
/// let result = infinite_value + 58.0;
/// assert!(matches!(f64::try_from(result), Err(Error::InfiniteValue)));
///
/// let invalid_value = CheckedF64::from(f64::NAN);
/// let infinite_value = CheckedF64::from(f64::INFINITY);
/// let result = invalid_value + infinite_value;
/// assert!(matches!(f64::try_from(result), Err(Error::NanValue)));
///
/// let infinite_value = CheckedF64::from(f64::INFINITY);
/// let invalid_value = CheckedF64::from(f64::NAN);
/// let result = infinite_value + invalid_value;
/// assert!(matches!(f64::try_from(result), Err(Error::NanValue)));
/// ```
impl std::ops::Add<f64> for CheckedF64 {
    type Output = Self;

    fn add(self, other: f64) -> Self::Output {
        Self(self.0 + other)
    }
}

/// Implementing the binary `+` operator for `f64` and `CheckedF64`.
///
/// This allows the addition of an ` f64 ` value and a `CheckedF64` value, returning a new `CheckedF64` instance.
///
/// # Example
///
/// ```rust
/// use checked_float::{CheckedF64, Error};
///
/// let value = 42.0;
/// let checked_value = CheckedF64::from(58.0);
/// let sum = value + checked_value;
/// assert_eq!(f64::try_from(sum).unwrap(), 100.0);
///
/// let invalid_value = CheckedF64::from(f64::NAN);
/// let result = 58.0 + invalid_value;
/// assert!(matches!(f64::try_from(result), Err(Error::NanValue)));
///
/// let infinite_value = CheckedF64::from(f64::INFINITY);
/// let result = 58.0 + infinite_value;
/// assert!(matches!(f64::try_from(result), Err(Error::InfiniteValue)));
///
/// let invalid_value = CheckedF64::from(f64::NAN);
/// let infinite_value = CheckedF64::from(f64::INFINITY);
/// let result = invalid_value + infinite_value;
/// assert!(matches!(f64::try_from(result), Err(Error::NanValue)));
///
/// let infinite_value = CheckedF64::from(f64::INFINITY);
/// let invalid_value = CheckedF64::from(f64::NAN);
/// let result = infinite_value + invalid_value;
/// assert!(matches!(f64::try_from(result), Err(Error::NanValue)));
/// ```
impl std::ops::Add<CheckedF64> for f64 {
    type Output = CheckedF64;

    fn add(self, other: CheckedF64) -> Self::Output {
        CheckedF64(self + other.0)
    }
}

/// Implementing the `AddAssign` trait for `CheckedF64`.
///
/// This allows the addition of another `CheckedF64` value to the current instance.
///
/// # Example
///
/// ```rust
/// use checked_float::{CheckedF64, Error};
/// let mut checked_value = CheckedF64::from(42.0);
/// let other_value = CheckedF64::from(58.0);
/// checked_value += other_value;
/// assert_eq!(f64::try_from(checked_value).unwrap(), 100.0);
///
/// let invalid_value = CheckedF64::from(f64::NAN);
/// let mut result = CheckedF64::from(42.0);
/// result += invalid_value;
/// assert!(matches!(f64::try_from(result), Err(Error::NanValue)));
///
/// let infinite_value = CheckedF64::from(f64::INFINITY);
/// let mut result = CheckedF64::from(42.0);
/// result += infinite_value;
/// assert!(matches!(f64::try_from(result), Err(Error::InfiniteValue)));
/// ```
impl std::ops::AddAssign for CheckedF64 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

/// Implementing the `AddAssign` trait for `CheckedF64` and `f64`.
///
/// This allows the addition of an `f64` value to the current `CheckedF64` instance.
///
/// # Example
///
/// ```rust
/// use checked_float::{CheckedF64, Error};
///
/// let mut checked_value = CheckedF64::from(42.0);
/// let other_value = 58.0;
/// checked_value += other_value;
/// assert_eq!(f64::try_from(checked_value).unwrap(), 100.0);
///
/// let invalid_value = CheckedF64::from(f64::NAN);
/// let mut result = CheckedF64::from(42.0);
/// result += invalid_value;
/// assert!(matches!(f64::try_from(result), Err(Error::NanValue)));
///
/// let infinite_value = CheckedF64::from(f64::INFINITY);
/// let mut result = CheckedF64::from(42.0);
/// result += infinite_value;
/// assert!(matches!(f64::try_from(result), Err(Error::InfiniteValue)));
/// ```
impl std::ops::AddAssign<f64> for CheckedF64 {
    fn add_assign(&mut self, other: f64) {
        self.0 += other;
    }
}

/// Implementing the binary `-` operator for `CheckedF64`.
///
/// This allows the subtraction of two `CheckedF64` values, returning a new `CheckedF64` instance.
///
/// # Example
///
/// ```rust
/// use checked_float::{CheckedF64, Error};
///
/// let value1 = CheckedF64::from(100.0);
/// let value2 = CheckedF64::from(42.0);
/// let difference = value1 - value2;
/// assert_eq!(f64::try_from(difference).unwrap(), 58.0);
///
/// let invalid_value = CheckedF64::from(f64::NAN);
/// let result = invalid_value - CheckedF64::from(42.0);
/// assert!(matches!(f64::try_from(result), Err(Error::NanValue)));
///
/// let infinite_value = CheckedF64::from(f64::INFINITY);
/// let result = infinite_value - CheckedF64::from(42.0);
/// assert!(matches!(f64::try_from(result), Err(Error::InfiniteValue)));
///
/// let invalid_value = CheckedF64::from(f64::NAN);
/// let infinite_value = CheckedF64::from(f64::INFINITY);
/// let result = invalid_value - infinite_value;
/// assert!(matches!(f64::try_from(result), Err(Error::NanValue)));
///
/// let infinite_value = CheckedF64::from(f64::INFINITY);
/// let invalid_value = CheckedF64::from(f64::NAN);
/// let result = infinite_value - invalid_value;
/// assert!(matches!(f64::try_from(result), Err(Error::NanValue)));
/// ```
impl std::ops::Sub for CheckedF64 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}

/// Implementing the binary `-` operator for `CheckedF64` and `f64`.
///
/// This allows the subtraction of a `CheckedF64` value from an `f64` value, returning a new `CheckedF64` instance.
///
/// # Example
///
/// ```rust
/// use checked_float::{CheckedF64, Error};
///
/// let checked_value = CheckedF64::from(100.0);
/// let difference = checked_value - 42.0;
/// assert_eq!(f64::try_from(difference).unwrap(), 58.0);
///
/// let invalid_value = CheckedF64::from(f64::NAN);
/// let result = invalid_value - 42.0;
/// assert!(matches!(f64::try_from(result), Err(Error::NanValue)));
///
/// let infinite_value = CheckedF64::from(f64::INFINITY);
/// let result = infinite_value - 42.0;
/// assert!(matches!(f64::try_from(result), Err(Error::InfiniteValue)));
///
/// let invalid_value = CheckedF64::from(f64::NAN);
/// let infinite_value = CheckedF64::from(f64::INFINITY);
/// let result = invalid_value - infinite_value;
/// assert!(matches!(f64::try_from(result), Err(Error::NanValue)));
///
/// let infinite_value = CheckedF64::from(f64::INFINITY);
/// let invalid_value = CheckedF64::from(f64::NAN);
/// let result = infinite_value - invalid_value;
/// assert!(matches!(f64::try_from(result), Err(Error::NanValue)));
/// ```
impl std::ops::Sub<f64> for CheckedF64 {
    type Output = Self;

    fn sub(self, other: f64) -> Self::Output {
        Self(self.0 - other)
    }
}

/// Implementing the binary `-` operator for `f64` and `CheckedF64`.
///
/// This allows the subtraction of a `CheckedF64` value from an `f64` value, returning a new `CheckedF64` instance.
///
/// # Example
///
/// ```rust
/// use checked_float::{CheckedF64, Error};
///
/// let value = 100.0;
/// let checked_value = CheckedF64::from(42.0);
/// let difference = value - checked_value;
/// assert_eq!(f64::try_from(difference).unwrap(), 58.0);
///
/// let invalid_value = CheckedF64::from(f64::NAN);
/// let result = 100.0 - invalid_value;
/// assert!(matches!(f64::try_from(result), Err(Error::NanValue)));
///
/// let infinite_value = CheckedF64::from(f64::INFINITY);
/// let result = 100.0 - infinite_value;
/// assert!(matches!(f64::try_from(result), Err(Error::InfiniteValue)));
///
/// let invalid_value = CheckedF64::from(f64::NAN);
/// let infinite_value = CheckedF64::from(f64::INFINITY);
/// let result = invalid_value - infinite_value;
/// assert!(matches!(f64::try_from(result), Err(Error::NanValue)));
///
/// let infinite_value = CheckedF64::from(f64::INFINITY);
/// let invalid_value = CheckedF64::from(f64::NAN);
/// let result = infinite_value - invalid_value;
/// assert!(matches!(f64::try_from(result), Err(Error::NanValue)));
/// ```
impl std::ops::Sub<CheckedF64> for f64 {
    type Output = CheckedF64;

    fn sub(self, other: CheckedF64) -> Self::Output {
        CheckedF64(self - other.0)
    }
}

/// Implementing the `SubAssign` trait for `CheckedF64`.
///
/// This allows the subtraction of another `CheckedF64` value from the current instance.
///
/// # Example
///
/// ```rust
/// use checked_float::{CheckedF64, Error};
///
/// let mut checked_value = CheckedF64::from(100.0);
/// let other_value = CheckedF64::from(42.0);
/// checked_value -= other_value;
/// assert_eq!(f64::try_from(checked_value).unwrap(), 58.0);
///
/// let invalid_value = CheckedF64::from(f64::NAN);
/// let mut result = CheckedF64::from(100.0);
/// result -= invalid_value;
/// assert!(matches!(f64::try_from(result), Err(Error::NanValue)));
///
/// let infinite_value = CheckedF64::from(f64::INFINITY);
/// let mut result = CheckedF64::from(100.0);
/// result -= infinite_value;
/// assert!(matches!(f64::try_from(result), Err(Error::InfiniteValue)));
/// ```
impl std::ops::SubAssign for CheckedF64 {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}

/// Implementing the `SubAssign` trait for `CheckedF64` and `f64`.
///
/// This allows the subtraction of an `f64` value from the current `CheckedF64` instance.
///
/// # Example
///
/// ```rust
/// use checked_float::{CheckedF64, Error};
///
/// let mut checked_value = CheckedF64::from(100.0);
/// let other_value = 42.0;
/// checked_value -= other_value;
/// assert_eq!(f64::try_from(checked_value).unwrap(), 58.0);
///
/// let invalid_value = CheckedF64::from(f64::NAN);
/// let mut result = CheckedF64::from(100.0);
/// result -= invalid_value;
/// assert!(matches!(f64::try_from(result), Err(Error::NanValue)));
///
/// let infinite_value = CheckedF64::from(f64::INFINITY);
/// let mut result = CheckedF64::from(100.0);
/// result -= infinite_value;
/// assert!(matches!(f64::try_from(result), Err(Error::InfiniteValue)));
/// ```
impl std::ops::SubAssign<f64> for CheckedF64 {
    fn sub_assign(&mut self, other: f64) {
        self.0 -= other;
    }
}

/// Implementing the `Mul` trait for `CheckedF64`.
///
/// This allows the multiplication of two `CheckedF64` values, returning a new `CheckedF64` instance.
///
/// # Example
///
/// ```rust
/// use checked_float::{CheckedF64, Error};
///
/// let value1 = CheckedF64::from(6.0);
/// let value2 = CheckedF64::from(7.0);
/// let product = value1 * value2;
/// assert_eq!(f64::try_from(product).unwrap(), 42.0);
///
/// let invalid_value = CheckedF64::from(f64::NAN);
/// let result = invalid_value * CheckedF64::from(7.0);
/// assert!(matches!(f64::try_from(result), Err(Error::NanValue)));
///
/// let infinite_value = CheckedF64::from(f64::INFINITY);
/// let result = infinite_value * CheckedF64::from(7.0);
/// assert!(matches!(f64::try_from(result), Err(Error::InfiniteValue)));
///
/// let invalid_value = CheckedF64::from(f64::NAN);
/// let infinite_value = CheckedF64::from(f64::INFINITY);
/// let result = invalid_value * infinite_value;
/// assert!(matches!(f64::try_from(result), Err(Error::NanValue)));
///
/// let infinite_value = CheckedF64::from(f64::INFINITY);
/// let invalid_value = CheckedF64::from(f64::NAN);
/// let result = infinite_value * invalid_value;
/// assert!(matches!(f64::try_from(result), Err(Error::NanValue)));
/// ```
impl std::ops::Mul for CheckedF64 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self(self.0 * other.0)
    }
}

/// Implementing the binary `*` operator for `CheckedF64` and `f64`.
///
/// This allows the multiplication of a `CheckedF64` value and an `f64` value, returning a new `CheckedF64` instance.
///
/// # Example
///
/// ```rust
/// use checked_float::{CheckedF64, Error};
///
/// let checked_value = CheckedF64::from(6.0);
/// let product = checked_value * 7.0;
/// assert_eq!(f64::try_from(product).unwrap(), 42.0);
///
/// let invalid_value = CheckedF64::from(f64::NAN);
/// let result = invalid_value * 7.0;
/// assert!(matches!(f64::try_from(result), Err(Error::NanValue)));
///
/// let infinite_value = CheckedF64::from(f64::INFINITY);
/// let result = infinite_value * 7.0;
/// assert!(matches!(f64::try_from(result), Err(Error::InfiniteValue)));
///
/// let invalid_value = CheckedF64::from(f64::NAN);
/// let infinite_value = CheckedF64::from(f64::INFINITY);
/// let result = invalid_value * infinite_value;
/// assert!(matches!(f64::try_from(result), Err(Error::NanValue)));
///
/// let infinite_value = CheckedF64::from(f64::INFINITY);
/// let invalid_value = CheckedF64::from(f64::NAN);
/// let result = infinite_value * invalid_value;
/// assert!(matches!(f64::try_from(result), Err(Error::NanValue)));
/// ```
impl std::ops::Mul<f64> for CheckedF64 {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Self(self.0 * other)
    }
}

/// Implementing the binary `*` operator for `f64` and `CheckedF64`.
///
/// This allows the multiplication of an `f64` value and a `CheckedF64` value, returning a new `CheckedF64` instance.
///
/// # Example
///
/// ```rust
/// use checked_float::{CheckedF64, Error};
///
/// let value = 6.0;
/// let checked_value = CheckedF64::from(7.0);
/// let product = value * checked_value;
/// assert_eq!(f64::try_from(product).unwrap(), 42.0);
///
/// let invalid_value = CheckedF64::from(f64::NAN);
/// let result = 6.0 * invalid_value;
/// assert!(matches!(f64::try_from(result), Err(Error::NanValue)));
///
/// let infinite_value = CheckedF64::from(f64::INFINITY);
/// let result = 6.0 * infinite_value;
/// assert!(matches!(f64::try_from(result), Err(Error::InfiniteValue)));
///
/// let invalid_value = CheckedF64::from(f64::NAN);
/// let infinite_value = CheckedF64::from(f64::INFINITY);
/// let result = invalid_value * infinite_value;
/// assert!(matches!(f64::try_from(result), Err(Error::NanValue)));
///
/// let infinite_value = CheckedF64::from(f64::INFINITY);
/// let invalid_value = CheckedF64::from(f64::NAN);
/// let result = infinite_value * invalid_value;
/// assert!(matches!(f64::try_from(result), Err(Error::NanValue)));
/// ```
impl std::ops::Mul<CheckedF64> for f64 {
    type Output = CheckedF64;

    fn mul(self, other: CheckedF64) -> Self::Output {
        CheckedF64(self * other.0)
    }
}

/// Implementing the `MulAssign` trait for `CheckedF64`.
///
/// This allows the multiplication of another `CheckedF64` value with the current instance.
///
/// # Example
///
/// ```rust
/// use checked_float::{CheckedF64, Error};
///
/// let mut checked_value = CheckedF64::from(6.0);
/// let other_value = CheckedF64::from(7.0);
/// checked_value *= other_value;
/// assert_eq!(f64::try_from(checked_value).unwrap(), 42.0);
///
/// let invalid_value = CheckedF64::from(f64::NAN);
/// let mut result = CheckedF64::from(6.0);
/// result *= invalid_value;
/// assert!(matches!(f64::try_from(result), Err(Error::NanValue)));
///
/// let infinite_value = CheckedF64::from(f64::INFINITY);
/// let mut result = CheckedF64::from(6.0);
/// result *= infinite_value;
/// assert!(matches!(f64::try_from(result), Err(Error::InfiniteValue)));
/// ```
impl std::ops::MulAssign for CheckedF64 {
    fn mul_assign(&mut self, other: Self) {
        self.0 *= other.0;
    }
}

/// Implementing the `MulAssign` trait for `CheckedF64` and `f64`.
///
/// This allows the multiplication of an `f64` value with the current `CheckedF64` instance.
///
/// # Example
///
/// ```rust
/// use checked_float::{CheckedF64, Error};
///
/// let mut checked_value = CheckedF64::from(6.0);
/// let other_value = 7.0;
/// checked_value *= other_value;
/// assert_eq!(f64::try_from(checked_value).unwrap(), 42.0);
///
/// let invalid_value = CheckedF64::from(f64::NAN);
/// let mut result = CheckedF64::from(6.0);
/// result *= invalid_value;
/// assert!(matches!(f64::try_from(result), Err(Error::NanValue)));
///
/// let infinite_value = CheckedF64::from(f64::INFINITY);
/// let mut result = CheckedF64::from(6.0);
/// result *= infinite_value;
/// assert!(matches!(f64::try_from(result), Err(Error::InfiniteValue)));
/// ```
impl std::ops::MulAssign<f64> for CheckedF64 {
    fn mul_assign(&mut self, other: f64) {
        self.0 *= other;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    macro_rules! operation_test {
        ($op:tt, $op_assign:tt, $a:expr, $b:expr, $expected:expr) => {
            let checked_a = CheckedF64::from($a);
            let checked_b = CheckedF64::from($b);
            
            let result_ca_cb = checked_a $op checked_b;
            let result_ca_b = checked_a $op ($b);
            let result_a_cb = ($a) $op checked_b;
            
            let mut result_mut_ca_cb = CheckedF64::from($a);
            result_mut_ca_cb $op_assign checked_b;

            let mut result_mut_ca_b = CheckedF64::from($a);
            result_mut_ca_b $op_assign ($b);

            if ($a $op $b).is_nan() || ($a $op $b).is_infinite() {
                assert!(f64::try_from(result_ca_cb).is_err());
                assert!(f64::try_from(result_ca_b).is_err());
                assert!(f64::try_from(result_a_cb).is_err());
                assert!(f64::try_from(result_mut_ca_cb).is_err());
                assert!(f64::try_from(result_mut_ca_b).is_err());
            } else {
                assert_eq!(f64::try_from(result_ca_cb).unwrap(), ($a) $op ($b));
                assert_eq!(f64::try_from(result_a_cb).unwrap(), ($a) $op ($b));
                assert_eq!(f64::try_from(result_a_cb).unwrap(), ($a) $op ($b));
                assert_eq!(f64::try_from(result_mut_ca_cb).unwrap(), ($a) $op ($b));
                assert_eq!(f64::try_from(result_mut_ca_b).unwrap(), ($a) $op ($b));
            }
        };
    }

    proptest! {
        #[test]
        fn test_checked_f64_negation(a in any::<f64>()) {
            let checked_a = CheckedF64::from(a);
            let negated = -checked_a;
            
            if a.is_nan() || a.is_infinite() {
                assert!(f64::try_from(negated).is_err());
            } else {
                assert_eq!(f64::try_from(negated).unwrap(), -a);
            }
        }

        #[test]
        fn test_checked_f64_addition(a in any::<f64>(), b in any::<f64>()) {
            operation_test!(+, +=, a, b, a + b);
        }

        #[test]
        fn test_checked_f64_subtraction(a in any::<f64>(), b in any::<f64>()) {
            operation_test!(-, -=, a, b, a + b);
        }

        #[test]
        fn test_checked_f64_multiplication(a in any::<f64>(), b in any::<f64>()) {
            operation_test!(*, *=, a, b, a + b);
        }
    }
}
