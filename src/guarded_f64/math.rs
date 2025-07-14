use super::{GuardedF64, UnguardedF64};
use crate::const_math;

macro_rules! math {
    ($name:ident, $doc:expr) => {
        math!($name, $name, $doc);
    };

    ($name:ident, $implementation:ident, $doc:expr) => {
        math!(
            $name,
            fn (base: f64) -> UnguardedF64 {
                UnguardedF64::new(base.$implementation())
            },
            $doc
        );
    };

    (
        $name:ident,
        fn ($base:ident : f64) -> $ret:ty $implementation:block,
        $doc:expr
    ) => {
        impl GuardedF64 {
            #[doc = $doc]
            #[must_use = "method returns a new instance and does not mutate the original value"]
            #[inline(always)]
            pub fn $name(self) -> $ret {
                let $base = self.0;
                $implementation
            }
        }

        impl UnguardedF64 {
            #[doc = $doc]
            #[must_use = "method returns a new instance and does not mutate the original value"]
            #[inline(always)]
            pub fn $name(self) -> $ret {
                let $base = self.0;
                $implementation
            }
        }
    };

    (
        $name:ident,
        $operand:ident : $t:ty,
        $doc:expr
    ) => {
        math!(
            $name,
            fn (base: f64, $operand: $t) -> UnguardedF64 {
                UnguardedF64::new(base.$name($operand))
            },
            $doc
        );
    };

    (
        $name:ident,
        fn ($base:ident : f64, $operand:ident : $t:ty) -> UnguardedF64 $implementation:block,
        $doc:expr
    ) => {
        impl GuardedF64 {
            #[doc = $doc]
            #[must_use = "method returns a new instance and does not mutate the original value"]
            #[inline(always)]
            pub fn $name(self, $operand: $t) -> UnguardedF64 {
                let $base = self.0;
                $implementation
            }
        }

        impl UnguardedF64 {
            #[doc = $doc]
            #[must_use = "method returns a new instance and does not mutate the original value"]
            #[inline(always)]
            pub fn $name(self, $operand: $t) -> UnguardedF64 {
                let $base = self.0;
                $implementation
            }
        }
    };
}

const_math!(
    (GuardedF64, UnguardedF64),
    fn abs(value: f64) -> Self {
        Self(value.abs())
    },
    r"
        Computes the absolute value of self. `GuardedF64::abs` returns a `GuardedF64` type because
        any value that is not NaN or infinite is guaranteed to return a valid value.

        See: [`f64::abs`]

        # Examples

        ```rust
        use floatguard::{GuardedF64, UnguardedF64};

        let checked = GuardedF64::new(3.5_f64).unwrap();
        assert_eq!(checked.abs(), 3.5_f64);

        let unchecked = UnguardedF64::new(-3.5_f64);
        assert_eq!(unchecked.abs().check(), GuardedF64::new(3.5_f64));
        ```
    "
);

const_math!(
    (GuardedF64, UnguardedF64),
    fn signum(value: f64) -> Self {
        Self(value.signum())
    },
    r"
        Returns a number that represents the sign of `self`. `GuardedF64::signum` returns a
        `GuardedF64` type because any value that is not NaN or infinite is guaranteed to return
        a valid value.

        See: [`f64::signum`]

        # Examples

        ```rust
        use floatguard::{GuardedF64, UnguardedF64};

        let pos = GuardedF64::new(3.5_f64).unwrap();
        let neg = UnguardedF64::new(-3.5_f64);

        assert_eq!(pos.signum(), GuardedF64::new(1.0).unwrap());
        assert_eq!(neg.signum().check(), GuardedF64::new(-1.0));
        ```
    "
);

math!(
    sqrt,
    r"
        Returns the square root of `self`.

        See: [`f64::sqrt`]

        # Examples

        ```rust
        use floatguard::{GuardedF64, FloatError, UnguardedF64};

        let positive = GuardedF64::new(4.0_f64).unwrap();
        let negative = GuardedF64::new(-4.0_f64).unwrap();
        let negative_zero = UnguardedF64::new(-0.0_f64);

        assert_eq!(positive.sqrt().check(), GuardedF64::new(2.0));
        assert_eq!(negative.sqrt().check(), Err(FloatError::NaN));
        assert_eq!(negative_zero.sqrt().check(), GuardedF64::new(-0.0));
        ```
    "
);

