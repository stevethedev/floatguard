use criterion::{criterion_group, criterion_main, Criterion};
use checked_float::CheckedF64;

fn bench_f64_gt(c: &mut Criterion) {
    c.bench_function("f64::gt", |b| {
        let lhs = std::hint::black_box(42.0f64);
        let rhs = std::hint::black_box(2.0);
        b.iter(|| lhs > rhs);
    });
}

fn bench_checked_f64_gt(c: &mut Criterion) {
    c.bench_function("CheckedF64::gt", |b| {
        let lhs = CheckedF64::new(std::hint::black_box(42.0f64)).unwrap();
        let rhs = CheckedF64::new(std::hint::black_box(2.0)).unwrap();
        b.iter(|| lhs > rhs)
    });
}

fn bench_checked_f64_result_gt(c: &mut Criterion) {
    c.bench_function("CheckedF64Result::gt", |b| {
        let lhs = CheckedF64::new(std::hint::black_box(42.0f64));
        let rhs = CheckedF64::new(std::hint::black_box(2.0));
        b.iter(|| lhs > rhs)
    });
}

fn bench_f64_lt(c: &mut Criterion) {
    c.bench_function("f64::lt", |b| {
        let lhs = std::hint::black_box(42.0f64);
        let rhs = std::hint::black_box(2.0);
        b.iter(|| lhs < rhs);
    });
}

fn bench_checked_f64_lt(c: &mut Criterion) {
    c.bench_function("CheckedF64::lt", |b| {
        let lhs = CheckedF64::new(std::hint::black_box(42.0f64)).unwrap();
        let rhs = CheckedF64::new(std::hint::black_box(2.0)).unwrap();
        b.iter(|| lhs < rhs)
    });
}

fn bench_checked_f64_result_lt(c: &mut Criterion) {
    c.bench_function("CheckedF64Result::lt", |b| {
        let lhs = CheckedF64::new(std::hint::black_box(42.0f64));
        let rhs = CheckedF64::new(std::hint::black_box(2.0));
        b.iter(|| lhs < rhs)
    });
}

fn bench_f64_eq(c: &mut Criterion) {
    c.bench_function("f64::eq", |b| {
        let lhs = std::hint::black_box(42.0f64);
        let rhs = std::hint::black_box(2.0);
        b.iter(|| lhs == rhs);
    });
}

fn bench_checked_f64_eq(c: &mut Criterion) {
    c.bench_function("CheckedF64::eq", |b| {
        let lhs = CheckedF64::new(std::hint::black_box(42.0f64)).unwrap();
        let rhs = CheckedF64::new(std::hint::black_box(2.0)).unwrap();
        b.iter(|| lhs == rhs)
    });
}

fn bench_checked_f64_result_eq(c: &mut Criterion) {
    c.bench_function("CheckedF64Result::eq", |b| {
        let lhs = CheckedF64::new(std::hint::black_box(42.0f64));
        let rhs = CheckedF64::new(std::hint::black_box(2.0));
        b.iter(|| lhs == rhs)
    });
}

criterion_group!(
    benches,
    bench_f64_gt,
    bench_checked_f64_gt,
    bench_checked_f64_result_gt,
    bench_f64_lt,
    bench_checked_f64_lt,
    bench_checked_f64_result_lt,
    bench_f64_eq,
    bench_checked_f64_eq,
    bench_checked_f64_result_eq
);
criterion_main!(benches);
