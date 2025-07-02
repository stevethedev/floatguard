use crate::{CheckedF64, CheckedF64Result, FloatError};

macro_rules! unary_operation {
    ($op_trait:ident, $op_method:ident, $doc:literal) => {
        impl std::ops::$op_trait for CheckedF64 {
            type Output = CheckedF64Result;

            #[doc = $doc]
            fn $op_method(self) -> Self::Output {
                let result = self.0.$op_method();
                if result.is_finite() {
                    Ok(Self(result))
                } else {
                    Err(FloatError)
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
        assert_eq!((-value).unwrap().get().unwrap(), -2.0);
        
        let invalid_value = CheckedF64::new(f64::NAN);
        assert_eq!(-invalid_value, Err(FloatError));
        
        let infinity_value = CheckedF64::new(f64::INFINITY);
        assert_eq!(-infinity_value, Err(FloatError));
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

            prop_assert_eq!(-checked_a, Ok(expected));
            prop_assert_eq!((-checked_a)?.get(), Ok(-a));
        }

        #[test]
        fn test_negation_invalid(a in invalid_f64()) {
            let checked_a = CheckedF64::new(a);
            prop_assert_eq!(-checked_a, Err(FloatError));
        }
    }
}