const_math!(
    (GuardedF64, UnguardedF64),
    fn recip(value: f64) -> UnguardedF64 {
        UnguardedF64(value.recip())
    },
    r"
        Takes the reciprocal (inverse) of `self`, `1/x` where `x` is `self`. This returns an
        `UncheckedF64` because `CheckedF64::new(0.0).unwrap().recip()` is invalid.

        See: [`f64::recip`]

        # Examples

        ```rust
        use floatguard::{GuardedF64, UnguardedF64};

        let x = UnguardedF64::new(2.0_f64);
        let abs_difference = (x.recip() - (1.0 / x)).abs().check().unwrap();

        assert!(abs_difference < GuardedF64::EPSILON);
        ```
    "
);

math!(
    exp,
    r"
        Returns <math>e<sup>(`self`)</sup></math>, (the exponential function).

        See: [`f64::exp`]

        # Examples

        ```rust
        use floatguard::UnguardedF64;

        let one = UnguardedF64::new(1.0_f64);

        // e^1
        let e = one.exp();

        // ln(e) - 1 == 0
        let abs_difference = (e.ln() - 1.0).abs().check().unwrap();

        assert!(abs_difference < 1e-10);
        ```
    "
);

math!(
    ln,
    r"
        Returns the natural logarithm of a number, `ln(self)`.

        See: [`f64::ln`]

        # Examples

        ```rust
        use floatguard::UnguardedF64;

        let e = UnguardedF64::new(2.718281828459045_f64);

        // ln(e) == 1
        let abs_difference = (e.ln() - 1.0).abs().check().unwrap();

        assert!(abs_difference < 1e-10);
        ```
    "
);

math!(
    log2,
    r"
        Returns the base-2 logarithm of a number, `log2(self)`.

        See: [`f64::log2`]

        # Examples

        ```rust
        use floatguard::UnguardedF64;

        let two = UnguardedF64::new(2.0_f64);

        // log2(2) == 1
        let abs_difference = (two.log2() - 1.0).abs().check().unwrap();

        assert!(abs_difference < 1e-10);
        ```
    "
);

math!(
    log10,
    r"
        Returns the base-10 logarithm of a number, `log10(self)`.

        See: [`f64::log10`]

        # Examples

        ```rust
        use floatguard::UnguardedF64;

        let ten = UnguardedF64::new(10.0_f64);

        // log10(10) == 1
        let abs_difference = (ten.log10() - 1.0).abs().check().unwrap();

        assert!(abs_difference < 1e-10);
        ```
    "
);

math!(
    log,
    fn (me: f64, base: impl Into<UnguardedF64>) -> UnguardedF64 {
        let UnguardedF64(base) = base.into();
        UnguardedF64::new(me.log(base))
    },
    r"
        Returns the logarithm of a number with a specified base, `log(self, base)`.

        See: [`f64::log`]

        # Arguments

        `base` - The base of the logarithm.

        # Examples

        ```rust
        use floatguard::UnguardedF64;

        let two = UnguardedF64::new(2.0_f64);
        let eight = UnguardedF64::new(8.0_f64);

        // log(8, 2) == 3
        let abs_difference = (eight.log(two) - 3.0).abs().check().unwrap();

        assert!(abs_difference < 1e-10);
        ```
    "
);

math!(
    powi,
    power: i32,
    r"
        Raises a number to an integer power.

        See: [`f64::powi`]

        # Examples

        ```rust
        use floatguard::{GuardedF64, UnguardedF64};

        let x = UnguardedF64::new(2.0_f64);
        let abs_difference = (x.powi(2) - (x * x)).abs().check().unwrap();
        assert!(abs_difference <= GuardedF64::EPSILON);

        assert!(UnguardedF64::new(f64::NAN).powi(2).check().is_err());
        ```
    "
);

math!(
    powf,
    fn (base: f64, power: impl Into<UnguardedF64>) -> UnguardedF64 {
        let UnguardedF64(power) = power.into();
        UnguardedF64::new(base.powf(power))
    },
    r"
        Raises a number to a floating-point power.

        See: [`f64::powf`]

        # Examples

        ```rust
        use floatguard::{GuardedF64, UnguardedF64};

        let x = UnguardedF64::new(2.0_f64);
        let cubed = UnguardedF64::new(3.0);
        let abs_difference = (x.powf(cubed) - (x * x * x)).abs().check().unwrap();
        assert!(abs_difference <= GuardedF64::EPSILON);

        let invalid = UnguardedF64::new(f64::NAN);
        assert!(invalid.powf(x).check().is_err());
        ```
    "
);

