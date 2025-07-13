use super::UnguardedF64;

macro_rules! assign_operation {
    (
        $AssignTrait:ident :: $assign_op:ident,
        $OpTrait:ident :: $op:ident,
        $doc:literal $(,)?
    ) => {
        impl<T> std::ops::$AssignTrait<T> for UnguardedF64
        where
            T: Into<Self>,
        {
            #[doc = $doc]
            #[allow(clippy::inline_always)]
            #[inline(always)]
            fn $assign_op(&mut self, rhs: T) {
                use std::ops::$OpTrait;
                *self = self.$op(rhs.into());
            }
        }
    };
}

assign_operation!(
    AddAssign::add_assign,
    Add::add,
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
    SubAssign::sub_assign,
    Sub::sub,
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
    MulAssign::mul_assign,
    Mul::mul,
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
    DivAssign::div_assign,
    Div::div,
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
    RemAssign::rem_assign,
    Rem::rem,
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
