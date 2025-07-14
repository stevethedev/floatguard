
/// Defines a binary operation for `GuardedF64` with the specified trait and method.
///
/// This macro generates implementations for various combinations of `GuardedF64`, `f64`, and `UnguardedF64`
/// types, allowing for flexible arithmetic operations while ensuring that the results are checked for validity.
///
/// # Arguments
///
/// * `$op_trait` - The trait representing the binary operation (e.g., `Add`, `Sub`, etc.).
/// * `$op_method` - The method name for the operation (e.g., `add`, `sub`, etc.).
/// * `$implementation` - A block of code that defines how the operation is performed.
/// * `$doc` - A documentation string that describes the operation and its usage.
#[macro_export]
macro_rules! binary_operation {
    (
        impl $op_trait:ident for ...($TGuarded:ty, $TUnguarded:ty) {
            $doc:literal
            fn $op_method:ident ($lhs:ident : $lhs_internal:ty, $rhs:ident : $rhs_internal:ty) -> $ret:ty
            $implementation:block
        }
    ) => {
        binary_operation!(
            $op_trait :: $op_method
            $doc
            fn ($lhs: $TGuarded, $rhs: $rhs_internal) -> $ret {
                let $lhs: $lhs_internal = $lhs.0;
                $implementation
            }
        );

        binary_operation!(
            $op_trait :: $op_method
            $doc
            fn ($lhs: $TUnguarded, $rhs: $rhs_internal) -> $ret {
                let $lhs: $lhs_internal = $lhs.0;
                $implementation
            }
        );

        binary_operation!(
            $op_trait :: $op_method
            $doc
            fn ($lhs: $lhs_internal, $rhs: $TGuarded) -> $ret {
                let $rhs: $rhs_internal = $rhs.0;
                $implementation
            }
        );

        binary_operation!(
            $op_trait :: $op_method
            $doc
            fn ($lhs: $lhs_internal, $rhs: $TUnguarded) -> $ret {
                let $rhs: $rhs_internal = $rhs.0;
                $implementation
            }
        );

        binary_operation!(
            $op_trait :: $op_method
            $doc
            fn ($lhs: $TGuarded, $rhs: $TGuarded) -> $ret {
                let $lhs: $lhs_internal = $lhs.0;
                let $rhs: $rhs_internal = $rhs.0;
                $implementation
            }
        );

        binary_operation!(
            $op_trait :: $op_method
            $doc
            fn ($lhs: $TGuarded, $rhs: $TUnguarded) -> $ret {
                let $lhs: $lhs_internal = $lhs.0;
                let $rhs: $rhs_internal = $rhs.0;
                $implementation
            }
        );

        binary_operation!(
            $op_trait :: $op_method
            $doc
            fn ($lhs: $TUnguarded, $rhs: $TGuarded) -> $ret {
                let $lhs: $lhs_internal = $lhs.0;
                let $rhs: $rhs_internal = $rhs.0;
                $implementation
            }
        );

        binary_operation!(
            $op_trait :: $op_method
            $doc
            fn ($lhs: $TUnguarded, $rhs: $TUnguarded) -> $ret {
                let $lhs: $lhs_internal = $lhs.0;
                let $rhs: $rhs_internal = $rhs.0;
                $implementation
            }
        );
    };

    (
        $op_trait:ident :: $op_method:ident
        $doc:literal
        fn ($lhs:ident : $LHS:ty, $rhs:ident : $RHS:ty) -> $ret:ty $implementation:block
    ) => {
        // | X | LHS   | RHS   | Result Type  |
        // |---|-------|-------|--------------|
        // | X | $LHS  | $RHS  | UnguardedF64 |
        // |   | $LHS  | &$RHS | UnguardedF64 |
        // |   | &$LHS | $RHS  | UnguardedF64 |
        // |   | &$LHS | &$RHS | UnguardedF64 |
        impl $op_trait<$RHS> for $LHS {
            type Output = $ret;

            #[doc = $doc]
            #[inline(always)]
            fn $op_method(self, $rhs: $RHS) -> Self::Output {
                let $lhs: $LHS = self;
                $implementation
            }
        }

        // | X | LHS   | RHS   | Result Type  |
        // |---|-------|-------|--------------|
        // |   | $LHS  | $RHS  | UnguardedF64 |
        // | X | $LHS  | &$RHS | UnguardedF64 |
        // |   | &$LHS | $RHS  | UnguardedF64 |
        // |   | &$LHS | &$RHS | UnguardedF64 |
        impl $op_trait<&$RHS> for $LHS {
            type Output = $ret;

            #[doc = $doc]
            #[inline(always)]
            fn $op_method(self, $rhs: &$RHS) -> Self::Output {
                self.$op_method(*$rhs)
            }
        }

        // | X | LHS   | RHS   | Result Type  |
        // |---|-------|-------|--------------|
        // |   | $LHS  | $RHS  | UnguardedF64 |
        // |   | $LHS  | &$RHS | UnguardedF64 |
        // | X | &$LHS | $RHS  | UnguardedF64 |
        // |   | &$LHS | &$RHS | UnguardedF64 |
        impl $op_trait<$RHS> for &$LHS {
            type Output = $ret;

            #[doc = $doc]
            #[inline(always)]
            fn $op_method(self, $rhs: $RHS) -> Self::Output {
                (*self).$op_method($rhs)
            }
        }

        // | X | LHS   | RHS   | Result Type  |
        // |---|-------|-------|--------------|
        // |   | $LHS  | $RHS  | UnguardedF64 |
        // |   | $LHS  | &$RHS | UnguardedF64 |
        // |   | &$LHS | $RHS  | UnguardedF64 |
        // | X | &$LHS | &$RHS | UnguardedF64 |
        impl $op_trait<&$RHS> for &$LHS {
            type Output = $ret;

            #[doc = $doc]
            #[inline(always)]
            fn $op_method(self, $rhs: &$RHS) -> Self::Output {
                (*self).$op_method(*$rhs)
            }
        }
    };
}
