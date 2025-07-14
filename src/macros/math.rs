macro_rules! math {
    (
        ($( $T:ty ),* )
        $doc:literal
        const fn $name:ident ($base:ident : $base_ty:ty) -> $returns:ty $implementation:block
    ) => {
        $(
            impl $T {
                #[doc = $doc]
                #[must_use = "method returns a new instance and does not mutate the original value"]
                #[inline(always)]
                pub const fn $name(self) -> $returns {
                    let $base: $base_ty = self.0;
                    $implementation
                }
            }
        )*
    };

    (
        ($( $T:ty ),*)
        $doc:literal
        fn $name:ident ($base:ident : $base_ty:ty) -> $ret:ty $implementation:block
    ) => {
        $(
            impl $T {
                #[doc = $doc]
                #[must_use = "method returns a new instance and does not mutate the original value"]
                #[inline(always)]
                pub fn $name(self) -> $ret {
                    let $base: $base_ty = self.0;
                    $implementation
                }
            }
        )*
    };

    (
        ($( $T:ty ),*)
        $doc:literal
        fn $name:ident ($base:ident : $base_ty:ty, $operand:ident : $operand_ty:ty ) -> $ret:ty $implementation:block
    ) => {
        $(
            impl $T {
                #[doc = $doc]
                #[must_use = "method returns a new instance and does not mutate the original value"]
                #[inline(always)]
                pub fn $name(self, $operand: $operand_ty) -> $ret {
                    let $base: $base_ty = self.0;
                    $implementation
                }
            }
        )*
    };
}

pub(crate) use math;
