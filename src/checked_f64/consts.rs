use super::CheckedF64;

macro_rules! copy_const_value {
    ($name:ident, f64, $doc:expr) => {
        copy_const_value!($name, Self, Self(f64::$name), $doc);
    };

    ($name:ident, $t:tt, $doc:expr) => {
        copy_const_value!($name, $t, f64::$name, $doc);
    };

    ($name:ident, $t:tt, $value:expr, $doc:expr) => {
        impl CheckedF64 {
            #[doc = $doc]
            pub const $name: $t = $value;
        }
    };
}

copy_const_value!(
    RADIX,
    u32,
    r"
        The radix or base of the internal representation of `f64`.

        See: [`f64::RADIX`].
    "
);

copy_const_value!(
    MANTISSA_DIGITS,
    u32,
    r"
        Number of significant digits in base 2.

        See: [`f64::MANTISSA_DIGITS`].
    "
);

copy_const_value!(
    DIGITS,
    u32,
    r"
        Approximate number of significant digits in base 10.

        See: [`f64::DIGITS`].
    "
);

copy_const_value!(
    EPSILON,
    f64,
    r"
        The difference between `1.0` and the next larger representable number. Equal to
        2<sup>1&nbsp;&minus;&nbsp;[`MANTISSA_DIGITS`]</sup>.

        See: [`f64::EPSILON`]

        [`MANTISSA_DIGITS`]: [`Self::MANTISSA_DIGITS`]
    "
);

copy_const_value!(
    MIN,
    f64,
    r"
        Smallest finite `f64` value.

        See: [`f64::MIN`]
    "
);

copy_const_value!(
    MIN_POSITIVE,
    f64,
    r"
        Smallest positive normal `f64` value.

        See: [`f64::MIN_POSITIVE`]
    "
);

copy_const_value!(
    MAX,
    f64,
    r"
        Largest finite `f64` value.

        See: [`f64::MAX`]
    "
);

copy_const_value!(
    MIN_EXP,
    i32,
    r"
        Minimum possible normal power of 2 exponent.

        See: [`f64::MIN_EXP`]
    "
);

copy_const_value!(
    MAX_EXP,
    i32,
    r"
        Maximum possible normal power of 2 exponent.

        See: [`f64::MAX_EXP`]
    "
);

copy_const_value!(
    MIN_10_EXP,
    i32,
    r"
        Minimum possible normal power of 10 exponent.

        See: [`f64::MIN_10_EXP`]
    "
);

copy_const_value!(
    MAX_10_EXP,
    i32,
    r"
        Maximum possible normal power of 10 exponent.

        See: [`f64::MAX_10_EXP`]
    "
);

copy_const_value!(
    PI,
    f64,
    std::f64::consts::PI,
    r"
        Archimedes' constant (&pi;)

        See: [`std::f64::consts::PI`]
    "
);

copy_const_value!(
    TAU,
    f64,
    std::f64::consts::TAU,
    r"
        The full circle constant (&tau; = 2&pi;)

        See: [`std::f64::consts::TAU`]
    "
);

copy_const_value!(
    FRAC_PI_2,
    f64,
    std::f64::consts::FRAC_PI_2,
    r"
        &pi;/2

        See: [`std::f64::consts::FRAC_PI_2`]
    "
);

copy_const_value!(
    FRAC_PI_3,
    f64,
    std::f64::consts::FRAC_PI_3,
    r"
        &pi;/3

        See: [`std::f64::consts::FRAC_PI_3`]
    "
);

copy_const_value!(
    FRAC_PI_4,
    f64,
    std::f64::consts::FRAC_PI_4,
    r"
        &pi;/4

        See: [`std::f64::consts::FRAC_PI_4`]
    "
);

copy_const_value!(
    FRAC_PI_6,
    f64,
    std::f64::consts::FRAC_PI_6,
    r"
        &pi;/6

        See: [`std::f64::consts::FRAC_PI_6`]
    "
);

copy_const_value!(
    FRAC_PI_8,
    f64,
    std::f64::consts::FRAC_PI_8,
    r"
        &pi;/8

        See: [`std::f64::consts::FRAC_PI_8`]
    "
);

copy_const_value!(
    FRAC_1_PI,
    f64,
    std::f64::consts::FRAC_1_PI,
    r"
        1/&pi;

        See: [`std::f64::consts::FRAC_1_PI`]
    "
);

