mod consts;
mod convert;
mod guarded;
mod math;
mod ops_binary;
mod ops_unary;
mod unguarded;

pub use guarded::GuardedF32;
pub use unguarded::UnguardedF32;

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    const INVALID_VALUES: &[f32; 3] = &[f32::NAN, f32::INFINITY, f32::NEG_INFINITY];

    pub fn valid_f32() -> impl Strategy<Value = f32> {
        // Avoid NaN, ±∞, and subnormals.
        // This gives good coverage while staying in safe computation territory.
        (f32::MIN..=f32::MAX).prop_filter("Reject NaN and infinities", |v| {
            v.is_finite() && !v.is_nan()
        })
    }

    pub fn invalid_f32() -> impl Strategy<Value = f32> {
        prop::sample::select(INVALID_VALUES)
    }
}
