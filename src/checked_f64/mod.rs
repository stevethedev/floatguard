mod cmp;
mod consts;
mod ops_binary;
mod ops_unary;

use crate::FloatError;

/// A result type for operations on `CheckedF64` that can return a `FloatError`.
pub type CheckedF64Result = std::result::Result<CheckedF64, FloatError>;

/// Represents a checked floating-point number that ensures it is neither NaN nor infinite.
///
/// # Example
///
/// ```rust
/// use checked_float::{CheckedF64, FloatError};
///
/// let checked_f64 = CheckedF64::try_from(1.0).expect("1.0 is a valid f64 value");
/// assert_eq!((checked_f64 + 1.0).unwrap().get(), Ok(2.0));
///
/// assert_eq!(checked_f64 / 0.0, Err(FloatError));
///
/// assert_eq!(checked_f64 - f64::INFINITY, Err(FloatError));
///
/// assert_eq!((checked_f64 % f64::NAN).get(), Err(FloatError));
/// ```
#[derive(Debug, Default, Clone, Copy)]
pub struct CheckedF64(f64);

impl std::fmt::Display for CheckedF64 {
    /// Formats the `CheckedF64` as a string.
    ///
    /// # Returns
    ///
    /// Returns a string representation of the inner `f64` value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use checked_float::CheckedF64;
    ///
    /// let value = CheckedF64::new(2.0);
    /// assert_eq!(value.to_string(), "CheckedF64(2)");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_valid() {
            write!(f, "CheckedF64({})", self.0)
        } else {
            write!(f, "CheckedF64(invalid)")
        }
    }
}

/// Implementing the ability to convert `CheckedF64` to `f64` safely.
///
/// This conversion will return an error if the value is NaN or infinite.
impl TryFrom<CheckedF64> for f64 {
    type Error = FloatError;

    /// Converts a `CheckedF64` to `f64`.
    ///
    /// # Returns
    ///
    /// Returns the inner `f64` value if it is valid (finite), otherwise returns an error.
    ///
    /// # Errors
    ///
    /// Returns `FloatError` if the value is NaN or infinite.
    ///
    /// # Example
    ///
    /// ```rust
    /// use checked_float::{CheckedF64, FloatError};
    ///
    /// let valid_value = CheckedF64::new(2.0);
    /// assert_eq!(f64::try_from(valid_value), Ok(2.0));
    ///
    /// let invalid_value = CheckedF64::new(f64::NAN);
    /// assert_eq!(f64::try_from(invalid_value), Err(FloatError));
    ///
    /// let inf_value = CheckedF64::new(f64::INFINITY);
    /// assert_eq!(f64::try_from(inf_value), Err(FloatError));
    /// ```
    fn try_from(value: CheckedF64) -> Result<Self, Self::Error> {
        if value.is_valid() {
            Ok(value.0)
        } else {
            Err(FloatError)
        }
    }
}

/// Implementing the ability to convert `f64` to `CheckedF64`.
impl TryFrom<f64> for CheckedF64 {
    type Error = FloatError;

    /// Converts an `f64` to `CheckedF64`.
    ///
    /// # Returns
    ///
    /// Returns a `CheckedF64` instance if the value is valid (finite), otherwise returns an error.
    ///
    /// # Errors
    ///
    /// Returns `FloatError` if the value is NaN or infinite.
    ///
    /// # Example
    ///
    /// ```rust
    /// use checked_float::{CheckedF64, FloatError};
    ///
    /// let valid_value = 2.0;
    /// assert_eq!(CheckedF64::try_from(valid_value), Ok(CheckedF64::new(2.0)));
    ///
    /// let invalid_value = f64::NAN;
    /// assert_eq!(CheckedF64::try_from(invalid_value), Err(FloatError));
    /// ```
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value.is_finite() {
            Ok(Self(value))
        } else {
            Err(FloatError)
        }
    }
}

