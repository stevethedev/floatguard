#![deny(clippy::all, clippy::pedantic, clippy::nursery)]

mod error;
mod guarded_f64;

pub use error::Error as FloatError;
pub use guarded_f64::{GuardedF64, UnguardedF64};
