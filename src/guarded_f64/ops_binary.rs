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
    fn (lhs: f64, rhs: f64) -> UnguardedF64 {
        UnguardedF64::new(lhs + rhs)
    },
    r"
        Adds two `GuardedF64` values or a `GuardedF64` and a `f64`.

        # Example

        ```rust
        use floatguard::{GuardedF64, FloatError};

        let value1 = GuardedF64::new(2.0).unwrap();
        let value2 = GuardedF64::new(3.0).unwrap();
        assert_eq!((value1 + value2).check().unwrap(), 5.0);

        assert_eq!((value1 + f64::NAN).check(), Err(FloatError::NaN));
        ```
    "
);

binary_operation!(
    Sub::sub,
    fn (lhs: f64, rhs: f64) -> UnguardedF64 {
        UnguardedF64::new(lhs - rhs)
    },
    r"
        Subtracts one `GuardedF64` value from another or a `f64` from a `GuardedF64`.

        # Example

        ```rust
        use floatguard::{GuardedF64, FloatError};

        let value1 = GuardedF64::new(5.0).unwrap();
        let value2 = GuardedF64::new(3.0).unwrap();
        assert_eq!(f64::try_from(value1 - value2), Ok(2.0));

        assert_eq!((value1 - f64::NAN).check(), Err(FloatError::NaN));
        ```
    "
);

binary_operation!(
    Mul::mul,
    fn (lhs: f64, rhs: f64) -> UnguardedF64 {
        UnguardedF64::new(lhs * rhs)
    },
    r"
        Multiplies two `GuardedF64` values or a `GuardedF64` and a `f64`.

        # Example

        ```rust
        use floatguard::{GuardedF64, FloatError};

        let value1 = GuardedF64::new(2.0).unwrap();
        let value2 = GuardedF64::new(3.0).unwrap();
        assert_eq!(f64::try_from(value1 * value2), Ok(6.0));

        assert_eq!((value1 * f64::NAN).check(), Err(FloatError::NaN));
        ```
    "
);

binary_operation!(
    Div::div,
    fn (lhs: f64, rhs: f64) -> UnguardedF64 {
        UnguardedF64::new({
            if lhs.is_finite() && rhs.is_finite() {
                lhs / rhs
            } else if rhs.is_nan() || lhs.is_nan() {
                f64::NAN
            } else {
                f64::INFINITY
            }
        })
    },
    r"
        Divides one `GuardedF64` value by another or a `f64` by a `GuardedF64`.

        # Example

        ```rust
        use floatguard::{GuardedF64, UnguardedF64, FloatError};

        let value1 = GuardedF64::new(6.0).unwrap();
        let value2 = GuardedF64::new(3.0).unwrap();
        assert_eq!(f64::try_from(value1 / value2), Ok(2.0));

        let value1 = UnguardedF64::new(6.0);
        assert_eq!((value1 / 0.0).check(), Err(FloatError::Infinity));

        let value1 = UnguardedF64::new(6.0);
        assert_eq!((value1 / f64::NAN).check(), Err(FloatError::NaN));
        assert_eq!((f64::NAN / value1).check(), Err(FloatError::NaN));

        let value1 = UnguardedF64::new(6.0);
        assert_eq!((f64::INFINITY / value1).check(), Err(FloatError::Infinity));
        assert_eq!((value1 / f64::INFINITY).check(), Err(FloatError::Infinity));

        let value1 = UnguardedF64::new(f64::INFINITY);
        let value2 = UnguardedF64::new(f64::NAN);
        assert_eq!((value1 / value2).check(), Err(FloatError::NaN));
        assert_eq!((value2 / value1).check(), Err(FloatError::NaN));
        ```
    "
);

