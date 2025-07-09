use criterion::{criterion_group, criterion_main, Criterion};
use floatguard::{GuardedF64, UnguardedF64};

fn bench_f64_neg(c: &mut Criterion) {
    c.bench_function("f64::neg", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| -value);
    });
}

fn bench_checked_neg(c: &mut Criterion) {
    c.bench_function("GuardedF64::neg", |b| {
        let value = std::hint::black_box(GuardedF64::new(42.0f64).unwrap());
        b.iter(|| -value);
    });
}

fn bench_unchecked_neg(c: &mut Criterion) {
    c.bench_function("UnguardedF64::neg", |b| {
        let value = std::hint::black_box(UnguardedF64::new(42.0f64));
        b.iter(|| -value);
    });
}

criterion_group!(
    benches,
    bench_f64_neg,
    bench_checked_neg,
    bench_unchecked_neg
);
criterion_main!(benches);
