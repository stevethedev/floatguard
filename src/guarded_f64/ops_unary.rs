use super::{GuardedF64, UnguardedF64};

/// Macro to implement unary operations for `GuardedF64` and `UnguardedF64`.
///
/// This macro generates implementations for unary operations like negation, ensuring that the operation
/// returns a `UnguardedF64`. It handles both `GuardedF64` and `UnguardedF64` types, allowing for
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
            fn (lhs: GuardedF64) -> GuardedF64 {
                let GuardedF64(lhs) = lhs;
                GuardedF64(lhs.$op_method())
            },
            $doc
        );

        unary_operation!(
            $op_trait :: $op_method,
            fn (lhs: UnguardedF64) -> UnguardedF64 {
                let UnguardedF64(lhs) = lhs;
                UnguardedF64(lhs.$op_method())
            },
            $doc
        );
    };

    (
        $op_trait:ident :: $op_method:ident,
        fn ($lhs:ident : $LHS:ty) -> $RET:ty $implementation:block,
        $doc:literal
    ) => {
        impl std::ops::$op_trait for $LHS {
            type Output = $RET;

            #[doc = $doc]
            #[inline(always)]
            fn $op_method(self) -> Self::Output {
                let $lhs: $LHS = self;
                $implementation
            }
        }

        impl std::ops::$op_trait for &$LHS {
            type Output = $RET;

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
        Negates the `GuardedF64` or `UnguardedF64` value.

        # Returns

        Returns a new `Self` instance with the negated value. Unlike other operations, this does not
        default to creating an `UnguardedF64` for `GuardedF64`, as `-x` is always valid for finite
        and non-NaN values.

        # Example

        ```rust
        use floatguard::{GuardedF64, FloatError, UnguardedF64};

        let value = GuardedF64::new(2.0).unwrap();
        assert_eq!(-value, -2.0);

        let value = UnguardedF64::new(2.0);
        assert_eq!(f64::try_from(-value), Ok(-2.0));

        let invalid_value = UnguardedF64::new(f64::NAN);
        assert_eq!((-invalid_value).check(), Err(FloatError));

        let infinity_value = UnguardedF64::new(f64::INFINITY);
        assert_eq!((-infinity_value).check(), Err(FloatError));
        ```
    "
);

#[cfg(test)]
mod tests {
    use crate::{
        FloatError, GuardedF64, UnguardedF64,
        guarded_f64::tests::{invalid_f64, valid_f64},
    };
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_negation(a in valid_f64()) {
            let checked_a = GuardedF64::new(a).unwrap();
            let expected = GuardedF64::new(-a).unwrap();

            prop_assert_eq!(-checked_a, expected);
            prop_assert_eq!(-(&checked_a), expected);
            prop_assert_eq!(-checked_a, -a);
            prop_assert_eq!(-(&checked_a), -a);

            let unchecked_a = UnguardedF64::new(a);

            prop_assert_eq!((-unchecked_a).check(), Ok(expected));
            prop_assert_eq!((-(&unchecked_a)).check(), Ok(expected));
        }

        #[test]
        fn test_negation_invalid(a in invalid_f64()) {
            let checked_a = UnguardedF64::new(a);
            prop_assert_eq!((-checked_a).check(), Err(FloatError));
            prop_assert_eq!((-(&checked_a)).check(), Err(FloatError));
        }
    }
}
