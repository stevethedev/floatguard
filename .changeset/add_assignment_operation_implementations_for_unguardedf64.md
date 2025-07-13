---
default: patch
---

# Add assignment operation implementations for `UnguardedF64`

* Implemented assignment operator traits (`AddAssign`, `SubAssign`, `MulAssign`, `DivAssign`, `RemAssign`) for
  `UnguardedF64`, supporting assignment with `UnguardedF64`, `GuardedF64`, and `f64`.
* Added detailed documentation and usage examples for each assignment operator.
* Introduced property-based tests for all assignment operators to verify correctness
