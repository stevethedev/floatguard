use super::{CheckedF64, UncheckedF64};

/// Macro to implement unary operations for `CheckedF64` and `UncheckedF64`.
///
/// This macro generates implementations for unary operations like negation, ensuring that the operation
/// returns a `UncheckedF64`. It handles both `CheckedF64` and `UncheckedF64` types, allowing for
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
            fn (lhs: CheckedF64) -> CheckedF64 {
                let CheckedF64(lhs) = lhs;
                CheckedF64(lhs.$op_method())
            },
            $doc
        );

        unary_operation!(
            $op_trait :: $op_method,
            fn (lhs: UncheckedF64) -> UncheckedF64 {
                let UncheckedF64(lhs) = lhs;
                UncheckedF64(lhs.$op_method())
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
        Negates the `CheckedF64` or `UncheckedF64` value.

        # Returns

        Returns a new `Self` instance with the negated value. Unlike other operations, this does not
        default to creating an `UncheckedF64` for `CheckedF64`, as `-x` is always valid for finite
        and non-NaN values.

        # Example

        ```rust
        use checked_float::{CheckedF64, FloatError, UncheckedF64};

        let value = CheckedF64::new(2.0).unwrap();
        assert_eq!(-value, -2.0);

        let value = UncheckedF64::new(2.0);
        assert_eq!(f64::try_from(-value), Ok(-2.0));

        let invalid_value = UncheckedF64::new(f64::NAN);
        assert_eq!((-invalid_value).check(), Err(FloatError));

        let infinity_value = UncheckedF64::new(f64::INFINITY);
        assert_eq!((-infinity_value).check(), Err(FloatError));
        ```
    "
);

#[cfg(test)]
mod tests {
    use crate::{CheckedF64, FloatError, checked_f64::tests::{invalid_f64, valid_f64}, UncheckedF64};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_negation(a in valid_f64()) {
            let checked_a = CheckedF64::new(a).unwrap();
            let expected = CheckedF64::new(-a).unwrap();

            prop_assert_eq!(-checked_a, expected);
            prop_assert_eq!(-checked_a, -a);

            let unchecked_a = UncheckedF64::new(a);

            prop_assert_eq!((-unchecked_a).check(), Ok(expected));
        }

        #[test]
        fn test_negation_invalid(a in invalid_f64()) {
            let checked_a = UncheckedF64::new(a);
            prop_assert_eq!((-checked_a).check(), Err(FloatError));
        }
    }
}
