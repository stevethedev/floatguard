use super::{GuardedF32, UnguardedF32};

use crate::macros::copy_const_value;

copy_const_value!(
    (GuardedF32, UnguardedF32)
    r"
        The radix or base of the internal representation of `f32`.

        See: [`f32::RADIX`]
    "
    RADIX: u32 = f32::RADIX
);

copy_const_value!(
    (GuardedF32, UnguardedF32)
    r"
        Number of significant digits in base 2.

        See: [`f32::MANTISSA_DIGITS`].
    "
    MANTISSA_DIGITS: u32 = f32::MANTISSA_DIGITS
);

copy_const_value!(
    (GuardedF32, UnguardedF32)
    r"
        Approximate number of significant digits in base 10.

        See: [`f32::DIGITS`].
    "
    DIGITS: u32 = f32::DIGITS
);

copy_const_value!(
    (GuardedF32, UnguardedF32)
    r"
        The difference between `1.0` and the next larger representable number. Equal to
        2<sup>1&nbsp;&minus;&nbsp;[`MANTISSA_DIGITS`]</sup>.

        See: [`f32::EPSILON`]

        [`MANTISSA_DIGITS`]: [`Self::MANTISSA_DIGITS`]
    "
    EPSILON: GuardedF32 = GuardedF32(f32::EPSILON)
);

copy_const_value!(
    (GuardedF32, UnguardedF32)
    r"
        Smallest finite `f32` value.

        See: [`f32::MIN`]
    "
    MIN: GuardedF32 = GuardedF32(f32::MIN)
);

copy_const_value!(
    (GuardedF32, UnguardedF32)
    r"
        Smallest positive normal `f32` value.

        See: [`f32::MIN_POSITIVE`]
    "
    MIN_POSITIVE: GuardedF32 = GuardedF32(f32::MIN_POSITIVE)
);

copy_const_value!(
    (GuardedF32, UnguardedF32)
    r"
        Largest finite `f32` value.

        See: [`f32::MAX`]
    "
    MAX: GuardedF32 = GuardedF32(f32::MAX)
);

copy_const_value!(
    (GuardedF32, UnguardedF32)
    r"
        Minimum possible normal power of 2 exponent.

        See: [`f32::MIN_EXP`]
    "
    MIN_EXP: i32 = f32::MIN_EXP
);

copy_const_value!(
    (GuardedF32, UnguardedF32)
    r"
        Maximum possible normal power of 2 exponent.

        See: [`f32::MAX_EXP`]
    "
    MAX_EXP: i32 = f32::MAX_EXP
);

copy_const_value!(
    (GuardedF32, UnguardedF32)
    r"
        Minimum possible normal power of 10 exponent.

        See: [`f32::MIN_10_EXP`]
    "
    MIN_10_EXP: i32 = f32::MIN_10_EXP
);

copy_const_value!(
    (GuardedF32, UnguardedF32)
    r"
        Maximum possible normal power of 10 exponent.

        See: [`f32::MAX_10_EXP`]
    "
    MAX_10_EXP: i32 = f32::MAX_10_EXP
);

copy_const_value!(
    (GuardedF32, UnguardedF32)
    r"
        Archimedes' constant (&pi;)

        See: [`std::f32::consts::PI`]
    "
    PI: GuardedF32 = GuardedF32(std::f32::consts::PI)
);

copy_const_value!(
    (GuardedF32, UnguardedF32)
    r"
        The full circle constant (&tau; = 2&pi;)

        See: [`std::f32::consts::TAU`]
    "
    TAU: GuardedF32 = GuardedF32(std::f32::consts::TAU)
);

copy_const_value!(
    (GuardedF32, UnguardedF32)
    r"
        &pi;/2

        See: [`std::f32::consts::FRAC_PI_2`]
    "
    FRAC_PI_2: GuardedF32 = GuardedF32(std::f32::consts::FRAC_PI_2)
);

copy_const_value!(
    (GuardedF32, UnguardedF32)
    r"
        &pi;/3

        See: [`std::f32::consts::FRAC_PI_3`]
    "
    FRAC_PI_3: GuardedF32 = GuardedF32(std::f32::consts::FRAC_PI_3)
);