math!(
    sin,
    r"
        Computes the sine of a number (in radians).

        See: [`f64::sin`]

        # Examples

        ```rust
        use floatguard::GuardedF64;

        let x = std::f64::consts::FRAC_PI_2;

        let abs_difference = (x.sin() - 1.0).abs();

        assert!(abs_difference < 1e-10);
        ```
    "
);

math!(
    asin,
    r"
        Computes the arcsine of a number. Return value is in radians in the range [-&pi;/2, &pi;/2] or
        invalid if the number is outside the range [-1, 1].

        See: [`f64::asin`]

        # Examples

        ```rust
        use floatguard::GuardedF64;

        let f = GuardedF64::FRAC_PI_2;

        // asin(sin(pi/2))
        let abs_difference = (f.sin().asin() - GuardedF64::FRAC_PI_2).abs().check().unwrap();

        assert!(abs_difference < 1e-10);
        ```
    "
);

math!(
    sinh,
    r"
        Hyperbolic sine function.

        See: [`f64::sinh`]

        # Examples

        ```rust
        use floatguard::GuardedF64;

        let e = GuardedF64::E;
        let x = 1.0_f64;

        let f = x.sinh();
        // Solving sinh() at 1 gives `(e^2-1)/(2e)`
        let g = ((e * e) - 1.0) / (2.0 * e);
        let abs_difference = (f - g).abs().check().unwrap();

        assert!(abs_difference < 1e-10);
        ```
    "
);

math!(
    asinh,
    r"
        Inverse hyperbolic sine function.

        See: [`f64::asinh`]

        # Examples

        ```rust
        use floatguard::UnguardedF64;

        let x = UnguardedF64::new(1.0_f64);
        let f = x.sinh().asinh();

        let abs_difference = (f - x).abs().check().unwrap();

        assert!(abs_difference < 1.0e-10);
        ```
    "
);

math!(
    cos,
    r"
        Computes the cosine of a number (in radians).

        See: [`f64::cos`]

        # Examples

        ```rust
        use floatguard::GuardedF64;

        let x = 2.0 * GuardedF64::PI;

        let abs_difference = (x.cos() - 1.0).abs().check().unwrap();

        assert!(abs_difference < 1e-10);
        ```
    "
);

math!(
    acos,
    r"
        Computes the arccosine of a number. Return value is in radians in the range [0, &pi;], if the
        value is in the range [-1, 1].

        See: [`f64::acos`]

        # Examples

        ```rust
        use floatguard::GuardedF64;

        let f = GuardedF64::FRAC_PI_4;

        // acos(cos(pi/4))
        let abs_difference = (f.cos().acos() - GuardedF64::FRAC_PI_4).abs().check().unwrap();

        assert!(abs_difference < 1e-10);
        ```
    "
);

math!(
    cosh,
    r"
        Hyperbolic cosine function.

        See: [`f64::cosh`]

        # Examples

        ```rust
        use floatguard::UnguardedF64;

        let e = UnguardedF64::E;
        let x = UnguardedF64::new(1.0_f64);
        let f = x.cosh();

        // Solving cosh() at 1 gives this result
        let g = ((e * e) + 1.0) / (2.0 * e);
        let abs_difference = (f - g).abs().check().unwrap();

        // Same result
        assert!(abs_difference < 1e-10);
        ```
    "
);

math!(
    acosh,
    r"
        Inverse hyperbolic cosine function.

        See: [`f64::acosh`]

        # Examples

        ```rust
        use floatguard::UnguardedF64;

        let x = UnguardedF64::new(1.0);
        let f = x.cosh().acosh();

        let abs_difference = (f - x).abs().check().unwrap();

        assert!(abs_difference < 1.0e-10);
        ```
    "
);

