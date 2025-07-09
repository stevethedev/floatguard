use super::{GuardedF64, UnguardedF64};

/// Defines a binary operation for `GuardedF64` with the specified trait and method.
///
/// This macro generates implementations for various combinations of `GuardedF64`, `f64`, and `UnguardedF64`
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
            fn (lhs: f64, rhs: f64) -> UnguardedF64 {
                let result = std::ops::$op_trait::<f64>::$op_method(lhs, rhs);
                UnguardedF64::new(result)
            },
            $doc
        );
    };

    (
        $op_trait:ident::$op_method:ident,
        fn ($lhs:ident : f64, $rhs:ident : f64) -> UnguardedF64 $implementation:block,
        $doc:literal
    ) => {
        // | X | LHS             | RHS            | Result Type    | Note                |
        // |---|-----------------|----------------|----------------|---------------------|
        // | X | `GuardedF64`    | `GuardedF64`   | `UnguardedF64` | Standard operations |
        // |   | `GuardedF64`    | `f64`          | `UnguardedF64` | For ease of use     |
        // |   | `GuardedF64`    | `UnguardedF64` | `UnguardedF64` | For chaining        |
        // |   | `f64`           | `GuardedF64`   | `UnguardedF64` | For ease of use     |
        // | - | `f64`           | `f64`          | `UnguardedF64` | Not supported       |
        // |   | `f64`           | `UnguardedF64` | `UnguardedF64` | For ease of use     |
        // |   | `UnguardedF64`  | `GuardedF64`   | `UnguardedF64` | For chaining        |
        // |   | `UnguardedF64`  | `f64`          | `UnguardedF64` | For chaining        |
        // |   | `UnguardedF64`  | `UnguardedF64` | `UnguardedF64` | For chaining        |
        binary_operation!(
            $op_trait :: $op_method,
            fn ($lhs: GuardedF64, $rhs: GuardedF64) -> UnguardedF64 {
                let GuardedF64($lhs) = $lhs;
                let GuardedF64($rhs) = $rhs;
                $implementation
            },
            $doc
        );

        // | X | LHS             | RHS            | Result Type    | Note                |
        // |---|-----------------|----------------|----------------|---------------------|
        // |   | `GuardedF64`    | `GuardedF64`   | `UnguardedF64` | Standard operations |
        // | X | `GuardedF64`    | `f64`          | `UnguardedF64` | For ease of use     |
        // |   | `GuardedF64`    | `UnguardedF64` | `UnguardedF64` | For chaining        |
        // |   | `f64`           | `GuardedF64`   | `UnguardedF64` | For ease of use     |
        // | - | `f64`           | `f64`          | `UnguardedF64` | Not supported       |
        // |   | `f64`           | `UnguardedF64` | `UnguardedF64` | For ease of use     |
        // |   | `UnguardedF64`  | `GuardedF64`   | `UnguardedF64` | For chaining        |
        // |   | `UnguardedF64`  | `f64`          | `UnguardedF64` | For chaining        |
        // |   | `UnguardedF64`  | `UnguardedF64` | `UnguardedF64` | For chaining        |
        binary_operation!($op_trait :: $op_method,
            fn ($lhs: GuardedF64, $rhs: f64) -> UnguardedF64 {
                let GuardedF64($lhs) = $lhs;
                $implementation
            },
            $doc
        );

        // | X | LHS             | RHS            | Result Type    | Note                |
        // |---|-----------------|----------------|----------------|---------------------|
        // |   | `GuardedF64`    | `GuardedF64`   | `UnguardedF64` | Standard operations |
        // |   | `GuardedF64`    | `f64`          | `UnguardedF64` | For ease of use     |
        // | X | `GuardedF64`    | `UnguardedF64` | `UnguardedF64` | For chaining        |
        // |   | `f64`           | `GuardedF64`   | `UnguardedF64` | For ease of use     |
        // | - | `f64`           | `f64`          | `UnguardedF64` | Not supported       |
        // |   | `f64`           | `UnguardedF64` | `UnguardedF64` | For ease of use     |
        // |   | `UnguardedF64`  | `GuardedF64`   | `UnguardedF64` | For chaining        |
        // |   | `UnguardedF64`  | `f64`          | `UnguardedF64` | For chaining        |
        // |   | `UnguardedF64`  | `UnguardedF64` | `UnguardedF64` | For chaining        |
        binary_operation!($op_trait :: $op_method,
            fn ($lhs: GuardedF64, $rhs: UnguardedF64) -> UnguardedF64 {
                let GuardedF64($lhs) = $lhs;
                let UnguardedF64($rhs) = $rhs;
                $implementation
            },
            $doc
        );

        // | X | LHS             | RHS            | Result Type    | Note                |
        // |---|-----------------|----------------|----------------|---------------------|
        // |   | `GuardedF64`    | `GuardedF64`   | `UnguardedF64` | Standard operations |
        // |   | `GuardedF64`    | `f64`          | `UnguardedF64` | For ease of use     |
        // |   | `GuardedF64`    | `UnguardedF64` | `UnguardedF64` | For chaining        |
        // | X | `f64`           | `GuardedF64`   | `UnguardedF64` | For ease of use     |
        // | - | `f64`           | `f64`          | `UnguardedF64` | Not supported       |
        // |   | `f64`           | `UnguardedF64` | `UnguardedF64` | For ease of use     |
        // |   | `UnguardedF64`  | `GuardedF64`   | `UnguardedF64` | For chaining        |
        // |   | `UnguardedF64`  | `f64`          | `UnguardedF64` | For chaining        |
        // |   | `UnguardedF64`  | `UnguardedF64` | `UnguardedF64` | For chaining        |
        binary_operation!($op_trait :: $op_method,
            fn ($lhs: f64, $rhs: GuardedF64) -> UnguardedF64 {
                let GuardedF64($rhs) = $rhs;
                $implementation
            },
            $doc
        );

        // | X | LHS             | RHS            | Result Type    | Note                |
        // |---|-----------------|----------------|----------------|---------------------|
        // |   | `GuardedF64`    | `GuardedF64`   | `UnguardedF64` | Standard operations |
        // |   | `GuardedF64`    | `f64`          | `UnguardedF64` | For ease of use     |
        // |   | `GuardedF64`    | `UnguardedF64` | `UnguardedF64` | For chaining        |
        // |   | `f64`           | `GuardedF64`   | `UnguardedF64` | For ease of use     |
        // | - | `f64`           | `f64`          | `UnguardedF64` | Not supported       |
        // | X | `f64`           | `UnguardedF64` | `UnguardedF64` | For ease of use     |
        // |   | `UnguardedF64`  | `GuardedF64`   | `UnguardedF64` | For chaining        |
        // |   | `UnguardedF64`  | `f64`          | `UnguardedF64` | For chaining        |
        // |   | `UnguardedF64`  | `UnguardedF64` | `UnguardedF64` | For chaining        |
        binary_operation!($op_trait :: $op_method,
            fn ($lhs: f64, $rhs: UnguardedF64) -> UnguardedF64 {
                let UnguardedF64($rhs) = $rhs;
                $implementation
            },
            $doc
        );

        // | X | LHS             | RHS            | Result Type    | Note                |
        // |---|-----------------|----------------|----------------|---------------------|
        // |   | `GuardedF64`    | `GuardedF64`   | `UnguardedF64` | Standard operations |
        // |   | `GuardedF64`    | `f64`          | `UnguardedF64` | For ease of use     |
        // |   | `GuardedF64`    | `UnguardedF64` | `UnguardedF64` | For chaining        |
        // |   | `f64`           | `GuardedF64`   | `UnguardedF64` | For ease of use     |
        // | - | `f64`           | `f64`          | `UnguardedF64` | Not supported       |
        // |   | `f64`           | `UnguardedF64` | `UnguardedF64` | For ease of use     |
        // | X | `UnguardedF64`  | `GuardedF64`   | `UnguardedF64` | For chaining        |
        // |   | `UnguardedF64`  | `f64`          | `UnguardedF64` | For chaining        |
        // |   | `UnguardedF64`  | `UnguardedF64` | `UnguardedF64` | For chaining        |
        binary_operation!($op_trait :: $op_method,
            fn ($lhs: UnguardedF64, $rhs: GuardedF64) -> UnguardedF64 {
                let UnguardedF64($lhs) = $lhs;
                let GuardedF64($rhs) = $rhs;
                $implementation
            },
            $doc
        );

        // | X | LHS             | RHS            | Result Type    | Note                |
        // |---|-----------------|----------------|----------------|---------------------|
        // |   | `GuardedF64`    | `GuardedF64`   | `UnguardedF64` | Standard operations |
        // |   | `GuardedF64`    | `f64`          | `UnguardedF64` | For ease of use     |
        // |   | `GuardedF64`    | `UnguardedF64` | `UnguardedF64` | For chaining        |
        // |   | `f64`           | `GuardedF64`   | `UnguardedF64` | For ease of use     |
        // | - | `f64`           | `f64`          | `UnguardedF64` | Not supported       |
        // |   | `f64`           | `UnguardedF64` | `UnguardedF64` | For ease of use     |
        // |   | `UnguardedF64`  | `GuardedF64`   | `UnguardedF64` | For chaining        |
        // | X | `UnguardedF64`  | `f64`          | `UnguardedF64` | For chaining        |
        // |   | `UnguardedF64`  | `UnguardedF64` | `UnguardedF64` | For chaining        |
        binary_operation!($op_trait :: $op_method,
            fn ($lhs: UnguardedF64, $rhs: f64) -> UnguardedF64 {
                let UnguardedF64($lhs) = $lhs;
                $implementation
            },
            $doc
        );

        // | X | LHS             | RHS            | Result Type    | Note                |
        // |---|-----------------|----------------|----------------|---------------------|
        // |   | `GuardedF64`    | `GuardedF64`   | `UnguardedF64` | Standard operations |
        // |   | `GuardedF64`    | `f64`          | `UnguardedF64` | For ease of use     |
        // |   | `GuardedF64`    | `UnguardedF64` | `UnguardedF64` | For chaining        |
        // |   | `f64`           | `GuardedF64`   | `UnguardedF64` | For ease of use     |
        // | - | `f64`           | `f64`          | `UnguardedF64` | Not supported       |
        // |   | `f64`           | `UnguardedF64` | `UnguardedF64` | For ease of use     |
        // |   | `UnguardedF64`  | `GuardedF64`   | `UnguardedF64` | For chaining        |
        // |   | `UnguardedF64`  | `f64`          | `UnguardedF64` | For chaining        |
        // | X | `UnguardedF64`  | `UnguardedF64` | `UnguardedF64` | For chaining        |
        binary_operation!($op_trait :: $op_method,
            fn ($lhs: UnguardedF64, $rhs: UnguardedF64) -> UnguardedF64 {
                let UnguardedF64($lhs) = $lhs;
                let UnguardedF64($rhs) = $rhs;
                $implementation
            },
            $doc
        );
    };

    (
        $op_trait:ident :: $op_method:ident,
        fn ($lhs:ident : $LHS:ty, $rhs:ident : $RHS:ty) -> UnguardedF64 $implementation:block,
        $doc:literal
    ) => {
        // | X | LHS   | RHS   | Result Type  |
        // |---|-------|-------|--------------|
        // | X | $LHS  | $RHS  | UnguardedF64 |
        // |   | $LHS  | &$RHS | UnguardedF64 |
        // |   | &$LHS | $RHS  | UnguardedF64 |
        // |   | &$LHS | &$RHS | UnguardedF64 |
        impl std::ops::$op_trait<$RHS> for $LHS {
            type Output = UnguardedF64;

            #[doc = $doc]
            #[inline(always)]
            fn $op_method(self, $rhs: $RHS) -> Self::Output {
                let $lhs: $LHS = self;
                $implementation
            }
        }

        // | X | LHS   | RHS   | Result Type  |
        // |---|-------|-------|--------------|
        // |   | $LHS  | $RHS  | UnguardedF64 |
        // | X | $LHS  | &$RHS | UnguardedF64 |
        // |   | &$LHS | $RHS  | UnguardedF64 |
        // |   | &$LHS | &$RHS | UnguardedF64 |
        impl std::ops::$op_trait<&$RHS> for $LHS {
            type Output = UnguardedF64;

            #[doc = $doc]
            #[inline(always)]
            fn $op_method(self, $rhs: &$RHS) -> Self::Output {
                self.$op_method(*$rhs)
            }
        }

        // | X | LHS   | RHS   | Result Type  |
        // |---|-------|-------|--------------|
        // |   | $LHS  | $RHS  | UnguardedF64 |
        // |   | $LHS  | &$RHS | UnguardedF64 |
        // | X | &$LHS | $RHS  | UnguardedF64 |
        // |   | &$LHS | &$RHS | UnguardedF64 |
        impl std::ops::$op_trait<$RHS> for &$LHS {
            type Output = UnguardedF64;

            #[doc = $doc]
            #[inline(always)]
            fn $op_method(self, $rhs: $RHS) -> Self::Output {
                (*self).$op_method($rhs)
            }
        }

        // | X | LHS   | RHS   | Result Type  |
        // |---|-------|-------|--------------|
        // |   | $LHS  | $RHS  | UnguardedF64 |
        // |   | $LHS  | &$RHS | UnguardedF64 |
        // |   | &$LHS | $RHS  | UnguardedF64 |
        // | X | &$LHS | &$RHS | UnguardedF64 |
        impl std::ops::$op_trait<&$RHS> for &$LHS {
            type Output = UnguardedF64;

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
        Adds two `GuardedF64` values or a `GuardedF64` and a `f64`.

        # Example

        ```rust
        use floatguard::{GuardedF64, FloatError};

        let value1 = GuardedF64::new(2.0).unwrap();
        let value2 = GuardedF64::new(3.0).unwrap();
        assert_eq!((value1 + value2).check().unwrap(), 5.0);

        assert_eq!((value1 + f64::NAN).check().unwrap_err(), FloatError);
        ```
    "
);

binary_operation!(
    Sub::sub,
    r"
        Subtracts one `GuardedF64` value from another or a `f64` from a `GuardedF64`.

        # Example

        ```rust
        use floatguard::{GuardedF64, FloatError};

        let value1 = GuardedF64::new(5.0).unwrap();
        let value2 = GuardedF64::new(3.0).unwrap();
        assert_eq!(f64::try_from(value1 - value2), Ok(2.0));

        assert_eq!((value1 - f64::NAN).check(), Err(FloatError));
        ```
    "
);

binary_operation!(
    Mul::mul,
    r"
        Multiplies two `GuardedF64` values or a `GuardedF64` and a `f64`.

        # Example

        ```rust
        use floatguard::{GuardedF64, FloatError};

        let value1 = GuardedF64::new(2.0).unwrap();
        let value2 = GuardedF64::new(3.0).unwrap();
        assert_eq!(f64::try_from(value1 * value2), Ok(6.0));

        assert_eq!((value1 * f64::NAN).check(), Err(FloatError));
        ```
    "
);

binary_operation!(
    Div::div,
    fn (lhs: f64, rhs: f64) -> UnguardedF64 {
        UnguardedF64::new({
            if rhs.is_finite() {
                lhs / rhs
            } else {
                f64::NAN
            }
        })
    },
    r"
        Divides one `GuardedF64` value by another or a `f64` by a `GuardedF64`.

        # Example

        ```rust
        use floatguard::{GuardedF64, FloatError};

        let value1 = GuardedF64::new(6.0).unwrap();
        let value2 = GuardedF64::new(3.0).unwrap();
        assert_eq!(f64::try_from(value1 / value2), Ok(2.0));

        assert_eq!((value1 / 0.0).check(), Err(FloatError));
        ```
    "
);

binary_operation!(
    Rem::rem,
    fn (lhs: f64, rhs: f64) -> UnguardedF64 {
        UnguardedF64::new({
            if rhs.is_finite() {
                lhs % rhs
            } else {
                f64::NAN
            }
        })
    },
    r"
        Computes the remainder of division between two `GuardedF64` values or a `GuardedF64` and
        a `f64`.

        # Example

        ```rust
        use floatguard::{GuardedF64, FloatError};

        let value1 = GuardedF64::new(5.0).unwrap();
        let value2 = GuardedF64::new(3.0).unwrap();
        assert_eq!(f64::try_from(value1 % value2), Ok(2.0));

        assert_eq!((value1 % 0.0).check(), Err(FloatError));
        ```
    "
);

#[cfg(test)]
mod tests {
    use crate::{GuardedF64, UnguardedF64};
    use proptest::prelude::*;

    proptest! {
        // Addition Operations
        #[test]
        fn test_valid_add_valid_eq_valid(a in any::<f64>(), b in any::<f64>()) {
            let unchecked_a = UnguardedF64::new(a);
            let unchecked_b = UnguardedF64::new(b);

            let expected = GuardedF64::new(a + b);
            if a.is_finite() && b.is_finite() {
                let checked_a = GuardedF64::new(a).unwrap();
                let checked_b = GuardedF64::new(b).unwrap();

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
        //     let valid = GuardedF64::new(a);
        //     let invalid = GuardedF64::new(b);
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
        //     prop_assert_float_error!(GuardedF64::new(a) + GuardedF64::new(b));
        //     prop_assert_float_error!(GuardedF64::new(a) + b);
        //     prop_assert_float_error!(a + GuardedF64::new(b));
        // }
        //
        // // Subtraction Operations
        // #[test]
        // fn test_valid_sub_valid_eq_valid(a in valid_f64(), b in valid_f64()) {
        //     if (a - b).is_finite() {
        //         prop_assert_eq!(GuardedF64::new(a) - GuardedF64::new(b), Ok(a - b));
        //         prop_assert_eq!(GuardedF64::new(a) - b, Ok(a - b));
        //         prop_assert_eq!(a - GuardedF64::new(b), Ok(a - b));
        //     }
        // }
        //
        // #[test]
        // fn test_valid_sub_invalid_eq_invalid(a in valid_f64(), b in invalid_f64()) {
        //     prop_assert_float_error!(GuardedF64::new(a) - GuardedF64::new(b));
        //     prop_assert_float_error!(GuardedF64::new(a) - b);
        //     prop_assert_float_error!(a - GuardedF64::new(b));
        // }
        //
        // #[test]
        // fn test_invalid_sub_valid_eq_invalid(a in invalid_f64(), b in valid_f64()) {
        //     prop_assert_float_error!(GuardedF64::new(a) - GuardedF64::new(b));
        //     prop_assert_float_error!(GuardedF64::new(a) - b);
        //     prop_assert_float_error!(a - GuardedF64::new(b));
        // }
        //
        // #[test]
        // fn test_invalid_sub_invalid_eq_invalid(a in invalid_f64(), b in invalid_f64()) {
        //     prop_assert_float_error!(GuardedF64::new(a) - GuardedF64::new(b));
        //     prop_assert_float_error!(GuardedF64::new(a) - b);
        //     prop_assert_float_error!(a - GuardedF64::new(b));
        // }
        //
        // // Multiplication Operations
        // #[test]
        // fn test_valid_mul_valid_eq_valid(a in valid_f64(), b in valid_f64()) {
        //     if (a * b).is_finite() {
        //         prop_assert_eq!(GuardedF64::new(a) * GuardedF64::new(b), Ok(a * b));
        //         prop_assert_eq!(GuardedF64::new(a) * b, Ok(a * b));
        //         prop_assert_eq!(a * GuardedF64::new(b), Ok(a * b));
        //     }
        // }
        //
        // #[test]
        // fn test_valid_mul_invalid_eq_invalid(a in valid_f64(), b in invalid_f64()) {
        //     prop_assert_float_error!(GuardedF64::new(a) * GuardedF64::new(b));
        //     prop_assert_float_error!(GuardedF64::new(a) * b);
        //     prop_assert_float_error!(a * GuardedF64::new(b));
        // }
        //
        // #[test]
        // fn test_invalid_mul_valid_eq_invalid(a in invalid_f64(), b in valid_f64()) {
        //     prop_assert_float_error!(GuardedF64::new(a) * GuardedF64::new(b));
        //     prop_assert_float_error!(GuardedF64::new(a) * b);
        //     prop_assert_float_error!(a * GuardedF64::new(b));
        // }
        //
        // #[test]
        // fn test_invalid_mul_invalid_eq_invalid(a in invalid_f64(), b in invalid_f64()) {
        //     prop_assert_float_error!(GuardedF64::new(a) * GuardedF64::new(b));
        //     prop_assert_float_error!(GuardedF64::new(a) * b);
        //     prop_assert_float_error!(a * GuardedF64::new(b));
        // }
        //
        // // Division Operations
        // #[test]
        // fn test_valid_div_valid_eq_valid(a in valid_f64(), b in valid_f64()) {
        //     if b != 0.0 && (a / b).is_finite() {
        //         prop_assert_eq!(GuardedF64::new(a) / GuardedF64::new(b), Ok(a / b));
        //         prop_assert_eq!(GuardedF64::new(a) / b, Ok(a / b));
        //         prop_assert_eq!(a / GuardedF64::new(b), Ok(a / b));
        //     }
        // }
        //
        // #[test]
        // fn test_valid_div_invalid_eq_invalid(a in valid_f64(), b in invalid_f64()) {
        //     prop_assert_float_error!(GuardedF64::new(a) / GuardedF64::new(b));
        //     prop_assert_float_error!(GuardedF64::new(a) / b);
        //     prop_assert_float_error!(a / GuardedF64::new(b));
        // }
        //
        // #[test]
        // fn test_invalid_div_valid_eq_invalid(a in invalid_f64(), b in valid_f64()) {
        //     prop_assert_float_error!(GuardedF64::new(a) / GuardedF64::new(b));
        //     prop_assert_float_error!(GuardedF64::new(a) / b);
        //     prop_assert_float_error!(a / GuardedF64::new(b));
        // }
        //
        // #[test]
        // fn test_invalid_div_invalid_eq_invalid(a in invalid_f64(), b in invalid_f64()) {
        //     prop_assert_float_error!(GuardedF64::new(a) / GuardedF64::new(b));
        //     prop_assert_float_error!(GuardedF64::new(a) / b);
        //     prop_assert_float_error!(a / GuardedF64::new(b));
        // }
        //
        // // Remainder Operations
        // #[test]
        // fn test_valid_rem_valid_eq_valid(a in valid_f64(), b in valid_f64()) {
        //     if b != 0.0 && (a % b).is_finite() {
        //         prop_assert_eq!(GuardedF64::new(a) % GuardedF64::new(b), Ok(a % b));
        //         prop_assert_eq!(GuardedF64::new(a) % b, Ok(a % b));
        //         prop_assert_eq!(a % GuardedF64::new(b), Ok(a % b));
        //     }
        // }
        //
        // #[test]
        // fn test_valid_rem_invalid_eq_invalid(a in valid_f64(), b in invalid_f64()) {
        //     prop_assert_float_error!(GuardedF64::new(a) % GuardedF64::new(b));
        //     prop_assert_float_error!(GuardedF64::new(a) % b);
        //     prop_assert_float_error!(a % GuardedF64::new(b));
        // }
        //
        // #[test]
        // fn test_invalid_rem_valid_eq_invalid(a in invalid_f64(), b in valid_f64()) {
        //     prop_assert_float_error!(GuardedF64::new(a) % GuardedF64::new(b));
        //     prop_assert_float_error!(GuardedF64::new(a) % b);
        //     prop_assert_float_error!(a % GuardedF64::new(b));
        // }
        //
        // #[test]
        // fn test_invalid_rem_invalid_eq_invalid(a in invalid_f64(), b in invalid_f64()) {
        //     prop_assert_float_error!(GuardedF64::new(a) % GuardedF64::new(b));
        //     prop_assert_float_error!(GuardedF64::new(a) % b);
        //     prop_assert_float_error!(a % GuardedF64::new(b));
        // }
    }
}