macro_rules! define_operation {
    ($op:tt, $op_trait:ident, $op_method:ident, $assign_trait:ident, $assign_method:ident, $implementation:expr) => {
        #[doc = concat!(
            "Implementing the binary [`", stringify!($op_trait), "`](core::ops::", stringify!($op_trait), ") operator for `CheckedF64`.\n",
            "\n",
            "This allows the ", stringify!($op), " operation between two `CheckedF64` values, returning a new `CheckedF64` instance.\n",
            "\n",
            "The operation will return an error if either value is NaN or infinite.\n",
        )]
        impl std::ops::$op_trait for CheckedF64 {
            type Output = Self;

            fn $op_method(self, other: Self) -> Self::Output {
                let result = ($implementation)(self.0, other.0);
                Self(result)
            }
        }

        #[doc = concat!(
            "Implementing the binary [`", stringify!($op_trait), "`](core::ops::", stringify!($op_trait), ") operator for `CheckedF64`.\n",
            "\n",
            "This allows the ", stringify!($op), " operation between an `f64` and a `CheckedF64`, returning a new `CheckedF64` instance.\n",
        )]
        impl std::ops::$op_trait<f64> for CheckedF64 {
            type Output = Self;

            fn $op_method(self, other: f64) -> Self::Output {
                let result = ($implementation)(self.0, other);
                Self(result)
            }
        }

        #[doc = concat!(
            "Implementing the binary [`", stringify!($op_trait), "`](core::ops::", stringify!($op_trait), ") operator for `CheckedF64`.\n",
            "\n",
            "This allows the ", stringify!($op), " operation between a `CheckedF64` and an `f64`, returning a new `CheckedF64` instance.\n",
        )]
        impl std::ops::$op_trait<CheckedF64> for f64 {
            type Output = CheckedF64;

            fn $op_method(self, other: CheckedF64) -> Self::Output {
                let result = ($implementation)(self, other.0);
                CheckedF64(result)
            }
        }

        #[doc = concat!(
            "Implementing the [`", stringify!($assign_trait), "`](std::ops::", stringify!($assign_trait), ") trait for `CheckedF64`.\n",
            "\n",
            "This allows `", stringify!($assign), "` to be used between two `CheckedF64` values.\n",
        )]
        impl std::ops::$assign_trait for CheckedF64 {
            fn $assign_method(&mut self, other: Self) {
                let result = ($implementation)(self.0, other.0);
                self.0 = result;
            }
        }

        #[doc = concat!(
            "Implementing the [`", stringify!($assign_trait), "`](std::ops::", stringify!($assign_trait), ") trait for `CheckedF64`.\n",
            "\n",
            "This allows `", stringify!($assign), "` to be used between a `CheckedF64` and an `f64`.\n",
        )]
        impl std::ops::$assign_trait<f64> for CheckedF64 {
            fn $assign_method(&mut self, other: f64) {
                let result = ($implementation)(self.0, other);
                self.0 = result;
            }
        }
    };
}

#[allow(clippy::inline_always)]
#[inline(always)]
fn rem(a: f64, b: f64) -> f64 {
    if b.is_infinite() { f64::NAN } else { a % b }
}
define_operation!(%, Rem, rem, RemAssign, rem_assign, rem);

macro_rules! copy_const_op {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[must_use = "method returns a new instance and does not mutate the original value"]
        #[inline(always)]
        pub const fn $name(self) -> Self {
            if self.is_valid() {
                Self(self.0.$name())
            } else {
                self
            }
        }
    };
}

macro_rules! copy_op {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[must_use = "method returns a new instance and does not mutate the original value"]
        #[inline(always)]
        pub fn $name(self) -> Self {
            if self.is_valid() {
                Self(self.0.$name())
            } else {
                self
            }
        }
    };

    ($name:ident, $operand:ident, $t:tt, $doc:expr) => {
        #[doc = $doc]
        #[must_use = "method returns a new instance and does not mutate the original value"]
        #[inline(always)]
        pub fn $name(self, $operand: $t) -> Self {
            if self.is_valid() {
                Self(self.0.$name($operand))
            } else {
                self
            }
        }
    };
}

impl CheckedF64 {
    /// Creates a new `CheckedF64` instance from a raw `f64` value.
    ///
    /// # Safety
    ///
    /// This function does not check if the value is finite. It is the caller's responsibility to
    /// ensure that the value is valid.
    ///
    /// # Example
    ///
    /// ```rust
    /// use checked_float::CheckedF64;
    ///
    /// let valid_value = CheckedF64::new(3.14);
    /// assert_eq!(valid_value.get(), Ok(3.14));
    ///
    /// let invalid_value = CheckedF64::new(f64::NAN);
    /// assert!(invalid_value.is_invalid());
    /// ```
    #[must_use = "this function returns a new CheckedF64 instance"]
    #[inline]
    pub const fn new(value: f64) -> Self {
        Self(value)
    }

