use crate::{CheckedF64, CheckedF64Result, FloatError};

macro_rules! binary_operation {
    ($op_trait:ident, $op_method:ident, $implementation:expr, $doc:literal) => {
        impl std::ops::$op_trait for CheckedF64 {
            type Output = CheckedF64Result;

            #[doc = $doc]
            fn $op_method(self, other: Self) -> Self::Output {
                match ($implementation)(self.0, other.0) {
                    result if result.is_finite() => Ok(Self(result)),
                    _ => Err(FloatError),
                }
            }
        }

        impl std::ops::$op_trait<f64> for CheckedF64 {
            type Output = CheckedF64Result;

            #[doc = $doc]
            fn $op_method(self, other: f64) -> Self::Output {
                match ($implementation)(self.0, other) {
                    result if result.is_finite() => Ok(Self(result)),
                    _ => Err(FloatError),
                }
            }
        }

        impl std::ops::$op_trait<CheckedF64> for f64 {
            type Output = CheckedF64Result;

            #[doc = $doc]
            fn $op_method(self, other: CheckedF64) -> Self::Output {
                match ($implementation)(self, other.0) {
                    result if result.is_finite() => Ok(CheckedF64::new(result)),
                    _ => Err(FloatError),
                }
            }
        }
    };
}

#[allow(clippy::inline_always)]
#[inline(always)]
fn add_impl(a: f64, b: f64) -> f64 {
    a + b
}

binary_operation!(
    Add,
    add,
    add_impl,
    r"
        Adds two `CheckedF64` values or a `CheckedF64` and a `f64`.

        # Example

        ```rust
        use checked_float::{CheckedF64, FloatError};

        let value1 = CheckedF64::new(2.0);
        let value2 = CheckedF64::new(3.0);
        assert_eq!(value1 + value2, Ok(CheckedF64::new(5.0)));

        let value3 = CheckedF64::new(f64::NAN);
        assert_eq!(value1 + value3, Err(FloatError));
        ```
    "
);

#[allow(clippy::inline_always)]
#[inline(always)]
fn sub_impl(a: f64, b: f64) -> f64 {
    a - b
}
binary_operation!(
    Sub,
    sub,
    sub_impl,
    r"
        Subtracts one `CheckedF64` value from another or a `f64` from a `CheckedF64`.

        # Example

        ```rust
        use checked_float::{CheckedF64, FloatError};

        let value1 = CheckedF64::new(5.0);
        let value2 = CheckedF64::new(3.0);
        assert_eq!(value1 - value2, Ok(CheckedF64::new(2.0)));

        let value3 = CheckedF64::new(f64::NAN);
        assert_eq!(value1 - value3, Err(FloatError));
        ```
    "
);

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use crate::{FloatError, CheckedF64, checked_f64::tests::{valid_f64, invalid_f64}};

    proptest! {

        // Addition Operations
        #[test]
        fn test_valid_add_valid_eq_valid(a in valid_f64(), b in valid_f64()) {
            if (a + b).is_finite() {
                let checked_a = CheckedF64::new(a);
                let checked_b = CheckedF64::new(b);

                prop_assert_eq!((checked_a + checked_b)?.get(), Ok(a + b));
                prop_assert_eq!((checked_a + b)?.get(), Ok(a + b));
                prop_assert_eq!((a + checked_b)?.get(), Ok(a + b));
            }
        }

        #[test]
        fn test_valid_add_invalid_eq_invalid(a in valid_f64(), b in invalid_f64()) {
            prop_assert_eq!(CheckedF64::new(a) + CheckedF64::new(b), Err(FloatError));
            prop_assert_eq!(CheckedF64::new(a) + b, Err(FloatError));
            prop_assert_eq!(a + CheckedF64::new(b), Err(FloatError));
        }

        #[test]
        fn test_invalid_add_valid_eq_invalid(a in invalid_f64(), b in valid_f64()) {
            prop_assert_eq!(CheckedF64::new(a) + CheckedF64::new(b), Err(FloatError));
            prop_assert_eq!(CheckedF64::new(a) + b, Err(FloatError));
            prop_assert_eq!(a + CheckedF64::new(b), Err(FloatError));
        }

        #[test]
        fn test_invalid_add_invalid_eq_invalid(a in invalid_f64(), b in invalid_f64()) {
            prop_assert_eq!(CheckedF64::new(a) + CheckedF64::new(b), Err(FloatError));
            prop_assert_eq!(CheckedF64::new(a) + b, Err(FloatError));
            prop_assert_eq!(a + CheckedF64::new(b), Err(FloatError));
        }

        // Subtraction Operations
        #[test]
        fn test_valid_sub_valid_eq_valid(a in valid_f64(), b in valid_f64()) {
            if (a - b).is_finite() {
                prop_assert_eq!((CheckedF64::new(a) - CheckedF64::new(b))?.get(), Ok(a - b));
                prop_assert_eq!((CheckedF64::new(a) - b)?.get(), Ok(a - b));
                prop_assert_eq!((a - CheckedF64::new(b))?.get(), Ok(a - b));
            }
        }
    
        #[test]
        fn test_valid_sub_invalid_eq_invalid(a in valid_f64(), b in invalid_f64()) {
            prop_assert_eq!(CheckedF64::new(a) - CheckedF64::new(b), Err(FloatError));
            prop_assert_eq!(CheckedF64::new(a) - b, Err(FloatError));
            prop_assert_eq!(a - CheckedF64::new(b), Err(FloatError));
        }
    
        #[test]
        fn test_invalid_sub_valid_eq_invalid(a in invalid_f64(), b in valid_f64()) {
            prop_assert_eq!(CheckedF64::new(a) - CheckedF64::new(b), Err(FloatError));
            prop_assert_eq!(CheckedF64::new(a) - b, Err(FloatError));
            prop_assert_eq!(a - CheckedF64::new(b), Err(FloatError));
        }
    
        #[test]
        fn test_invalid_sub_invalid_eq_invalid(a in invalid_f64(), b in invalid_f64()) {
            prop_assert_eq!(CheckedF64::new(a) - CheckedF64::new(b), Err(FloatError));
            prop_assert_eq!(CheckedF64::new(a) - b, Err(FloatError));
            prop_assert_eq!(a - CheckedF64::new(b), Err(FloatError));
        }
    }
}
