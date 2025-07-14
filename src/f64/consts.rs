use super::{GuardedF64, UnguardedF64};

use crate::copy_const_value;

copy_const_value!(
    (GuardedF64, UnguardedF64)
    r"
        The radix or base of the internal representation of `f64`.

        See: [`f64::RADIX`]
    "
    RADIX: u32 = f64::RADIX
);

copy_const_value!(
    (GuardedF64, UnguardedF64)
    r"
        Number of significant digits in base 2.

        See: [`f64::MANTISSA_DIGITS`].
    "
    MANTISSA_DIGITS: u32 = f64::MANTISSA_DIGITS
);

copy_const_value!(
    (GuardedF64, UnguardedF64)
    r"
        Approximate number of significant digits in base 10.

        See: [`f64::DIGITS`].
    "
    DIGITS: u32 = f64::DIGITS
);

copy_const_value!(
    (GuardedF64, UnguardedF64)
    r"
        The difference between `1.0` and the next larger representable number. Equal to
        2<sup>1&nbsp;&minus;&nbsp;[`MANTISSA_DIGITS`]</sup>.

        See: [`f64::EPSILON`]

        [`MANTISSA_DIGITS`]: [`Self::MANTISSA_DIGITS`]
    "
    EPSILON: GuardedF64 = GuardedF64(f64::EPSILON)
);

copy_const_value!(
    (GuardedF64, UnguardedF64)
    r"
        Smallest finite `f64` value.

        See: [`f64::MIN`]
    "
    MIN: GuardedF64 = GuardedF64(f64::MIN)
);

copy_const_value!(
    (GuardedF64, UnguardedF64)
    r"
        Smallest positive normal `f64` value.

        See: [`f64::MIN_POSITIVE`]
    "
    MIN_POSITIVE: GuardedF64 = GuardedF64(f64::MIN_POSITIVE)
);

copy_const_value!(
    (GuardedF64, UnguardedF64)
    r"
        Largest finite `f64` value.

        See: [`f64::MAX`]
    "
    MAX: GuardedF64 = GuardedF64(f64::MAX)
);

copy_const_value!(
    (GuardedF64, UnguardedF64)
    r"
        Minimum possible normal power of 2 exponent.

        See: [`f64::MIN_EXP`]
    "
    MIN_EXP: i32 = f64::MIN_EXP
);

copy_const_value!(
    (GuardedF64, UnguardedF64)
    r"
        Maximum possible normal power of 2 exponent.

        See: [`f64::MAX_EXP`]
    "
    MAX_EXP: i32 = f64::MAX_EXP
);

copy_const_value!(
    (GuardedF64, UnguardedF64)
    r"
        Minimum possible normal power of 10 exponent.

        See: [`f64::MIN_10_EXP`]
    "
    MIN_10_EXP: i32 = f64::MIN_10_EXP
);

copy_const_value!(
    (GuardedF64, UnguardedF64)
    r"
        Maximum possible normal power of 10 exponent.

        See: [`f64::MAX_10_EXP`]
    "
    MAX_10_EXP: i32 = f64::MAX_10_EXP
);

copy_const_value!(
    (GuardedF64, UnguardedF64)
    r"
        Archimedes' constant (&pi;)

        See: [`std::f64::consts::PI`]
    "
    PI: GuardedF64 = GuardedF64(std::f64::consts::PI)
);

copy_const_value!(
    (GuardedF64, UnguardedF64)
    r"
        The full circle constant (&tau; = 2&pi;)

        See: [`std::f64::consts::TAU`]
    "
    TAU: GuardedF64 = GuardedF64(std::f64::consts::TAU)
);

copy_const_value!(
    (GuardedF64, UnguardedF64)
    r"
        &pi;/2

        See: [`std::f64::consts::FRAC_PI_2`]
    "
    FRAC_PI_2: GuardedF64 = GuardedF64(std::f64::consts::FRAC_PI_2)
);

copy_const_value!(
    (GuardedF64, UnguardedF64)
    r"
        &pi;/3

        See: [`std::f64::consts::FRAC_PI_3`]
    "
    FRAC_PI_3: GuardedF64 = GuardedF64(std::f64::consts::FRAC_PI_3)
);

