use super::{GuardedF32, UnguardedF32};
use crate::math;

math!(
    (GuardedF32, UnguardedF32)
    r"
        Computes the absolute value of self. `GuardedF32::abs` returns a `GuardedF32` type because
        any value that is not NaN or infinite is guaranteed to return a valid value.

        See: [`f32::abs`]

        # Examples

        ```rust
        use floatguard::{GuardedF32, UnguardedF32};

        let checked = GuardedF32::new(3.5_f32).unwrap();
        assert_eq!(checked.abs(), 3.5_f32);

        let unchecked = UnguardedF32::new(-3.5_f32);
        assert_eq!(unchecked.abs().check(), GuardedF32::new(3.5_f32));
        ```
    "
    const fn abs(value: f32) -> Self {
        Self(value.abs())
    }
);

math!(
    (GuardedF32, UnguardedF32)
    r"
        Returns a number that represents the sign of `self`. `GuardedF32::signum` returns a
        `GuardedF32` type because any value that is not NaN or infinite is guaranteed to return
        a valid value.

        See: [`f32::signum`]

        # Examples

        ```rust
        use floatguard::{GuardedF32, UnguardedF32};

        let pos = GuardedF32::new(3.5_f32).unwrap();
        let neg = UnguardedF32::new(-3.5_f32);

        assert_eq!(pos.signum(), GuardedF32::new(1.0).unwrap());
        assert_eq!(neg.signum().check(), GuardedF32::new(-1.0));
        ```
    "
    const fn signum(value: f32) -> Self {
        Self(value.signum())
    }
);

math!(
    (GuardedF32, UnguardedF32)
    r"
        Returns the square root of `self`.

        See: [`f32::sqrt`]

        # Examples

        ```rust
        use floatguard::{GuardedF32, FloatError, UnguardedF32};

        let positive = GuardedF32::new(4.0_f32).unwrap();
        let negative = GuardedF32::new(-4.0_f32).unwrap();
        let negative_zero = UnguardedF32::new(-0.0_f32);

        assert_eq!(positive.sqrt().check(), GuardedF32::new(2.0));
        assert_eq!(negative.sqrt().check(), Err(FloatError::NaN));
        assert_eq!(negative_zero.sqrt().check(), GuardedF32::new(-0.0));
        ```
    "
    fn sqrt(value: f32) -> UnguardedF32 {
        UnguardedF32(value.sqrt())
    }
);

math!(
    (GuardedF32, UnguardedF32)
    r"
        Takes the reciprocal (inverse) of `self`, `1/x` where `x` is `self`. This returns an
        `UncheckedF32` because `CheckedF32::new(0.0).unwrap().recip()` is invalid.

        See: [`f32::recip`]

        # Examples

        ```rust
        use floatguard::{GuardedF32, UnguardedF32};

        let x = UnguardedF32::new(2.0_f32);
        let abs_difference = (x.recip() - (1.0 / x)).abs().check().unwrap();

        assert!(abs_difference < GuardedF32::EPSILON);
        ```
    "
    const fn recip(value: f32) -> UnguardedF32 {
        UnguardedF32(value.recip())
    }
);

math!(
    (GuardedF32, UnguardedF32)
    r"
        Returns <math>e<sup>(`self`)</sup></math>, (the exponential function).

        See: [`f32::exp`]

        # Examples

        ```rust
        use floatguard::UnguardedF32;

        let one = UnguardedF32::new(1.0_f32);

        // e^1
        let e = one.exp();

        // ln(e) - 1 == 0
        let abs_difference = (e.ln() - 1.0).abs().check().unwrap();

        assert!(abs_difference < 1.0e-7);
        ```
    "
    fn exp(value: f32) -> UnguardedF32 {
        UnguardedF32(value.exp())
    }
);

