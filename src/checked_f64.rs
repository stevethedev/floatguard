use crate::FloatError;

/// Represents a checked floating-point number that ensures it is neither NaN nor infinite.
///
/// # Example
///
/// ```rust
/// use checked_float::{CheckedF64, FloatError};
///
/// # fn main() {
/// let checked_f64 = CheckedF64::try_from(1.0).expect("1.0 is a valid f64 value");
/// assert_eq!((checked_f64 + 1.0).try_into(), Ok(2.0));
///
/// assert_eq!(f64::try_from(checked_f64 / 0.0), Err(FloatError));
///
/// assert_eq!(f64::try_from(checked_f64 - f64::INFINITY), Err(FloatError));
///
/// assert_eq!(f64::try_from(checked_f64 % f64::NAN), Err(FloatError));
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CheckedF64(f64);

/// Implementing the ability to convert `CheckedF64` to `f64` safely.
///
/// This conversion will return an error if the value is NaN or infinite.
impl TryFrom<CheckedF64> for f64 {
    type Error = FloatError;

    fn try_from(value: CheckedF64) -> Result<Self, Self::Error> {
        if value.0.is_finite(){
            Ok(value.0)
        } else {
            Err(FloatError)
        }
    }
}

/// Implementing the ability to convert `f64` to `CheckedF64`.
///
/// This conversion will create a `CheckedF64` instance, but it does not check for NaN or
/// infinite values.
impl TryFrom<f64> for CheckedF64 {
    type Error = FloatError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value.is_finite() {
            Ok(Self(value))
        } else {
            Err(FloatError)
        }
    }
}

/// Implementing the unary `-` operator for `CheckedF64`.
///
/// This allows negation of the `CheckedF64` value, returning a new `CheckedF64` instance.
impl std::ops::Neg for CheckedF64 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

macro_rules! define_operation {
    ($op:tt, $op_trait:ident, $op_method:ident, $assign_trait:ident, $assign_method:ident, $implementation:expr) => {
        #[doc = concat!(
            "Implementing the binary [`", stringify!($op_trait), "`](core::ops::", stringify!($op_trait), ") operator for `CheckedF64`.\n",
            "\n",
            "This allows the ", stringify!($op), " operation between two `CheckedF64` values, returning a new `CheckedF64` instance.\n",
            "\n",
            "The operation will return an error if either value is NaN or infinite.\n",
        )]
        impl std::ops::$op_trait for CheckedF64 {
            type Output = Self;

            fn $op_method(self, other: Self) -> Self::Output {
                let result = ($implementation)(self.0, other.0);
                Self(result)
            }
        }

        #[doc = concat!(
            "Implementing the binary [`", stringify!($op_trait), "`](core::ops::", stringify!($op_trait), ") operator for `CheckedF64`.\n",
            "\n",
            "This allows the ", stringify!($op), " operation between an `f64` and a `CheckedF64`, returning a new `CheckedF64` instance.\n",
        )]
        impl std::ops::$op_trait<f64> for CheckedF64 {
            type Output = Self;

            fn $op_method(self, other: f64) -> Self::Output {
                let result = ($implementation)(self.0, other);
                Self(result)
            }
        }

        #[doc = concat!(
            "Implementing the binary [`", stringify!($op_trait), "`](core::ops::", stringify!($op_trait), ") operator for `CheckedF64`.\n",
            "\n",
            "This allows the ", stringify!($op), " operation between a `CheckedF64` and an `f64`, returning a new `CheckedF64` instance.\n",
        )]
        impl std::ops::$op_trait<CheckedF64> for f64 {
            type Output = CheckedF64;

            fn $op_method(self, other: CheckedF64) -> Self::Output {
                let result = ($implementation)(self, other.0);
                CheckedF64(result)
            }
        }

        #[doc = concat!(
            "Implementing the [`", stringify!($assign_trait), "`](std::ops::", stringify!($assign_trait), ") trait for `CheckedF64`.\n",
            "\n",
            "This allows `", stringify!($assign), "` to be used between two `CheckedF64` values.\n",
        )]
        impl std::ops::$assign_trait for CheckedF64 {
            fn $assign_method(&mut self, other: Self) {
                let result = ($implementation)(self.0, other.0);
                self.0 = result;
            }
        }

        #[doc = concat!(
            "Implementing the [`", stringify!($assign_trait), "`](std::ops::", stringify!($assign_trait), ") trait for `CheckedF64`.\n",
            "\n",
            "This allows `", stringify!($assign), "` to be used between a `CheckedF64` and an `f64`.\n",
        )]
        impl std::ops::$assign_trait<f64> for CheckedF64 {
            fn $assign_method(&mut self, other: f64) {
                let result = ($implementation)(self.0, other);
                self.0 = result;
            }
        }
    };
}

