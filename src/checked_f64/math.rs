use crate::{CheckedF64Result, CheckedF64, FloatError};

macro_rules! const_math {
    ($name:ident, $doc:expr) => {
        impl CheckedF64 {
            #[doc = $doc]
            #[inline(always)]
            pub const fn $name(self) -> CheckedF64Result {
                match self.0.$name() {
                    result if result.is_finite() => Ok(Self(result)),
                    _ => Err(FloatError),
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
            prop_assert_eq!(CheckedF64::new(a).abs(), Ok(CheckedF64::new(a.abs())));
        }

        #[test]
        fn test_abs_invalid(a in invalid_f64()) {
            prop_assert_eq!(CheckedF64::new(a).abs(), Err(FloatError));
        }
    }
}
