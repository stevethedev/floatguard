mod consts;
mod convert;
mod guarded;
mod math;
mod ops_binary;
mod ops_unary;
mod unguarded;

pub use guarded::GuardedF64;
pub use unguarded::UnguardedF64;

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    const INVALID_VALUES: &[f64; 3] = &[f64::NAN, f64::INFINITY, f64::NEG_INFINITY];

    pub fn valid_f64() -> impl Strategy<Value = f64> {
        // Avoid NaN, ±∞, and subnormals.
        // This gives good coverage while staying in safe computation territory.
        (f64::MIN..=f64::MAX).prop_filter("Reject NaN and infinities", |v| {
            v.is_finite() && !v.is_nan()
        })
    }

    pub fn invalid_f64() -> impl Strategy<Value = f64> {
        prop::sample::select(INVALID_VALUES)
    }
}