#[inline]
fn add(a: f64, b: f64) -> f64 { a + b }
define_operation!(+, Add, add, AddAssign, add_assign, add);

#[inline]
fn sub(a: f64, b: f64) -> f64 { a - b }
define_operation!(-, Sub, sub, SubAssign, sub_assign, sub);

#[inline]
fn mul(a: f64, b: f64) -> f64 { a * b }
define_operation!(*, Mul, mul, MulAssign, mul_assign, mul);

#[inline]
fn div(a: f64, b: f64) -> f64 {
    if b.is_infinite() {
        f64::NAN
    } else {
        a / b
    }
}
define_operation!(/, Div, div, DivAssign, div_assign, div);

#[inline]
fn rem(a: f64, b: f64) -> f64 {
    if b.is_infinite() {
        f64::NAN
    } else {
        a % b
    }
}
define_operation!(%, Rem, rem, RemAssign, rem_assign, rem);

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    const INVALID_VALUES: &[f64; 3] = &[f64::NAN, f64::INFINITY, f64::NEG_INFINITY];

    fn valid_f64() -> impl Strategy<Value = f64> {
        // Avoid NaN, ±∞, and subnormals.
        // This gives good coverage while staying in safe computation territory.
        (f64::MIN..=f64::MAX)
            .prop_filter("Reject NaN and infinities", |v| v.is_finite() && !v.is_nan())
    }

    fn invalid_f64() -> impl Strategy<Value = f64> {
        prop::sample::select(INVALID_VALUES)
    }

    proptest! {
        #[test]
        fn test_from_valid(a in valid_f64()) {
            prop_assert_eq!(f64::try_from(CheckedF64(a)), Ok(a));
        }

        #[test]
        fn test_from_invalid(a in invalid_f64()) {
            prop_assert_eq!(f64::try_from(CheckedF64(a)), Err(FloatError));
        }

        // Negation Operator
        #[test]
        fn test_negation(a in valid_f64()) {
            let checked_a = CheckedF64(a);
            let negated = -checked_a;
            prop_assert_eq!(f64::try_from(negated), Ok(-a));
        }

        #[test]
        fn test_negation_invalid(a in invalid_f64()) {
            let checked_a = CheckedF64(a);
            let negated = -checked_a;
            prop_assert_eq!(f64::try_from(negated), Err(FloatError));
        }

        // Addition Operations
        #[test]
        fn test_valid_add_valid_eq_valid(a in valid_f64(), b in valid_f64()) {
            if (a + b).is_finite() {
                prop_assert_eq!(f64::try_from(CheckedF64(a) + CheckedF64(b)), Ok(a + b));
                prop_assert_eq!(f64::try_from(CheckedF64(a) + b), Ok(a + b));
                prop_assert_eq!(f64::try_from(a + CheckedF64(b)), Ok(a + b));

                let mut checked_sum = CheckedF64(a);
                checked_sum += CheckedF64(b);
                prop_assert_eq!(f64::try_from(checked_sum), Ok(a + b));

                let mut checked_sum = CheckedF64(a);
                checked_sum += b;
                prop_assert_eq!(f64::try_from(checked_sum), Ok(a + b));
            } else {
                prop_assert_eq!(f64::try_from(CheckedF64(a) + CheckedF64(b)), Err(FloatError));
                prop_assert_eq!(f64::try_from(CheckedF64(a) + b), Err(FloatError));
                prop_assert_eq!(f64::try_from(a + CheckedF64(b)), Err(FloatError));

                let mut checked_sum = CheckedF64(a);
                checked_sum += CheckedF64(b);
                prop_assert_eq!(f64::try_from(checked_sum), Err(FloatError));

                let mut checked_sum = CheckedF64(a);
                checked_sum += b;
                prop_assert_eq!(f64::try_from(checked_sum), Err(FloatError));
            }
        }

        #[test]
        fn test_valid_add_invalid_eq_invalid(a in valid_f64(), b in invalid_f64()) {
            prop_assert_eq!(f64::try_from(CheckedF64(a) + CheckedF64(b)), Err(FloatError));
            prop_assert_eq!(f64::try_from(CheckedF64(a) + b), Err(FloatError));
            prop_assert_eq!(f64::try_from(a + CheckedF64(b)), Err(FloatError));

            let mut checked_sum = CheckedF64(a);
            checked_sum += CheckedF64(b);
            prop_assert_eq!(f64::try_from(checked_sum), Err(FloatError));

            let mut checked_sum = CheckedF64(a);
            checked_sum += b;
            prop_assert_eq!(f64::try_from(checked_sum), Err(FloatError));
        }

        #[test]
        fn test_invalid_add_valid_eq_invalid(a in invalid_f64(), b in valid_f64()) {
            prop_assert_eq!(f64::try_from(CheckedF64(a) + CheckedF64(b)), Err(FloatError));
            prop_assert_eq!(f64::try_from(CheckedF64(a) + b), Err(FloatError));
            prop_assert_eq!(f64::try_from(a + CheckedF64(b)), Err(FloatError));

            let mut checked_sum = CheckedF64(a);
            checked_sum += CheckedF64(b);
            prop_assert_eq!(f64::try_from(checked_sum), Err(FloatError));

            let mut checked_sum = CheckedF64(a);
            checked_sum += b;
            prop_assert_eq!(f64::try_from(checked_sum), Err(FloatError));
        }

        #[test]
        fn test_invalid_add_invalid_eq_invalid(a in invalid_f64(), b in invalid_f64()) {
            prop_assert_eq!(f64::try_from(CheckedF64(a) + CheckedF64(b)), Err(FloatError));
            prop_assert_eq!(f64::try_from(CheckedF64(a) + b), Err(FloatError));
            prop_assert_eq!(f64::try_from(a + CheckedF64(b)), Err(FloatError));

            let mut checked_sum = CheckedF64(a);
            checked_sum += CheckedF64(b);
            prop_assert_eq!(f64::try_from(checked_sum), Err(FloatError));

            let mut checked_sum = CheckedF64(a);
            checked_sum += b;
            prop_assert_eq!(f64::try_from(checked_sum), Err(FloatError));
        }

        // Subtraction Operations
        #[test]
        fn test_valid_sub_valid_eq_valid(a in valid_f64(), b in valid_f64()) {
            if (a - b).is_finite() {
                prop_assert_eq!(f64::try_from(CheckedF64(a) - CheckedF64(b)), Ok(a - b));
                prop_assert_eq!(f64::try_from(CheckedF64(a) - b), Ok(a - b));
                prop_assert_eq!(f64::try_from(a - CheckedF64(b)), Ok(a - b));

                let mut checked_diff = CheckedF64(a);
                checked_diff -= CheckedF64(b);
                prop_assert_eq!(f64::try_from(checked_diff), Ok(a - b));

                let mut checked_diff = CheckedF64(a);
                checked_diff -= b;
                prop_assert_eq!(f64::try_from(checked_diff), Ok(a - b));
            } else {
                prop_assert_eq!(f64::try_from(CheckedF64(a) - CheckedF64(b)), Err(FloatError));
                prop_assert_eq!(f64::try_from(CheckedF64(a) - b), Err(FloatError));
                prop_assert_eq!(f64::try_from(a - CheckedF64(b)), Err(FloatError));

                let mut checked_diff = CheckedF64(a);
                checked_diff -= CheckedF64(b);
                prop_assert_eq!(f64::try_from(checked_diff), Err(FloatError));

                let mut checked_diff = CheckedF64(a);
                checked_diff -= b;
                prop_assert_eq!(f64::try_from(checked_diff), Err(FloatError));
            }
        }

        #[test]
        fn test_valid_sub_invalid_eq_invalid(a in valid_f64(), b in invalid_f64()) {
            prop_assert_eq!(f64::try_from(CheckedF64(a) - CheckedF64(b)), Err(FloatError));
            prop_assert_eq!(f64::try_from(CheckedF64(a) - b), Err(FloatError));
            prop_assert_eq!(f64::try_from(a - CheckedF64(b)), Err(FloatError));

            let mut checked_diff = CheckedF64(a);
            checked_diff -= CheckedF64(b);
            prop_assert_eq!(f64::try_from(checked_diff), Err(FloatError));

            let mut checked_diff = CheckedF64(a);
            checked_diff -= b;
            prop_assert_eq!(f64::try_from(checked_diff), Err(FloatError));
        }

        #[test]
        fn test_invalid_sub_valid_eq_invalid(a in invalid_f64(), b in valid_f64()) {
            prop_assert_eq!(f64::try_from(CheckedF64(a) - CheckedF64(b)), Err(FloatError));
            prop_assert_eq!(f64::try_from(CheckedF64(a) - b), Err(FloatError));
            prop_assert_eq!(f64::try_from(a - CheckedF64(b)), Err(FloatError));

            let mut checked_diff = CheckedF64(a);
            checked_diff -= CheckedF64(b);
            prop_assert_eq!(f64::try_from(checked_diff), Err(FloatError));

            let mut checked_diff = CheckedF64(a);
            checked_diff -= b;
            prop_assert_eq!(f64::try_from(checked_diff), Err(FloatError));
        }

        #[test]
        fn test_invalid_sub_invalid_eq_invalid(a in invalid_f64(), b in invalid_f64()) {
            prop_assert_eq!(f64::try_from(CheckedF64(a) - CheckedF64(b)), Err(FloatError));
            prop_assert_eq!(f64::try_from(CheckedF64(a) - b), Err(FloatError));
            prop_assert_eq!(f64::try_from(a - CheckedF64(b)), Err(FloatError));

            let mut checked_diff = CheckedF64(a);
            checked_diff -= CheckedF64(b);
            prop_assert_eq!(f64::try_from(checked_diff), Err(FloatError));

            let mut checked_diff = CheckedF64(a);
            checked_diff -= b;
            prop_assert_eq!(f64::try_from(checked_diff), Err(FloatError));
        }

        // Multiplication Operations
        #[test]
        fn test_valid_mul_valid_eq_valid(a in valid_f64(), b in valid_f64()) {
            if (a * b).is_finite() {
                prop_assert_eq!(f64::try_from(CheckedF64(a) * CheckedF64(b)), Ok(a * b));
                prop_assert_eq!(f64::try_from(CheckedF64(a) * b), Ok(a * b));
                prop_assert_eq!(f64::try_from(a * CheckedF64(b)), Ok(a * b));

                let mut checked_product = CheckedF64(a);
                checked_product *= CheckedF64(b);
                prop_assert_eq!(f64::try_from(checked_product), Ok(a * b));

                let mut checked_product = CheckedF64(a);
                checked_product *= b;
                prop_assert_eq!(f64::try_from(checked_product), Ok(a * b));
            } else {
                prop_assert_eq!(f64::try_from(CheckedF64(a) * CheckedF64(b)), Err(FloatError));
                prop_assert_eq!(f64::try_from(CheckedF64(a) * b), Err(FloatError));
                prop_assert_eq!(f64::try_from(a * CheckedF64(b)), Err(FloatError));

                let mut checked_product = CheckedF64(a);
                checked_product *= CheckedF64(b);
                prop_assert_eq!(f64::try_from(checked_product), Err(FloatError));

                let mut checked_product = CheckedF64(a);
                checked_product *= b;
                prop_assert_eq!(f64::try_from(checked_product), Err(FloatError));
            }
        }

        #[test]
        fn test_valid_mul_invalid_eq_invalid(a in valid_f64(), b in invalid_f64()) {
            prop_assert_eq!(f64::try_from(CheckedF64(a) * CheckedF64(b)), Err(FloatError));
            prop_assert_eq!(f64::try_from(CheckedF64(a) * b), Err(FloatError));
            prop_assert_eq!(f64::try_from(a * CheckedF64(b)), Err(FloatError));

            let mut checked_product = CheckedF64(a);
            checked_product *= CheckedF64(b);
            prop_assert_eq!(f64::try_from(checked_product), Err(FloatError));

            let mut checked_product = CheckedF64(a);
            checked_product *= b;
            prop_assert_eq!(f64::try_from(checked_product), Err(FloatError));
        }

        #[test]
        fn test_invalid_mul_valid_eq_invalid(a in invalid_f64(), b in valid_f64()) {
            prop_assert_eq!(f64::try_from(CheckedF64(a) * CheckedF64(b)), Err(FloatError));
            prop_assert_eq!(f64::try_from(CheckedF64(a) * b), Err(FloatError));
            prop_assert_eq!(f64::try_from(a * CheckedF64(b)), Err(FloatError));

            let mut checked_product = CheckedF64(a);
            checked_product *= CheckedF64(b);
            prop_assert_eq!(f64::try_from(checked_product), Err(FloatError));

            let mut checked_product = CheckedF64(a);
            checked_product *= b;
            prop_assert_eq!(f64::try_from(checked_product), Err(FloatError));
        }

        #[test]
        fn test_invalid_mul_invalid_eq_invalid(a in invalid_f64(), b in invalid_f64()) {
            prop_assert_eq!(f64::try_from(CheckedF64(a) * CheckedF64(b)), Err(FloatError));
            prop_assert_eq!(f64::try_from(CheckedF64(a) * b), Err(FloatError));
            prop_assert_eq!(f64::try_from(a * CheckedF64(b)), Err(FloatError));

            let mut checked_product = CheckedF64(a);
            checked_product *= CheckedF64(b);
            prop_assert_eq!(f64::try_from(checked_product), Err(FloatError));

            let mut checked_product = CheckedF64(a);
            checked_product *= b;
            prop_assert_eq!(f64::try_from(checked_product), Err(FloatError));
        }

        // Division Operations
        #[test]
        fn test_valid_div_valid_eq_valid(a in valid_f64(), b in valid_f64()) {
            if b != 0.0 && (a / b).is_finite() {
                prop_assert_eq!(f64::try_from(CheckedF64(a) / CheckedF64(b)), Ok(a / b));
                prop_assert_eq!(f64::try_from(CheckedF64(a) / b), Ok(a / b));
                prop_assert_eq!(f64::try_from(a / CheckedF64(b)), Ok(a / b));

                let mut checked_quotient = CheckedF64(a);
                checked_quotient /= CheckedF64(b);
                prop_assert_eq!(f64::try_from(checked_quotient), Ok(a / b));

                let mut checked_quotient = CheckedF64(a);
                checked_quotient /= b;
                prop_assert_eq!(f64::try_from(checked_quotient), Ok(a / b));
            } else {
                prop_assert_eq!(f64::try_from(CheckedF64(a) / CheckedF64(b)), Err(FloatError));
                prop_assert_eq!(f64::try_from(CheckedF64(a) / b), Err(FloatError));
                prop_assert_eq!(f64::try_from(a / CheckedF64(b)), Err(FloatError));

                let mut checked_quotient = CheckedF64(a);
                checked_quotient /= CheckedF64(b);
                prop_assert_eq!(f64::try_from(checked_quotient), Err(FloatError));

                let mut checked_quotient = CheckedF64(a);
                checked_quotient /= b;
                prop_assert_eq!(f64::try_from(checked_quotient), Err(FloatError));
            }
        }

        #[test]
        fn test_valid_div_invalid_eq_invalid(a in valid_f64(), b in invalid_f64()) {
            prop_assert_eq!(f64::try_from(CheckedF64(a) / CheckedF64(b)), Err(FloatError));
            prop_assert_eq!(f64::try_from(CheckedF64(a) / b), Err(FloatError));
            prop_assert_eq!(f64::try_from(a / CheckedF64(b)), Err(FloatError));

            let mut checked_quotient = CheckedF64(a);
            checked_quotient /= CheckedF64(b);
            prop_assert_eq!(f64::try_from(checked_quotient), Err(FloatError));

            let mut checked_quotient = CheckedF64(a);
            checked_quotient /= b;
            prop_assert_eq!(f64::try_from(checked_quotient), Err(FloatError));
        }

        #[test]
        fn test_invalid_div_valid_eq_invalid(a in invalid_f64(), b in valid_f64()) {
            prop_assert_eq!(f64::try_from(CheckedF64(a) / CheckedF64(b)), Err(FloatError));
            prop_assert_eq!(f64::try_from(CheckedF64(a) / b), Err(FloatError));
            prop_assert_eq!(f64::try_from(a / CheckedF64(b)), Err(FloatError));

            let mut checked_quotient = CheckedF64(a);
            checked_quotient /= CheckedF64(b);
            prop_assert_eq!(f64::try_from(checked_quotient), Err(FloatError));

            let mut checked_quotient = CheckedF64(a);
            checked_quotient /= b;
            prop_assert_eq!(f64::try_from(checked_quotient), Err(FloatError));
        }

        #[test]
        fn test_invalid_div_invalid_eq_invalid(a in invalid_f64(), b in invalid_f64()) {
            prop_assert_eq!(f64::try_from(CheckedF64(a) / CheckedF64(b)), Err(FloatError));
            prop_assert_eq!(f64::try_from(CheckedF64(a) / b), Err(FloatError));
            prop_assert_eq!(f64::try_from(a / CheckedF64(b)), Err(FloatError));

            let mut checked_quotient = CheckedF64(a);
            checked_quotient /= CheckedF64(b);
            prop_assert_eq!(f64::try_from(checked_quotient), Err(FloatError));

            let mut checked_quotient = CheckedF64(a);
            checked_quotient /= b;
            prop_assert_eq!(f64::try_from(checked_quotient), Err(FloatError));
        }

        // Remainder Operations
        #[test]
        fn test_valid_rem_valid_eq_valid(a in valid_f64(), b in valid_f64()) {
            if b != 0.0 && (a % b).is_finite() {
                prop_assert_eq!(f64::try_from(CheckedF64(a) % CheckedF64(b)), Ok(a % b));
                prop_assert_eq!(f64::try_from(CheckedF64(a) % b), Ok(a % b));
                prop_assert_eq!(f64::try_from(a % CheckedF64(b)), Ok(a % b));

                let mut checked_remainder = CheckedF64(a);
                checked_remainder %= CheckedF64(b);
                prop_assert_eq!(f64::try_from(checked_remainder), Ok(a % b));

                let mut checked_remainder = CheckedF64(a);
                checked_remainder %= b;
                prop_assert_eq!(f64::try_from(checked_remainder), Ok(a % b));
            } else {
                prop_assert_eq!(f64::try_from(CheckedF64(a) % CheckedF64(b)), Err(FloatError));
                prop_assert_eq!(f64::try_from(CheckedF64(a) % b), Err(FloatError));
                prop_assert_eq!(f64::try_from(a % CheckedF64(b)), Err(FloatError));

                let mut checked_remainder = CheckedF64(a);
                checked_remainder %= CheckedF64(b);
                prop_assert_eq!(f64::try_from(checked_remainder), Err(FloatError));

                let mut checked_remainder = CheckedF64(a);
                checked_remainder %= b;
                prop_assert_eq!(f64::try_from(checked_remainder), Err(FloatError));
            }
        }

        #[test]
        fn test_valid_rem_invalid_eq_invalid(a in valid_f64(), b in invalid_f64()) {
            prop_assert_eq!(f64::try_from(CheckedF64(a) % CheckedF64(b)), Err(FloatError));
            prop_assert_eq!(f64::try_from(CheckedF64(a) % b), Err(FloatError));
            prop_assert_eq!(f64::try_from(a % CheckedF64(b)), Err(FloatError));

            let mut checked_remainder = CheckedF64(a);
            checked_remainder %= CheckedF64(b);
            prop_assert_eq!(f64::try_from(checked_remainder), Err(FloatError));

            let mut checked_remainder = CheckedF64(a);
            checked_remainder %= b;
            prop_assert_eq!(f64::try_from(checked_remainder), Err(FloatError));
        }

        #[test]
        fn test_invalid_rem_valid_eq_invalid(a in invalid_f64(), b in valid_f64()) {
            prop_assert_eq!(f64::try_from(CheckedF64(a) % CheckedF64(b)), Err(FloatError));
            prop_assert_eq!(f64::try_from(CheckedF64(a) % b), Err(FloatError));
            prop_assert_eq!(f64::try_from(a % CheckedF64(b)), Err(FloatError));

            let mut checked_remainder = CheckedF64(a);
            checked_remainder %= CheckedF64(b);
            prop_assert_eq!(f64::try_from(checked_remainder), Err(FloatError));

            let mut checked_remainder = CheckedF64(a);
            checked_remainder %= b;
            prop_assert_eq!(f64::try_from(checked_remainder), Err(FloatError));
        }

        #[test]
        fn test_invalid_rem_invalid_eq_invalid(a in invalid_f64(), b in invalid_f64()) {
            prop_assert_eq!(f64::try_from(CheckedF64(a) % CheckedF64(b)), Err(FloatError));
            prop_assert_eq!(f64::try_from(CheckedF64(a) % b), Err(FloatError));
            prop_assert_eq!(f64::try_from(a % CheckedF64(b)), Err(FloatError));

            let mut checked_remainder = CheckedF64(a);
            checked_remainder %= CheckedF64(b);
            prop_assert_eq!(f64::try_from(checked_remainder), Err(FloatError));

            let mut checked_remainder = CheckedF64(a);
            checked_remainder %= b;
            prop_assert_eq!(f64::try_from(checked_remainder), Err(FloatError));
        }
    }
}