math!(
    (GuardedF32, UnguardedF32)
    r"
        Returns the natural logarithm of a number, `ln(self)`.

        See: [`f32::ln`]

        # Examples

        ```rust
        use floatguard::UnguardedF32;

        let e = UnguardedF32::new(2.718281828459045_f32);

        // ln(e) == 1
        let abs_difference = (e.ln() - 1.0).abs().check().unwrap();

        assert!(abs_difference < 1.0e-7);
        ```
    "
    fn ln(value: f32) -> UnguardedF32 {
        UnguardedF32(value.ln())
    }
);

math!(
    (GuardedF32, UnguardedF32)
    r"
        Returns the base-2 logarithm of a number, `log2(self)`.

        See: [`f32::log2`]

        # Examples

        ```rust
        use floatguard::UnguardedF32;

        let two = UnguardedF32::new(2.0_f32);
        let abs_difference = (two.log2() - 1.0).abs().check().unwrap();
        assert!(abs_difference < 1.0e-7);

        let two = two.check().unwrap();
        let abs_difference = (two.log2() - 1.0).abs().check().unwrap();
        assert!(abs_difference < 1.0e-7);
        ```
    "
    fn log2(value: f32) -> UnguardedF32 {
        UnguardedF32(value.log2())
    }
);

math!(
    (GuardedF32, UnguardedF32)
    r"
        Returns the base-10 logarithm of a number, `log10(self)`.

        See: [`f32::log10`]

        # Examples

        ```rust
        use floatguard::UnguardedF32;

        let ten = UnguardedF32::new(10.0_f32);
        let abs_difference = (ten.log10() - 1.0).abs().check().unwrap();
        assert!(abs_difference < 1.0e-7);

        let ten = ten.check().unwrap();
        let abs_difference = (ten.log10() - 1.0).abs().check().unwrap();
        assert!(abs_difference < 1.0e-7);
        ```
    "
    fn log10(value: f32) -> UnguardedF32 {
        UnguardedF32(value.log10())
    }
);

math!(
    (GuardedF32, UnguardedF32)
    r"
        Returns the logarithm of a number with a specified base, `log(self, base)`.

        See: [`f32::log`]

        # Arguments

        `base` - The base of the logarithm.

        # Examples

        ```rust
        use floatguard::UnguardedF32;

        let two = UnguardedF32::new(2.0_f32);
        let eight = UnguardedF32::new(8.0_f32);

        // log(8, 2) == 3
        let abs_difference = (eight.log(two) - 3.0).abs().check().unwrap();

        assert!(abs_difference < 1.0e-7);
        ```
    "
    fn log(me: f32, base: impl Into<UnguardedF32>) -> UnguardedF32 {
        let UnguardedF32(base) = base.into();
        UnguardedF32::new(me.log(base))
    }
);

math!(
    (GuardedF32, UnguardedF32)
    r"
        Raises a number to an integer power.

        See: [`f32::powi`]

        # Examples

        ```rust
        use floatguard::{GuardedF32, UnguardedF32};

        let x = UnguardedF32::new(2.0_f32);
        let abs_difference = (x.powi(2) - (x * x)).abs().check().unwrap();
        assert!(abs_difference <= GuardedF32::EPSILON);

        assert!(UnguardedF32::new(f32::NAN).powi(2).check().is_err());
        ```
    "
    fn powi(base: f32, power: i32) -> UnguardedF32 {
        UnguardedF32::new(base.powi(power))
    }
);

math!(
    (GuardedF32, UnguardedF32)
    r"
        Raises a number to a floating-point power.

        See: [`f32::powf`]

        # Examples

        ```rust
        use floatguard::{GuardedF32, UnguardedF32};

        let x = UnguardedF32::new(2.0_f32);
        let cubed = UnguardedF32::new(3.0);
        let abs_difference = (x.powf(cubed) - (x * x * x)).abs().check().unwrap();
        assert!(abs_difference <= GuardedF32::EPSILON);

        let invalid = UnguardedF32::new(f32::NAN);
        assert!(invalid.powf(x).check().is_err());
        ```
    "
    fn powf(base: f32, power: impl Into<UnguardedF32>) -> UnguardedF32 {
        let UnguardedF32(power) = power.into();
        UnguardedF32::new(base.powf(power))
    }
);