    /// Extracts the inner `f64` value from `CheckedF64`.
    ///
    /// # Returns
    ///
    /// Returns the inner `f64` value if it is finite, otherwise returns an error.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is NaN or infinite.
    ///
    /// # Example
    ///
    /// ```rust
    /// use checked_float::{CheckedF64, FloatError};
    ///
    /// let checked_value = CheckedF64::new(2.5);
    /// assert_eq!(checked_value.get(), Ok(2.5));
    ///
    /// let invalid_value = CheckedF64::new(f64::NAN);
    /// assert_eq!(invalid_value.get(), Err(FloatError));
    /// ```
    #[allow(clippy::inline_always)]
    #[inline(always)]
    pub fn get(self) -> Result<f64, FloatError> {
        f64::try_from(self)
    }

    /// Checks if the value is invalid (NaN or infinite).
    ///
    /// # Returns
    ///
    /// Returns `true` if the value is NaN or infinite, otherwise returns `false`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use checked_float::CheckedF64;
    ///
    /// let valid_value = CheckedF64::new(1.0);
    /// assert!(!valid_value.is_invalid());
    ///
    /// let invalid_value = CheckedF64::new(f64::NAN);
    /// assert!(invalid_value.is_invalid());
    /// ```
    #[must_use = "this function returns true if the value is invalid, otherwise false"]
    #[allow(clippy::inline_always)]
    #[inline(always)]
    pub const fn is_invalid(self) -> bool {
        !self.is_valid()
    }

    /// Checks if the value is valid (not NaN or infinite).
    ///
    /// # Returns
    ///
    /// Returns `true` if the value is finite, otherwise returns `false`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use checked_float::CheckedF64;
    ///
    /// let valid_value = CheckedF64::new(1.0);
    /// assert!(valid_value.is_valid());
    ///
    /// let invalid_value = CheckedF64::new(f64::NAN);
    /// assert!(!invalid_value.is_valid());
    /// ```
    #[must_use = "this function returns true if the value is valid, otherwise false"]
    #[allow(clippy::inline_always)]
    #[inline(always)]
    pub const fn is_valid(self) -> bool {
        self.0.is_finite()
    }