impl GuardedF64 {
    /// Simultaneously computes the sine and cosine of the number, `x`. Returns (sin(x), cos(x)).
    ///
    /// See: [`f64::sin_cos`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// use floatguard::GuardedF64;
    ///
    /// let x = GuardedF64::FRAC_PI_4;
    /// let f = x.sin_cos();
    ///
    /// let abs_difference_0 = (f.0 - x.sin()).abs().check().unwrap();
    /// let abs_difference_1 = (f.1 - x.cos()).abs().check().unwrap();
    ///
    /// assert!(abs_difference_0 < 1e-10);
    /// assert!(abs_difference_1 < 1e-10);
    /// ```
    #[must_use = "method returns a new instance and does not mutate the original value"]
    pub fn sin_cos(self) -> (UnguardedF64, UnguardedF64) {
        let (sin, cos) = self.0.sin_cos();
        (UnguardedF64::new(sin), UnguardedF64::new(cos))
    }
}

impl UnguardedF64 {
    /// Simultaneously computes the sine and cosine of the number, `x`. Returns (sin(x), cos(x)).
    ///
    /// See: [`f64::sin_cos`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// use floatguard::GuardedF64;
    ///
    /// let x = GuardedF64::FRAC_PI_4;
    /// let f = x.sin_cos();
    ///
    /// let abs_difference_0 = (f.0 - x.sin()).abs().check().unwrap();
    /// let abs_difference_1 = (f.1 - x.cos()).abs().check().unwrap();
    ///
    /// assert!(abs_difference_0 < 1e-10);
    /// assert!(abs_difference_1 < 1e-10);
    /// ```
    #[must_use = "method returns a new instance and does not mutate the original value"]
    pub fn sin_cos(self) -> (Self, Self) {
        let (sin, cos) = self.0.sin_cos();
        (Self::new(sin), Self::new(cos))
    }
}

math!(
    tan,
    r"
        Computes the tangent of a number (in radians).

        See: [`f64::tan`]

        # Examples

        ```rust
        use floatguard::GuardedF64;

        let x = GuardedF64::FRAC_PI_4;
        let abs_difference = (x.tan() - 1.0).abs().check().unwrap();

        assert!(abs_difference < 1e-14);
        ```
    "
);

math!(
    atan,
    r"
        Computes the arctangent of a number. Return value is in radians in the range [-&pi;/2, &pi;/2].

        See: [`f64::atan`]

        # Examples

        ```rust
        use floatguard::UnguardedF64;

        let f = UnguardedF64::new(1.0);

        // atan(tan(1))
        let abs_difference = (f.tan().atan() - 1.0).abs().check().unwrap();

        assert!(abs_difference < 1e-10)
        ```
    "
);

math!(
    tanh,
    r"
        Computes the hyperbolic tangent of a number.

        See: [`f64::tanh`]

        # Examples

        ```rust
        use floatguard::GuardedF64;

        let x = GuardedF64::new(1.0_f64).unwrap();
        let f = x.tanh();

        // tanh(1) is approximately 0.7615941559557649
        let abs_difference = (f - 0.7615941559557649).abs().check().unwrap();

        assert!(abs_difference < 1e-10);
        ```
    "
);

math!(
    atanh,
    r"
        Computes the inverse hyperbolic tangent of a number. Return value is in the range (-∞, ∞)
        for inputs in the range (-1, 1).

        See: [`f64::atanh`]

        # Examples

        ```rust
        use floatguard::UnguardedF64;

        let x = UnguardedF64::new(0.5_f64);
        let f = x.tanh().atanh();

        let abs_difference = (f - x).abs().check().unwrap();

        assert!(abs_difference < 1e-10);
        ```
    "
);

math!(
    atan2,
    fn (base: f64, other: impl Into<UnguardedF64>) -> UnguardedF64 {
        let UnguardedF64(other) = other.into();
        UnguardedF64::new(base.atan2(other))
    },
    r"
        Computes the arctangent of `self` divided by `other`.

        See: [`f64::atan2`]

        # Arguments

        `other` - The `GuardedF64` value to divide `self` by.

        # Returns

        Returns a new `GuardedF64` instance containing the result of the arctangent operation.

        # Examples

        ```rust
        use floatguard::UnguardedF64;

        let a = UnguardedF64::new(1.0);
        let b = UnguardedF64::new(2.0);
        assert_eq!(f64::try_from(a.atan2(b)), Ok(0.4636476090008061)); // atan2(1.0, 2.0)

        let invalid = UnguardedF64::new(f64::NAN);
        assert!(invalid.atan2(a).check().is_err());
        ```
    "
);