math!(
    (GuardedF32, UnguardedF32)
    r"
        Computes the sine of a number (in radians).

        See: [`f32::sin`]

        # Examples

        ```rust
        use floatguard::GuardedF32;

        let x = std::f32::consts::FRAC_PI_2;

        let abs_difference = (x.sin() - 1.0).abs();

        assert!(abs_difference < 1.0e-7);
        ```
    "
    fn sin(value: f32) -> UnguardedF32 {
        UnguardedF32(value.sin())
    }
);

math!(
    (GuardedF32, UnguardedF32)
    r"
        Computes the arcsine of a number. Return value is in radians in the range [-&pi;/2, &pi;/2] or
        invalid if the number is outside the range [-1, 1].

        See: [`f32::asin`]

        # Examples

        ```rust
        use floatguard::GuardedF32;

        let f = GuardedF32::FRAC_PI_2;

        // asin(sin(pi/2))
        let abs_difference = (f.sin().asin() - GuardedF32::FRAC_PI_2).abs().check().unwrap();

        assert!(abs_difference < 1.0e-7);
        ```
    "
    fn asin(value: f32) -> UnguardedF32 {
        UnguardedF32(value.asin())
    }
);

math!(
    (GuardedF32, UnguardedF32)
    r"
        Hyperbolic sine function.

        See: [`f32::sinh`]

        # Examples

        ```rust
        use floatguard::GuardedF32;

        let e = GuardedF32::E;
        let x = 1.0_f32;

        let f = x.sinh();
        // Solving sinh() at 1 gives `(e^2-1)/(2e)`
        let g = ((e * e) - 1.0) / (2.0 * e);
        let abs_difference = (f - g).abs().check().unwrap();

        assert!(abs_difference < 1.0e-7);
        ```
    "
    fn sinh(value: f32) -> UnguardedF32 {
        UnguardedF32(value.sinh())
    }
);

math!(
    (GuardedF32, UnguardedF32)
    r"
        Inverse hyperbolic sine function.

        See: [`f32::asinh`]

        # Examples

        ```rust
        use floatguard::UnguardedF32;

        let x = UnguardedF32::new(1.0_f32);
        let f = x.sinh().asinh();

        let abs_difference = (f - x).abs().check().unwrap();

        assert!(abs_difference < 1.0e-7);
        ```
    "
    fn asinh(value: f32) -> UnguardedF32 {
        UnguardedF32(value.asinh())
    }
);

math!(
    (GuardedF32, UnguardedF32)
    r"
        Computes the cosine of a number (in radians).

        See: [`f32::cos`]

        # Examples

        ```rust
        use floatguard::GuardedF32;

        let x = 2.0 * GuardedF32::PI;

        let abs_difference = (x.cos() - 1.0).abs().check().unwrap();

        assert!(abs_difference < 1.0e-7);
        ```
    "
    fn cos(value: f32) -> UnguardedF32 {
        UnguardedF32(value.cos())
    }
);

math!(
    (GuardedF32, UnguardedF32)
    r"
        Computes the arccosine of a number. Return value is in radians in the range [0, &pi;], if the
        value is in the range [-1, 1].

        See: [`f32::acos`]

        # Examples

        ```rust
        use floatguard::GuardedF32;

        let f = GuardedF32::FRAC_PI_4;

        // acos(cos(pi/4))
        let abs_difference = (f.cos().acos() - GuardedF32::FRAC_PI_4).abs().check().unwrap();

        assert!(abs_difference < 1.0e-7);
        ```
    "
    fn acos(value: f32) -> UnguardedF32 {
        UnguardedF32(value.acos())
    }
);

