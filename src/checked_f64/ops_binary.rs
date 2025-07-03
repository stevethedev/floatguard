use crate::{CheckedF64, CheckedF64Result};

macro_rules! binary_operation {
    (
        $op_trait:ident :: $op_method:ident,
        $doc:literal
    ) => {
        binary_operation!(
            $op_trait :: $op_method,
            fn (lhs: f64, rhs: f64) -> CheckedF64Result {
                let result = std::ops::$op_trait::<f64>::$op_method(lhs, rhs);
                CheckedF64::new(result)
            },
            $doc
        );
    };

    (
        $op_trait:ident::$op_method:ident,
        fn ($lhs:ident : f64, $rhs:ident : f64) -> CheckedF64Result $implementation:block,
        $doc:literal
    ) => {
        // | X |  LHS                | RHS                 | Result Type        | Note                |
        // |---|---------------------|---------------------|--------------------|---------------------|
        // | X | `CheckedF64`        | `CheckedF64`        | `CheckedF64Result` | Standard operations |
        // |   | `CheckedF64`        | `f64`               | `CheckedF64Result` | For ease of use     |
        // |   | `CheckedF64`        | `CheckedF64Result`  | `CheckedF64Result` | For chaining        |
        // |   | `f64`               | `CheckedF64`        | `CheckedF64Result` | For ease of use     |
        // | - | `f64`               | `f64`               | `CheckedF64Result` | Not supported       |
        // |   | `f64`               | `CheckedF64Result`  | `CheckedF64Result` | For ease of use     |
        // |   | `CheckedF64Result`  | `CheckedF64`        | `CheckedF64Result` | For chaining        |
        // |   | `CheckedF64Result`  | `f64`               | `CheckedF64Result` | For chaining        |
        // |   | `CheckedF64Result`  | `CheckedF64Result`  | `CheckedF64Result` | For chaining        |
        binary_operation!(
            $op_trait :: $op_method,
            fn ($lhs: CheckedF64, $rhs: CheckedF64) -> CheckedF64Result {
                let $lhs: f64 = $lhs.0;
                let $rhs: f64 = $rhs.0;
                $implementation
            },
            $doc
        );

        // | X |  LHS                | RHS                 | Result Type        | Note                |
        // |---|---------------------|---------------------|--------------------|---------------------|
        // |   | `CheckedF64`        | `CheckedF64`        | `CheckedF64Result` | Standard operations |
        // | X | `CheckedF64`        | `f64`               | `CheckedF64Result` | For ease of use     |
        // |   | `CheckedF64`        | `CheckedF64Result`  | `CheckedF64Result` | For chaining        |
        // |   | `f64`               | `CheckedF64`        | `CheckedF64Result` | For ease of use     |
        // | - | `f64`               | `f64`               | `CheckedF64Result` | Not supported       |
        // |   | `f64`               | `CheckedF64Result`  | `CheckedF64Result` | For ease of use     |
        // |   | `CheckedF64Result`  | `CheckedF64`        | `CheckedF64Result` | For chaining        |
        // |   | `CheckedF64Result`  | `f64`               | `CheckedF64Result` | For chaining        |
        // |   | `CheckedF64Result`  | `CheckedF64Result`  | `CheckedF64Result` | For chaining        |
        binary_operation!($op_trait :: $op_method,
            fn ($lhs: CheckedF64, $rhs: f64) -> CheckedF64Result {
                let $lhs: f64 = $lhs.0;
                $implementation
            },
            $doc
        );

        // | X |  LHS                | RHS                 | Result Type        | Note                |
        // |---|---------------------|---------------------|--------------------|---------------------|
        // |   | `CheckedF64`        | `CheckedF64`        | `CheckedF64Result` | Standard operations |
        // |   | `CheckedF64`        | `f64`               | `CheckedF64Result` | For ease of use     |
        // | X | `CheckedF64`        | `CheckedF64Result`  | `CheckedF64Result` | For chaining        |
        // |   | `f64`               | `CheckedF64`        | `CheckedF64Result` | For ease of use     |
        // | - | `f64`               | `f64`               | `CheckedF64Result` | Not supported       |
        // |   | `f64`               | `CheckedF64Result`  | `CheckedF64Result` | For ease of use     |
        // |   | `CheckedF64Result`  | `CheckedF64`        | `CheckedF64Result` | For chaining        |
        // |   | `CheckedF64Result`  | `f64`               | `CheckedF64Result` | For chaining        |
        // |   | `CheckedF64Result`  | `CheckedF64Result`  | `CheckedF64Result` | For chaining        |
        binary_operation!($op_trait :: $op_method,
            fn ($lhs: CheckedF64, $rhs: CheckedF64Result) -> CheckedF64Result {
                let Ok($rhs) = *$rhs else { return $rhs; };
                let $lhs: f64 = $lhs.0;
                let $rhs: f64 = $rhs.0;
                $implementation
            },
            $doc
        );

        // | X |  LHS                | RHS                 | Result Type        | Note                |
        // |---|---------------------|---------------------|--------------------|---------------------|
        // |   | `CheckedF64`        | `CheckedF64`        | `CheckedF64Result` | Standard operations |
        // |   | `CheckedF64`        | `f64`               | `CheckedF64Result` | For ease of use     |
        // |   | `CheckedF64`        | `CheckedF64Result`  | `CheckedF64Result` | For chaining        |
        // | X | `f64`               | `CheckedF64`        | `CheckedF64Result` | For ease of use     |
        // | - | `f64`               | `f64`               | `CheckedF64Result` | Not supported       |
        // |   | `f64`               | `CheckedF64Result`  | `CheckedF64Result` | For ease of use     |
        // |   | `CheckedF64Result`  | `CheckedF64`        | `CheckedF64Result` | For chaining        |
        // |   | `CheckedF64Result`  | `f64`               | `CheckedF64Result` | For chaining        |
        // |   | `CheckedF64Result`  | `CheckedF64Result`  | `CheckedF64Result` | For chaining        |
        binary_operation!($op_trait :: $op_method,
            fn ($lhs: f64, $rhs: CheckedF64) -> CheckedF64Result {
                let $rhs = $rhs.0;
                $implementation
            },
            $doc
        );

        // | X |  LHS                | RHS                 | Result Type        | Note                |
        // |---|---------------------|---------------------|--------------------|---------------------|
        // |   | `CheckedF64`        | `CheckedF64`        | `CheckedF64Result` | Standard operations |
        // |   | `CheckedF64`        | `f64`               | `CheckedF64Result` | For ease of use     |
        // |   | `CheckedF64`        | `CheckedF64Result`  | `CheckedF64Result` | For chaining        |
        // |   | `f64`               | `CheckedF64`        | `CheckedF64Result` | For ease of use     |
        // | - | `f64`               | `f64`               | `CheckedF64Result` | Not supported       |
        // | X | `f64`               | `CheckedF64Result`  | `CheckedF64Result` | For ease of use     |
        // |   | `CheckedF64Result`  | `CheckedF64`        | `CheckedF64Result` | For chaining        |
        // |   | `CheckedF64Result`  | `f64`               | `CheckedF64Result` | For chaining        |
        // |   | `CheckedF64Result`  | `CheckedF64Result`  | `CheckedF64Result` | For chaining        |
        binary_operation!($op_trait :: $op_method,
            fn ($lhs: f64, $rhs: CheckedF64Result) -> CheckedF64Result {
                let Ok($rhs) = *$rhs else { return $rhs; };
                let $rhs: f64 = $rhs.0;
                $implementation
            },
            $doc
        );

        // | X |  LHS                | RHS                 | Result Type        | Note                |
        // |---|---------------------|---------------------|--------------------|---------------------|
        // |   | `CheckedF64`        | `CheckedF64`        | `CheckedF64Result` | Standard operations |
        // |   | `CheckedF64`        | `f64`               | `CheckedF64Result` | For ease of use     |
        // |   | `CheckedF64`        | `CheckedF64Result`  | `CheckedF64Result` | For chaining        |
        // |   | `f64`               | `CheckedF64`        | `CheckedF64Result` | For ease of use     |
        // | - | `f64`               | `f64`               | `CheckedF64Result` | Not supported       |
        // |   | `f64`               | `CheckedF64Result`  | `CheckedF64Result` | For ease of use     |
        // | X | `CheckedF64Result`  | `CheckedF64`        | `CheckedF64Result` | For chaining        |
        // |   | `CheckedF64Result`  | `f64`               | `CheckedF64Result` | For chaining        |
        // |   | `CheckedF64Result`  | `CheckedF64Result`  | `CheckedF64Result` | For chaining        |
        binary_operation!($op_trait :: $op_method,
            fn ($lhs: CheckedF64Result, $rhs: CheckedF64) -> CheckedF64Result {
                let Ok($lhs) = *$lhs else { return $lhs; };
                let $lhs: f64 = $lhs.0;
                let $rhs: f64 = $rhs.0;
                $implementation
            },
            $doc
        );

        // | X |  LHS                | RHS                 | Result Type        | Note                |
        // |---|---------------------|---------------------|--------------------|---------------------|
        // |   | `CheckedF64`        | `CheckedF64`        | `CheckedF64Result` | Standard operations |
        // |   | `CheckedF64`        | `f64`               | `CheckedF64Result` | For ease of use     |
        // |   | `CheckedF64`        | `CheckedF64Result`  | `CheckedF64Result` | For chaining        |
        // |   | `f64`               | `CheckedF64`        | `CheckedF64Result` | For ease of use     |
        // | - | `f64`               | `f64`               | `CheckedF64Result` | Not supported       |
        // |   | `f64`               | `CheckedF64Result`  | `CheckedF64Result` | For ease of use     |
        // |   | `CheckedF64Result`  | `CheckedF64`        | `CheckedF64Result` | For chaining        |
        // | X | `CheckedF64Result`  | `f64`               | `CheckedF64Result` | For chaining        |
        // |   | `CheckedF64Result`  | `CheckedF64Result`  | `CheckedF64Result` | For chaining        |
        binary_operation!($op_trait :: $op_method,
            fn ($lhs: CheckedF64Result, $rhs: f64) -> CheckedF64Result {
                let Ok($lhs) = *$lhs else { return $lhs; };
                let $lhs: f64 = $lhs.0;
                $implementation
            },
            $doc
        );

        // | X |  LHS                | RHS                 | Result Type        | Note                |
        // |---|---------------------|---------------------|--------------------|---------------------|
        // |   | `CheckedF64`        | `CheckedF64`        | `CheckedF64Result` | Standard operations |
        // |   | `CheckedF64`        | `f64`               | `CheckedF64Result` | For ease of use     |
        // |   | `CheckedF64`        | `CheckedF64Result`  | `CheckedF64Result` | For chaining        |
        // |   | `f64`               | `CheckedF64`        | `CheckedF64Result` | For ease of use     |
        // | - | `f64`               | `f64`               | `CheckedF64Result` | Not supported       |
        // |   | `f64`               | `CheckedF64Result`  | `CheckedF64Result` | For ease of use     |
        // |   | `CheckedF64Result`  | `CheckedF64`        | `CheckedF64Result` | For chaining        |
        // |   | `CheckedF64Result`  | `f64`               | `CheckedF64Result` | For chaining        |
        // | X | `CheckedF64Result`  | `CheckedF64Result`  | `CheckedF64Result` | For chaining        |
        binary_operation!($op_trait :: $op_method,
            fn ($lhs: CheckedF64Result, $rhs: CheckedF64Result) -> CheckedF64Result {
                let Ok($lhs) = *$lhs else { return $lhs; };
                let Ok($rhs) = *$rhs else { return $rhs; };
                let $lhs: f64 = $lhs.0;
                let $rhs: f64 = $rhs.0;
                $implementation
            },
            $doc
        );
    };

    (
        $op_trait:ident :: $op_method:ident,
        fn ($lhs:ident : $LHS:tt, $rhs:ident : $RHS:tt) -> CheckedF64Result $implementation:block,
        $doc:literal
    ) => {
        #[doc = concat!(
            "Binary Operation `std::ops::", stringify!($op_trait), "` must work with the following types:\n",
            "\n",
            "| X |  LHS                      | RHS                       | Result Type        |\n",
            "|---|---------------------------|---------------------------|--------------------|\n",
            "| X | `", stringify!($LHS), "`  | `", stringify!($RHS), "`  | `CheckedF64Result` |\n",
            "|   | `", stringify!($LHS), "`  | `&", stringify!($RHS), "` | `CheckedF64Result` |\n",
            "|   | `&", stringify!($LHS), "` | `", stringify!($RHS), "`  | `CheckedF64Result` |\n",
            "|   | `&", stringify!($LHS), "` | `&", stringify!($RHS), "` | `CheckedF64Result` |\n",
        )]
        impl std::ops::$op_trait<$RHS> for $LHS {
            type Output = CheckedF64Result;

            #[doc = $doc]
            #[inline(always)]
            fn $op_method(self, $rhs: $RHS) -> Self::Output {
                let $lhs: $LHS = self;
                $implementation
            }
        }

        #[doc = concat!(
            "Binary Operation `std::ops::", stringify!($op_trait), "` must work with the following types:\n",
            "\n",
            "| X |  LHS                      | RHS                       | Result Type        |\n",
            "|---|---------------------------|---------------------------|--------------------|\n",
            "|   | `", stringify!($LHS), "`  | `", stringify!($RHS), "`  | `CheckedF64Result` |\n",
            "| X | `", stringify!($LHS), "`  | `&", stringify!($RHS), "` | `CheckedF64Result` |\n",
            "|   | `&", stringify!($LHS), "` | `", stringify!($RHS), "`  | `CheckedF64Result` |\n",
            "|   | `&", stringify!($LHS), "` | `&", stringify!($RHS), "` | `CheckedF64Result` |\n",
        )]
        impl std::ops::$op_trait<&$RHS> for $LHS {
            type Output = CheckedF64Result;

            #[doc = $doc]
            #[inline(always)]
            fn $op_method(self, $rhs: &$RHS) -> Self::Output {
                self.$op_method(*$rhs)
            }
        }

        #[doc = concat!(
            "Binary Operation `std::ops::", stringify!($op_trait), "` must work with the following types:\n",
            "\n",
            "| X |  LHS                      | RHS                       | Result Type        |\n",
            "|---|---------------------------|---------------------------|--------------------|\n",
            "|   | `", stringify!($LHS), "`  | `", stringify!($RHS), "`  | `CheckedF64Result` |\n",
            "|   | `", stringify!($LHS), "`  | `&", stringify!($RHS), "` | `CheckedF64Result` |\n",
            "| X | `&", stringify!($LHS), "` | `", stringify!($RHS), "`  | `CheckedF64Result` |\n",
            "|   | `&", stringify!($LHS), "` | `&", stringify!($RHS), "` | `CheckedF64Result` |\n",
        )]
        impl std::ops::$op_trait<$RHS> for &$LHS {
            type Output = CheckedF64Result;

            #[doc = $doc]
            #[inline(always)]
            fn $op_method(self, $rhs: $RHS) -> Self::Output {
                (*self).$op_method($rhs)
            }
        }

        #[doc = concat!(
            "Binary Operation `std::ops::", stringify!($op_trait), "` must work with the following types:\n",
            "\n",
            "| X |  LHS                      | RHS                       | Result Type        |\n",
            "|---|---------------------------|---------------------------|--------------------|\n",
            "|   | `", stringify!($LHS), "`  | `", stringify!($RHS), "`  | `CheckedF64Result` |\n",
            "|   | `", stringify!($LHS), "`  | `&", stringify!($RHS), "` | `CheckedF64Result` |\n",
            "|   | `&", stringify!($LHS), "` | `", stringify!($RHS), "`  | `CheckedF64Result` |\n",
            "| X | `&", stringify!($LHS), "` | `&", stringify!($RHS), "` | `CheckedF64Result` |\n",
        )]
        impl std::ops::$op_trait<&$RHS> for &$LHS {
            type Output = CheckedF64Result;

            #[doc = $doc]
            #[inline(always)]
            fn $op_method(self, $rhs: &$RHS) -> Self::Output {
                (*self).$op_method(*$rhs)
            }
        }
    };
}

