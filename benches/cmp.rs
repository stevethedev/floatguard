use criterion::{criterion_group, criterion_main, Criterion};
use checked_float::CheckedF64;

fn bench_f64_cmp(c: &mut Criterion) {
    c.bench_function("f64::gt", |b| {
        let lhs: f64 = std::hint::black_box(42.0);
        let rhs: f64 = std::hint::black_box(2.0);
        b.iter(|| lhs > rhs);
    });
}

fn bench_checked_cmp(c: &mut Criterion) {
    c.bench_function("CheckedF64::gt", |b| {
        let lhs = CheckedF64::new(std::hint::black_box(42.0)).unwrap();
        let rhs = CheckedF64::new(std::hint::black_box(2.0)).unwrap();
        b.iter(|| lhs > rhs)
    });
}

fn bench_f64_eq(c: &mut Criterion) {
    c.bench_function("f64::eq", |b| {
        let lhs: f64 = std::hint::black_box(42.0);
        let rhs: f64 = std::hint::black_box(2.0);
        b.iter(|| lhs == rhs);
    });
}

fn bench_checked_eq(c: &mut Criterion) {
    c.bench_function("CheckedF64::eq", |b| {
        let lhs = CheckedF64::new(std::hint::black_box(42.0)).unwrap();
        let rhs = CheckedF64::new(std::hint::black_box(2.0)).unwrap();
        b.iter(|| lhs == rhs)
    });
}

criterion_group!(
    benches,
    bench_f64_cmp,
    bench_checked_cmp,
    bench_f64_eq,
    bench_checked_eq,
);
criterion_main!(benches);
