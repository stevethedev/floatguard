#![deny(clippy::all, clippy::pedantic, clippy::nursery)]

mod checked_f64;
mod error;

pub use error::Error as FloatError;
pub use checked_f64::CheckedF64;