math!(
    (GuardedF32, UnguardedF32)
    r"
        Hyperbolic cosine function.

        See: [`f32::cosh`]

        # Examples

        ```rust
        use floatguard::UnguardedF32;

        let e = UnguardedF32::E;
        let x = UnguardedF32::new(1.0_f32);
        let f = x.cosh();

        // Solving cosh() at 1 gives this result
        let g = ((e * e) + 1.0) / (2.0 * e);
        let abs_difference = (f - g).abs().check().unwrap();

        // Same result
        assert!(abs_difference < 1.0e-6);
        ```
    "
    fn cosh(value: f32) -> UnguardedF32 {
        UnguardedF32(value.cosh())
    }
);

math!(
    (GuardedF32, UnguardedF32)
    r"
        Inverse hyperbolic cosine function.

        See: [`f32::acosh`]

        # Examples

        ```rust
        use floatguard::UnguardedF32;

        let x = UnguardedF32::new(1.0);
        let f = x.cosh().acosh();

        let abs_difference = (f - x).abs().check().unwrap();

        assert!(abs_difference < 1.0e-10);
        ```
    "
    fn acosh(value: f32) -> UnguardedF32 {
        UnguardedF32(value.acosh())
    }
);

math!(
    (GuardedF32, UnguardedF32)
    r"
        Simultaneously computes the sine and cosine of a number, `x`. Returns (sin(x), cos(x)).

        See: [`f32::sin_cos`]

        # Examples

        ```rust
        use floatguard::GuardedF32;

        let x = GuardedF32::FRAC_PI_4;
        let f = x.sin_cos();

        let abs_difference_0 = (f.0 - x.sin()).abs().check().unwrap();
        let abs_difference_1 = (f.1 - x.cos()).abs().check().unwrap();

        assert!(abs_difference_0 < 1.0e-7);
        assert!(abs_difference_1 < 1.0e-7);
        ```
    "
    fn sin_cos(value: f32) -> (UnguardedF32, UnguardedF32) {
        let (sin, cos) = value.sin_cos();
        (UnguardedF32(sin), UnguardedF32(cos))
    }
);

math!(
    (GuardedF32, UnguardedF32)
    r"
        Computes the tangent of a number (in radians).

        See: [`f32::tan`]

        # Examples

        ```rust
        use floatguard::GuardedF32;

        let x = GuardedF32::FRAC_PI_4;
        let abs_difference = (x.tan() - 1.0).abs().check().unwrap();

        assert!(abs_difference < 1e-14);
        ```
    "
    fn tan(value: f32) -> UnguardedF32 {
        UnguardedF32(value.tan())
    }
);

math!(
    (GuardedF32, UnguardedF32)
    r"
        Computes the arctangent of a number. Return value is in radians in the range [-&pi;/2, &pi;/2].

        See: [`f32::atan`]

        # Examples

        ```rust
        use floatguard::UnguardedF32;

        let f = UnguardedF32::new(1.0);

        // atan(tan(1))
        let abs_difference = (f.tan().atan() - 1.0).abs().check().unwrap();

        assert!(abs_difference < 1.0e-7)
        ```
    "
    fn atan(value: f32) -> UnguardedF32 {
        UnguardedF32(value.atan())
    }
);

math!(
    (GuardedF32, UnguardedF32)
    r"
        Computes the hyperbolic tangent of a number.

        See: [`f32::tanh`]

        # Examples

        ```rust
        use floatguard::GuardedF32;

        let x = GuardedF32::new(1.0_f32).unwrap();
        let f = x.tanh();

        // tanh(1) is approximately 0.7615941559557649
        let abs_difference = (f - 0.7615941559557649).abs().check().unwrap();

        assert!(abs_difference < 1.0e-7);
        ```
    "
    fn tanh(value: f32) -> UnguardedF32 {
        UnguardedF32(value.tanh())
    }
);

