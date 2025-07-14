#![deny(clippy::all, clippy::pedantic, clippy::nursery)]

mod error;
mod f32;
mod f64;

#[macro_use]
mod macros;

pub use error::Error as FloatError;
pub use f32::{GuardedF32, UnguardedF32};
pub use f64::{GuardedF64, UnguardedF64};
