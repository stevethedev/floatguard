#![deny(clippy::all, clippy::pedantic, clippy::nursery)]

mod error;
mod f64;

#[macro_use]
mod macros;

pub use error::Error as FloatError;
pub use f64::{GuardedF64, UnguardedF64};