binary_operation!(
    Rem::rem,
    fn (lhs: f64, rhs: f64) -> UnguardedF64 {
        UnguardedF64::new({
            if lhs.is_finite() && rhs.is_finite() {
                lhs % rhs
            } else if rhs.is_nan() || lhs.is_nan() {
                f64::NAN
            } else {
                f64::INFINITY
            }
        })
    },
    r"
        Computes the remainder of division between two `GuardedF64` values or a `GuardedF64` and
        a `f64`.

        # Example

        ```rust
        use floatguard::{GuardedF64, UnguardedF64, FloatError};

        let value1 = GuardedF64::new(5.0).unwrap();
        let value2 = GuardedF64::new(3.0).unwrap();
        assert_eq!(f64::try_from(value1 % value2), Ok(2.0));

        assert_eq!((value1 % 0.0).check(), Err(FloatError::NaN));

        let value1 = UnguardedF64::new(6.0);
        assert_eq!((value1 % 0.0).check(), Err(FloatError::NaN));

        let value1 = UnguardedF64::new(6.0);
        assert_eq!((value1 % f64::NAN).check(), Err(FloatError::NaN));
        assert_eq!((f64::NAN % value1).check(), Err(FloatError::NaN));

        let value1 = UnguardedF64::new(6.0);
        assert_eq!((f64::INFINITY % value1).check(), Err(FloatError::Infinity));
        assert_eq!((value1 % f64::INFINITY).check(), Err(FloatError::Infinity));

        let value1 = UnguardedF64::new(f64::INFINITY);
        let value2 = UnguardedF64::new(f64::NAN);
        assert_eq!((value1 % value2).check(), Err(FloatError::NaN));
        assert_eq!((value2 % value1).check(), Err(FloatError::NaN));
        ```
    "
);

#[cfg(test)]
mod tests {
    #![allow(clippy::op_ref)]

    use crate::{GuardedF64, UnguardedF64};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_addition(a in any::<f64>(), b in any::<f64>()) {
            let unguarded_a = UnguardedF64::new(a);
            let unguarded_b = UnguardedF64::new(b);

            let expected = GuardedF64::new(a + b);
            if a.is_finite() && b.is_finite() {
                let guarded_a = GuardedF64::new(a).unwrap();
                let guarded_b = GuardedF64::new(b).unwrap();

                prop_assert_eq!((guarded_a + guarded_b).check(), expected);
                prop_assert_eq!((guarded_a + &guarded_b).check(), expected);
                prop_assert_eq!((&guarded_a + guarded_b).check(), expected);
                prop_assert_eq!((&guarded_a + &guarded_b).check(), expected);

                prop_assert_eq!((guarded_a + b).check(), expected);
                prop_assert_eq!((guarded_a + &b).check(), expected);
                prop_assert_eq!((&guarded_a + b).check(), expected);
                prop_assert_eq!((&guarded_a + &b).check(), expected);

                prop_assert_eq!((a + guarded_b).check(), expected);
                prop_assert_eq!((a + &guarded_b).check(), expected);
                prop_assert_eq!((&a + guarded_b).check(), expected);
                prop_assert_eq!((&a + &guarded_b).check(), expected);

                prop_assert_eq!((guarded_a + unguarded_b).check(), expected);
                prop_assert_eq!((guarded_a + &unguarded_b).check(), expected);
                prop_assert_eq!((&guarded_a + unguarded_b).check(), expected);
                prop_assert_eq!((&guarded_a + &unguarded_b).check(), expected);

                prop_assert_eq!((unguarded_a + guarded_b).check(), expected);
                prop_assert_eq!((unguarded_a + &guarded_b).check(), expected);
                prop_assert_eq!((&unguarded_a + guarded_b).check(), expected);
                prop_assert_eq!((&unguarded_a + &guarded_b).check(), expected);
            }

            prop_assert_eq!((unguarded_a + unguarded_b).check(), expected);
            prop_assert_eq!((unguarded_a + &unguarded_b).check(), expected);
            prop_assert_eq!((&unguarded_a + unguarded_b).check(), expected);
            prop_assert_eq!((&unguarded_a + &unguarded_b).check(), expected);

            prop_assert_eq!((unguarded_a + b).check(), expected);
            prop_assert_eq!((unguarded_a + &b).check(), expected);
            prop_assert_eq!((&unguarded_a + b).check(), expected);
            prop_assert_eq!((&unguarded_a + &b).check(), expected);

            prop_assert_eq!((a + unguarded_b).check(), expected);
            prop_assert_eq!((a + &unguarded_b).check(), expected);
            prop_assert_eq!((&a + unguarded_b).check(), expected);
            prop_assert_eq!((&a + &unguarded_b).check(), expected);
        }

