# checked-float

A checked wrapper around `f64` to eliminate the risks of `NaN` and `Infinity` in numerical computations.

## Why?

Floating-point math in Rust (and most languages) silently permits invalid values like `NaN`, `+∞`, or `-∞`, which can
lead to subtle and hard-to-trace bugs. `checked-float` wraps `f64` in a safe type that propagates invalid states and
prevents invalid values from ever entering your system.

## Features

- Guaranteed no `NaN` or `Infinity` in valid values
- Propagates invalid results like a poison value
- Drop-in arithmetic: `+`, `-`, `*`, `/`, `+=`, `-=`, etc.
- `From<f64>`, `TryFrom<CheckedF64>`, `Display`, `Debug`
- Optional `serde` support
- `#![no_std]` compatible

## Example

```rust
use checked_float::CheckedF64;

fn main() {
    let a = CheckedF64::from(1.0);
    let b = CheckedF64::from(2.0);

    // Valid arithmetic
    let c = a + b; // c is CheckedF64(3.0)

    // Invalid arithmetic
    let d = c / CheckedF64::from(0.0); // d is CheckedF64(NaN)
    
    // Check if the result is valid
    if d.is_valid() {
        println!("Result: {}", d);
    } else {
        println!("Invalid result: {:?}", d);
    }
}
```

## Crate Features

* `serde` — Enables Serialize and Deserialize support
* `std` (default) — Enables std-based functionality (currently unused but future-proofed)

## License

MIT OR Apache-2.0