#[cfg(test)]
mod tests {
    use crate::{GuardedF64, UnguardedF64};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_abs(a in any::<f64>()) {
            let expected = GuardedF64::new(a.abs());

            if a.is_finite() {
                prop_assert_eq!(GuardedF64::new(a).unwrap().abs(), expected.unwrap());
            }
            prop_assert_eq!(UnguardedF64::new(a).abs().check(), expected);
        }

        #[test]
        fn test_signum_valid(a in any::<f64>()) {
            let expected = GuardedF64::new(a.signum());
            if a.is_finite() {
                prop_assert_eq!(GuardedF64::new(a).unwrap().signum(), expected.unwrap());
            }
            prop_assert_eq!(UnguardedF64::new(a).signum().check(), expected);
        }

        #[test]
        fn test_sqrt_valid(a in any::<f64>()) {
            let expected = GuardedF64::new(a.sqrt());
            if a.is_finite() {
                prop_assert_eq!(GuardedF64::new(a).unwrap().sqrt().check(), expected);
            }
            prop_assert_eq!(UnguardedF64::new(a).sqrt().check(), expected);
        }

        #[test]
        fn test_recip_valid(a in any::<f64>()) {
            let expected = GuardedF64::new(a.recip());
            if a.is_finite() {
                prop_assert_eq!(GuardedF64::new(a).unwrap().recip().check(), expected);
            }
            prop_assert_eq!(UnguardedF64::new(a).recip().check(), expected);
        }

        #[test]
        fn test_exp_valid(a in any::<f64>()) {
            let expected = GuardedF64::new(a.exp());
            if a.is_finite() {
                prop_assert_eq!(GuardedF64::new(a).unwrap().exp().check(), expected);
            }
            prop_assert_eq!(UnguardedF64::new(a).exp().check(), expected);
        }
        #[test]
        fn test_ln_valid(a in any::<f64>()) {
            let expected = GuardedF64::new(a.ln());
            if a.is_finite() {
                prop_assert_eq!(GuardedF64::new(a).unwrap().ln().check(), expected);
            }
            prop_assert_eq!(UnguardedF64::new(a).ln().check(), expected);
        }

        #[test]
        fn test_powf_valid(a in any::<f64>(), b in any::<f64>()) {
            let expected = GuardedF64::new(a.powf(b));

            if a.is_finite() {
                prop_assert_eq!(GuardedF64::new(a).unwrap().powf(GuardedF64::new(b).unwrap()).check(), expected);
                prop_assert_eq!(UnguardedF64::new(a).powf(GuardedF64::new(b).unwrap()).check(), expected);
                prop_assert_eq!(GuardedF64::new(a).unwrap().powf(b).check(), expected);
                prop_assert_eq!(UnguardedF64::new(a).powf(b).check(), expected);
            }
            prop_assert_eq!(UnguardedF64::new(a).powf(UnguardedF64::new(b)).check(), expected);
        }

        #[test]
        fn test_powi_valid(a in any::<f64>(), b in -10i32..=10i32) {
            let expected = GuardedF64::new(a.powi(b));
            if a.is_finite() {
                prop_assert_eq!(GuardedF64::new(a).unwrap().powi(b).check(), expected);
            }
            prop_assert_eq!(UnguardedF64::new(a).powi(b).check(), expected);
        }

        #[test]
        fn test_sin_valid(a in any::<f64>()) {
            let expected = GuardedF64::new(a.sin());
            if a.is_finite() {
                prop_assert_eq!(GuardedF64::new(a).unwrap().sin().check(), expected);
            }
            prop_assert_eq!(UnguardedF64::new(a).sin().check(), expected);
        }

        #[test]
        fn test_asin_valid(a in any::<f64>()) {
            let expected = GuardedF64::new(a.asin());
            if a.is_finite() {
                prop_assert_eq!(GuardedF64::new(a).unwrap().asin().check(), expected);
            }
            prop_assert_eq!(UnguardedF64::new(a).asin().check(), expected);
        }

        #[test]
        fn test_sinh_valid(a in any::<f64>()) {
            let expected = GuardedF64::new(a.sinh());
            if a.is_finite() {
                prop_assert_eq!(GuardedF64::new(a).unwrap().sinh().check(), expected);
            }
            prop_assert_eq!(UnguardedF64::new(a).sinh().check(), expected);
        }

        #[test]
        fn test_asinh_valid(a in any::<f64>()) {
            let expected = GuardedF64::new(a.asinh());
            if a.is_finite() {
                prop_assert_eq!(GuardedF64::new(a).unwrap().asinh().check(), expected);
            }
            prop_assert_eq!(UnguardedF64::new(a).asinh().check(), expected);
        }