math!(
    (GuardedF32, UnguardedF32)
    r"
        Computes the inverse hyperbolic tangent of a number. Return value is in the range (-∞, ∞)
        for inputs in the range (-1, 1).

        See: [`f32::atanh`]

        # Examples

        ```rust
        use floatguard::UnguardedF32;

        let x = UnguardedF32::new(0.5_f32);
        let f = x.tanh().atanh();

        let abs_difference = (f - x).abs().check().unwrap();

        assert!(abs_difference < 1.0e-7);
        ```
    "
    fn atanh(value: f32) -> UnguardedF32 {
        UnguardedF32(value.atanh())
    }
);

math!(
    (GuardedF32, UnguardedF32)
    r"
        Computes the arctangent of `self` divided by `other`.

        See: [`f32::atan2`]

        # Arguments

        `other` - The `GuardedF32` value to divide `self` by.

        # Returns

        Returns a new `GuardedF32` instance containing the result of the arctangent operation.

        # Examples

        ```rust
        use floatguard::UnguardedF32;

        let a = UnguardedF32::new(1.0);
        let b = UnguardedF32::new(2.0);
        assert_eq!(f32::try_from(a.atan2(b)), Ok(0.4636476090008061)); // atan2(1.0, 2.0)

        let invalid = UnguardedF32::new(f32::NAN);
        assert!(invalid.atan2(a).check().is_err());
        ```
    "
    fn atan2(base: f32, other: impl Into<UnguardedF32>) -> UnguardedF32 {
        let UnguardedF32(other) = other.into();
        UnguardedF32::new(base.atan2(other))
    }
);

#[cfg(test)]
mod tests {
    use crate::{GuardedF32, UnguardedF32};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_abs(a in any::<f32>()) {
            let expected = GuardedF32::new(a.abs());

            if a.is_finite() {
                prop_assert_eq!(GuardedF32::new(a).unwrap().abs(), expected.unwrap());
            }
            prop_assert_eq!(UnguardedF32::new(a).abs().check(), expected);
        }

        #[test]
        fn test_signum_valid(a in any::<f32>()) {
            let expected = GuardedF32::new(a.signum());
            if a.is_finite() {
                prop_assert_eq!(GuardedF32::new(a).unwrap().signum(), expected.unwrap());
            }
            prop_assert_eq!(UnguardedF32::new(a).signum().check(), expected);
        }

        #[test]
        fn test_sqrt_valid(a in any::<f32>()) {
            let expected = GuardedF32::new(a.sqrt());
            if a.is_finite() {
                prop_assert_eq!(GuardedF32::new(a).unwrap().sqrt().check(), expected);
            }
            prop_assert_eq!(UnguardedF32::new(a).sqrt().check(), expected);
        }

        #[test]
        fn test_recip_valid(a in any::<f32>()) {
            let expected = GuardedF32::new(a.recip());
            if a.is_finite() {
                prop_assert_eq!(GuardedF32::new(a).unwrap().recip().check(), expected);
            }
            prop_assert_eq!(UnguardedF32::new(a).recip().check(), expected);
        }

        #[test]
        fn test_exp_valid(a in any::<f32>()) {
            let expected = GuardedF32::new(a.exp());
            if a.is_finite() {
                prop_assert_eq!(GuardedF32::new(a).unwrap().exp().check(), expected);
            }
            prop_assert_eq!(UnguardedF32::new(a).exp().check(), expected);
        }

        #[test]
        fn test_log2(a in any::<f32>()) {
            let expected = GuardedF32::new(a.log2());
            if a.is_finite() {
                prop_assert_eq!(GuardedF32::new(a).unwrap().log2().check(), expected);
            }
            prop_assert_eq!(UnguardedF32::new(a).log2().check(), expected);
        }

        #[test]
        fn test_log10(a in any::<f32>()) {
            let expected = GuardedF32::new(a.log10());
            if a.is_finite() {
                prop_assert_eq!(GuardedF32::new(a).unwrap().log10().check(), expected);
            }
            prop_assert_eq!(UnguardedF32::new(a).log10().check(), expected);
        }

