/// Macro to implement unary operations for `GuardedF64` and `UnguardedF64`.
///
/// This macro generates implementations for unary operations like negation, ensuring that the operation
/// returns a `UnguardedF64`. It handles both `GuardedF64` and `UnguardedF64` types, allowing for
/// safe operations on floating-point numbers while checking for invalid values like NaN or Infinity.
///
/// # Arguments
///
/// - `$op_trait`: The trait for the unary operation (e.g., `Neg`).
/// - `$op_method`: The method name for the operation (e.g., `neg`).
/// - `$implementation`: The implementation function that defines how the operation is performed.
/// - `$doc`: A documentation string that describes the operation and its behavior.
macro_rules! unary_operation {
    (
        impl $op_trait:ident for ...($( $T:ty ),*) {
            $doc:literal
            fn $op_method:ident ($base:ident : $base_ty:ty) -> Self::Output
            $implementation:block
        }
    ) => {
        $(
            impl $op_trait for $T {
                type Output = $T;

                #[doc = $doc]
                #[inline(always)]
                fn $op_method(self) -> Self::Output {
                    let $base: $base_ty = self.0;
                    $implementation
                }
            }

            impl $op_trait for &$T {
                type Output = $T;

                #[doc = $doc]
                #[inline(always)]
                fn $op_method(self) -> Self::Output {
                    (*self).$op_method()
                }
            }
        )*
    };
}

pub(crate) use unary_operation;