binary_operation!(
    Add::add,
    r"
        Adds two `CheckedF64` values or a `CheckedF64` and a `f64`.

        # Example

        ```rust
        use checked_float::{CheckedF64, FloatError};

        let value1 = CheckedF64::new(2.0);
        let value2 = CheckedF64::new(3.0);
        assert_eq!(value1 + value2, 5.0);

        let value3 = CheckedF64::new(f64::NAN);
        assert_eq!((value1 + value3).unwrap_err(), FloatError);
        ```
    "
);

binary_operation!(
    Sub::sub,
    r"
        Subtracts one `CheckedF64` value from another or a `f64` from a `CheckedF64`.

        # Example

        ```rust
        use checked_float::{CheckedF64, FloatError};

        let value1 = CheckedF64::new(5.0);
        let value2 = CheckedF64::new(3.0);
        assert_eq!(value1 - value2, 2.0);

        let value3 = CheckedF64::new(f64::NAN);
        assert_eq!((value1 - value3).unwrap_err(), FloatError);
        ```
    "
);

binary_operation!(
    Mul::mul,
    r"
        Multiplies two `CheckedF64` values or a `CheckedF64` and a `f64`.

        # Example

        ```rust
        use checked_float::{CheckedF64, FloatError};

        let value1 = CheckedF64::new(2.0);
        let value2 = CheckedF64::new(3.0);
        assert_eq!(value1 * value2, 6.0);

        let value3 = CheckedF64::new(f64::NAN);
        assert_eq!((value1 * value3).unwrap_err(), FloatError);
        ```
    "
);