        #[test]
        fn test_log(a in any::<f32>(), b in any::<f32>()) {
            let expected = GuardedF32::new(a.log(b));
            if a.is_finite() && b.is_finite() {
                prop_assert_eq!(GuardedF32::new(a).unwrap().log(b).check(), expected);
                prop_assert_eq!(GuardedF32::new(a).unwrap().log(GuardedF32::new(b).unwrap()).check(), expected);
                prop_assert_eq!(GuardedF32::new(a).unwrap().log(UnguardedF32::new(b)).check(), expected);
                prop_assert_eq!(UnguardedF32::new(a).log(GuardedF32::new(b).unwrap()).check(), expected);
            }
            prop_assert_eq!(UnguardedF32::new(a).log(b).check(), expected);
            prop_assert_eq!(UnguardedF32::new(a).log(UnguardedF32::new(b)).check(), expected);
        }

        #[test]
        fn test_ln_valid(a in any::<f32>()) {
            let expected = GuardedF32::new(a.ln());
            if a.is_finite() {
                prop_assert_eq!(GuardedF32::new(a).unwrap().ln().check(), expected);
            }
            prop_assert_eq!(UnguardedF32::new(a).ln().check(), expected);
        }

        #[test]
        fn test_powf_valid(a in any::<f32>(), b in any::<f32>()) {
            let expected = GuardedF32::new(a.powf(b));

            if a.is_finite() {
                prop_assert_eq!(GuardedF32::new(a).unwrap().powf(GuardedF32::new(b).unwrap()).check(), expected);
                prop_assert_eq!(UnguardedF32::new(a).powf(GuardedF32::new(b).unwrap()).check(), expected);
                prop_assert_eq!(GuardedF32::new(a).unwrap().powf(b).check(), expected);
                prop_assert_eq!(UnguardedF32::new(a).powf(b).check(), expected);
            }
            prop_assert_eq!(UnguardedF32::new(a).powf(UnguardedF32::new(b)).check(), expected);
        }

        #[test]
        fn test_powi_valid(a in any::<f32>(), b in -10i32..=10i32) {
            let expected = GuardedF32::new(a.powi(b));
            if a.is_finite() {
                prop_assert_eq!(GuardedF32::new(a).unwrap().powi(b).check(), expected);
            }
            prop_assert_eq!(UnguardedF32::new(a).powi(b).check(), expected);
        }

        #[test]
        fn test_sin_valid(a in any::<f32>()) {
            let expected = GuardedF32::new(a.sin());
            if a.is_finite() {
                prop_assert_eq!(GuardedF32::new(a).unwrap().sin().check(), expected);
            }
            prop_assert_eq!(UnguardedF32::new(a).sin().check(), expected);
        }

        #[test]
        fn test_asin_valid(a in any::<f32>()) {
            let expected = GuardedF32::new(a.asin());
            if a.is_finite() {
                prop_assert_eq!(GuardedF32::new(a).unwrap().asin().check(), expected);
            }
            prop_assert_eq!(UnguardedF32::new(a).asin().check(), expected);
        }

        #[test]
        fn test_sinh_valid(a in any::<f32>()) {
            let expected = GuardedF32::new(a.sinh());
            if a.is_finite() {
                prop_assert_eq!(GuardedF32::new(a).unwrap().sinh().check(), expected);
            }
            prop_assert_eq!(UnguardedF32::new(a).sinh().check(), expected);
        }

        #[test]
        fn test_asinh_valid(a in any::<f32>()) {
            let expected = GuardedF32::new(a.asinh());
            if a.is_finite() {
                prop_assert_eq!(GuardedF32::new(a).unwrap().asinh().check(), expected);
            }
            prop_assert_eq!(UnguardedF32::new(a).asinh().check(), expected);
        }

        #[test]
        fn test_cos_valid(a in any::<f32>()) {
            let expected = GuardedF32::new(a.cos());
            if a.is_finite() {
                prop_assert_eq!(GuardedF32::new(a).unwrap().cos().check(), expected);
            }
            prop_assert_eq!(UnguardedF32::new(a).cos().check(), expected);
        }

