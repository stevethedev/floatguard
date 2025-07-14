use super::{GuardedF64, UnguardedF64};
use crate::macros::ops_binary::binary_operation;
use std::ops::{Add, Div, Mul, Rem, Sub};

binary_operation!(
    impl Add for ...(GuardedF64, UnguardedF64) {
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
        fn add(lhs: f64, rhs: f64) -> UnguardedF64 {
            UnguardedF64::new(lhs + rhs)
        }
    }
);

binary_operation!(
    impl Sub for ...(GuardedF64, UnguardedF64) {
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
        fn sub(lhs: f64, rhs: f64) -> UnguardedF64 {
            UnguardedF64::new(lhs - rhs)
        }
    }
);

binary_operation!(
    impl Mul for ...(GuardedF64, UnguardedF64) {
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
        fn mul(lhs: f64, rhs: f64) -> UnguardedF64 {
            UnguardedF64::new(lhs * rhs)
        }
    }
);

binary_operation!(
    impl Div for ...(GuardedF64, UnguardedF64) {
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
        fn div (lhs: f64, rhs: f64) -> UnguardedF64 {
            UnguardedF64::new({
                if lhs.is_finite() && rhs.is_finite() {
                    lhs / rhs
                } else if rhs.is_nan() || lhs.is_nan() {
                    f64::NAN
                } else {
                    f64::INFINITY
                }
            })
        }
    }
);

binary_operation!(
    impl Rem for ...(GuardedF64, UnguardedF64) {
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
        fn rem(lhs: f64, rhs: f64) -> UnguardedF64 {
            UnguardedF64::new({
                if lhs.is_finite() && rhs.is_finite() {
                    lhs % rhs
                } else if rhs.is_nan() || lhs.is_nan() {
                    f64::NAN
                } else {
                    f64::INFINITY
                }
            })
        }
    }
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
