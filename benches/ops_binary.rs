use criterion::{criterion_group, criterion_main, Criterion};
use checked_float::{CheckedF64, UncheckedF64};

fn bench_f64_add(c: &mut Criterion) {
    c.bench_function("f64::add", |b| {
        let lhs = std::hint::black_box(42.0f64);
        let rhs = std::hint::black_box(2.0);

        b.iter(|| lhs + rhs);
    });
}

fn bench_checked_add(c: &mut Criterion) {
    c.bench_function("CheckedF64::add", |b| {
        let lhs = std::hint::black_box(CheckedF64::new(42.0f64).unwrap());
        let rhs = std::hint::black_box(CheckedF64::new(2.0).unwrap());

        b.iter(|| lhs + rhs)
    });
}

fn bench_unchecked_add(c: &mut Criterion) {
    c.bench_function("UncheckedF64::add", |b| {
        let lhs = std::hint::black_box(UncheckedF64::new(42.0f64));
        let rhs = std::hint::black_box(UncheckedF64::new(2.0));

        b.iter(|| lhs + rhs)
    });
}

fn bench_f64_sub(c: &mut Criterion) {
    c.bench_function("f64::sub", |b| {
        let lhs = std::hint::black_box(42.0f64);
        let rhs = std::hint::black_box(2.0);

        b.iter(|| lhs - rhs);
    });
}

fn bench_checked_sub(c: &mut Criterion) {
    c.bench_function("CheckedF64::sub", |b| {
        let lhs = std::hint::black_box(CheckedF64::new(42.0f64).unwrap());
        let rhs = std::hint::black_box(CheckedF64::new(2.0).unwrap());

        b.iter(|| lhs - rhs)
    });
}

fn bench_unchecked_sub(c: &mut Criterion) {
    c.bench_function("UncheckedF64::sub", |b| {
        let lhs = std::hint::black_box(UncheckedF64::new(42.0f64));
        let rhs = std::hint::black_box(UncheckedF64::new(2.0));

        b.iter(|| lhs + rhs)
    });
}

fn bench_f64_mul(c: &mut Criterion) {
    c.bench_function("f64::mul", |b| {
        let lhs = std::hint::black_box(42.0f64);
        let rhs = std::hint::black_box(2.0);

        b.iter(|| lhs * rhs);
    });
}

fn bench_checked_mul(c: &mut Criterion) {
    c.bench_function("CheckedF64::mul", |b| {
        let lhs = std::hint::black_box(CheckedF64::new(42.0f64).unwrap());
        let rhs = std::hint::black_box(CheckedF64::new(2.0).unwrap());

        b.iter(|| lhs * rhs)
    });
}

fn bench_unchecked_mul(c: &mut Criterion) {
    c.bench_function("UncheckedF64::mul", |b| {
        let lhs = std::hint::black_box(UncheckedF64::new(42.0f64));
        let rhs = std::hint::black_box(UncheckedF64::new(2.0));

        b.iter(|| lhs * rhs)
    });
}

fn bench_f64_div(c: &mut Criterion) {
    c.bench_function("f64::div", |b| {
        let lhs = std::hint::black_box(42.0f64);
        let rhs = std::hint::black_box(2.0);

        b.iter(|| lhs / rhs);
    });
}

fn bench_checked_div(c: &mut Criterion) {
    c.bench_function("CheckedF64::div", |b| {
        let lhs = std::hint::black_box(CheckedF64::new(42.0f64).unwrap());
        let rhs = std::hint::black_box(CheckedF64::new(2.0).unwrap());

        b.iter(|| lhs / rhs)
    });
}

fn bench_unchecked_div(c: &mut Criterion) {
    c.bench_function("UncheckedF64::div", |b| {
        let lhs = std::hint::black_box(UncheckedF64::new(42.0f64));
        let rhs = std::hint::black_box(UncheckedF64::new(2.0));

        b.iter(|| lhs / rhs)
    });
}

fn bench_f64_rem(c: &mut Criterion) {
    c.bench_function("f64::rem", |b| {
        let lhs = std::hint::black_box(42.0f64);
        let rhs = std::hint::black_box(2.0);

        b.iter(|| lhs % rhs);
    });
}

fn bench_checked_rem(c: &mut Criterion) {
    c.bench_function("CheckedF64::rem", |b| {
        let lhs = std::hint::black_box(CheckedF64::new(42.0f64).unwrap());
        let rhs = std::hint::black_box(CheckedF64::new(2.0).unwrap());

        b.iter(|| lhs % rhs)
    });
}

fn bench_unchecked_rem(c: &mut Criterion) {
    c.bench_function("UncheckedF64::rem", |b| {
        let lhs = std::hint::black_box(UncheckedF64::new(42.0f64));
        let rhs = std::hint::black_box(UncheckedF64::new(2.0));

        b.iter(|| lhs % rhs)
    });
}

criterion_group!(
    benches,
    bench_f64_add,
    bench_checked_add,
    bench_unchecked_add,
    bench_f64_sub,
    bench_checked_sub,
    bench_unchecked_sub,
    bench_f64_mul,
    bench_checked_mul,
    bench_unchecked_mul,
    bench_f64_div,
    bench_checked_div,
    bench_unchecked_div,
    bench_f64_rem,
    bench_checked_rem,
    bench_unchecked_rem,
);
criterion_main!(benches);
