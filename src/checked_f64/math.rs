use crate::{CheckedF64Result, CheckedF64, FloatError};

macro_rules! const_math {
    ($name:ident, $doc:expr) => {
        impl CheckedF64 {
            #[doc = $doc]
            #[must_use = "method returns a new instance and does not mutate the original value"]
            #[inline(always)]
            pub const fn $name(self) -> CheckedF64Result {
                CheckedF64Result::new(match self.0.$name() {
                    result if result.is_finite() => Ok(Self(result)),
                    _ => Err(FloatError),
                })
            }
        }

        impl CheckedF64Result {
            #[doc = $doc]
            #[must_use = "method returns a new instance and does not mutate the original value"]
            #[inline(always)]
            pub const fn $name(self) -> CheckedF64Result {
                match self.as_inner() {
                    Ok(value) => value.$name(),
                    Err(err) => CheckedF64Result::new(Err(*err)),
                }
            }
        }
    };
}

macro_rules! math {
    ($name:ident, $doc:expr) => {
        impl CheckedF64 {
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
        }
    };

    ($name:ident, $operand:ident, $t:tt, $doc:expr) => {
        impl CheckedF64 {
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

#[cfg(test)]
mod tests {
    use crate::{
        CheckedF64, FloatError,
        checked_f64::tests::{invalid_f64, valid_f64},
    };
    use proptest::prelude::*;

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
    }
}