copy_const_value!(
    (GuardedF32, UnguardedF32)
    r"
        &pi;/4

        See: [`std::f32::consts::FRAC_PI_4`]
    "
    FRAC_PI_4: GuardedF32 = GuardedF32(std::f32::consts::FRAC_PI_4)
);

copy_const_value!(
    (GuardedF32, UnguardedF32)
    r"
        &pi;/6

        See: [`std::f32::consts::FRAC_PI_6`]
    "
    FRAC_PI_6: GuardedF32 = GuardedF32(std::f32::consts::FRAC_PI_6)
);

copy_const_value!(
    (GuardedF32, UnguardedF32)
    r"
        &pi;/8

        See: [`std::f32::consts::FRAC_PI_8`]
    "
    FRAC_PI_8: GuardedF32 = GuardedF32(std::f32::consts::FRAC_PI_8)
);

copy_const_value!(
    (GuardedF32, UnguardedF32)
    r"
        1/&pi;

        See: [`std::f32::consts::FRAC_1_PI`]
    "
    FRAC_1_PI: GuardedF32 = GuardedF32(std::f32::consts::FRAC_1_PI)
);

copy_const_value!(
    (GuardedF32, UnguardedF32)
    r"
        2/&pi;

        See: [`std::f32::consts::FRAC_2_PI`]
    "
    FRAC_2_PI: GuardedF32 = GuardedF32(std::f32::consts::FRAC_2_PI)
);

copy_const_value!(
    (GuardedF32, UnguardedF32)
    r"
        1/√&pi;

        See: [`std::f32::consts::FRAC_2_SQRT_PI`]
    "
    FRAC_2_SQRT_PI: GuardedF32 = GuardedF32(std::f32::consts::FRAC_2_SQRT_PI)
);

copy_const_value!(
    (GuardedF32, UnguardedF32)
    r"
        √2

        See: [`std::f32::consts::SQRT_2`]
    "
    SQRT_2: GuardedF32 = GuardedF32(std::f32::consts::SQRT_2)
);

copy_const_value!(
    (GuardedF32, UnguardedF32)
    r"
        1/√2

        See: [`std::f32::consts::FRAC_1_SQRT_2`]
    "
    FRAC_1_SQRT_2: GuardedF32 = GuardedF32(std::f32::consts::FRAC_1_SQRT_2)
);

copy_const_value!(
    (GuardedF32, UnguardedF32)
    r"
        Euler's number (e)

        See: [`std::f32::consts::E`]
    "
    E: GuardedF32 = GuardedF32(std::f32::consts::E)
);

copy_const_value!(
    (GuardedF32, UnguardedF32)
    r"
        log<sub>2</sub>(e)

        See: [`std::f32::consts::LOG2_E`]
    "
    LOG2_E: GuardedF32 = GuardedF32(std::f32::consts::LOG2_E)
);

copy_const_value!(
    (GuardedF32, UnguardedF32)
    r"
        log<sub>2</sub>(10)

        See: [`std::f32::consts::LOG2_10`]
    "
    LOG2_10: GuardedF32 = GuardedF32(std::f32::consts::LOG2_10)
);

copy_const_value!(
    (GuardedF32, UnguardedF32)
    r"
        log<sub>10</sub>(2)

        See: [`std::f32::consts::LOG10_2`]
    "
    LOG10_2: GuardedF32 = GuardedF32(std::f32::consts::LOG10_2)
);
copy_const_value!(
    (GuardedF32, UnguardedF32)
    r"
        log<sub>10</sub>(e)

        See: [`std::f32::consts::LOG10_E`]
    "
    LOG10_E: GuardedF32 = GuardedF32(std::f32::consts::LOG10_E)
);

copy_const_value!(
    (GuardedF32, UnguardedF32)
    r"
        ln(2)

        See: [`std::f32::consts::LN_2`]
    "
    LN_2: GuardedF32 = GuardedF32(std::f32::consts::LN_2)
);

