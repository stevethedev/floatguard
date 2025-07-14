## 0.1.2 (2025-07-14)

### Fixes

- Refactor `FloatError` to represent `f64::NAN`, `f64::INFINITY`, and `f64::NEG_INFINITY` states

#### Add assignment operation implementations for `UnguardedF64`

* Implemented assignment operator traits (`AddAssign`, `SubAssign`, `MulAssign`, `DivAssign`, `RemAssign`) for
  `UnguardedF64`, supporting assignment with `UnguardedF64`, `GuardedF64`, and `f64`.
* Added detailed documentation and usage examples for each assignment operator.
* Introduced property-based tests for all assignment operators to verify correctness

#### Added `GuardedF32` and `UnguardedF32` types

This change introduces the `GuardedF32` and `UnguardedF32` types in the codebase. These types are designed to handle
floating-point numbers with specific guard conditions, enhancing the precision and safety of numerical computations.

## 0.1.1 (2025-07-11)

### Fixes

#### Implemented `Eq` and `Cmp` for `GuardedF64`

Since `GuardedF64` is guaranteed to be non-NaN and finite, it is now safe to implement `Eq` and `Ord` traits for it.
This allows for strict comparisons between `GuardedF64` values, which can be useful in various contexts where ordering
or equality checks are necessary.
