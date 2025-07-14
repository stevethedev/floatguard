use criterion::{Criterion, criterion_group, criterion_main};
use floatguard::{GuardedF32, GuardedF64, UnguardedF32, UnguardedF64};

macro_rules! bench {
    ($id:ident, $group:literal, $( ($bench:literal, $expr:expr) ),* $(,)?) => {
        fn $id(c: &mut Criterion) {
            let mut group = c.benchmark_group($group);

            $(
                group.bench_function($bench, $expr);
            )*

            group.finish();
        }
    };
}

bench!(
    bench_add,
    "Addition",
    ("f64::add", |b| {
        let lhs = std::hint::black_box(42.0f64);
        let rhs = std::hint::black_box(2.0);
        b.iter(|| lhs + rhs)
    }),
    ("GuardedF64::add", |b| {
        let lhs = GuardedF64::new(std::hint::black_box(42.0f64)).unwrap();
        let rhs = GuardedF64::new(std::hint::black_box(2.0)).unwrap();
        b.iter(|| lhs + rhs)
    }),
    ("UnguardedF64::add", |b| {
        let lhs = UnguardedF64::new(std::hint::black_box(42.0f64));
        let rhs = UnguardedF64::new(std::hint::black_box(2.0));
        b.iter(|| lhs + rhs)
    }),
    ("f32::add", |b| {
        let lhs = std::hint::black_box(42.0f32);
        let rhs = std::hint::black_box(2.0);
        b.iter(|| lhs + rhs)
    }),
    ("GuardedF32::add", |b| {
        let lhs = GuardedF32::new(std::hint::black_box(42.0f32)).unwrap();
        let rhs = GuardedF32::new(std::hint::black_box(2.0)).unwrap();
        b.iter(|| lhs + rhs)
    }),
    ("UnguardedF32::add", |b| {
        let lhs = UnguardedF32::new(std::hint::black_box(42.0f32));
        let rhs = UnguardedF32::new(std::hint::black_box(2.0));
        b.iter(|| lhs + rhs)
    }),
);

bench!(
    bench_sub,
    "Subtraction",
    ("f64::sub", |b| {
        let lhs = std::hint::black_box(42.0f64);
        let rhs = std::hint::black_box(2.0);
        b.iter(|| lhs - rhs)
    }),
    ("GuardedF64::sub", |b| {
        let lhs = GuardedF64::new(std::hint::black_box(42.0f64)).unwrap();
        let rhs = GuardedF64::new(std::hint::black_box(2.0)).unwrap();
        b.iter(|| lhs - rhs)
    }),
    ("UnguardedF64::sub", |b| {
        let lhs = UnguardedF64::new(std::hint::black_box(42.0f64));
        let rhs = UnguardedF64::new(std::hint::black_box(2.0));
        b.iter(|| lhs - rhs)
    }),
    ("f32::sub", |b| {
        let lhs = std::hint::black_box(42.0f32);
        let rhs = std::hint::black_box(2.0);
        b.iter(|| lhs - rhs)
    }),
    ("GuardedF32::sub", |b| {
        let lhs = GuardedF32::new(std::hint::black_box(42.0f32)).unwrap();
        let rhs = GuardedF32::new(std::hint::black_box(2.0)).unwrap();
        b.iter(|| lhs - rhs)
    }),
    ("UnguardedF32::sub", |b| {
        let lhs = UnguardedF32::new(std::hint::black_box(42.0f32));
        let rhs = UnguardedF32::new(std::hint::black_box(2.0));
        b.iter(|| lhs - rhs)
    }),
);

