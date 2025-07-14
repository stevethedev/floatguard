use super::{GuardedF32, UnguardedF32};
use crate::binary_operation;
use std::ops::{Add, Div, Mul, Rem, Sub};

binary_operation!(
    impl Add for ...(GuardedF32, UnguardedF32) {
        r"
            Adds two `GuardedF32` values or a `GuardedF32` and a `f32`.

            # Example

            ```rust
            use floatguard::{GuardedF32, FloatError};

            let value1 = GuardedF32::new(2.0).unwrap();
            let value2 = GuardedF32::new(3.0).unwrap();
            assert_eq!((value1 + value2).check().unwrap(), 5.0);

            assert_eq!((value1 + f32::NAN).check(), Err(FloatError::NaN));
            ```
        "
        fn add (lhs: f32, rhs: f32) -> UnguardedF32 {
            UnguardedF32::new(lhs + rhs)
        }
    }
);

binary_operation!(
    impl Sub for ...(GuardedF32, UnguardedF32) {
        r"
            Subtracts one `GuardedF32` value from another or a `f32` from a `GuardedF32`.

            # Example

            ```rust
            use floatguard::{GuardedF32, FloatError};

            let value1 = GuardedF32::new(5.0).unwrap();
            let value2 = GuardedF32::new(3.0).unwrap();
            assert_eq!(f32::try_from(value1 - value2), Ok(2.0));

            assert_eq!((value1 - f32::NAN).check(), Err(FloatError::NaN));
            ```
        "
        fn sub(lhs: f32, rhs: f32) -> UnguardedF32 {
            UnguardedF32::new(lhs - rhs)
        }
    }
);

binary_operation!(
    impl Mul for ...(GuardedF32, UnguardedF32) {
        r"
            Multiplies two `GuardedF32` values or a `GuardedF32` and a `f32`.

            # Example

            ```rust
            use floatguard::{GuardedF32, FloatError};

            let value1 = GuardedF32::new(2.0).unwrap();
            let value2 = GuardedF32::new(3.0).unwrap();
            assert_eq!(f32::try_from(value1 * value2), Ok(6.0));

            assert_eq!((value1 * f32::NAN).check(), Err(FloatError::NaN));
            ```
        "
        fn mul(lhs: f32, rhs: f32) -> UnguardedF32 {
            UnguardedF32::new(lhs * rhs)
        }
    }
);

binary_operation!(
    impl Div for ...(GuardedF32, UnguardedF32) {
        r"
            Divides one `GuardedF32` value by another or a `f32` by a `GuardedF32`.

            # Example

            ```rust
            use floatguard::{GuardedF32, UnguardedF32, FloatError};

            let value1 = GuardedF32::new(6.0).unwrap();
            let value2 = GuardedF32::new(3.0).unwrap();
            assert_eq!(f32::try_from(value1 / value2), Ok(2.0));

            let value1 = UnguardedF32::new(6.0);
            assert_eq!((value1 / 0.0).check(), Err(FloatError::Infinity));

            let value1 = UnguardedF32::new(6.0);
            assert_eq!((value1 / f32::NAN).check(), Err(FloatError::NaN));
            assert_eq!((f32::NAN / value1).check(), Err(FloatError::NaN));

            let value1 = UnguardedF32::new(6.0);
            assert_eq!((f32::INFINITY / value1).check(), Err(FloatError::Infinity));
            assert_eq!((value1 / f32::INFINITY).check(), Err(FloatError::Infinity));

            let value1 = UnguardedF32::new(f32::INFINITY);
            let value2 = UnguardedF32::new(f32::NAN);
            assert_eq!((value1 / value2).check(), Err(FloatError::NaN));
            assert_eq!((value2 / value1).check(), Err(FloatError::NaN));
            ```
        "
        fn div (lhs: f32, rhs: f32) -> UnguardedF32 {
            UnguardedF32::new({
                if lhs.is_finite() && rhs.is_finite() {
                    lhs / rhs
                } else if rhs.is_nan() || lhs.is_nan() {
                    f32::NAN
                } else {
                    f32::INFINITY
                }
            })
        }
    }
);