        #[test]
        fn test_subtraction(a in any::<f64>(), b in any::<f64>()) {
            let unguarded_a = UnguardedF64::new(a);
            let unguarded_b = UnguardedF64::new(b);

            let expected = GuardedF64::new(a - b);
            if a.is_finite() && b.is_finite() {
                let guarded_a = GuardedF64::new(a).unwrap();
                let guarded_b = GuardedF64::new(b).unwrap();

                prop_assert_eq!((guarded_a - guarded_b).check(), expected);
                prop_assert_eq!((guarded_a - &guarded_b).check(), expected);
                prop_assert_eq!((&guarded_a - guarded_b).check(), expected);
                prop_assert_eq!((&guarded_a - &guarded_b).check(), expected);

                prop_assert_eq!((guarded_a - b).check(), expected);
                prop_assert_eq!((guarded_a - &b).check(), expected);
                prop_assert_eq!((&guarded_a - b).check(), expected);
                prop_assert_eq!((&guarded_a - &b).check(), expected);

                prop_assert_eq!((a - guarded_b).check(), expected);
                prop_assert_eq!((a - &guarded_b).check(), expected);
                prop_assert_eq!((&a - guarded_b).check(), expected);
                prop_assert_eq!((&a - &guarded_b).check(), expected);

                prop_assert_eq!((guarded_a - unguarded_b).check(), expected);
                prop_assert_eq!((guarded_a - &unguarded_b).check(), expected);
                prop_assert_eq!((&guarded_a - unguarded_b).check(), expected);
                prop_assert_eq!((&guarded_a - &unguarded_b).check(), expected);

                prop_assert_eq!((unguarded_a - guarded_b).check(), expected);
                prop_assert_eq!((unguarded_a - &guarded_b).check(), expected);
                prop_assert_eq!((&unguarded_a - guarded_b).check(), expected);
                prop_assert_eq!((&unguarded_a - &guarded_b).check(), expected);
            }
            prop_assert_eq!((unguarded_a - unguarded_b).check(), expected);
            prop_assert_eq!((unguarded_a - &unguarded_b).check(), expected);
            prop_assert_eq!((&unguarded_a - unguarded_b).check(), expected);
            prop_assert_eq!((&unguarded_a - &unguarded_b).check(), expected);

            prop_assert_eq!((unguarded_a - b).check(), expected);
            prop_assert_eq!((unguarded_a - &b).check(), expected);
            prop_assert_eq!((&unguarded_a - b).check(), expected);
            prop_assert_eq!((&unguarded_a - &b).check(), expected);

            prop_assert_eq!((a - unguarded_b).check(), expected);
            prop_assert_eq!((a - &unguarded_b).check(), expected);
            prop_assert_eq!((&a - unguarded_b).check(), expected);
            prop_assert_eq!((&a - &unguarded_b).check(), expected);
        }