        #[test]
        fn test_acos_valid(a in any::<f32>()) {
            let expected = GuardedF32::new(a.acos());
            if a.is_finite() {
                prop_assert_eq!(GuardedF32::new(a).unwrap().acos().check(), expected);
            }
            prop_assert_eq!(UnguardedF32::new(a).acos().check(), expected);
        }

        #[test]
        fn test_cosh_valid(a in any::<f32>()) {
            let expected = GuardedF32::new(a.cosh());
            if a.is_finite() {
                prop_assert_eq!(GuardedF32::new(a).unwrap().cosh().check(), expected);
            }
            prop_assert_eq!(UnguardedF32::new(a).cosh().check(), expected);
        }

        #[test]
        fn test_acosh_valid(a in any::<f32>()) {
            let expected = GuardedF32::new(a.acosh());
            if a.is_finite() {
                prop_assert_eq!(GuardedF32::new(a).unwrap().acosh().check(), expected);
            }
            prop_assert_eq!(UnguardedF32::new(a).acosh().check(), expected);
        }

        #[test]
        fn test_tan_valid(a in any::<f32>()) {
            let expected = GuardedF32::new(a.tan());
            if a.is_finite() {
                prop_assert_eq!(GuardedF32::new(a).unwrap().tan().check(), expected);
            }
            prop_assert_eq!(UnguardedF32::new(a).tan().check(), expected);
        }

        #[test]
        fn test_atan_valid(a in any::<f32>()) {
            let expected = GuardedF32::new(a.atan());
            if a.is_finite() {
                prop_assert_eq!(GuardedF32::new(a).unwrap().atan().check(), expected);
            }
            prop_assert_eq!(UnguardedF32::new(a).atan().check(), expected);
        }

        #[test]
        fn test_tanh_valid(a in any::<f32>()) {
            let expected = GuardedF32::new(a.tanh());
            if a.is_finite() {
                prop_assert_eq!(GuardedF32::new(a).unwrap().tanh().check(), expected);
            }
            prop_assert_eq!(UnguardedF32::new(a).tanh().check(), expected);
        }

        #[test]
        fn test_atanh_valid(a in any::<f32>()) {
            let expected = GuardedF32::new(a.atanh());
            if a.is_finite() {
                prop_assert_eq!(GuardedF32::new(a).unwrap().atanh().check(), expected);
            }
            prop_assert_eq!(UnguardedF32::new(a).atanh().check(), expected);
        }

        #[test]
        fn test_atan2_valid(a in any::<f32>(), b in any::<f32>()) {
            let checked_a = GuardedF32::new(a);
            let checked_b = GuardedF32::new(b);
            let expected = GuardedF32::new(a.atan2(b));

            if a.is_finite() && b.is_finite() {
                prop_assert_eq!(checked_a.unwrap().atan2(checked_b.unwrap()).check(), expected);
                prop_assert_eq!(UnguardedF32::new(a).atan2(checked_b.unwrap()).check(), expected);
                prop_assert_eq!(checked_a.unwrap().atan2(b).check(), expected);
            }
            prop_assert_eq!(UnguardedF32::new(a).atan2(b).check(), expected);
        }

        #[test]
        fn test_sin_cos_valid(a in any::<f32>()) {
            let (sin, cos) = a.sin_cos();
            let expected_sin = GuardedF32::new(sin);
            let expected_cos = GuardedF32::new(cos);
            if a.is_finite() {
                let (sin, cos) = GuardedF32::new(a).unwrap().sin_cos();
                prop_assert_eq!(sin.check(), expected_sin);
                prop_assert_eq!(cos.check(), expected_cos);
            }
            let (sin, cos) = UnguardedF32::new(a).sin_cos();
            prop_assert_eq!(sin.check(), expected_sin);
            prop_assert_eq!(cos.check(), expected_cos);
        }
    }
}
