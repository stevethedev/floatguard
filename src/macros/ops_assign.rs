macro_rules! assign_operation {
    (
        use $OpTrait:ident :: $op:ident impl $AssignTrait:ident :: $assign_op:ident for ...($($TSelf:ty),*)
        $doc:literal
    ) => {
        $(
            impl<T> $AssignTrait<T> for $TSelf
            where
                T: Into<Self>,
            {
                #[doc = $doc]
                #[allow(clippy::inline_always)]
                #[inline(always)]
                fn $assign_op(&mut self, rhs: T) {
                    *self = self.$op(rhs.into());
                }
            }
        )*
    };
}

pub(crate) use assign_operation;
