#[macro_export]
macro_rules! const_math {
    (
        ($( $T:ty $(,)? )* ),
        fn $name:ident ($base:ident : $base_ty:ty) -> $returns:ty $implementation:block,
        $doc:expr
    ) => {
        $(
            impl $T {
                #[doc = $doc]
                #[must_use = "method returns a new instance and does not mutate the original value"]
                #[inline(always)]
                pub const fn $name(self) -> $returns {
                    let $base = self.0;
                    $implementation
                }
            }
        )*
    };
}
