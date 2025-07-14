/// Macro to define constants in `GuardedF64` that mirror the constants in `f64`.
///
/// This macro allows you to define constants with a specific type and documentation.
/// It simplifies the process of creating constants that are directly related to the `f64` type,
/// ensuring that they are accessible through the `GuardedF64` type.
///
/// # Arguments
///
/// - `$name`: The name of the constant to define.
/// - `$t`: The type of the constant (e.g., `f64`, `u32`, etc.).
/// - `$value`: The value of the constant, which can be a direct reference to an `f64` constant or a specific value.
/// - `$doc`: A documentation string that describes the constant and its purpose.
#[macro_export]
macro_rules! copy_const_value {
    ( ( $( $T:ty $(,)? )* ), $name:ident : $name_t:ty = $value:expr, $doc:expr) => {
        $(
            impl $T {
                #[doc = $doc]
                pub const $name: $name_t = $value;
            }
        )*
    };
}
