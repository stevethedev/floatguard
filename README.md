# FloatGuard (`floatguard`)

[![Crates.io](https://img.shields.io/crates/v/floatguard.svg)](https://crates.io/crates/floatguard)
[![Docs.rs](https://img.shields.io/badge/docs.rs-floatguard-blue)](https://docs.rs/floatguard)
[![License](https://img.shields.io/crates/l/floatguard)](https://github.com/stevethedev/floatguard/blob/main/LICENSE.MID.md)

[![Build Status](https://github.com/stevethedev/floatguard/actions/workflows/ci.yml/badge.svg)](https://github.com/stevethedev/floatguard/actions)
[![codecov](https://codecov.io/gh/stevethedev/floatguard/graph/badge.svg?token=9OhzXIlNjL)](https://codecov.io/gh/stevethedev/floatguard)
[![Release](https://github.com/stevethedev/floatguard/actions/workflows/release.yml/badge.svg)](https://github.com/stevethedev/floatguard/actions/workflows/release.yml)

A low-cost, zero-overhead wrapper around `f64` that eliminates the risk of `NaN` and infinity in floating-point
computations.

## Why?

Floating-point math in Rust (and most languages) silently permits values like `NaN`, `INFINITY`, and `-INFINITY`. These
values conform to the IEEE 754 standard and exist to support interoperability with C libraries and hardware.

However, in many applications, they behave like floating-point landmines — silently propagating through computations and
causing unexpected or confusing results. They're the numerical equivalent of a null pointer or an uncaught exception.

`floatguard` introduces two types to address this problem explicitly:

- **`GuardedF64`**: A wrapper around `f64` that guarantees the value is _finite_ — it will never contain `NaN`,
  `INFINITY`, or `-INFINITY`. Any operation that could produce an invalid value returns an `UnguardedF64` instead.
- **`UnguardedF64`**: A lightweight wrapper that may contain invalid states like `NaN` or infinities. It allows chained
  operations without incurring a validity check at every step. Before use, the value must be validated with `.check()`,
  which either returns a valid `GuardedF64` or an error.

This model provides both safety and performance, by enabling deferred validation and avoiding silent propagation of
invalid values.

## Example

```rust
use floatguard::GuardedF64;

fn main() {
    let a = GuardedF64::new(1.0).unwrap();
    let b = GuardedF64::new(2.0).unwrap();

    // Valid arithmetic
    let c = a + b; // c is UnguardedF64(3.0)

    match c.check() {
        Ok(valid) => println!("Valid result: {valid}"),
        Err(e) => println!("Error: {e}"),
    }

    // Invalid arithmetic
    let d = c / GuardedF64::new(0.0).unwrap(); // d is UnguardedF64(NaN)

    match d.check() {
        Ok(valid) => println!("Valid result: {valid}"),
        Err(e) => println!("Error: {e}"),
    }
}
```

Output:

```plaintext
Valid result: 3
Error: The floating-point value is poisoned
```

## Features

- Finite guarantees: `GuardedF64` never contains NaN or infinities
- Deferred validation: `UnguardedF64` allows efficient math, checked only when needed
- Drop-in operators: Full support for `+`, `-`, `*`, `/`, `+=`, `-=`, etc.
- Conversions: `TryFrom<f64>`, `Into<f64>`, and more
- `#![no_std]` compatible

### Crate Features

- `std` (default) — Enables std-based functionality (currently unused but reserved for future expansion)

## Safety and Limitations

- Only `f64` is supported; `f32` is not yet implemented.
- Any operation that may result in an invalid value returns an `UnguardedF64`.
- Use `.check()` to convert an `UnguardedF64` into a `GuardedF64`, or catch an error if the value is invalid.

## MSRV

Minimum Supported Rust Version: 1.85.1

## License

MIT OR Apache-2.0
