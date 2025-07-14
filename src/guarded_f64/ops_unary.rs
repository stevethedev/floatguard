use std::ops::Neg;
use super::{GuardedF64, UnguardedF64};
use crate::unary_operation;

unary_operation!(
    impl Neg for (GuardedF64, UnguardedF64) {
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
            assert_eq!((-invalid_value).check(), Err(FloatError::NaN));
    
            let infinity_value = UnguardedF64::new(f64::INFINITY);
            assert_eq!((-infinity_value).check(), Err(FloatError::Infinity));
            ```
        "
        fn neg(base: f64) -> Self::Output {
            Self(base.neg())
        }
    }
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