        #[test]
        fn test_multiplication(a in any::<f64>(), b in any::<f64>()) {
            let unguarded_a = UnguardedF64::new(a);
            let unguarded_b = UnguardedF64::new(b);

            let expected = GuardedF64::new(a * b);
            if a.is_finite() && b.is_finite() {
                let guarded_a = GuardedF64::new(a).unwrap();
                let guarded_b = GuardedF64::new(b).unwrap();

                prop_assert_eq!((guarded_a * guarded_b).check(), expected);
                prop_assert_eq!((guarded_a * &guarded_b).check(), expected);
                prop_assert_eq!((&guarded_a * guarded_b).check(), expected);
                prop_assert_eq!((&guarded_a * &guarded_b).check(), expected);

                prop_assert_eq!((guarded_a * b).check(), expected);
                prop_assert_eq!((guarded_a * &b).check(), expected);
                prop_assert_eq!((&guarded_a * b).check(), expected);
                prop_assert_eq!((&guarded_a * &b).check(), expected);

                prop_assert_eq!((a * guarded_b).check(), expected);
                prop_assert_eq!((a * &guarded_b).check(), expected);
                prop_assert_eq!((&a * guarded_b).check(), expected);
                prop_assert_eq!((&a * &guarded_b).check(), expected);

                prop_assert_eq!((guarded_a * unguarded_b).check(), expected);
                prop_assert_eq!((guarded_a * &unguarded_b).check(), expected);
                prop_assert_eq!((&guarded_a * unguarded_b).check(), expected);
                prop_assert_eq!((&guarded_a * &unguarded_b).check(), expected);

                prop_assert_eq!((unguarded_a * guarded_b).check(), expected);
                prop_assert_eq!((unguarded_a * &guarded_b).check(), expected);
                prop_assert_eq!((&unguarded_a * guarded_b).check(), expected);
                prop_assert_eq!((&unguarded_a * &guarded_b).check(), expected);
            }
            prop_assert_eq!((unguarded_a * unguarded_b).check(), expected);
            prop_assert_eq!((unguarded_a * &unguarded_b).check(), expected);
            prop_assert_eq!((&unguarded_a * unguarded_b).check(), expected);
            prop_assert_eq!((&unguarded_a * &unguarded_b).check(), expected);

            prop_assert_eq!((unguarded_a * b).check(), expected);
            prop_assert_eq!((unguarded_a * &b).check(), expected);
            prop_assert_eq!((&unguarded_a * b).check(), expected);
            prop_assert_eq!((&unguarded_a * &b).check(), expected);

            prop_assert_eq!((a * unguarded_b).check(), expected);
            prop_assert_eq!((a * &unguarded_b).check(), expected);
            prop_assert_eq!((&a * unguarded_b).check(), expected);
            prop_assert_eq!((&a * &unguarded_b).check(), expected);
        }

        #[test]
        fn test_division(a in any::<f64>(), b in any::<f64>()) {
            let unguarded_a = UnguardedF64::new(a);
            let unguarded_b = UnguardedF64::new(b);

            let expected = GuardedF64::new({
                if a.is_finite() && b.is_finite() {
                    a / b
                } else if b.is_nan() || a.is_nan() {
                    f64::NAN
                } else {
                    f64::INFINITY
                }
            });
            if a.is_finite() && b.is_finite() {
                let guarded_a = GuardedF64::new(a).unwrap();
                let guarded_b = GuardedF64::new(b).unwrap();

                prop_assert_eq!((guarded_a / guarded_b).check(), expected);
                prop_assert_eq!((guarded_a / &guarded_b).check(), expected);
                prop_assert_eq!((&guarded_a / guarded_b).check(), expected);
                prop_assert_eq!((&guarded_a / &guarded_b).check(), expected);

                prop_assert_eq!((guarded_a / b).check(), expected);
                prop_assert_eq!((guarded_a / &b).check(), expected);
                prop_assert_eq!((&guarded_a / b).check(), expected);
                prop_assert_eq!((&guarded_a / &b).check(), expected);

                prop_assert_eq!((a / guarded_b).check(), expected);
                prop_assert_eq!((a / &guarded_b).check(), expected);
                prop_assert_eq!((&a / guarded_b).check(), expected);
                prop_assert_eq!((&a / &guarded_b).check(), expected);

                prop_assert_eq!((guarded_a / unguarded_b).check(), expected);
                prop_assert_eq!((guarded_a / &unguarded_b).check(), expected);
                prop_assert_eq!((&guarded_a / unguarded_b).check(), expected);
                prop_assert_eq!((&guarded_a / &unguarded_b).check(), expected);

                prop_assert_eq!((unguarded_a / guarded_b).check(), expected);
                prop_assert_eq!((unguarded_a / &guarded_b).check(), expected);
                prop_assert_eq!((&unguarded_a / guarded_b).check(), expected);
                prop_assert_eq!((&unguarded_a / &guarded_b).check(), expected);
            }
            prop_assert_eq!((unguarded_a / unguarded_b).check(), expected);
            prop_assert_eq!((unguarded_a / &unguarded_b).check(), expected);
            prop_assert_eq!((&unguarded_a / unguarded_b).check(), expected);
            prop_assert_eq!((&unguarded_a / &unguarded_b).check(), expected);

            prop_assert_eq!((unguarded_a / b).check(), expected);
            prop_assert_eq!((unguarded_a / &b).check(), expected);
            prop_assert_eq!((&unguarded_a / b).check(), expected);
            prop_assert_eq!((&unguarded_a / &b).check(), expected);

            prop_assert_eq!((a / unguarded_b).check(), expected);
            prop_assert_eq!((a / &unguarded_b).check(), expected);
            prop_assert_eq!((&a / unguarded_b).check(), expected);
            prop_assert_eq!((&a / &unguarded_b).check(), expected);
        }

