use super::UnguardedF64;
use crate::macros::ops_assign::assign_operation;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

assign_operation!(
    use Add::add impl AddAssign::add_assign for ...(UnguardedF64)
    r"
        Assigns the result of adding another `UnguardedF64` to this one.

        ## Example

        ```rust
        use floatguard::{GuardedF64, UnguardedF64};

        let mut a = UnguardedF64::from(1.0);
        let b = UnguardedF64::from(2.0);
        a += b;
        assert_eq!(a.check(), GuardedF64::new(3.0));
        ```

        ```rust
        use floatguard::{GuardedF64, UnguardedF64};

        let mut a = UnguardedF64::from(1.0);
        let b = 2.0;
        a += b;
        assert_eq!(a.check(), GuardedF64::new(3.0));
        ```

        ```rust
        use floatguard::{GuardedF64, UnguardedF64};

        let mut a = UnguardedF64::from(1.0);
        let b = GuardedF64::new(2.0).unwrap();
        a += b;
        assert_eq!(a.check(), GuardedF64::new(3.0));
        ```
    "
);

assign_operation!(
    use Sub::sub impl SubAssign::sub_assign for ...(UnguardedF64)
    r"
        Assigns the result of subtracting another `UnguardedF64` from this one.

        ## Example

        ```rust
        use floatguard::{GuardedF64, UnguardedF64};

        let mut a = UnguardedF64::from(3.0);
        let b = UnguardedF64::from(2.0);
        a -= b;
        assert_eq!(a.check(), GuardedF64::new(1.0));
        ```

        ```rust
        use floatguard::{GuardedF64, UnguardedF64};

        let mut a = UnguardedF64::from(3.0);
        let b = 2.0;
        a -= b;
        assert_eq!(a.check(), GuardedF64::new(1.0));
        ```

        ```rust
        use floatguard::{GuardedF64, UnguardedF64};

        let mut a = UnguardedF64::from(3.0);
        let b = GuardedF64::new(2.0).unwrap();
        a -= b;
        assert_eq!(a.check(), GuardedF64::new(1.0));
        ```
    "
);

assign_operation!(
    use Mul::mul impl MulAssign::mul_assign for ...(UnguardedF64)
    r"
        Assigns the result of multiplying this `UnguardedF64` by another.

        ## Example

        ```rust
        use floatguard::{GuardedF64, UnguardedF64};

        let mut a = UnguardedF64::from(2.0);
        let b = UnguardedF64::from(3.0);
        a *= b;
        assert_eq!(a.check(), GuardedF64::new(6.0));
        ```

        ```rust
        use floatguard::{GuardedF64, UnguardedF64};

        let mut a = UnguardedF64::from(2.0);
        let b = 3.0;
        a *= b;
        assert_eq!(a.check(), GuardedF64::new(6.0));
        ```

        ```rust
        use floatguard::{GuardedF64, UnguardedF64};

        let mut a = UnguardedF64::from(2.0);
        let b = GuardedF64::new(3.0).unwrap();
        a *= b;
        assert_eq!(a.check(), GuardedF64::new(6.0));
        ```
    "
);

assign_operation!(
    use Div::div impl DivAssign::div_assign for ...(UnguardedF64)
    r"
        Assigns the result of dividing this `UnguardedF64` by another.

        ## Example

        ```rust
        use floatguard::{GuardedF64, UnguardedF64};

        let mut a = UnguardedF64::from(6.0);
        let b = UnguardedF64::from(3.0);
        a /= b;
        assert_eq!(a.check(), GuardedF64::new(2.0));
        ```

        ```rust
        use floatguard::{GuardedF64, UnguardedF64};

        let mut a = UnguardedF64::from(6.0);
        let b = 3.0;
        a /= b;
        assert_eq!(a.check(), GuardedF64::new(2.0));
        ```

        ```rust
        use floatguard::{GuardedF64, UnguardedF64};

        let mut a = UnguardedF64::from(6.0);
        let b = GuardedF64::new(3.0).unwrap();
        a /= b;
        assert_eq!(a.check(), GuardedF64::new(2.0));
        ```
    "
);

assign_operation!(
    use Rem::rem impl RemAssign::rem_assign for ...(UnguardedF64)
    r"
        Assigns the result of taking the remainder of this `UnguardedF64` divided by another.

        ## Example

        ```rust
        use floatguard::{GuardedF64, UnguardedF64};

        let mut a = UnguardedF64::from(5.0);
        let b = UnguardedF64::from(2.0);
        a %= b;
        assert_eq!(a.check(), GuardedF64::new(1.0));
        ```

        ```rust
        use floatguard::{GuardedF64, UnguardedF64};

        let mut a = UnguardedF64::from(5.0);
        let b = 2.0;
        a %= b;
        assert_eq!(a.check(), GuardedF64::new(1.0));
        ```

        ```rust
        use floatguard::{GuardedF64, UnguardedF64};

        let mut a = UnguardedF64::from(5.0);
        let b = GuardedF64::new(2.0).unwrap();
        a %= b;
        assert_eq!(a.check(), GuardedF64::new(1.0));
        ```
    "
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GuardedF64;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_add_assign(a in any::<f64>(), b in any::<f64>()) {
            let mut unchecked_a = UnguardedF64::new(a);
            unchecked_a += b;
            prop_assert_eq!(unchecked_a.check(), GuardedF64::new(a + b));
        }

        #[test]
        fn test_sub_assign(a in any::<f64>(), b in any::<f64>()) {
            let mut unchecked_a = UnguardedF64::new(a);
            unchecked_a -= b;
            prop_assert_eq!(unchecked_a.check(), GuardedF64::new(a - b));
        }

        #[test]
        fn test_mul_assign(a in any::<f64>(), b in any::<f64>()) {
            let mut unchecked_a = UnguardedF64::new(a);
            unchecked_a *= b;
            prop_assert_eq!(unchecked_a.check(), GuardedF64::new(a * b));
        }

        #[test]
        fn test_div_assign(a in any::<f64>(), b in any::<f64>().prop_filter("b != 0", |&b| b != 0.0)) {
            let mut unchecked_a = UnguardedF64::new(a);
            unchecked_a /= b;
            prop_assert_eq!(unchecked_a.check(), GuardedF64::new(a / b));
        }

        #[test]
        fn test_rem_assign(a in any::<f64>(), b in any::<f64>().prop_filter("b != 0", |&b| b != 0.0)) {
            let mut unchecked_a = UnguardedF64::new(a);
            unchecked_a %= b;
            prop_assert_eq!(unchecked_a.check(), GuardedF64::new(a % b));
        }
    }
}
