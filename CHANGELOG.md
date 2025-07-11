## 0.1.1 (2025-07-11)

### Fixes

#### Implemented `Eq` and `Cmp` for `GuardedF64`

Since `GuardedF64` is guaranteed to be non-NaN and finite, it is now safe to implement `Eq` and `Ord` traits for it.
This allows for strict comparisons between `GuardedF64` values, which can be useful in various contexts where ordering
or equality checks are necessary.
