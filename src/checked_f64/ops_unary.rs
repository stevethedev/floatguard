use crate::{CheckedF64, CheckedF64Result};

macro_rules! unary_operation {
    ($op_trait:ident, $op_method:ident, $doc:literal) => {
        impl std::ops::$op_trait for CheckedF64 {
            type Output = CheckedF64Result;

            #[doc = $doc]
            fn $op_method(self) -> Self::Output {
                Self::new(self.0.$op_method())
            }
        }

        impl std::ops::$op_trait for CheckedF64Result {
            type Output = Self;

            #[doc = $doc]
            fn $op_method(self) -> Self::Output {
                match self.as_inner() {
                    Ok(value) => value.$op_method(),
                    Err(_) => self,
                }
            }
        }
    };
}

unary_operation!(
    Neg,
    neg,
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
        assert!((-invalid_value).is_err());

        let infinity_value = CheckedF64::new(f64::INFINITY);
        assert!((-infinity_value).is_err());
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
            prop_assert!((-checked_a).is_err());
        }
    }
}
