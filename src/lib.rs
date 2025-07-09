#![deny(clippy::all, clippy::pedantic, clippy::nursery)]

mod guarded_f64;
mod error;

pub use guarded_f64::{GuardedF64, UnguardedF64};
pub use error::{Error as FloatError};