bench!(
    bench_mul,
    "Multiplication",
    ("f64::mul", |b| {
        let lhs = std::hint::black_box(42.0f64);
        let rhs = std::hint::black_box(2.0);
        b.iter(|| lhs * rhs)
    }),
    ("GuardedF64::mul", |b| {
        let lhs = GuardedF64::new(std::hint::black_box(42.0f64)).unwrap();
        let rhs = GuardedF64::new(std::hint::black_box(2.0)).unwrap();
        b.iter(|| lhs * rhs)
    }),
    ("UnguardedF64::mul", |b| {
        let lhs = UnguardedF64::new(std::hint::black_box(42.0f64));
        let rhs = UnguardedF64::new(std::hint::black_box(2.0));
        b.iter(|| lhs * rhs)
    }),
    ("f32::mul", |b| {
        let lhs = std::hint::black_box(42.0f32);
        let rhs = std::hint::black_box(2.0);
        b.iter(|| lhs * rhs)
    }),
    ("GuardedF32::mul", |b| {
        let lhs = GuardedF32::new(std::hint::black_box(42.0f32)).unwrap();
        let rhs = GuardedF32::new(std::hint::black_box(2.0)).unwrap();
        b.iter(|| lhs * rhs)
    }),
    ("UnguardedF32::mul", |b| {
        let lhs = UnguardedF32::new(std::hint::black_box(42.0f32));
        let rhs = UnguardedF32::new(std::hint::black_box(2.0));
        b.iter(|| lhs * rhs)
    }),
);

bench!(
    bench_div,
    "Division",
    ("f64::div", |b| {
        let lhs = std::hint::black_box(42.0f64);
        let rhs = std::hint::black_box(2.0);
        b.iter(|| lhs / rhs)
    }),
    ("GuardedF64::div", |b| {
        let lhs = GuardedF64::new(std::hint::black_box(42.0f64)).unwrap();
        let rhs = GuardedF64::new(std::hint::black_box(2.0)).unwrap();
        b.iter(|| lhs / rhs)
    }),
    ("UnguardedF64::div", |b| {
        let lhs = UnguardedF64::new(std::hint::black_box(42.0f64));
        let rhs = UnguardedF64::new(std::hint::black_box(2.0));
        b.iter(|| lhs / rhs)
    }),
    ("f32::div", |b| {
        let lhs = std::hint::black_box(42.0f32);
        let rhs = std::hint::black_box(2.0);
        b.iter(|| lhs / rhs)
    }),
    ("GuardedF32::div", |b| {
        let lhs = GuardedF32::new(std::hint::black_box(42.0f32)).unwrap();
        let rhs = GuardedF32::new(std::hint::black_box(2.0)).unwrap();
        b.iter(|| lhs / rhs)
    }),
    ("UnguardedF32::div", |b| {
        let lhs = UnguardedF32::new(std::hint::black_box(42.0f32));
        let rhs = UnguardedF32::new(std::hint::black_box(2.0));
        b.iter(|| lhs / rhs)
    }),
);

bench!(
    bench_rem,
    "Remainder",
    ("f64::rem", |b| {
        let lhs = std::hint::black_box(42.0f64);
        let rhs = std::hint::black_box(2.0);
        b.iter(|| lhs % rhs)
    }),
    ("GuardedF64::rem", |b| {
        let lhs = GuardedF64::new(std::hint::black_box(42.0f64)).unwrap();
        let rhs = GuardedF64::new(std::hint::black_box(2.0)).unwrap();
        b.iter(|| lhs % rhs)
    }),
    ("UnguardedF64::rem", |b| {
        let lhs = UnguardedF64::new(std::hint::black_box(42.0f64));
        let rhs = UnguardedF64::new(std::hint::black_box(2.0));
        b.iter(|| lhs % rhs)
    }),
    ("f32::rem", |b| {
        let lhs = std::hint::black_box(42.0f32);
        let rhs = std::hint::black_box(2.0);
        b.iter(|| lhs % rhs)
    }),
    ("GuardedF32::rem", |b| {
        let lhs = GuardedF32::new(std::hint::black_box(42.0f32)).unwrap();
        let rhs = GuardedF32::new(std::hint::black_box(2.0)).unwrap();
        b.iter(|| lhs % rhs)
    }),
    ("UnguardedF32::rem", |b| {
        let lhs = UnguardedF32::new(std::hint::black_box(42.0f32));
        let rhs = UnguardedF32::new(std::hint::black_box(2.0));
        b.iter(|| lhs % rhs)
    }),
);

criterion_group!(
    benches, bench_add, bench_sub, bench_mul, bench_div, bench_rem,
);
criterion_main!(benches);
