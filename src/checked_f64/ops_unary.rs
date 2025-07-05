use crate::{CheckedF64, CheckedF64Result};

/// Macro to implement unary operations for `CheckedF64` and `CheckedF64Result`.
/// 
/// This macro generates implementations for unary operations like negation, ensuring that the operation
/// returns a `CheckedF64Result`. It handles both `CheckedF64` and `CheckedF64Result` types, allowing for
/// safe operations on floating-point numbers while checking for invalid values like NaN or Infinity.
/// 
/// # Arguments
/// 
/// - `$op_trait`: The trait for the unary operation (e.g., `Neg`).
/// - `$op_method`: The method name for the operation (e.g., `neg`).
/// - `$implementation`: The implementation function that defines how the operation is performed.
/// - `$doc`: A documentation string that describes the operation and its behavior.
macro_rules! unary_operation {
    (
        $op_trait:ident :: $op_method:ident,
        $doc:literal
    ) => {
        unary_operation!(
            $op_trait :: $op_method,
            fn (lhs: CheckedF64) -> CheckedF64Result { CheckedF64::new(lhs.0.$op_method()) },
            $doc
        );

        unary_operation!(
            $op_trait :: $op_method,
            fn (lhs: CheckedF64Result) -> CheckedF64Result {
                let Ok(lhs) = *lhs else {
                    return lhs;
                };
                lhs.$op_method()
            },
            $doc
        );
    };

    (
        $op_trait:ident :: $op_method:ident,
        fn ($lhs:ident : $LHS:ty) -> CheckedF64Result $implementation:block,
        $doc:literal
    ) => {
        impl std::ops::$op_trait for $LHS {
            type Output = CheckedF64Result;

            #[doc = $doc]
            #[inline(always)]
            fn $op_method(self) -> Self::Output {
                let $lhs: $LHS = self;
                $implementation
            }
        }

        impl std::ops::$op_trait for &$LHS {
            type Output = CheckedF64Result;

            #[doc = $doc]
            fn $op_method(self) -> Self::Output {
                (*self).$op_method()
            }
        }
    };
}

unary_operation!(
    Neg::neg,
    r"
        Negates the `CheckedF64` value.

        # Returns

        Returns a new `CheckedF64` instance with the negated value, or an error if the operation
        results in an invalid floating-point number (like NaN or Infinity).

        # Example

        ```rust
        use checked_float::{CheckedF64, FloatError};

        let value = CheckedF64::new(2.0);
        assert_eq!(-value, -2.0);

        let invalid_value = CheckedF64::new(f64::NAN);
        assert_eq!((-invalid_value).unwrap_err(), FloatError);

        let infinity_value = CheckedF64::new(f64::INFINITY);
        assert_eq!((-infinity_value).unwrap_err(), FloatError);
        ```
    "
);

#[cfg(test)]
mod tests {
    use crate::{
        CheckedF64, FloatError,
        checked_f64::tests::{invalid_f64, valid_f64},
    };
    use proptest::prelude::*;

    macro_rules! prop_assert_float_error {
        ($result:expr) => {
            prop_assert_eq!($result.unwrap_err(), FloatError);
        };

        ($result:expr, $msg:expr) => {
            prop_assert_eq!($result.unwrap_err().to_string(), $msg);
        };
    }

    proptest! {
        #[test]
        fn test_negation(a in valid_f64()) {
            let checked_a = CheckedF64::new(a);
            let expected = CheckedF64::new(-a);

            prop_assert_eq!(-checked_a, expected);
            prop_assert_eq!(-checked_a, -a);
        }

        #[test]
        fn test_negation_invalid(a in invalid_f64()) {
            let checked_a = CheckedF64::new(a);
            prop_assert_float_error!(-checked_a);
        }
    }
}