copy_const_value!(
    (GuardedF32, UnguardedF32)
    r"
        ln(10)

        See: [`std::f32::consts::LN_10`]
    "
    LN_10: GuardedF32 = GuardedF32(std::f32::consts::LN_10)
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
        assert_eq!(GuardedF32::RADIX, f32::RADIX);
        assert_type_eq!(GuardedF32::RADIX, u32);

        assert_eq!(UnguardedF32::RADIX, f32::RADIX);
        assert_type_eq!(UnguardedF32::RADIX, u32);
    }

    #[test]
    fn test_digits() {
        assert_eq!(GuardedF32::DIGITS, f32::DIGITS);
        assert_type_eq!(GuardedF32::DIGITS, u32);

        assert_eq!(UnguardedF32::DIGITS, f32::DIGITS);
        assert_type_eq!(UnguardedF32::DIGITS, u32);
    }

    #[test]
    fn test_mantissa_digits() {
        assert_eq!(GuardedF32::MANTISSA_DIGITS, f32::MANTISSA_DIGITS);
        assert_type_eq!(GuardedF32::MANTISSA_DIGITS, u32);

        assert_eq!(UnguardedF32::MANTISSA_DIGITS, f32::MANTISSA_DIGITS);
        assert_type_eq!(UnguardedF32::MANTISSA_DIGITS, u32);
    }

    #[test]
    fn test_epsilon() {
        assert_eq!(GuardedF32::EPSILON, f32::EPSILON);
        assert_type_eq!(GuardedF32::EPSILON, GuardedF32);

        assert_eq!(UnguardedF32::EPSILON, f32::EPSILON);
        assert_type_eq!(UnguardedF32::EPSILON, GuardedF32);
    }

    #[test]
    fn test_min() {
        assert_eq!(GuardedF32::MIN, f32::MIN);
        assert_type_eq!(GuardedF32::MIN, GuardedF32);

        assert_eq!(UnguardedF32::MIN, f32::MIN);
        assert_type_eq!(UnguardedF32::MIN, GuardedF32);
    }

    #[test]
    fn test_min_positive() {
        assert_eq!(GuardedF32::MIN_POSITIVE, f32::MIN_POSITIVE);
        assert_type_eq!(GuardedF32::MIN_POSITIVE, GuardedF32);

        assert_eq!(UnguardedF32::MIN_POSITIVE, f32::MIN_POSITIVE);
        assert_type_eq!(UnguardedF32::MIN_POSITIVE, GuardedF32);
    }

    #[test]
    fn test_max() {
        assert_eq!(GuardedF32::MAX, f32::MAX);
        assert_type_eq!(GuardedF32::MAX, GuardedF32);

        assert_eq!(UnguardedF32::MAX, f32::MAX);
        assert_type_eq!(UnguardedF32::MAX, GuardedF32);
    }

    #[test]
    fn test_min_exp() {
        assert_eq!(GuardedF32::MIN_EXP, f32::MIN_EXP);
        assert_type_eq!(GuardedF32::MIN_EXP, i32);

        assert_eq!(UnguardedF32::MIN_EXP, f32::MIN_EXP);
        assert_type_eq!(UnguardedF32::MIN_EXP, i32);
    }

    #[test]
    fn test_max_exp() {
        assert_eq!(GuardedF32::MAX_EXP, f32::MAX_EXP);
        assert_type_eq!(GuardedF32::MAX_EXP, i32);

        assert_eq!(UnguardedF32::MAX_EXP, f32::MAX_EXP);
        assert_type_eq!(UnguardedF32::MAX_EXP, i32);
    }

    #[test]
    fn test_min_10_exp() {
        assert_eq!(GuardedF32::MIN_10_EXP, f32::MIN_10_EXP);
        assert_type_eq!(GuardedF32::MIN_10_EXP, i32);

        assert_eq!(UnguardedF32::MIN_10_EXP, f32::MIN_10_EXP);
        assert_type_eq!(UnguardedF32::MIN_10_EXP, i32);
    }

    #[test]
    fn test_max_10_exp() {
        assert_eq!(GuardedF32::MAX_10_EXP, f32::MAX_10_EXP);
        assert_type_eq!(GuardedF32::MAX_10_EXP, i32);

        assert_eq!(UnguardedF32::MAX_10_EXP, f32::MAX_10_EXP);
        assert_type_eq!(UnguardedF32::MAX_10_EXP, i32);
    }
}