        #[test]
        fn test_remainder(a in any::<f64>(), b in any::<f64>()) {
            let unguarded_a = UnguardedF64::new(a);
            let unguarded_b = UnguardedF64::new(b);

            let expected = GuardedF64::new({
                if b.is_finite() { a % b } else if b.is_nan() { f64::NAN } else { f64::INFINITY }
            });

            if a.is_finite() && b.is_finite() {
                let guarded_a = GuardedF64::new(a).unwrap();
                let guarded_b = GuardedF64::new(b).unwrap();

                prop_assert_eq!((guarded_a % guarded_b).check(), expected);
                prop_assert_eq!((guarded_a % &guarded_b).check(), expected);
                prop_assert_eq!((&guarded_a % guarded_b).check(), expected);
                prop_assert_eq!((&guarded_a % &guarded_b).check(), expected);

                prop_assert_eq!((guarded_a % b).check(), expected);
                prop_assert_eq!((guarded_a % &b).check(), expected);
                prop_assert_eq!((&guarded_a % b).check(), expected);
                prop_assert_eq!((&guarded_a % &b).check(), expected);

                prop_assert_eq!((a % guarded_b).check(), expected);
                prop_assert_eq!((a % &guarded_b).check(), expected);
                prop_assert_eq!((&a % guarded_b).check(), expected);
                prop_assert_eq!((&a % &guarded_b).check(), expected);

                prop_assert_eq!((guarded_a % unguarded_b).check(), expected);
                prop_assert_eq!((guarded_a % &unguarded_b).check(), expected);
                prop_assert_eq!((&guarded_a % unguarded_b).check(), expected);
                prop_assert_eq!((&guarded_a % &unguarded_b).check(), expected);

                prop_assert_eq!((unguarded_a % guarded_b).check(), expected);
                prop_assert_eq!((unguarded_a % &guarded_b).check(), expected);
                prop_assert_eq!((&unguarded_a % guarded_b).check(), expected);
                prop_assert_eq!((&unguarded_a % &guarded_b).check(), expected);
            }
            prop_assert_eq!((unguarded_a % unguarded_b).check(), expected);
            prop_assert_eq!((unguarded_a % &unguarded_b).check(), expected);
            prop_assert_eq!((&unguarded_a % unguarded_b).check(), expected);
            prop_assert_eq!((&unguarded_a % &unguarded_b).check(), expected);

            prop_assert_eq!((unguarded_a % b).check(), expected);
            prop_assert_eq!((unguarded_a % &b).check(), expected);
            prop_assert_eq!((&unguarded_a % b).check(), expected);
            prop_assert_eq!((&unguarded_a % &b).check(), expected);

            prop_assert_eq!((a % unguarded_b).check(), expected);
            prop_assert_eq!((a % &unguarded_b).check(), expected);
            prop_assert_eq!((&a % unguarded_b).check(), expected);
            prop_assert_eq!((&a % &unguarded_b).check(), expected);
        }
    }
}