copy_const_value!(
    (GuardedF64, UnguardedF64)
    r"
        &pi;/4

        See: [`std::f64::consts::FRAC_PI_4`]
    "
    FRAC_PI_4: GuardedF64 = GuardedF64(std::f64::consts::FRAC_PI_4)
);

copy_const_value!(
    (GuardedF64, UnguardedF64)
    r"
        &pi;/6

        See: [`std::f64::consts::FRAC_PI_6`]
    "
    FRAC_PI_6: GuardedF64 = GuardedF64(std::f64::consts::FRAC_PI_6)
);

copy_const_value!(
    (GuardedF64, UnguardedF64)
    r"
        &pi;/8

        See: [`std::f64::consts::FRAC_PI_8`]
    "
    FRAC_PI_8: GuardedF64 = GuardedF64(std::f64::consts::FRAC_PI_8)
);

copy_const_value!(
    (GuardedF64, UnguardedF64)
    r"
        1/&pi;

        See: [`std::f64::consts::FRAC_1_PI`]
    "
    FRAC_1_PI: GuardedF64 = GuardedF64(std::f64::consts::FRAC_1_PI)
);

copy_const_value!(
    (GuardedF64, UnguardedF64)
    r"
        2/&pi;

        See: [`std::f64::consts::FRAC_2_PI`]
    "
    FRAC_2_PI: GuardedF64 = GuardedF64(std::f64::consts::FRAC_2_PI)
);

copy_const_value!(
    (GuardedF64, UnguardedF64)
    r"
        1/√&pi;

        See: [`std::f64::consts::FRAC_2_SQRT_PI`]
    "
    FRAC_2_SQRT_PI: GuardedF64 = GuardedF64(std::f64::consts::FRAC_2_SQRT_PI)
);

copy_const_value!(
    (GuardedF64, UnguardedF64)
    r"
        √2

        See: [`std::f64::consts::SQRT_2`]
    "
    SQRT_2: GuardedF64 = GuardedF64(std::f64::consts::SQRT_2)
);

copy_const_value!(
    (GuardedF64, UnguardedF64)
    r"
        1/√2

        See: [`std::f64::consts::FRAC_1_SQRT_2`]
    "
    FRAC_1_SQRT_2: GuardedF64 = GuardedF64(std::f64::consts::FRAC_1_SQRT_2)
);

copy_const_value!(
    (GuardedF64, UnguardedF64)
    r"
        Euler's number (e)

        See: [`std::f64::consts::E`]
    "
    E: GuardedF64 = GuardedF64(std::f64::consts::E)
);

copy_const_value!(
    (GuardedF64, UnguardedF64)
    r"
        log<sub>2</sub>(e)

        See: [`std::f64::consts::LOG2_E`]
    "
    LOG2_E: GuardedF64 = GuardedF64(std::f64::consts::LOG2_E)
);

copy_const_value!(
    (GuardedF64, UnguardedF64)
    r"
        log<sub>2</sub>(10)

        See: [`std::f64::consts::LOG2_10`]
    "
    LOG2_10: GuardedF64 = GuardedF64(std::f64::consts::LOG2_10)
);

copy_const_value!(
    (GuardedF64, UnguardedF64)
    r"
        log<sub>10</sub>(2)

        See: [`std::f64::consts::LOG10_2`]
    "
    LOG10_2: GuardedF64 = GuardedF64(std::f64::consts::LOG10_2)
);
copy_const_value!(
    (GuardedF64, UnguardedF64)
    r"
        log<sub>10</sub>(e)

        See: [`std::f64::consts::LOG10_E`]
    "
    LOG10_E: GuardedF64 = GuardedF64(std::f64::consts::LOG10_E)
);

copy_const_value!(
    (GuardedF64, UnguardedF64)
    r"
        ln(2)

        See: [`std::f64::consts::LN_2`]
    "
    LN_2: GuardedF64 = GuardedF64(std::f64::consts::LN_2)
);