copy_const_value!(
    FRAC_2_PI,
    f64,
    std::f64::consts::FRAC_2_PI,
    r"
        2/&pi;

        See: [`std::f64::consts::FRAC_2_PI`]
    "
);

copy_const_value!(
    FRAC_2_SQRT_PI,
    f64,
    std::f64::consts::FRAC_2_SQRT_PI,
    r"
        1/√&pi;

        See: [`std::f64::consts::FRAC_2_SQRT_PI`]
    "
);

copy_const_value!(
    SQRT_2,
    f64,
    std::f64::consts::SQRT_2,
    r"
        √2

        See: [`std::f64::consts::SQRT_2`]
    "
);

copy_const_value!(
    FRAC_1_SQRT_2,
    f64,
    std::f64::consts::FRAC_1_SQRT_2,
    r"
        1/√2

        See: [`std::f64::consts::FRAC_1_SQRT_2`]
    "
);

copy_const_value!(
    E,
    f64,
    std::f64::consts::E,
    r"
        Euler's number (e)

        See: [`std::f64::consts::E`]
    "
);

copy_const_value!(
    LOG2_E,
    f64,
    std::f64::consts::LOG2_E,
    r"
        log<sub>2</sub>(e)

        See: [`std::f64::consts::LOG2_E`]
    "
);

copy_const_value!(
    LOG2_10,
    f64,
    std::f64::consts::LOG2_10,
    r"
        log<sub>2</sub>(10)

        See: [`std::f64::consts::LOG2_10`]
    "
);

copy_const_value!(
    LOG10_2,
    f64,
    std::f64::consts::LOG10_2,
    r"
        log<sub>10</sub>(2)

        See: [`std::f64::consts::LOG10_2`]
    "
);
copy_const_value!(
    LOG10_E,
    f64,
    std::f64::consts::LOG10_E,
    r"
        log<sub>10</sub>(e)

        See: [`std::f64::consts::LOG10_E`]
    "
);

copy_const_value!(
    LN_2,
    f64,
    std::f64::consts::LN_2,
    r"
        ln(2)

        See: [`std::f64::consts::LN_2`]
    "
);

copy_const_value!(
    LN_10,
    f64,
    std::f64::consts::LN_10,
    r"
        ln(10)

        See: [`std::f64::consts::LN_10`]
    "
);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_type_eq {
        ($value:expr, $t:ty) => {
            assert_eq!(
                std::any::type_name_of_val(&$value),
                std::any::type_name::<$t>()
            );
        };
    }

    #[test]
    fn test_digits() {
        assert_eq!(CheckedF64::DIGITS, f64::DIGITS);
        assert_type_eq!(CheckedF64::DIGITS, u32);
    }

    #[test]
    fn test_mantissa_digits() {
        assert_eq!(CheckedF64::MANTISSA_DIGITS, f64::MANTISSA_DIGITS);
        assert_type_eq!(CheckedF64::MANTISSA_DIGITS, u32);
    }

    #[test]
    fn test_min() {
        assert_eq!(CheckedF64::MIN, f64::MIN);
        assert_type_eq!(CheckedF64::MIN, CheckedF64);
    }

    #[test]
    fn test_min_positive() {
        assert_eq!(CheckedF64::MIN_POSITIVE, f64::MIN_POSITIVE);
        assert_type_eq!(CheckedF64::MIN_POSITIVE, CheckedF64);
    }

    #[test]
    fn test_max() {
        assert_eq!(CheckedF64::MAX, f64::MAX);
        assert_type_eq!(CheckedF64::MAX, CheckedF64);
    }

    #[test]
    fn test_min_exp() {
        assert_eq!(CheckedF64::MIN_EXP, f64::MIN_EXP);
        assert_type_eq!(CheckedF64::MIN_EXP, i32);
    }

    #[test]
    fn test_max_exp() {
        assert_eq!(CheckedF64::MAX_EXP, f64::MAX_EXP);
        assert_type_eq!(CheckedF64::MAX_EXP, i32);
    }

    #[test]
    fn test_min_10_exp() {
        assert_eq!(CheckedF64::MIN_10_EXP, f64::MIN_10_EXP);
        assert_type_eq!(CheckedF64::MIN_10_EXP, i32);
    }

    #[test]
    fn test_max_10_exp() {
        assert_eq!(CheckedF64::MAX_10_EXP, f64::MAX_10_EXP);
        assert_type_eq!(CheckedF64::MAX_10_EXP, i32);
    }
}