    copy_const_op!(
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

    copy_const_op!(
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

    copy_op!(
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

            assert_eq!(positive.sqrt().get(), Ok(2.0));
            assert!(negative.sqrt().is_invalid());
            assert_eq!(negative_zero.sqrt(), negative_zero);
            ```
        "
    );

    copy_const_op!(
        recip,
        r"
            Takes the reciprocal (inverse) of `self`, `1/x` where `x` is `self`.

            See: [`f64::recip`]

            # Examples

            ```rust
            use checked_float::CheckedF64;

            let x = CheckedF64::new(2.0_f64);
            let abs_difference = (x.recip() - (1.0 / x)).unwrap().abs();

            assert!(abs_difference < CheckedF64::EPSILON);
            ```
        "
    );

    copy_op!(
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
            let abs_difference = (e.ln() - 1.0).unwrap().abs();

            assert!(abs_difference < 1e-10);
            ```
        "
    );

    copy_op!(
        ln,
        r"
            Returns the natural logarithm of a number, `ln(self)`.

            See: [`f64::ln`]

            # Examples

            ```rust
            use checked_float::CheckedF64;

            let e = CheckedF64::new(2.718281828459045_f64);

            // ln(e) == 1
            let abs_difference = (e.ln() - 1.0).unwrap().abs();

            assert!(abs_difference < 1e-10);
            ```
        "
    );

    copy_op!(
        powi,
        power,
        i32,
        r"
            Raises a number to an integer power.

            See: [`f64::powi`]

            # Examples

            ```rust
            use checked_float::CheckedF64;

            let x = CheckedF64::new(2.0_f64);
            let abs_difference = (x.powi(2) - (x * x)).unwrap().abs();
            assert!(abs_difference <= CheckedF64::EPSILON);

            assert!(CheckedF64::new(f64::NAN).powi(2).is_invalid());
            ```
        "
    );

    /// Raises a number to a floating-point power.
    ///
    /// See: [`f64::powf`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// use checked_float::CheckedF64;
    ///
    /// let x = CheckedF64::new(2.0_f64);
    /// let abs_difference = (x.powf(3.0) - (x * x * x)).unwrap().abs();
    /// assert!(abs_difference <= CheckedF64::EPSILON);
    ///
    /// let invalid = CheckedF64::new(f64::NAN);
    /// assert!(invalid.powf(2.0).is_invalid());
    /// assert!(CheckedF64::new(2.0).powf(f64::NAN).is_invalid());
    /// ```
    #[must_use = "this function returns a new CheckedF64 instance"]
    #[allow(clippy::inline_always)]
    #[inline(always)]
    pub fn powf(self, power: impl TryInto<Self>) -> Self {
        match power.try_into() {
            Ok(power) if power.is_valid() && self.is_valid() => Self(self.0.powf(power.0)),
            _ => Self(f64::NAN),
        }
    }

    copy_op!(
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

    copy_op!(
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

    copy_op!(
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

    copy_op!(
        asinh,
        r"
            Inverse hyperbolic sine function.

            See: [`f64::asinh`]

            # Examples

            ```rust
            use checked_float::CheckedF64;

            let x = CheckedF64::new(1.0_f64);
            let f = x.sinh().asinh();

            let abs_difference = (f - x).unwrap().abs();

            assert!(abs_difference < 1.0e-10);
            ```
        "
    );

    copy_op!(
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

    copy_op!(
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

    copy_op!(
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
            let abs_difference = (f - g).unwrap().abs();

            // Same result
            assert!(abs_difference < 1e-10);
            ```
        "
    );

    copy_op!(
        acosh,
        r"
            Inverse hyperbolic cosine function.

            See: [`f64::acosh`]

            # Examples

            ```rust
            use checked_float::CheckedF64;

            let x = CheckedF64::new(1.0);
            let f = x.cosh().acosh();

            let abs_difference = (f - x).unwrap().abs();

            assert!(abs_difference < 1.0e-10);
            ```
        "
    );

    /// Simultaneously computes the sine and cosine of the number, x. Returns (sin(x), cos(x)).
    ///
    /// See: [`f64::sin_cos`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// use checked_float::CheckedF64;
    ///
    /// let x = CheckedF64::FRAC_PI_4;
    /// let f = x.sin_cos();
    ///
    /// let abs_difference_0 = (f.0 - x.sin()).abs();
    /// let abs_difference_1 = (f.1 - x.cos()).abs();
    ///
    /// assert!(abs_difference_0 < 1e-10);
    /// assert!(abs_difference_1 < 1e-10);
    /// ```
    #[must_use = "this function returns a tuple of two CheckedF64 instances"]
    #[allow(clippy::inline_always)]
    #[inline(always)]
    pub fn sin_cos(self) -> (Self, Self) {
        match self.0.sin_cos() {
            (sin, cos) if sin.is_finite() && cos.is_finite() => (Self(sin), Self(cos)),
            _ => (Self(f64::NAN), Self(f64::NAN)),
        }
    }

    copy_op!(
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

    copy_op!(
        atan,
        r"
            Computes the arctangent of a number. Return value is in radians in the range [-&pi;/2, &pi;/2].

            See: [`f64::atan`]

            # Examples

            ```rust
            use checked_float::CheckedF64;

            let f = CheckedF64::new(1.0);

            // atan(tan(1))
            let abs_difference = (f.tan().atan() - 1.0).unwrap().abs();

            assert!(abs_difference < 1e-10)
            ```
        "
    );

    copy_op!(
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
            let abs_difference = (f - 0.7615941559557649).unwrap().abs();

            assert!(abs_difference < 1e-10);
            ```
        "
    );

    copy_op!(
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

            let abs_difference = (f - x).unwrap().abs();

            assert!(abs_difference < 1e-10);
            ```
        "
    );

    /// Computes the arctangent of `self` divided by `other`.
    ///
    /// See: [`f64::atan2`]
    ///
    /// # Arguments
    ///
    /// `other` - The `CheckedF64` value to divide `self` by.
    ///
    /// # Returns
    ///
    /// Returns a new `CheckedF64` instance containing the result of the arctangent operation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use checked_float::CheckedF64;
    ///
    /// let a = CheckedF64::new(1.0);
    /// let b = CheckedF64::new(2.0);
    /// assert_eq!(a.atan2(b).get(), Ok(0.4636476090008061)); // atan2(1.0, 2.0)
    ///
    /// let invalid = CheckedF64::new(f64::NAN);
    /// assert!(invalid.atan2(CheckedF64::new(1.0)).is_invalid());
    /// assert!(CheckedF64::new(1.0).atan2(CheckedF64::new(f64::NAN)).is_invalid());
    /// assert!(CheckedF64::new(f64::INFINITY).atan2(CheckedF64::new(1.0)).is_invalid());
    /// assert!(CheckedF64::new(1.0).atan2(CheckedF64::new(f64::INFINITY)).is_invalid());
    /// ```
    #[must_use = "this function returns a new CheckedF64 instance"]
    #[allow(clippy::inline_always)]
    #[inline(always)]
    pub fn atan2(self, other: impl TryInto<Self>) -> Self {
        match other.try_into() {
            Ok(other) if self.is_valid() && other.is_valid() => Self(self.0.atan2(other.0)),
            _ => Self(f64::NAN),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    const INVALID_VALUES: &[f64; 3] = &[f64::NAN, f64::INFINITY, f64::NEG_INFINITY];

    pub fn valid_f64() -> impl Strategy<Value = f64> {
        // Avoid NaN, ±∞, and subnormals.
        // This gives good coverage while staying in safe computation territory.
        (f64::MIN..=f64::MAX).prop_filter("Reject NaN and infinities", |v| {
            v.is_finite() && !v.is_nan()
        })
    }

    pub fn invalid_f64() -> impl Strategy<Value = f64> {
        prop::sample::select(INVALID_VALUES)
    }

    proptest! {
        #[test]
        fn test_from_valid(a in valid_f64()) {
            prop_assert_eq!(CheckedF64(a).get(), Ok(a));
        }

        #[test]
        fn test_from_invalid(a in invalid_f64()) {
            prop_assert_eq!(CheckedF64(a).get(), Err(FloatError));
        }

        // Remainder Operations
        #[test]
        fn test_valid_rem_valid_eq_valid(a in valid_f64(), b in valid_f64()) {
            if b != 0.0 && (a % b).is_finite() {
                prop_assert_eq!((CheckedF64(a) % CheckedF64(b)).get(), Ok(a % b));
                prop_assert_eq!((CheckedF64(a) % b).get(), Ok(a % b));
                prop_assert_eq!((a % CheckedF64(b)).get(), Ok(a % b));

                let mut checked_remainder = CheckedF64(a);
                checked_remainder %= CheckedF64(b);
                prop_assert_eq!(checked_remainder.get(), Ok(a % b));

                let mut checked_remainder = CheckedF64(a);
                checked_remainder %= b;
                prop_assert_eq!(checked_remainder.get(), Ok(a % b));
            } else {
                prop_assert_eq!((CheckedF64(a) % CheckedF64(b)).get(), Err(FloatError));
                prop_assert_eq!((CheckedF64(a) % b).get(), Err(FloatError));
                prop_assert_eq!((a % CheckedF64(b)).get(), Err(FloatError));

                let mut checked_remainder = CheckedF64(a);
                checked_remainder %= CheckedF64(b);
                prop_assert_eq!(checked_remainder.get(), Err(FloatError));

                let mut checked_remainder = CheckedF64(a);
                checked_remainder %= b;
                prop_assert_eq!(checked_remainder.get(), Err(FloatError));
            }
        }

        #[test]
        fn test_valid_rem_invalid_eq_invalid(a in valid_f64(), b in invalid_f64()) {
            prop_assert_eq!((CheckedF64(a) % CheckedF64(b)).get(), Err(FloatError));
            prop_assert_eq!((CheckedF64(a) % b).get(), Err(FloatError));
            prop_assert_eq!((a % CheckedF64(b)).get(), Err(FloatError));

            let mut checked_remainder = CheckedF64(a);
            checked_remainder %= CheckedF64(b);
            prop_assert_eq!(checked_remainder.get(), Err(FloatError));

            let mut checked_remainder = CheckedF64(a);
            checked_remainder %= b;
            prop_assert_eq!(checked_remainder.get(), Err(FloatError));
        }

        #[test]
        fn test_invalid_rem_valid_eq_invalid(a in invalid_f64(), b in valid_f64()) {
            prop_assert_eq!((CheckedF64(a) % CheckedF64(b)).get(), Err(FloatError));
            prop_assert_eq!((CheckedF64(a) % b).get(), Err(FloatError));
            prop_assert_eq!((a % CheckedF64(b)).get(), Err(FloatError));

            let mut checked_remainder = CheckedF64(a);
            checked_remainder %= CheckedF64(b);
            prop_assert_eq!(checked_remainder.get(), Err(FloatError));

            let mut checked_remainder = CheckedF64(a);
            checked_remainder %= b;
            prop_assert_eq!(checked_remainder.get(), Err(FloatError));
        }

        #[test]
        fn test_invalid_rem_invalid_eq_invalid(a in invalid_f64(), b in invalid_f64()) {
            prop_assert_eq!((CheckedF64(a) % CheckedF64(b)).get(), Err(FloatError));
            prop_assert_eq!((CheckedF64(a) % b).get(), Err(FloatError));
            prop_assert_eq!((a % CheckedF64(b)).get(), Err(FloatError));

            let mut checked_remainder = CheckedF64(a);
            checked_remainder %= CheckedF64(b);
            prop_assert_eq!(checked_remainder.get(), Err(FloatError));

            let mut checked_remainder = CheckedF64(a);
            checked_remainder %= b;
            prop_assert_eq!(checked_remainder.get(), Err(FloatError));
        }

        // Absolute value
        #[test]
        fn test_abs_valid(a in valid_f64()) {
            prop_assert_eq!(CheckedF64(a).abs().get(), Ok(a.abs()));
        }

        #[test]
        fn test_abs_invalid(a in invalid_f64()) {
            prop_assert_eq!(CheckedF64(a).abs().get(), Err(FloatError));
        }

        // Signing Number
        #[test]
        fn test_signum_valid(a in valid_f64()) {
            prop_assert_eq!(CheckedF64(a).signum().get(), Ok(a.signum()));
        }

        #[test]
        fn test_signum_invalid(a in invalid_f64()) {
            prop_assert_eq!(CheckedF64(a).signum().get(), Err(FloatError));
        }

        // Square Root
        #[test]
        fn test_sqrt_valid(a in valid_f64()) {
            prop_assert_eq!(CheckedF64(a).sqrt().get(), match (a.sqrt().is_finite(), a.sqrt()) {
                (true, value) => Ok(value),
                _ => Err(FloatError),
            });
        }

        #[test]
        fn test_sqrt_invalid(a in invalid_f64()) {
            prop_assert_eq!(CheckedF64(a).sqrt().get(), Err(FloatError));
        }

        // Reciprocal
        #[test]
        fn test_recip_valid(a in valid_f64()) {
            prop_assert_eq!(CheckedF64(a).recip().get(), match (a.recip().is_finite(), a.recip()) {
                (true, value) => Ok(value),
                _ => Err(FloatError),
            });
        }

        #[test]
        fn test_recip_invalid(a in invalid_f64()) {
            prop_assert_eq!(CheckedF64(a).recip().get(), Err(FloatError));
        }

        // Euler's Number
        #[test]
        fn test_exp_valid(a in valid_f64()) {
            prop_assert_eq!(CheckedF64(a).exp().get(), match (a.exp().is_finite(), a.exp()) {
                (true, value) => Ok(value),
                _ => Err(FloatError),
            });
        }

        #[test]
        fn test_exp_invalid(a in invalid_f64()) {
            prop_assert_eq!(CheckedF64(a).exp().get(), Err(FloatError));
        }

        // Natural Logarithm
        #[test]
        fn test_ln_valid(a in valid_f64()) {
            if a > 0.0 && a.ln().is_finite() {
                prop_assert_eq!(CheckedF64(a).ln().get(), Ok(a.ln()));
            } else {
                prop_assert_eq!(CheckedF64(a).ln().get(), Err(FloatError));
            }
        }

        #[test]
        fn test_ln_invalid(a in invalid_f64()) {
            prop_assert_eq!(CheckedF64(a).ln().get(), Err(FloatError));
        }

        // Exponentiation
        #[test]
        fn test_powf_valid(a in valid_f64(), b in valid_f64()) {
            if a.powf(b).is_finite() && b.is_finite() {
                prop_assert_eq!(CheckedF64(a).powf(CheckedF64(b)).get(), Ok(a.powf(b)));
                prop_assert_eq!(CheckedF64(a).powf(b).get(), Ok(a.powf(b)));
            } else {
                prop_assert_eq!(CheckedF64(a).powf(CheckedF64(b)).get(), Err(FloatError));
                prop_assert_eq!(CheckedF64(a).powf(b).get(), Err(FloatError));
            }
        }

        #[test]
        fn test_powf_invalid(a in valid_f64(), b in invalid_f64()) {
            prop_assert_eq!(CheckedF64(a).powf(CheckedF64(b)).get(), Err(FloatError));
            prop_assert_eq!(CheckedF64(a).powf(b).get(), Err(FloatError));
        }

        #[test]
        fn test_powf_invalid_base(a in invalid_f64(), b in valid_f64()) {
            prop_assert_eq!(CheckedF64(a).powf(CheckedF64(b)).get(), Err(FloatError));
            prop_assert_eq!(CheckedF64(a).powf(b).get(), Err(FloatError));
        }

        #[test]
        fn test_powf_invalid_base_and_exponent(a in invalid_f64(), b in invalid_f64()) {
            prop_assert_eq!(CheckedF64(a).powf(CheckedF64(b)).get(), Err(FloatError));
            prop_assert_eq!(CheckedF64(a).powf(b).get(), Err(FloatError));
        }

        // Int Exponentiation
        #[test]
        fn test_powi_valid(a in valid_f64(), b in -10i32..=10i32) {
            if a.powi(b).is_finite() {
                prop_assert_eq!(CheckedF64(a).powi(b).get(), Ok(a.powi(b)));
            } else {
                prop_assert_eq!(CheckedF64(a).powi(b).get(), Err(FloatError));
            }
        }

        #[test]
        fn test_powi_invalid(a in invalid_f64(), b in -10i32..=10i32) {
            prop_assert_eq!(CheckedF64(a).powi(b).get(), Err(FloatError));
        }

        // Sine Functions
        #[test]
        fn test_sin_valid(a in valid_f64()) {
            prop_assert_eq!(CheckedF64(a).sin().get(), match (a.sin().is_finite(), a.sin()) {
                (true, value) => Ok(value),
                _ => Err(FloatError),
            });
        }

        #[test]
        fn test_sin_invalid(a in invalid_f64()) {
            prop_assert_eq!(CheckedF64(a).sin().get(), Err(FloatError));
        }

        // Inverse Sine Functions
        #[test]
        fn test_asin_valid(a in valid_f64()) {
            if a >= -1.0 && a <= 1.0 && a.asin().is_finite() {
                prop_assert_eq!(CheckedF64(a).asin().get(), Ok(a.asin()));
            } else {
                prop_assert_eq!(CheckedF64(a).asin().get(), Err(FloatError));
            }
        }

        #[test]
        fn test_asin_invalid(a in invalid_f64()) {
            prop_assert_eq!(CheckedF64(a).asin().get(), Err(FloatError));
        }

        // Hyperbolic Sine Functions
        #[test]
        fn test_sinh_valid(a in valid_f64()) {
            prop_assert_eq!(CheckedF64(a).sinh().get(), match (a.sinh().is_finite(), a.sinh()) {
                (true, value) => Ok(value),
                _ => Err(FloatError),
            });
        }

        #[test]
        fn test_sinh_invalid(a in invalid_f64()) {
            prop_assert_eq!(CheckedF64(a).sinh().get(), Err(FloatError));
        }

        // Inverse Hyperbolic Sine Functions
        #[test]
        fn test_asinh_valid(a in valid_f64()) {
            prop_assert_eq!(CheckedF64(a).asinh().get(), match (a.asinh().is_finite(), a.asinh()) {
                (true, value) => Ok(value),
                _ => Err(FloatError),
            });
        }

        #[test]
        fn test_asinh_invalid(a in invalid_f64()) {
            prop_assert_eq!(CheckedF64(a).asinh().get(), Err(FloatError));
        }

        // Cosine Functions
        #[test]
        fn test_cos_valid(a in valid_f64()) {
            prop_assert_eq!(CheckedF64(a).cos().get(), match (a.cos().is_finite(), a.cos()) {
                (true, value) => Ok(value),
                _ => Err(FloatError),
            });
        }

        #[test]
        fn test_cos_invalid(a in invalid_f64()) {
            prop_assert_eq!(CheckedF64(a).cos().get(), Err(FloatError));
        }

        // Inverse Cosine Functions
        #[test]
        fn test_acos_valid(a in valid_f64()) {
            prop_assert_eq!(CheckedF64(a).acos().get(), match (a.acos().is_finite(), a) {
                (true, a) if a >= -1.0 && a <= 1.0 => Ok(a.acos()),
                _ => Err(FloatError)
            })
        }

        #[test]
        fn test_acos_invalid(a in invalid_f64()) {
            prop_assert_eq!(CheckedF64(a).acos().get(), Err(FloatError));
        }

        // Hyperbolic Cosine Functions
        #[test]
        fn test_cosh_valid(a in valid_f64()) {
            prop_assert_eq!(CheckedF64(a).cosh().get(), match (a.cosh().is_finite(), a.cosh()) {
                (true, value) => Ok(value),
                _ => Err(FloatError),
            });
        }

        #[test]
        fn test_cosh_invalid(a in invalid_f64()) {
            prop_assert_eq!(CheckedF64(a).cosh().get(), Err(FloatError));
        }

        // Inverse Hyperbolic Cosine Functions
        #[test]
        fn test_acosh_valid(a in valid_f64()) {
            prop_assert_eq!(CheckedF64(a).acosh().get(), match (a.acosh().is_finite(), a) {
                (true, a) if a >= 1.0 => Ok(a.acosh()),
                _ => Err(FloatError)
            });
        }

        #[test]
        fn test_acosh_invalid(a in invalid_f64()) {
            prop_assert_eq!(CheckedF64(a).acosh().get(), Err(FloatError));
        }

        // Tangent Functions
        #[test]
        fn test_tan_valid(a in valid_f64()) {
            prop_assert_eq!(CheckedF64(a).tan().get(), match (a.tan().is_finite(), a.tan()) {
                (true, value) => Ok(value),
                _ => Err(FloatError),
            });
        }

        #[test]
        fn test_tan_invalid(a in invalid_f64()) {
            prop_assert_eq!(CheckedF64(a).tan().get(), Err(FloatError));
        }

        // Inverse Tangent Functions
        #[test]
        fn test_atan_valid(a in valid_f64()) {
            prop_assert_eq!(CheckedF64(a).atan().get(), match (a.atan().is_finite(), a.atan()) {
                (true, value) => Ok(value),
                _ => Err(FloatError),
            });
        }

        #[test]
        fn test_atan_invalid(a in invalid_f64()) {
            prop_assert_eq!(CheckedF64(a).atan().get(), Err(FloatError));
        }

        // Hyperbolic Tangent Functions
        #[test]
        fn test_tanh_valid(a in valid_f64()) {
            prop_assert_eq!(CheckedF64(a).tanh().get(), match (a.tanh().is_finite(), a.tanh()) {
                (true, value) => Ok(value),
                _ => Err(FloatError),
            });
        }

        #[test]
        fn test_tanh_invalid(a in invalid_f64()) {
            prop_assert_eq!(CheckedF64(a).tanh().get(), Err(FloatError));
        }

        // Inverse Hyperbolic Tangent Functions
        #[test]
        fn test_atanh_valid(a in valid_f64()) {
            prop_assert_eq!(CheckedF64(a).atanh().get(), match (a > -1.0 && a < 1.0 && a.atanh().is_finite(), a.atanh()) {
                (true, value) => Ok(value),
                _ => Err(FloatError),
            });
        }

        #[test]
        fn test_atanh_invalid(a in invalid_f64()) {
            prop_assert_eq!(CheckedF64(a).atanh().get(), Err(FloatError));
        }

        // Arctan2 Functions
        #[test]
        fn test_atan2_valid(a in valid_f64(), b in valid_f64()) {
            if b != 0.0 && (a.atan2(b).is_finite()) {
                prop_assert_eq!(CheckedF64(a).atan2(CheckedF64(b)).get(), Ok(a.atan2(b)));
                prop_assert_eq!(CheckedF64(a).atan2(b).get(), Ok(a.atan2(b)));
            } else {
                prop_assert_eq!(CheckedF64(a).atan2(CheckedF64(b)).get(), Err(FloatError));
                prop_assert_eq!(CheckedF64(a).atan2(b).get(), Err(FloatError));
            }
        }

        #[test]
        fn test_atan2_invalid(a in valid_f64(), b in invalid_f64()) {
            prop_assert_eq!(CheckedF64(a).atan2(CheckedF64(b)).get(), Err(FloatError));
            prop_assert_eq!(CheckedF64(a).atan2(b).get(), Err(FloatError));
        }

        #[test]
        fn test_atan2_invalid_base(a in invalid_f64(), b in valid_f64()) {
            prop_assert_eq!(CheckedF64(a).atan2(CheckedF64(b)).get(), Err(FloatError));
            prop_assert_eq!(CheckedF64(a).atan2(b).get(), Err(FloatError));
        }

        #[test]
        fn test_atan2_invalid_both(a in invalid_f64(), b in invalid_f64()) {
            prop_assert_eq!(CheckedF64(a).atan2(CheckedF64(b)).get(), Err(FloatError));
            prop_assert_eq!(CheckedF64(a).atan2(b).get(), Err(FloatError));
        }

        // Sin & Cos Functions
        #[test]
        fn test_sin_cos_valid(a in valid_f64()) {
            if a.sin().is_finite() && a.cos().is_finite() {
                let (sin_val, cos_val) = CheckedF64(a).sin_cos();
                prop_assert_eq!(sin_val.get(), Ok(a.sin()));
                prop_assert_eq!(cos_val.get(), Ok(a.cos()));
            } else {
                let (sin_val, cos_val) = CheckedF64(a).sin_cos();
                prop_assert_eq!(sin_val.get(), Err(FloatError));
                prop_assert_eq!(cos_val.get(), Err(FloatError));
            }
        }

        #[test]
        fn test_sin_cos_invalid(a in invalid_f64()) {
            let (sin_val, cos_val) = CheckedF64(a).sin_cos();
            prop_assert_eq!(sin_val.get(), Err(FloatError));
            prop_assert_eq!(cos_val.get(), Err(FloatError));
        }
    }
}