binary_operation!(
    impl Rem for ...(GuardedF32, UnguardedF32) {
        r"
            Computes the remainder of division between two `GuardedF32` values or a `GuardedF32` and
            a `f32`.

            # Example

            ```rust
            use floatguard::{GuardedF32, UnguardedF32, FloatError};

            let value1 = GuardedF32::new(5.0).unwrap();
            let value2 = GuardedF32::new(3.0).unwrap();
            assert_eq!(f32::try_from(value1 % value2), Ok(2.0));

            assert_eq!((value1 % 0.0).check(), Err(FloatError::NaN));

            let value1 = UnguardedF32::new(6.0);
            assert_eq!((value1 % 0.0).check(), Err(FloatError::NaN));

            let value1 = UnguardedF32::new(6.0);
            assert_eq!((value1 % f32::NAN).check(), Err(FloatError::NaN));
            assert_eq!((f32::NAN % value1).check(), Err(FloatError::NaN));

            let value1 = UnguardedF32::new(6.0);
            assert_eq!((f32::INFINITY % value1).check(), Err(FloatError::Infinity));
            assert_eq!((value1 % f32::INFINITY).check(), Err(FloatError::Infinity));

            let value1 = UnguardedF32::new(f32::INFINITY);
            let value2 = UnguardedF32::new(f32::NAN);
            assert_eq!((value1 % value2).check(), Err(FloatError::NaN));
            assert_eq!((value2 % value1).check(), Err(FloatError::NaN));
            ```
        "
        fn rem(lhs: f32, rhs: f32) -> UnguardedF32 {
            UnguardedF32::new({
                if lhs.is_finite() && rhs.is_finite() {
                    lhs % rhs
                } else if rhs.is_nan() || lhs.is_nan() {
                    f32::NAN
                } else {
                    f32::INFINITY
                }
            })
        }
    }
);

#[cfg(test)]
mod tests {
    #![allow(clippy::op_ref)]

    use crate::{GuardedF32, UnguardedF32};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_addition(a in any::<f32>(), b in any::<f32>()) {
            let unguarded_a = UnguardedF32::new(a);
            let unguarded_b = UnguardedF32::new(b);

            let expected = GuardedF32::new(a + b);
            if a.is_finite() && b.is_finite() {
                let guarded_a = GuardedF32::new(a).unwrap();
                let guarded_b = GuardedF32::new(b).unwrap();

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
        fn test_subtraction(a in any::<f32>(), b in any::<f32>()) {
            let unguarded_a = UnguardedF32::new(a);
            let unguarded_b = UnguardedF32::new(b);

            let expected = GuardedF32::new(a - b);
            if a.is_finite() && b.is_finite() {
                let guarded_a = GuardedF32::new(a).unwrap();
                let guarded_b = GuardedF32::new(b).unwrap();

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
        fn test_multiplication(a in any::<f32>(), b in any::<f32>()) {
            let unguarded_a = UnguardedF32::new(a);
            let unguarded_b = UnguardedF32::new(b);

            let expected = GuardedF32::new(a * b);
            if a.is_finite() && b.is_finite() {
                let guarded_a = GuardedF32::new(a).unwrap();
                let guarded_b = GuardedF32::new(b).unwrap();

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
        fn test_division(a in any::<f32>(), b in any::<f32>()) {
            let unguarded_a = UnguardedF32::new(a);
            let unguarded_b = UnguardedF32::new(b);

            let expected = GuardedF32::new({
                if a.is_finite() && b.is_finite() {
                    a / b
                } else if b.is_nan() || a.is_nan() {
                    f32::NAN
                } else {
                    f32::INFINITY
                }
            });
            if a.is_finite() && b.is_finite() {
                let guarded_a = GuardedF32::new(a).unwrap();
                let guarded_b = GuardedF32::new(b).unwrap();

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
        fn test_remainder(a in any::<f32>(), b in any::<f32>()) {
            let unguarded_a = UnguardedF32::new(a);
            let unguarded_b = UnguardedF32::new(b);

            let expected = GuardedF32::new({
                if b.is_finite() { a % b } else if b.is_nan() { f32::NAN } else { f32::INFINITY }
            });

            if a.is_finite() && b.is_finite() {
                let guarded_a = GuardedF32::new(a).unwrap();
                let guarded_b = GuardedF32::new(b).unwrap();

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
