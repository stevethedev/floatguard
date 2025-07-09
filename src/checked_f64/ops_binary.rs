use super::{CheckedF64, UncheckedF64};

/// Defines a binary operation for `CheckedF64` with the specified trait and method.
///
/// This macro generates implementations for various combinations of `CheckedF64`, `f64`, and `UncheckedF64`
/// types, allowing for flexible arithmetic operations while ensuring that the results are checked for validity.
///
/// # Arguments
///
/// * `$op_trait` - The trait representing the binary operation (e.g., `Add`, `Sub`, etc.).
/// * `$op_method` - The method name for the operation (e.g., `add`, `sub`, etc.).
/// * `$implementation` - A block of code that defines how the operation is performed.
/// * `$doc` - A documentation string that describes the operation and its usage.
macro_rules! binary_operation {
    (
        $op_trait:ident :: $op_method:ident,
        $doc:literal
    ) => {
        binary_operation!(
            $op_trait :: $op_method,
            fn (lhs: f64, rhs: f64) -> UncheckedF64 {
                let result = std::ops::$op_trait::<f64>::$op_method(lhs, rhs);
                UncheckedF64::new(result)
            },
            $doc
        );
    };

    (
        $op_trait:ident::$op_method:ident,
        fn ($lhs:ident : f64, $rhs:ident : f64) -> UncheckedF64 $implementation:block,
        $doc:literal
    ) => {
        // | X | LHS             | RHS            | Result Type    | Note                |
        // |---|-----------------|----------------|----------------|---------------------|
        // | X | `CheckedF64`    | `CheckedF64`   | `UncheckedF64` | Standard operations |
        // |   | `CheckedF64`    | `f64`          | `UncheckedF64` | For ease of use     |
        // |   | `CheckedF64`    | `UncheckedF64` | `UncheckedF64` | For chaining        |
        // |   | `f64`           | `CheckedF64`   | `UncheckedF64` | For ease of use     |
        // | - | `f64`           | `f64`          | `UncheckedF64` | Not supported       |
        // |   | `f64`           | `UncheckedF64` | `UncheckedF64` | For ease of use     |
        // |   | `UncheckedF64`  | `CheckedF64`   | `UncheckedF64` | For chaining        |
        // |   | `UncheckedF64`  | `f64`          | `UncheckedF64` | For chaining        |
        // |   | `UncheckedF64`  | `UncheckedF64` | `UncheckedF64` | For chaining        |
        binary_operation!(
            $op_trait :: $op_method,
            fn ($lhs: CheckedF64, $rhs: CheckedF64) -> UncheckedF64 {
                let CheckedF64($lhs) = $lhs;
                let CheckedF64($rhs) = $rhs;
                $implementation
            },
            $doc
        );

        // | X | LHS             | RHS            | Result Type    | Note                |
        // |---|-----------------|----------------|----------------|---------------------|
        // |   | `CheckedF64`    | `CheckedF64`   | `UncheckedF64` | Standard operations |
        // | X | `CheckedF64`    | `f64`          | `UncheckedF64` | For ease of use     |
        // |   | `CheckedF64`    | `UncheckedF64` | `UncheckedF64` | For chaining        |
        // |   | `f64`           | `CheckedF64`   | `UncheckedF64` | For ease of use     |
        // | - | `f64`           | `f64`          | `UncheckedF64` | Not supported       |
        // |   | `f64`           | `UncheckedF64` | `UncheckedF64` | For ease of use     |
        // |   | `UncheckedF64`  | `CheckedF64`   | `UncheckedF64` | For chaining        |
        // |   | `UncheckedF64`  | `f64`          | `UncheckedF64` | For chaining        |
        // |   | `UncheckedF64`  | `UncheckedF64` | `UncheckedF64` | For chaining        |
        binary_operation!($op_trait :: $op_method,
            fn ($lhs: CheckedF64, $rhs: f64) -> UncheckedF64 {
                let CheckedF64($lhs) = $lhs;
                $implementation
            },
            $doc
        );

        // | X | LHS             | RHS            | Result Type    | Note                |
        // |---|-----------------|----------------|----------------|---------------------|
        // |   | `CheckedF64`    | `CheckedF64`   | `UncheckedF64` | Standard operations |
        // |   | `CheckedF64`    | `f64`          | `UncheckedF64` | For ease of use     |
        // | X | `CheckedF64`    | `UncheckedF64` | `UncheckedF64` | For chaining        |
        // |   | `f64`           | `CheckedF64`   | `UncheckedF64` | For ease of use     |
        // | - | `f64`           | `f64`          | `UncheckedF64` | Not supported       |
        // |   | `f64`           | `UncheckedF64` | `UncheckedF64` | For ease of use     |
        // |   | `UncheckedF64`  | `CheckedF64`   | `UncheckedF64` | For chaining        |
        // |   | `UncheckedF64`  | `f64`          | `UncheckedF64` | For chaining        |
        // |   | `UncheckedF64`  | `UncheckedF64` | `UncheckedF64` | For chaining        |
        binary_operation!($op_trait :: $op_method,
            fn ($lhs: CheckedF64, $rhs: UncheckedF64) -> UncheckedF64 {
                let CheckedF64($lhs) = $lhs;
                let UncheckedF64($rhs) = $rhs;
                $implementation
            },
            $doc
        );

        // | X | LHS             | RHS            | Result Type    | Note                |
        // |---|-----------------|----------------|----------------|---------------------|
        // |   | `CheckedF64`    | `CheckedF64`   | `UncheckedF64` | Standard operations |
        // |   | `CheckedF64`    | `f64`          | `UncheckedF64` | For ease of use     |
        // |   | `CheckedF64`    | `UncheckedF64` | `UncheckedF64` | For chaining        |
        // | X | `f64`           | `CheckedF64`   | `UncheckedF64` | For ease of use     |
        // | - | `f64`           | `f64`          | `UncheckedF64` | Not supported       |
        // |   | `f64`           | `UncheckedF64` | `UncheckedF64` | For ease of use     |
        // |   | `UncheckedF64`  | `CheckedF64`   | `UncheckedF64` | For chaining        |
        // |   | `UncheckedF64`  | `f64`          | `UncheckedF64` | For chaining        |
        // |   | `UncheckedF64`  | `UncheckedF64` | `UncheckedF64` | For chaining        |
        binary_operation!($op_trait :: $op_method,
            fn ($lhs: f64, $rhs: CheckedF64) -> UncheckedF64 {
                let CheckedF64($rhs) = $rhs;
                $implementation
            },
            $doc
        );

        // | X | LHS             | RHS            | Result Type    | Note                |
        // |---|-----------------|----------------|----------------|---------------------|
        // |   | `CheckedF64`    | `CheckedF64`   | `UncheckedF64` | Standard operations |
        // |   | `CheckedF64`    | `f64`          | `UncheckedF64` | For ease of use     |
        // |   | `CheckedF64`    | `UncheckedF64` | `UncheckedF64` | For chaining        |
        // |   | `f64`           | `CheckedF64`   | `UncheckedF64` | For ease of use     |
        // | - | `f64`           | `f64`          | `UncheckedF64` | Not supported       |
        // | X | `f64`           | `UncheckedF64` | `UncheckedF64` | For ease of use     |
        // |   | `UncheckedF64`  | `CheckedF64`   | `UncheckedF64` | For chaining        |
        // |   | `UncheckedF64`  | `f64`          | `UncheckedF64` | For chaining        |
        // |   | `UncheckedF64`  | `UncheckedF64` | `UncheckedF64` | For chaining        |
        binary_operation!($op_trait :: $op_method,
            fn ($lhs: f64, $rhs: UncheckedF64) -> UncheckedF64 {
                let UncheckedF64($rhs) = $rhs;
                $implementation
            },
            $doc
        );

        // | X | LHS             | RHS            | Result Type    | Note                |
        // |---|-----------------|----------------|----------------|---------------------|
        // |   | `CheckedF64`    | `CheckedF64`   | `UncheckedF64` | Standard operations |
        // |   | `CheckedF64`    | `f64`          | `UncheckedF64` | For ease of use     |
        // |   | `CheckedF64`    | `UncheckedF64` | `UncheckedF64` | For chaining        |
        // |   | `f64`           | `CheckedF64`   | `UncheckedF64` | For ease of use     |
        // | - | `f64`           | `f64`          | `UncheckedF64` | Not supported       |
        // |   | `f64`           | `UncheckedF64` | `UncheckedF64` | For ease of use     |
        // | X | `UncheckedF64`  | `CheckedF64`   | `UncheckedF64` | For chaining        |
        // |   | `UncheckedF64`  | `f64`          | `UncheckedF64` | For chaining        |
        // |   | `UncheckedF64`  | `UncheckedF64` | `UncheckedF64` | For chaining        |
        binary_operation!($op_trait :: $op_method,
            fn ($lhs: UncheckedF64, $rhs: CheckedF64) -> UncheckedF64 {
                let UncheckedF64($lhs) = $lhs;
                let CheckedF64($rhs) = $rhs;
                $implementation
            },
            $doc
        );

        // | X | LHS             | RHS            | Result Type    | Note                |
        // |---|-----------------|----------------|----------------|---------------------|
        // |   | `CheckedF64`    | `CheckedF64`   | `UncheckedF64` | Standard operations |
        // |   | `CheckedF64`    | `f64`          | `UncheckedF64` | For ease of use     |
        // |   | `CheckedF64`    | `UncheckedF64` | `UncheckedF64` | For chaining        |
        // |   | `f64`           | `CheckedF64`   | `UncheckedF64` | For ease of use     |
        // | - | `f64`           | `f64`          | `UncheckedF64` | Not supported       |
        // |   | `f64`           | `UncheckedF64` | `UncheckedF64` | For ease of use     |
        // |   | `UncheckedF64`  | `CheckedF64`   | `UncheckedF64` | For chaining        |
        // | X | `UncheckedF64`  | `f64`          | `UncheckedF64` | For chaining        |
        // |   | `UncheckedF64`  | `UncheckedF64` | `UncheckedF64` | For chaining        |
        binary_operation!($op_trait :: $op_method,
            fn ($lhs: UncheckedF64, $rhs: f64) -> UncheckedF64 {
                let UncheckedF64($lhs) = $lhs;
                $implementation
            },
            $doc
        );

        // | X | LHS             | RHS            | Result Type    | Note                |
        // |---|-----------------|----------------|----------------|---------------------|
        // |   | `CheckedF64`    | `CheckedF64`   | `UncheckedF64` | Standard operations |
        // |   | `CheckedF64`    | `f64`          | `UncheckedF64` | For ease of use     |
        // |   | `CheckedF64`    | `UncheckedF64` | `UncheckedF64` | For chaining        |
        // |   | `f64`           | `CheckedF64`   | `UncheckedF64` | For ease of use     |
        // | - | `f64`           | `f64`          | `UncheckedF64` | Not supported       |
        // |   | `f64`           | `UncheckedF64` | `UncheckedF64` | For ease of use     |
        // |   | `UncheckedF64`  | `CheckedF64`   | `UncheckedF64` | For chaining        |
        // |   | `UncheckedF64`  | `f64`          | `UncheckedF64` | For chaining        |
        // | X | `UncheckedF64`  | `UncheckedF64` | `UncheckedF64` | For chaining        |
        binary_operation!($op_trait :: $op_method,
            fn ($lhs: UncheckedF64, $rhs: UncheckedF64) -> UncheckedF64 {
                let UncheckedF64($lhs) = $lhs;
                let UncheckedF64($rhs) = $rhs;
                $implementation
            },
            $doc
        );
    };

    (
        $op_trait:ident :: $op_method:ident,
        fn ($lhs:ident : $LHS:ty, $rhs:ident : $RHS:ty) -> UncheckedF64 $implementation:block,
        $doc:literal
    ) => {
        // | X | LHS   | RHS   | Result Type  |
        // |---|-------|-------|--------------|
        // | X | $LHS  | $RHS  | UncheckedF64 |
        // |   | $LHS  | &$RHS | UncheckedF64 |
        // |   | &$LHS | $RHS  | UncheckedF64 |
        // |   | &$LHS | &$RHS | UncheckedF64 |
        impl std::ops::$op_trait<$RHS> for $LHS {
            type Output = UncheckedF64;

            #[doc = $doc]
            #[inline(always)]
            fn $op_method(self, $rhs: $RHS) -> Self::Output {
                let $lhs: $LHS = self;
                $implementation
            }
        }

        // | X | LHS   | RHS   | Result Type  |
        // |---|-------|-------|--------------|
        // |   | $LHS  | $RHS  | UncheckedF64 |
        // | X | $LHS  | &$RHS | UncheckedF64 |
        // |   | &$LHS | $RHS  | UncheckedF64 |
        // |   | &$LHS | &$RHS | UncheckedF64 |
        impl std::ops::$op_trait<&$RHS> for $LHS {
            type Output = UncheckedF64;

            #[doc = $doc]
            #[inline(always)]
            fn $op_method(self, $rhs: &$RHS) -> Self::Output {
                self.$op_method(*$rhs)
            }
        }

        // | X | LHS   | RHS   | Result Type  |
        // |---|-------|-------|--------------|
        // |   | $LHS  | $RHS  | UncheckedF64 |
        // |   | $LHS  | &$RHS | UncheckedF64 |
        // | X | &$LHS | $RHS  | UncheckedF64 |
        // |   | &$LHS | &$RHS | UncheckedF64 |
        impl std::ops::$op_trait<$RHS> for &$LHS {
            type Output = UncheckedF64;

            #[doc = $doc]
            #[inline(always)]
            fn $op_method(self, $rhs: $RHS) -> Self::Output {
                (*self).$op_method($rhs)
            }
        }

        // | X | LHS   | RHS   | Result Type  |
        // |---|-------|-------|--------------|
        // |   | $LHS  | $RHS  | UncheckedF64 |
        // |   | $LHS  | &$RHS | UncheckedF64 |
        // |   | &$LHS | $RHS  | UncheckedF64 |
        // | X | &$LHS | &$RHS | UncheckedF64 |
        impl std::ops::$op_trait<&$RHS> for &$LHS {
            type Output = UncheckedF64;

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

        let value1 = CheckedF64::new(2.0).unwrap();
        let value2 = CheckedF64::new(3.0).unwrap();
        assert_eq!((value1 + value2).check().unwrap(), 5.0);

        assert_eq!((value1 + f64::NAN).check().unwrap_err(), FloatError);
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

        let value1 = CheckedF64::new(5.0).unwrap();
        let value2 = CheckedF64::new(3.0).unwrap();
        assert_eq!(f64::try_from(value1 - value2), Ok(2.0));

        assert_eq!((value1 - f64::NAN).check(), Err(FloatError));
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

        let value1 = CheckedF64::new(2.0).unwrap();
        let value2 = CheckedF64::new(3.0).unwrap();
        assert_eq!(f64::try_from(value1 * value2), Ok(6.0));

        assert_eq!((value1 * f64::NAN).check(), Err(FloatError));
        ```
    "
);

binary_operation!(
    Div::div,
    fn (lhs: f64, rhs: f64) -> UncheckedF64 {
        UncheckedF64::new({
            if rhs.is_finite() {
                lhs / rhs
            } else {
                f64::NAN
            }
        })
    },
    r"
        Divides one `CheckedF64` value by another or a `f64` by a `CheckedF64`.

        # Example

        ```rust
        use checked_float::{CheckedF64, FloatError};

        let value1 = CheckedF64::new(6.0).unwrap();
        let value2 = CheckedF64::new(3.0).unwrap();
        assert_eq!(f64::try_from(value1 / value2), Ok(2.0));

        assert_eq!((value1 / 0.0).check(), Err(FloatError));
        ```
    "
);

binary_operation!(
    Rem::rem,
    fn (lhs: f64, rhs: f64) -> UncheckedF64 {
        UncheckedF64::new({
            if rhs.is_finite() {
                lhs % rhs
            } else {
                f64::NAN
            }
        })
    },
    r"
        Computes the remainder of division between two `CheckedF64` values or a `CheckedF64` and
        a `f64`.

        # Example

        ```rust
        use checked_float::{CheckedF64, FloatError};

        let value1 = CheckedF64::new(5.0).unwrap();
        let value2 = CheckedF64::new(3.0).unwrap();
        assert_eq!(f64::try_from(value1 % value2), Ok(2.0));

        assert_eq!((value1 % 0.0).check(), Err(FloatError));
        ```
    "
);

#[cfg(test)]
mod tests {
    use crate::{CheckedF64, UncheckedF64};
    use proptest::prelude::*;

    proptest! {
        // Addition Operations
        #[test]
        fn test_valid_add_valid_eq_valid(a in any::<f64>(), b in any::<f64>()) {
            let unchecked_a = UncheckedF64::new(a);
            let unchecked_b = UncheckedF64::new(b);

            let expected = CheckedF64::new(a + b);
            if a.is_finite() && b.is_finite() {
                let checked_a = CheckedF64::new(a).unwrap();
                let checked_b = CheckedF64::new(b).unwrap();

                prop_assert_eq!((checked_a + checked_b).check(), expected);
                prop_assert_eq!((checked_a + b).check(), expected);
                prop_assert_eq!((a + checked_b).check(), expected);
                prop_assert_eq!((checked_a + unchecked_b).check(), expected);
                prop_assert_eq!((unchecked_a + checked_b).check(), expected);
            }

            prop_assert_eq!((unchecked_a + unchecked_b).check(), expected);
            prop_assert_eq!((unchecked_a + b).check(), expected);
            prop_assert_eq!((a + unchecked_b).check(), expected);
        }

        // #[test]
        // fn test_valid_add_invalid_eq_invalid(a in valid_f64(), b in invalid_f64()) {
        //     let valid = CheckedF64::new(a);
        //     let invalid = CheckedF64::new(b);
        //
        //     prop_assert_float_error!(valid + invalid);
        //     prop_assert_float_error!(invalid + valid);
        //
        //     prop_assert_float_error!(a + invalid);
        //     prop_assert_float_error!(invalid + a);
        //
        //     prop_assert_float_error!(valid + b);
        //     prop_assert_float_error!(b + valid);
        // }
        //
        // #[test]
        // fn test_invalid_add_invalid_eq_invalid(a in invalid_f64(), b in invalid_f64()) {
        //     prop_assert_float_error!(CheckedF64::new(a) + CheckedF64::new(b));
        //     prop_assert_float_error!(CheckedF64::new(a) + b);
        //     prop_assert_float_error!(a + CheckedF64::new(b));
        // }
        //
        // // Subtraction Operations
        // #[test]
        // fn test_valid_sub_valid_eq_valid(a in valid_f64(), b in valid_f64()) {
        //     if (a - b).is_finite() {
        //         prop_assert_eq!(CheckedF64::new(a) - CheckedF64::new(b), Ok(a - b));
        //         prop_assert_eq!(CheckedF64::new(a) - b, Ok(a - b));
        //         prop_assert_eq!(a - CheckedF64::new(b), Ok(a - b));
        //     }
        // }
        //
        // #[test]
        // fn test_valid_sub_invalid_eq_invalid(a in valid_f64(), b in invalid_f64()) {
        //     prop_assert_float_error!(CheckedF64::new(a) - CheckedF64::new(b));
        //     prop_assert_float_error!(CheckedF64::new(a) - b);
        //     prop_assert_float_error!(a - CheckedF64::new(b));
        // }
        //
        // #[test]
        // fn test_invalid_sub_valid_eq_invalid(a in invalid_f64(), b in valid_f64()) {
        //     prop_assert_float_error!(CheckedF64::new(a) - CheckedF64::new(b));
        //     prop_assert_float_error!(CheckedF64::new(a) - b);
        //     prop_assert_float_error!(a - CheckedF64::new(b));
        // }
        //
        // #[test]
        // fn test_invalid_sub_invalid_eq_invalid(a in invalid_f64(), b in invalid_f64()) {
        //     prop_assert_float_error!(CheckedF64::new(a) - CheckedF64::new(b));
        //     prop_assert_float_error!(CheckedF64::new(a) - b);
        //     prop_assert_float_error!(a - CheckedF64::new(b));
        // }
        //
        // // Multiplication Operations
        // #[test]
        // fn test_valid_mul_valid_eq_valid(a in valid_f64(), b in valid_f64()) {
        //     if (a * b).is_finite() {
        //         prop_assert_eq!(CheckedF64::new(a) * CheckedF64::new(b), Ok(a * b));
        //         prop_assert_eq!(CheckedF64::new(a) * b, Ok(a * b));
        //         prop_assert_eq!(a * CheckedF64::new(b), Ok(a * b));
        //     }
        // }
        //
        // #[test]
        // fn test_valid_mul_invalid_eq_invalid(a in valid_f64(), b in invalid_f64()) {
        //     prop_assert_float_error!(CheckedF64::new(a) * CheckedF64::new(b));
        //     prop_assert_float_error!(CheckedF64::new(a) * b);
        //     prop_assert_float_error!(a * CheckedF64::new(b));
        // }
        //
        // #[test]
        // fn test_invalid_mul_valid_eq_invalid(a in invalid_f64(), b in valid_f64()) {
        //     prop_assert_float_error!(CheckedF64::new(a) * CheckedF64::new(b));
        //     prop_assert_float_error!(CheckedF64::new(a) * b);
        //     prop_assert_float_error!(a * CheckedF64::new(b));
        // }
        //
        // #[test]
        // fn test_invalid_mul_invalid_eq_invalid(a in invalid_f64(), b in invalid_f64()) {
        //     prop_assert_float_error!(CheckedF64::new(a) * CheckedF64::new(b));
        //     prop_assert_float_error!(CheckedF64::new(a) * b);
        //     prop_assert_float_error!(a * CheckedF64::new(b));
        // }
        //
        // // Division Operations
        // #[test]
        // fn test_valid_div_valid_eq_valid(a in valid_f64(), b in valid_f64()) {
        //     if b != 0.0 && (a / b).is_finite() {
        //         prop_assert_eq!(CheckedF64::new(a) / CheckedF64::new(b), Ok(a / b));
        //         prop_assert_eq!(CheckedF64::new(a) / b, Ok(a / b));
        //         prop_assert_eq!(a / CheckedF64::new(b), Ok(a / b));
        //     }
        // }
        //
        // #[test]
        // fn test_valid_div_invalid_eq_invalid(a in valid_f64(), b in invalid_f64()) {
        //     prop_assert_float_error!(CheckedF64::new(a) / CheckedF64::new(b));
        //     prop_assert_float_error!(CheckedF64::new(a) / b);
        //     prop_assert_float_error!(a / CheckedF64::new(b));
        // }
        //
        // #[test]
        // fn test_invalid_div_valid_eq_invalid(a in invalid_f64(), b in valid_f64()) {
        //     prop_assert_float_error!(CheckedF64::new(a) / CheckedF64::new(b));
        //     prop_assert_float_error!(CheckedF64::new(a) / b);
        //     prop_assert_float_error!(a / CheckedF64::new(b));
        // }
        //
        // #[test]
        // fn test_invalid_div_invalid_eq_invalid(a in invalid_f64(), b in invalid_f64()) {
        //     prop_assert_float_error!(CheckedF64::new(a) / CheckedF64::new(b));
        //     prop_assert_float_error!(CheckedF64::new(a) / b);
        //     prop_assert_float_error!(a / CheckedF64::new(b));
        // }
        //
        // // Remainder Operations
        // #[test]
        // fn test_valid_rem_valid_eq_valid(a in valid_f64(), b in valid_f64()) {
        //     if b != 0.0 && (a % b).is_finite() {
        //         prop_assert_eq!(CheckedF64::new(a) % CheckedF64::new(b), Ok(a % b));
        //         prop_assert_eq!(CheckedF64::new(a) % b, Ok(a % b));
        //         prop_assert_eq!(a % CheckedF64::new(b), Ok(a % b));
        //     }
        // }
        //
        // #[test]
        // fn test_valid_rem_invalid_eq_invalid(a in valid_f64(), b in invalid_f64()) {
        //     prop_assert_float_error!(CheckedF64::new(a) % CheckedF64::new(b));
        //     prop_assert_float_error!(CheckedF64::new(a) % b);
        //     prop_assert_float_error!(a % CheckedF64::new(b));
        // }
        //
        // #[test]
        // fn test_invalid_rem_valid_eq_invalid(a in invalid_f64(), b in valid_f64()) {
        //     prop_assert_float_error!(CheckedF64::new(a) % CheckedF64::new(b));
        //     prop_assert_float_error!(CheckedF64::new(a) % b);
        //     prop_assert_float_error!(a % CheckedF64::new(b));
        // }
        //
        // #[test]
        // fn test_invalid_rem_invalid_eq_invalid(a in invalid_f64(), b in invalid_f64()) {
        //     prop_assert_float_error!(CheckedF64::new(a) % CheckedF64::new(b));
        //     prop_assert_float_error!(CheckedF64::new(a) % b);
        //     prop_assert_float_error!(a % CheckedF64::new(b));
        // }
    }
}
