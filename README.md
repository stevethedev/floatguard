# checked-float

A low-cost checked wrapper around `f64` to eliminate the risks of `NaN` and `Infinity` in numerical computations.

## Why?

Floating-point math in Rust (and most languages) silently permits invalid values like `NaN`, `+∞`, or `-∞`, which can
lead to subtle and hard-to-trace bugs. `checked-float` wraps `f64` in a safe type that propagates invalid states and
prevents invalid values from ever entering your system.

## Example

```rust
use checked_float::CheckedF64;

fn main() {
    let a = CheckedF64::new(1.0).unwrap();
    let b = CheckedF64::new(2.0).unwrap();

    // Valid arithmetic
    let c = a + b; // c is UncheckedF64(3.0)

    match c.check() {
        Ok(valid) => println!("Valid result: {valid}"),
        Err(e) => println!("Error: {e}"),
    }

    // Invalid arithmetic
    let d = c / CheckedF64::new(0.0).unwrap(); // d is UncheckedF64(NaN)

    // Check if the result is valid
    match d.check() {
        Ok(valid) => println!("Valid result: {valid}"),
        Err(e) => println!("Error: {e}"),
    }
}
```

Executes as:

```plaintext
Valid result: 3
Error: The floating-point value is poisoned
```

## Features

- Guaranteed no `NaN` or `Infinity` in `CheckedF64` values
- Propagates invalid results in `UncheckedF64` like a poison value
- Drop-in arithmetic: `+`, `-`, `*`, `/`, `+=`, `-=`, etc.
- `TryFrom<f64>`, `From<CheckedF64>`
- `#![no_std]` compatible

### Crate Features

* `std` (default) — Enables std-based functionality (currently unused but future-proofed)

## Safety and Limitations

- Only `f64` values are supported; no support for `f32` at this time.
- Arithmetic operations that could produce `NaN` or `Infinity` will result in an `UncheckedF64` value for efficiency;
  you must check the result using `.check()` to ensure validity.

## MSRV

`cargo msrv` marks this as MSRV=`1.85.1`

## License

MIT OR Apache-2.0