binary_operation!(
    Div::div,
    fn (lhs: f64, rhs: f64) -> CheckedF64Result {
        CheckedF64::new({
            if rhs.is_infinite() {
                f64::NAN
            } else {
                lhs / rhs
            }
        })
    },
    r"
        Divides one `CheckedF64` value by another or a `f64` by a `CheckedF64`.

        # Example

        ```rust
        use checked_float::{CheckedF64, FloatError};

        let value1 = CheckedF64::new(6.0);
        let value2 = CheckedF64::new(3.0);
        assert_eq!(value1 / value2, 2.0);

        let value3 = CheckedF64::new(f64::NAN);
        assert_eq!((value1 / value3).unwrap_err(), FloatError);
        ```
    "
);

binary_operation!(
    Rem::rem,
    fn (lhs: f64, rhs: f64) -> CheckedF64Result {
        CheckedF64::new({
            if rhs.is_infinite() {
                f64::NAN
            } else {
                lhs % rhs
            }
        })
    },
    r"
        Computes the remainder of division between two `CheckedF64` values or a `CheckedF64` and
        a `f64`.

        # Example

        ```rust
        use checked_float::{CheckedF64, FloatError};

        let value1 = CheckedF64::new(5.0);
        let value2 = CheckedF64::new(3.0);
        assert_eq!(value1 % value2, 2.0);

        let value3 = CheckedF64::new(f64::NAN);
        assert_eq!((value1 % value3).unwrap_err(), FloatError);
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

    proptest! {

        // Addition Operations
        #[test]
        fn test_valid_add_valid_eq_valid(a in valid_f64(), b in valid_f64()) {
            if (a + b).is_finite() {
                let checked_a = CheckedF64::new(a);
                let checked_b = CheckedF64::new(b);

                prop_assert_eq!(checked_a + checked_b, Ok(a + b));
                prop_assert_eq!(checked_a + b, Ok(a + b));
                prop_assert_eq!(a + checked_b, Ok(a + b));
            }
        }

        #[test]
        fn test_valid_add_invalid_eq_invalid(a in valid_f64(), b in invalid_f64()) {
            prop_assert_eq!(*(CheckedF64::new(a) + CheckedF64::new(b)), Err(FloatError));
            prop_assert_eq!(*(CheckedF64::new(a) + b), Err(FloatError));
            prop_assert_eq!(*(a + CheckedF64::new(b)), Err(FloatError));
        }

        #[test]
        fn test_invalid_add_valid_eq_invalid(a in invalid_f64(), b in valid_f64()) {
            prop_assert_eq!(*(CheckedF64::new(a) + CheckedF64::new(b)), Err(FloatError));
            prop_assert_eq!(*(CheckedF64::new(a) + b), Err(FloatError));
            prop_assert_eq!(*(a + CheckedF64::new(b)), Err(FloatError));
        }

        #[test]
        fn test_invalid_add_invalid_eq_invalid(a in invalid_f64(), b in invalid_f64()) {
            prop_assert_eq!(*(CheckedF64::new(a) + CheckedF64::new(b)), Err(FloatError));
            prop_assert_eq!(*(CheckedF64::new(a) + b), Err(FloatError));
            prop_assert_eq!(*(a + CheckedF64::new(b)), Err(FloatError));
        }

        // Subtraction Operations
        #[test]
        fn test_valid_sub_valid_eq_valid(a in valid_f64(), b in valid_f64()) {
            if (a - b).is_finite() {
                prop_assert_eq!(CheckedF64::new(a) - CheckedF64::new(b), Ok(a - b));
                prop_assert_eq!(CheckedF64::new(a) - b, Ok(a - b));
                prop_assert_eq!(a - CheckedF64::new(b), Ok(a - b));
            }
        }

        #[test]
        fn test_valid_sub_invalid_eq_invalid(a in valid_f64(), b in invalid_f64()) {
            prop_assert_eq!(*(CheckedF64::new(a) - CheckedF64::new(b)), Err(FloatError));
            prop_assert_eq!(*(CheckedF64::new(a) - b), Err(FloatError));
            prop_assert_eq!(*(a - CheckedF64::new(b)), Err(FloatError));
        }

        #[test]
        fn test_invalid_sub_valid_eq_invalid(a in invalid_f64(), b in valid_f64()) {
            prop_assert_eq!(*(CheckedF64::new(a) - CheckedF64::new(b)), Err(FloatError));
            prop_assert_eq!(*(CheckedF64::new(a) - b), Err(FloatError));
            prop_assert_eq!(*(a - CheckedF64::new(b)), Err(FloatError));
        }

        #[test]
        fn test_invalid_sub_invalid_eq_invalid(a in invalid_f64(), b in invalid_f64()) {
            prop_assert_eq!(*(CheckedF64::new(a) - CheckedF64::new(b)), Err(FloatError));
            prop_assert_eq!(*(CheckedF64::new(a) - b), Err(FloatError));
            prop_assert_eq!(*(a - CheckedF64::new(b)), Err(FloatError));
        }

        // Multiplication Operations
        #[test]
        fn test_valid_mul_valid_eq_valid(a in valid_f64(), b in valid_f64()) {
            if (a * b).is_finite() {
                prop_assert_eq!(CheckedF64::new(a) * CheckedF64::new(b), Ok(a * b));
                prop_assert_eq!(CheckedF64::new(a) * b, Ok(a * b));
                prop_assert_eq!(a * CheckedF64::new(b), Ok(a * b));
            }
        }

        #[test]
        fn test_valid_mul_invalid_eq_invalid(a in valid_f64(), b in invalid_f64()) {
            prop_assert_eq!(*(CheckedF64::new(a) * CheckedF64::new(b)), Err(FloatError));
            prop_assert_eq!(*(CheckedF64::new(a) * b), Err(FloatError));
            prop_assert_eq!(*(a * CheckedF64::new(b)), Err(FloatError));
        }

        #[test]
        fn test_invalid_mul_valid_eq_invalid(a in invalid_f64(), b in valid_f64()) {
            prop_assert_eq!(*(CheckedF64::new(a) * CheckedF64::new(b)), Err(FloatError));
            prop_assert_eq!(*(CheckedF64::new(a) * b), Err(FloatError));
            prop_assert_eq!(*(a * CheckedF64::new(b)), Err(FloatError));
        }

        #[test]
        fn test_invalid_mul_invalid_eq_invalid(a in invalid_f64(), b in invalid_f64()) {
            prop_assert_eq!(*(CheckedF64::new(a) * CheckedF64::new(b)), Err(FloatError));
            prop_assert_eq!(*(CheckedF64::new(a) * b), Err(FloatError));
            prop_assert_eq!(*(a * CheckedF64::new(b)), Err(FloatError));
        }

        // Division Operations
        #[test]
        fn test_valid_div_valid_eq_valid(a in valid_f64(), b in valid_f64()) {
            if b != 0.0 && (a / b).is_finite() {
                prop_assert_eq!(CheckedF64::new(a) / CheckedF64::new(b), Ok(a / b));
                prop_assert_eq!(CheckedF64::new(a) / b, Ok(a / b));
                prop_assert_eq!(a / CheckedF64::new(b), Ok(a / b));
            }
        }

        #[test]
        fn test_valid_div_invalid_eq_invalid(a in valid_f64(), b in invalid_f64()) {
            prop_assert_eq!(*(CheckedF64::new(a) / CheckedF64::new(b)), Err(FloatError));
            prop_assert_eq!(*(CheckedF64::new(a) / b), Err(FloatError));
            prop_assert_eq!(*(a / CheckedF64::new(b)), Err(FloatError));
        }

        #[test]
        fn test_invalid_div_valid_eq_invalid(a in invalid_f64(), b in valid_f64()) {
            prop_assert_eq!(*(CheckedF64::new(a) / CheckedF64::new(b)), Err(FloatError));
            prop_assert_eq!(*(CheckedF64::new(a) / b), Err(FloatError));
            prop_assert_eq!(*(a / CheckedF64::new(b)), Err(FloatError));
        }

        #[test]
        fn test_invalid_div_invalid_eq_invalid(a in invalid_f64(), b in invalid_f64()) {
            prop_assert_eq!(*(CheckedF64::new(a) / CheckedF64::new(b)), Err(FloatError));
            prop_assert_eq!(*(CheckedF64::new(a) / b), Err(FloatError));
            prop_assert_eq!(*(a / CheckedF64::new(b)), Err(FloatError));
        }

        // Remainder Operations
        #[test]
        fn test_valid_rem_valid_eq_valid(a in valid_f64(), b in valid_f64()) {
            if b != 0.0 && (a % b).is_finite() {
                prop_assert_eq!(CheckedF64::new(a) % CheckedF64::new(b), Ok(a % b));
                prop_assert_eq!(CheckedF64::new(a) % b, Ok(a % b));
                prop_assert_eq!(a % CheckedF64::new(b), Ok(a % b));
            }
        }

        #[test]
        fn test_valid_rem_invalid_eq_invalid(a in valid_f64(), b in invalid_f64()) {
            prop_assert_eq!(*(CheckedF64::new(a) % CheckedF64::new(b)), Err(FloatError));
            prop_assert_eq!(*(CheckedF64::new(a) % b), Err(FloatError));
            prop_assert_eq!(*(a % CheckedF64::new(b)), Err(FloatError));
        }

        #[test]
        fn test_invalid_rem_valid_eq_invalid(a in invalid_f64(), b in valid_f64()) {
            prop_assert_eq!(*(CheckedF64::new(a) % CheckedF64::new(b)), Err(FloatError));
            prop_assert_eq!(*(CheckedF64::new(a) % b), Err(FloatError));
            prop_assert_eq!(*(a % CheckedF64::new(b)), Err(FloatError));
        }

        #[test]
        fn test_invalid_rem_invalid_eq_invalid(a in invalid_f64(), b in invalid_f64()) {
            prop_assert_eq!(*(CheckedF64::new(a) % CheckedF64::new(b)), Err(FloatError));
            prop_assert_eq!(*(CheckedF64::new(a) % b), Err(FloatError));
            prop_assert_eq!(*(a % CheckedF64::new(b)), Err(FloatError));
        }
    }
}
