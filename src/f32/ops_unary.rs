use super::{GuardedF32, UnguardedF32};
use crate::unary_operation;
use std::ops::Neg;

unary_operation!(
    impl Neg for ...(GuardedF32, UnguardedF32) {
        r"
            Negates the `GuardedF32` or `UnguardedF32` value.

            # Returns

            Returns a new `Self` instance with the negated value. Unlike other operations, this does not
            default to creating an `UnguardedF32` for `GuardedF32`, as `-x` is always valid for finite
            and non-NaN values.

            # Example

            ```rust
            use floatguard::{GuardedF32, FloatError, UnguardedF32};

            let value = GuardedF32::new(2.0).unwrap();
            assert_eq!(-value, -2.0);

            let value = UnguardedF32::new(2.0);
            assert_eq!(f32::try_from(-value), Ok(-2.0));

            let invalid_value = UnguardedF32::new(f32::NAN);
            assert_eq!((-invalid_value).check(), Err(FloatError::NaN));

            let infinity_value = UnguardedF32::new(f32::INFINITY);
            assert_eq!((-infinity_value).check(), Err(FloatError::Infinity));
            ```
        "
        fn neg(base: f32) -> Self::Output {
            Self(base.neg())
        }
    }
);

#[cfg(test)]
mod tests {
    use crate::{
        FloatError, GuardedF32, UnguardedF32,
        f32::tests::{invalid_f32, valid_f32},
    };
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_negation(a in valid_f32()) {
            let checked_a = GuardedF32::new(a).unwrap();
            let expected = GuardedF32::new(-a).unwrap();

            prop_assert_eq!(-checked_a, expected);
            prop_assert_eq!(-(&checked_a), expected);
            prop_assert_eq!(-checked_a, -a);
            prop_assert_eq!(-(&checked_a), -a);

            let unchecked_a = UnguardedF32::new(a);

            prop_assert_eq!((-unchecked_a).check(), Ok(expected));
            prop_assert_eq!((-(&unchecked_a)).check(), Ok(expected));
        }

        #[test]
        fn test_negation_invalid(a in invalid_f32()) {
            let checked_a = UnguardedF32::new(a);
            let float_error = if a.is_nan() {
                FloatError::NaN
            } else if a.is_infinite() {
                FloatError::Infinity
            } else {
                unreachable!()
            };

            prop_assert_eq!((-checked_a).check(), Err(float_error));
            prop_assert_eq!((-(&checked_a)).check(), Err(float_error));
        }
    }
}
