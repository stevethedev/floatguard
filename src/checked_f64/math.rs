use crate::{CheckedF64, CheckedF64Result};

macro_rules! const_math {
    (
        $name:ident,
        $doc:expr
    ) => {
        const_math!(
            $name,
            fn (base: f64) -> CheckedF64Result {
                CheckedF64::new(base.$name())
            },
            $doc
        );
    };

    (
        $name:ident,
        fn ($base:ident : f64) -> $ret:ty $implementation:block,
        $doc:expr
    ) => {
        impl CheckedF64 {
            #[doc = $doc]
            #[must_use = "method returns a new instance and does not mutate the original value"]
            #[inline(always)]
            pub const fn $name(self) -> $ret {
                let $base = self.0;
                $implementation
            }
        }

        impl CheckedF64Result {
            #[doc = $doc]
            #[must_use = "method returns a new instance and does not mutate the original value"]
            #[inline(always)]
            pub const fn $name(self) -> $ret {
                let Ok($base) = self.as_inner() else { return self; };
                $base.$name()
            }
        }
    };
}

macro_rules! math {
    ($name:ident, $doc:expr) => {
        math!($name, $name, $doc);
    };

    ($name:ident, $implementation:ident, $doc:expr) => {
        math!(
            $name,
            fn (base: f64) -> CheckedF64Result {
                CheckedF64::new(base.$implementation())
            },
            $doc
        );
    };

    (
        $name:ident,
        fn ($base:ident : f64) -> $ret:ty $implementation:block,
        $doc:expr
    ) => {
        impl CheckedF64 {
            #[doc = $doc]
            #[must_use = "method returns a new instance and does not mutate the original value"]
            #[inline(always)]
            pub fn $name(self) -> $ret {
                let $base = self.0;
                $implementation
            }
        }

        impl CheckedF64Result {
            #[doc = $doc]
            #[must_use = "method returns a new instance and does not mutate the original value"]
            #[inline(always)]
            pub fn $name(self) -> $ret {
                let Ok($base) = self.as_inner() else { return self; };
                $base.$name()
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
            fn (base: f64, $operand: $t) -> CheckedF64Result {
                CheckedF64::new(base.$name($operand))
            },
            $doc
        );
    };

    (
        $name:ident,
        fn ($base:ident : f64, $operand:ident : $t:ty) -> $ret:ty $implementation:block,
        $doc:expr
    ) => {
        impl CheckedF64 {
            #[doc = $doc]
            #[must_use = "method returns a new instance and does not mutate the original value"]
            #[inline(always)]
            pub fn $name(self, $operand: $t) -> $ret {
                let $base = self.0;
                $implementation
            }
        }

        impl CheckedF64Result {
            #[doc = $doc]
            #[must_use = "method returns a new instance and does not mutate the original value"]
            #[inline(always)]
            pub fn $name(self, $operand: $t) -> $ret {
                let Ok($base) = self.as_inner() else { return self };
                $base.$name($operand)
            }
        }
    };
}

const_math!(
    abs,
    r"
        Computes the absolute value of self.

        See: [`f64::abs`]

        # Examples

        ```rust
        use checked_float::CheckedF64;

        let x = CheckedF64::new(3.5_f64);
        let y = CheckedF64::new(-3.5_f64);

        assert_eq!(x.abs(), x);
        assert_eq!(y.abs(), -y);
        ```
    "
);

const_math!(
    signum,
    r"
        Returns a number that represents the sign of `self`.

        See: [`f64::signum`]

        # Examples

        ```rust
        use checked_float::CheckedF64;

        let pos = CheckedF64::new(3.5_f64);
        let neg = CheckedF64::new(-3.5_f64);

        assert_eq!(pos.signum(), 1.0);
        assert_eq!(neg.signum(), -1.0);
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
        use checked_float::CheckedF64;

        let positive = CheckedF64::new(4.0_f64);
        let negative = CheckedF64::new(-4.0_f64);
        let negative_zero = CheckedF64::new(-0.0_f64);

        assert_eq!(positive.sqrt(), 2.0);
        assert!(negative.sqrt().is_err());
        assert_eq!(negative_zero.sqrt(), negative_zero);
        ```
    "
);

const_math!(
    recip,
    r"
        Takes the reciprocal (inverse) of `self`, `1/x` where `x` is `self`.

        See: [`f64::recip`]

        # Examples

        ```rust
        use checked_float::CheckedF64;

        let x = CheckedF64::new(2.0_f64);
        let abs_difference = (x.recip() - (1.0 / x)).unwrap().abs().unwrap();

        assert!(abs_difference < CheckedF64::EPSILON);
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
        use checked_float::CheckedF64;

        let one = CheckedF64::new(1.0_f64);

        // e^1
        let e = one.exp();

        // ln(e) - 1 == 0
        let abs_difference = (e.ln() - 1.0).unwrap().abs().unwrap();

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
        use checked_float::CheckedF64;

        let e = CheckedF64::new(2.718281828459045_f64);

        // ln(e) == 1
        let abs_difference = (e.ln() - 1.0).unwrap().abs().unwrap();

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
        use checked_float::CheckedF64;

        let x = CheckedF64::new(2.0_f64);
        let abs_difference = (x.powi(2) - (x * x)).unwrap().abs().unwrap();
        assert!(abs_difference <= CheckedF64::EPSILON);

        assert!(CheckedF64::new(f64::NAN).powi(2).is_err());
        ```
    "
);

math!(
    powf,
    fn (base: f64, power: impl Into<CheckedF64Result>) -> CheckedF64Result {
        let power: CheckedF64Result = power.into();
        let Ok(power) = power.as_inner() else { return power };
        CheckedF64::new(base.powf(power.0))
    },
    r"
        Raises a number to a floating-point power.

        See: [`f64::powf`]

        # Examples

        ```rust
        use checked_float::CheckedF64;

        let x = CheckedF64::new(2.0_f64);
        let cubed = CheckedF64::new(3.0).unwrap();
        let abs_difference = (x.powf(cubed) - (x * x * x)).abs();
        assert!(abs_difference.unwrap() <= CheckedF64::EPSILON);

        let invalid = CheckedF64::new(f64::NAN);
        assert!(invalid.powf(x.unwrap()).is_err());
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
        use checked_float::CheckedF64;

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
        use checked_float::CheckedF64;

        let f = CheckedF64::FRAC_PI_2;

        // asin(sin(pi/2))
        let abs_difference = (f.sin().asin() - CheckedF64::FRAC_PI_2).abs();

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
        use checked_float::CheckedF64;

        let e = CheckedF64::E;
        let x = 1.0_f64;

        let f = x.sinh();
        // Solving sinh() at 1 gives `(e^2-1)/(2e)`
        let g = ((e * e) - 1.0) / (2.0 * e);
        let abs_difference = (f - g).abs();

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
        use checked_float::CheckedF64;

        let x = CheckedF64::new(1.0_f64);
        let f = x.sinh().asinh();

        let abs_difference = (f - x).unwrap().abs().unwrap();

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
        use checked_float::CheckedF64;

        let x = 2.0 * CheckedF64::PI;

        let abs_difference = (x.cos() - 1.0).abs();

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
        use checked_float::CheckedF64;

        let f = CheckedF64::FRAC_PI_4;

        // acos(cos(pi/4))
        let abs_difference = (f.cos().acos() - CheckedF64::FRAC_PI_4).abs();

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
        use checked_float::CheckedF64;

        let e = CheckedF64::E;
        let x = CheckedF64::new(1.0_f64);
        let f = x.cosh();

        // Solving cosh() at 1 gives this result
        let g = ((e * e) + 1.0) / (2.0 * e);
        let abs_difference = (f - g).unwrap().abs().unwrap();

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
        use checked_float::CheckedF64;

        let x = CheckedF64::new(1.0);
        let f = x.cosh().acosh();

        let abs_difference = (f - x).unwrap().abs().unwrap();

        assert!(abs_difference < 1.0e-10);
        ```
    "
);

// math!(
//     sin_cos,
//     fn (base: f64) -> (CheckedF64Result, CheckedF64Result) {
//         let (sin, cos) = base.sin_cos();
//         (CheckedF64::new(sin), CheckedF64::new(cos))
//     },
//     r"
//         Simultaneously computes the sine and cosine of the number, `x`. Returns (sin(x), cos(x)).
//
//         See: [`f64::sin_cos`]
//
//         # Examples
//
//         ```rust
//         use checked_float::CheckedF64;
//
//         let x = CheckedF64::FRAC_PI_4;
//         let f = x.sin_cos();
//
//         let abs_difference_0 = (f.0 - x.sin()).abs();
//         let abs_difference_1 = (f.1 - x.cos()).abs();
//
//         assert!(abs_difference_0 < 1e-10);
//         assert!(abs_difference_1 < 1e-10);
//         ```
//     "
// );

math!(
    tan,
    r"
        Computes the tangent of a number (in radians).

        See: [`f64::tan`]

        # Examples

        ```rust
        use checked_float::CheckedF64;

        let x = CheckedF64::FRAC_PI_4;
        let abs_difference = (x.tan() - 1.0).abs();

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
        use checked_float::CheckedF64;

        let f = CheckedF64::new(1.0);

        // atan(tan(1))
        let abs_difference = (f.tan().atan() - 1.0).unwrap().abs().unwrap();

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
        use checked_float::CheckedF64;

        let x = CheckedF64::new(1.0_f64);
        let f = x.tanh();

        // tanh(1) is approximately 0.7615941559557649
        let abs_difference = (f - 0.7615941559557649).unwrap().abs().unwrap();

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
        use checked_float::CheckedF64;

        let x = CheckedF64::new(0.5_f64);
        let f = x.tanh().atanh();

        let abs_difference = (f - x).unwrap().abs().unwrap();

        assert!(abs_difference < 1e-10);
        ```
    "
);

math!(
    atan2,
    fn (base: f64, other: CheckedF64) -> CheckedF64Result {
        CheckedF64::new(base.atan2(other.0))
    },
    r"
        Computes the arctangent of `self` divided by `other`.

        See: [`f64::atan2`]

        # Arguments

        `other` - The `CheckedF64` value to divide `self` by.

        # Returns

        Returns a new `CheckedF64` instance containing the result of the arctangent operation.

        # Examples

        ```rust
        use checked_float::CheckedF64;

        let a = CheckedF64::new(1.0);
        let b = CheckedF64::new(2.0).unwrap();
        assert_eq!(a.atan2(b), Ok(0.4636476090008061)); // atan2(1.0, 2.0)

        let invalid = CheckedF64::new(f64::NAN);
        assert!(invalid.atan2(a.unwrap()).is_err());
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

    macro_rules! prop_assert_float_error {
        ($result:expr) => {
            prop_assert_eq!($result.unwrap_err(), FloatError);
        };

        ($result:expr, $msg:expr) => {
            prop_assert_eq!($result.unwrap_err().to_string(), $msg);
        };
    }
    
    proptest! {
        // Absolute value
        #[test]
        fn test_abs_valid(a in valid_f64()) {
            prop_assert_eq!(CheckedF64::new(a).abs(), CheckedF64::new(a.abs()));
        }

        #[test]
        fn test_abs_invalid(a in invalid_f64()) {
            prop_assert_eq!(*CheckedF64::new(a).abs(), Err(FloatError));
        }

        // Signing Number
        #[test]
        fn test_signum_valid(a in valid_f64()) {
            prop_assert_eq!(CheckedF64::new(a).signum(), a.signum());
        }

        #[test]
        fn test_signum_invalid(a in invalid_f64()) {
            prop_assert_eq!(*CheckedF64::new(a).signum(), Err(FloatError));
        }

        // Square Root
        #[test]
        fn test_sqrt_valid(a in valid_f64()) {
            if a.sqrt().is_finite() {
                prop_assert_eq!(CheckedF64::new(a).sqrt(), Ok(a.sqrt()));
            } else {
                prop_assert_eq!(*CheckedF64::new(a).sqrt(), Err(FloatError));
            }
        }

        #[test]
        fn test_sqrt_invalid(a in invalid_f64()) {
            prop_assert_eq!(*CheckedF64::new(a).sqrt(), Err(FloatError));
        }

        // Reciprocal
        #[test]
        fn test_recip_valid(a in valid_f64()) {
            prop_assert_eq!(CheckedF64::new(a).recip(), match (a.recip().is_finite(), a.recip()) {
                (true, value) => Ok(value),
                _ => Err(FloatError),
            });
        }

        #[test]
        fn test_recip_invalid(a in invalid_f64()) {
            prop_assert_eq!(*CheckedF64::new(a).recip(), Err(FloatError));
        }

        // Euler's Number
        #[test]
        fn test_exp_valid(a in valid_f64()) {
            if a.exp().is_finite() {
                prop_assert_eq!(CheckedF64::new(a).exp(), Ok(a.exp()));
            } else {
                prop_assert_eq!(*CheckedF64::new(a).exp(), Err(FloatError));
            }
        }

        #[test]
        fn test_exp_invalid(a in invalid_f64()) {
            prop_assert_eq!(*CheckedF64::new(a).exp(), Err(FloatError));
        }

        // Natural Logarithm
        #[test]
        fn test_ln_valid(a in valid_f64()) {
            if a > 0.0 && a.ln().is_finite() {
                prop_assert_eq!(CheckedF64::new(a).ln(), Ok(a.ln()));
            } else {
                prop_assert_eq!(*CheckedF64::new(a).ln(), Err(FloatError));
            }
        }

        #[test]
        fn test_ln_invalid(a in invalid_f64()) {
            prop_assert_eq!(*CheckedF64::new(a).ln(), Err(FloatError));
        }

        // Exponentiation
        #[test]
        fn test_powf_valid(a in valid_f64(), b in valid_f64()) {
            let checked_a = CheckedF64::new(a);
            let checked_b = CheckedF64::new(b).unwrap();
            let expected = a.powf(b);

            if expected.is_finite() {
                prop_assert_eq!(checked_a.powf(checked_b), expected);
                prop_assert_eq!(checked_a.unwrap().powf(checked_b), expected);
            } else {
                prop_assert_float_error!(checked_a.powf(checked_b));
                prop_assert_float_error!(checked_a.unwrap().powf(checked_b));
            }
        }

        #[test]
        fn test_powf_invalid_base(a in invalid_f64(), b in valid_f64()) {
            let checked_a = CheckedF64::new(a);
            let checked_b = CheckedF64::new(b).unwrap();

            prop_assert_float_error!(checked_a.powf(checked_b));
        }

        // Int Exponentiation
        #[test]
        fn test_powi_valid(a in valid_f64(), b in -10i32..=10i32) {
            if a.powi(b).is_finite() {
                prop_assert_eq!(CheckedF64::new(a).powi(b), Ok(a.powi(b)));
            } else {
                prop_assert_eq!(*CheckedF64::new(a).powi(b), Err(FloatError));
            }
        }

        #[test]
        fn test_powi_invalid(a in invalid_f64(), b in -10i32..=10i32) {
            prop_assert_eq!(*CheckedF64::new(a).powi(b), Err(FloatError));
        }

        // Sine Functions
        #[test]
        fn test_sin_valid(a in valid_f64()) {
            prop_assert_eq!(CheckedF64::new(a).sin(), match (a.sin().is_finite(), a.sin()) {
                (true, value) => Ok(value),
                _ => Err(FloatError),
            });
        }

        #[test]
        fn test_sin_invalid(a in invalid_f64()) {
            prop_assert_eq!(*CheckedF64::new(a).sin(), Err(FloatError));
        }

        // Inverse Sine Functions
        #[test]
        fn test_asin_valid(a in valid_f64()) {
            if a >= -1.0 && a <= 1.0 && a.asin().is_finite() {
                prop_assert_eq!(CheckedF64::new(a).asin(), Ok(a.asin()));
            } else {
                prop_assert_eq!(*CheckedF64::new(a).asin(), Err(FloatError));
            }
        }

        #[test]
        fn test_asin_invalid(a in invalid_f64()) {
            prop_assert_eq!(*CheckedF64::new(a).asin(), Err(FloatError));
        }

        // Hyperbolic Sine Functions
        #[test]
        fn test_sinh_valid(a in valid_f64()) {
            if a.sinh().is_finite() {
                prop_assert_eq!(CheckedF64::new(a).sinh(), Ok(a.sinh()));
            } else {
                prop_assert_eq!(*CheckedF64::new(a).sinh(), Err(FloatError));
            }
        }

        #[test]
        fn test_sinh_invalid(a in invalid_f64()) {
            prop_assert_eq!(*CheckedF64::new(a).sinh(), Err(FloatError));
        }

        // Inverse Hyperbolic Sine Functions
        #[test]
        fn test_asinh_valid(a in valid_f64()) {
            if a.asinh().is_finite() {
                prop_assert_eq!(CheckedF64::new(a).asinh(), Ok(a.asinh()));
            } else {
                prop_assert_eq!(*CheckedF64::new(a).asinh(), Err(FloatError));
            }
        }

        #[test]
        fn test_asinh_invalid(a in invalid_f64()) {
            prop_assert_eq!(*CheckedF64::new(a).asinh(), Err(FloatError));
        }

        // Cosine Functions
        #[test]
        fn test_cos_valid(a in valid_f64()) {
            if a.cos().is_finite() {
                prop_assert_eq!(CheckedF64::new(a).cos(), Ok(a.cos()));
            } else {
                prop_assert_eq!(*CheckedF64::new(a).cos(), Err(FloatError));
            }
        }

        #[test]
        fn test_cos_invalid(a in invalid_f64()) {
            prop_assert_eq!(*CheckedF64::new(a).cos(), Err(FloatError));
        }

        // Inverse Cosine Functions
        #[test]
        fn test_acos_valid(a in valid_f64()) {
            if a >= -1.0 && a <= 1.0 && a.acos().is_finite() {
                prop_assert_eq!(CheckedF64::new(a).acos(), Ok(a.acos()));
            } else {
                prop_assert_eq!(*CheckedF64::new(a).acos(), Err(FloatError));
            }
        }

        #[test]
        fn test_acos_invalid(a in invalid_f64()) {
            prop_assert_eq!(*CheckedF64::new(a).acos(), Err(FloatError));
        }

        // Hyperbolic Cosine Functions
        #[test]
        fn test_cosh_valid(a in valid_f64()) {
            if a.cosh().is_finite() {
                prop_assert_eq!(CheckedF64::new(a).cosh(), Ok(a.cosh()));
            } else {
                prop_assert_eq!(*CheckedF64::new(a).cosh(), Err(FloatError));
            }
        }

        #[test]
        fn test_cosh_invalid(a in invalid_f64()) {
            prop_assert_eq!(*CheckedF64::new(a).cosh(), Err(FloatError));
        }

        // Inverse Hyperbolic Cosine Functions
        #[test]
        fn test_acosh_valid(a in valid_f64()) {
            if a >= 1.0 && a.acosh().is_finite() {
                prop_assert_eq!(CheckedF64::new(a).acosh(), Ok(a.acosh()));
            } else {
                prop_assert_eq!(*CheckedF64::new(a).acosh(), Err(FloatError));
            }
        }

        #[test]
        fn test_acosh_invalid(a in invalid_f64()) {
            prop_assert_eq!(*CheckedF64::new(a).acosh(), Err(FloatError));
        }

        // Tangent Functions
        #[test]
        fn test_tan_valid(a in valid_f64()) {
            if a.tan().is_finite() {
                prop_assert_eq!(CheckedF64::new(a).tan(), Ok(a.tan()));
            } else {
                prop_assert_eq!(*CheckedF64::new(a).tan(), Err(FloatError));
            }
        }

        #[test]
        fn test_tan_invalid(a in invalid_f64()) {
            prop_assert_eq!(*CheckedF64::new(a).tan(), Err(FloatError));
        }

        // Inverse Tangent Functions
        #[test]
        fn test_atan_valid(a in valid_f64()) {
            if a.atan().is_finite() {
                prop_assert_eq!(CheckedF64::new(a).atan(), Ok(a.atan()));
            } else {
                prop_assert_eq!(*CheckedF64::new(a).atan(), Err(FloatError));
            }
        }

        #[test]
        fn test_atan_invalid(a in invalid_f64()) {
            prop_assert_eq!(*CheckedF64::new(a).atan(), Err(FloatError));
        }

        // Hyperbolic Tangent Functions
        #[test]
        fn test_tanh_valid(a in valid_f64()) {
            if a.tanh().is_finite() {
                prop_assert_eq!(CheckedF64::new(a).tanh(), Ok(a.tanh()));
            } else {
                prop_assert_eq!(*CheckedF64::new(a).tanh(), Err(FloatError));
            }
        }

        #[test]
        fn test_tanh_invalid(a in invalid_f64()) {
            prop_assert_eq!(*CheckedF64::new(a).tanh(), Err(FloatError));
        }

        // Inverse Hyperbolic Tangent Functions
        #[test]
        fn test_atanh_valid(a in valid_f64()) {
            if a > -1.0 && a < 1.0 && a.atanh().is_finite() {
                prop_assert_eq!(CheckedF64::new(a).atanh(), Ok(a.atanh()));
            } else {
                prop_assert_eq!(*CheckedF64::new(a).atanh(), Err(FloatError));
            }
        }

        #[test]
        fn test_atanh_invalid(a in invalid_f64()) {
            prop_assert_eq!(*CheckedF64::new(a).atanh(), Err(FloatError));
        }

        // Arctan2 Functions
        #[test]
        fn test_atan2_valid(a in valid_f64(), b in valid_f64()) {
            let checked_a = CheckedF64::new(a);
            let checked_b = CheckedF64::new(b).unwrap();
            let expected = a.atan2(b);

            if b != 0.0 && expected.is_finite() {
                prop_assert_eq!(checked_a.atan2(checked_b), expected);
                prop_assert_eq!(checked_a.unwrap().atan2(checked_b), expected);
            } else {
                prop_assert_float_error!(checked_a.atan2(checked_b));
                prop_assert_float_error!(checked_a.unwrap().atan2(checked_b));
            }
        }

        #[test]
        fn test_atan2_invalid_base(a in invalid_f64(), b in valid_f64()) {
            let checked_a = CheckedF64::new(a);
            let checked_b = CheckedF64::new(b).unwrap();

            prop_assert_float_error!(checked_a.atan2(checked_b));
        }

    //     // Sin & Cos Functions
    //     #[test]
    //     fn test_sin_cos_valid(a in valid_f64()) {
    //         if a.sin().is_finite() && a.cos().is_finite() {
    //             let (sin_val, cos_val) = CheckedF64::new(a).sin_cos();
    //             prop_assert_eq!(sin_val, Ok(a.sin()));
    //             prop_assert_eq!(cos_val, Ok(a.cos()));
    //         } else {
    //             let (sin_val, cos_val) = CheckedF64::new(a).sin_cos();
    //             prop_assert_eq!(*sin_val, Err(FloatError));
    //             prop_assert_eq!(*cos_val, Err(FloatError));
    //         }
    //     }
    //
    //     #[test]
    //     fn test_sin_cos_invalid(a in invalid_f64()) {
    //         let (sin_val, cos_val) = CheckedF64::new(a).sin_cos();
    //         prop_assert_eq!(*sin_val, Err(FloatError));
    //         prop_assert_eq!(*cos_val, Err(FloatError));
    //     }
    }
}