copy_const_value!(
    (GuardedF64, UnguardedF64)
    r"
        ln(10)

        See: [`std::f64::consts::LN_10`]
    "
    LN_10: GuardedF64 = GuardedF64(std::f64::consts::LN_10)
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
    fn test_radix() {
        assert_eq!(GuardedF64::RADIX, f64::RADIX);
        assert_type_eq!(GuardedF64::RADIX, u32);

        assert_eq!(UnguardedF64::RADIX, f64::RADIX);
        assert_type_eq!(UnguardedF64::RADIX, u32);
    }

    #[test]
    fn test_digits() {
        assert_eq!(GuardedF64::DIGITS, f64::DIGITS);
        assert_type_eq!(GuardedF64::DIGITS, u32);

        assert_eq!(UnguardedF64::DIGITS, f64::DIGITS);
        assert_type_eq!(UnguardedF64::DIGITS, u32);
    }

    #[test]
    fn test_mantissa_digits() {
        assert_eq!(GuardedF64::MANTISSA_DIGITS, f64::MANTISSA_DIGITS);
        assert_type_eq!(GuardedF64::MANTISSA_DIGITS, u32);

        assert_eq!(UnguardedF64::MANTISSA_DIGITS, f64::MANTISSA_DIGITS);
        assert_type_eq!(UnguardedF64::MANTISSA_DIGITS, u32);
    }

    #[test]
    fn test_epsilon() {
        assert_eq!(GuardedF64::EPSILON, f64::EPSILON);
        assert_type_eq!(GuardedF64::EPSILON, GuardedF64);

        assert_eq!(UnguardedF64::EPSILON, f64::EPSILON);
        assert_type_eq!(UnguardedF64::EPSILON, GuardedF64);
    }

    #[test]
    fn test_min() {
        assert_eq!(GuardedF64::MIN, f64::MIN);
        assert_type_eq!(GuardedF64::MIN, GuardedF64);

        assert_eq!(UnguardedF64::MIN, f64::MIN);
        assert_type_eq!(UnguardedF64::MIN, GuardedF64);
    }

    #[test]
    fn test_min_positive() {
        assert_eq!(GuardedF64::MIN_POSITIVE, f64::MIN_POSITIVE);
        assert_type_eq!(GuardedF64::MIN_POSITIVE, GuardedF64);

        assert_eq!(UnguardedF64::MIN_POSITIVE, f64::MIN_POSITIVE);
        assert_type_eq!(UnguardedF64::MIN_POSITIVE, GuardedF64);
    }

    #[test]
    fn test_max() {
        assert_eq!(GuardedF64::MAX, f64::MAX);
        assert_type_eq!(GuardedF64::MAX, GuardedF64);

        assert_eq!(UnguardedF64::MAX, f64::MAX);
        assert_type_eq!(UnguardedF64::MAX, GuardedF64);
    }

    #[test]
    fn test_min_exp() {
        assert_eq!(GuardedF64::MIN_EXP, f64::MIN_EXP);
        assert_type_eq!(GuardedF64::MIN_EXP, i32);

        assert_eq!(UnguardedF64::MIN_EXP, f64::MIN_EXP);
        assert_type_eq!(UnguardedF64::MIN_EXP, i32);
    }

    #[test]
    fn test_max_exp() {
        assert_eq!(GuardedF64::MAX_EXP, f64::MAX_EXP);
        assert_type_eq!(GuardedF64::MAX_EXP, i32);

        assert_eq!(UnguardedF64::MAX_EXP, f64::MAX_EXP);
        assert_type_eq!(UnguardedF64::MAX_EXP, i32);
    }

    #[test]
    fn test_min_10_exp() {
        assert_eq!(GuardedF64::MIN_10_EXP, f64::MIN_10_EXP);
        assert_type_eq!(GuardedF64::MIN_10_EXP, i32);

        assert_eq!(UnguardedF64::MIN_10_EXP, f64::MIN_10_EXP);
        assert_type_eq!(UnguardedF64::MIN_10_EXP, i32);
    }

    #[test]
    fn test_max_10_exp() {
        assert_eq!(GuardedF64::MAX_10_EXP, f64::MAX_10_EXP);
        assert_type_eq!(GuardedF64::MAX_10_EXP, i32);

        assert_eq!(UnguardedF64::MAX_10_EXP, f64::MAX_10_EXP);
        assert_type_eq!(UnguardedF64::MAX_10_EXP, i32);
    }
}