        #[test]
        fn test_cos_valid(a in any::<f64>()) {
            let expected = GuardedF64::new(a.cos());
            if a.is_finite() {
                prop_assert_eq!(GuardedF64::new(a).unwrap().cos().check(), expected);
            }
            prop_assert_eq!(UnguardedF64::new(a).cos().check(), expected);
        }

        #[test]
        fn test_acos_valid(a in any::<f64>()) {
            let expected = GuardedF64::new(a.acos());
            if a.is_finite() {
                prop_assert_eq!(GuardedF64::new(a).unwrap().acos().check(), expected);
            }
            prop_assert_eq!(UnguardedF64::new(a).acos().check(), expected);
        }

        #[test]
        fn test_cosh_valid(a in any::<f64>()) {
            let expected = GuardedF64::new(a.cosh());
            if a.is_finite() {
                prop_assert_eq!(GuardedF64::new(a).unwrap().cosh().check(), expected);
            }
            prop_assert_eq!(UnguardedF64::new(a).cosh().check(), expected);
        }

        #[test]
        fn test_acosh_valid(a in any::<f64>()) {
            let expected = GuardedF64::new(a.acosh());
            if a.is_finite() {
                prop_assert_eq!(GuardedF64::new(a).unwrap().acosh().check(), expected);
            }
            prop_assert_eq!(UnguardedF64::new(a).acosh().check(), expected);
        }

        #[test]
        fn test_tan_valid(a in any::<f64>()) {
            let expected = GuardedF64::new(a.tan());
            if a.is_finite() {
                prop_assert_eq!(GuardedF64::new(a).unwrap().tan().check(), expected);
            }
            prop_assert_eq!(UnguardedF64::new(a).tan().check(), expected);
        }

        #[test]
        fn test_atan_valid(a in any::<f64>()) {
            let expected = GuardedF64::new(a.atan());
            if a.is_finite() {
                prop_assert_eq!(GuardedF64::new(a).unwrap().atan().check(), expected);
            }
            prop_assert_eq!(UnguardedF64::new(a).atan().check(), expected);
        }

        #[test]
        fn test_tanh_valid(a in any::<f64>()) {
            let expected = GuardedF64::new(a.tanh());
            if a.is_finite() {
                prop_assert_eq!(GuardedF64::new(a).unwrap().tanh().check(), expected);
            }
            prop_assert_eq!(UnguardedF64::new(a).tanh().check(), expected);
        }

        #[test]
        fn test_atanh_valid(a in any::<f64>()) {
            let expected = GuardedF64::new(a.atanh());
            if a.is_finite() {
                prop_assert_eq!(GuardedF64::new(a).unwrap().atanh().check(), expected);
            }
            prop_assert_eq!(UnguardedF64::new(a).atanh().check(), expected);
        }

        #[test]
        fn test_atan2_valid(a in any::<f64>(), b in any::<f64>()) {
            let checked_a = GuardedF64::new(a);
            let checked_b = GuardedF64::new(b);
            let expected = GuardedF64::new(a.atan2(b));

            if a.is_finite() && b.is_finite() {
                prop_assert_eq!(checked_a.unwrap().atan2(checked_b.unwrap()).check(), expected);
                prop_assert_eq!(UnguardedF64::new(a).atan2(checked_b.unwrap()).check(), expected);
                prop_assert_eq!(checked_a.unwrap().atan2(b).check(), expected);
            }
            prop_assert_eq!(UnguardedF64::new(a).atan2(b).check(), expected);
        }

        #[test]
        fn test_sin_cos_valid(a in any::<f64>()) {
            let (sin, cos) = a.sin_cos();
            let expected_sin = GuardedF64::new(sin);
            let expected_cos = GuardedF64::new(cos);
            if a.is_finite() {
                let (sin, cos) = GuardedF64::new(a).unwrap().sin_cos();
                prop_assert_eq!(sin.check(), expected_sin);
                prop_assert_eq!(cos.check(), expected_cos);
            }
            let (sin, cos) = UnguardedF64::new(a).sin_cos();
            prop_assert_eq!(sin.check(), expected_sin);
            prop_assert_eq!(cos.check(), expected_cos);
        }
    }
}
