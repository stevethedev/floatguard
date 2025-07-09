use criterion::{criterion_group, criterion_main, Criterion};
use checked_float::{CheckedF64, UncheckedF64};

fn bench_f64_neg(c: &mut Criterion) {
    c.bench_function("f64::neg", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| -value);
    });
}

fn bench_checked_neg(c: &mut Criterion) {
    c.bench_function("CheckedF64::neg", |b| {
        let value = std::hint::black_box(CheckedF64::new(42.0f64).unwrap());
        b.iter(|| -value);
    });
}

fn bench_unchecked_neg(c: &mut Criterion) {
    c.bench_function("UncheckedF64::neg", |b| {
        let value = std::hint::black_box(UncheckedF64::new(42.0f64));
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
