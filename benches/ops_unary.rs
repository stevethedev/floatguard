use criterion::{criterion_group, criterion_main, Criterion};
use checked_float::CheckedF64;

fn bench_f64_neg(c: &mut Criterion) {
    c.bench_function("f64::neg", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| -value);
    });
}

fn bench_checked_f64_neg(c: &mut Criterion) {
    c.bench_function("CheckedF64::neg", |b| {
        let value = CheckedF64::new(std::hint::black_box(42.0f64)).unwrap();
        b.iter(|| (-value).unwrap());
    });
}

fn bench_checked_f64_result_neg(c: &mut Criterion) {
    c.bench_function("CheckedF64Result::neg", |b| {
        let value = CheckedF64::new(std::hint::black_box(42.0f64));
        b.iter(|| -value);
    });
}

criterion_group!(
    benches,
    bench_f64_neg,
    bench_checked_f64_neg,
    bench_checked_f64_result_neg
);
criterion_main!(benches);
