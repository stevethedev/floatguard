use crate::{CheckedF64, CheckedF64Result};

macro_rules! unary_operation {
    ($op_trait:ident :: $op_method:ident, $doc:literal) => {
        unary_operation!(
            $op_trait :: $op_method,
            fn (lhs: CheckedF64) -> f64 { -lhs.0 },
            $doc
        );
        
        unary_operation!(
            $op_trait :: $op_method,
            fn (lhs: CheckedF64Result) -> f64 {
                match *lhs {
                    Ok(lhs) => -lhs.0,
                    err => f64::NAN,
                }
            },
            $doc
        );
    };

    (
        $op_trait:ident :: $op_method:ident,
        fn ($lhs:ident : $LHS:ty) -> f64 $implementation:block,
        $doc:literal
    ) => {
        impl std::ops::$op_trait for $LHS {
            type Output = CheckedF64Result;

            #[doc = $doc]
            fn $op_method(self) -> Self::Output {
                CheckedF64::new({
                    let $lhs: $LHS = self;
                    $implementation
                })
            }
        }

        impl std::ops::$op_trait for &$LHS {
            type Output = CheckedF64Result;

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
        CheckedF64,
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
