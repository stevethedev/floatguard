use criterion::{Criterion, criterion_group, criterion_main};
use floatguard::GuardedF64;

macro_rules! bench {
    ($id:ident, $group:literal, $( ($bench:literal, $expr:expr) ),* ) => {
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
    bench_cmp,
    "Ordering",
    ("f64", |b| {
        let lhs: f64 = std::hint::black_box(42.0);
        let rhs: f64 = std::hint::black_box(2.0);
        b.iter(|| lhs < rhs)
    }),
    ("GuardedF64", |b| {
        let lhs = GuardedF64::new(std::hint::black_box(42.0)).unwrap();
        let rhs = GuardedF64::new(std::hint::black_box(2.0)).unwrap();
        b.iter(|| lhs < rhs)
    })
);

bench!(
    bench_eq,
    "Equality",
    ("f64::eq", |b| {
        let lhs: f64 = std::hint::black_box(42.0);
        let rhs: f64 = std::hint::black_box(2.0);
        b.iter(|| lhs == rhs)
    }),
    ("GuardedF64::eq", |b| {
        let lhs = GuardedF64::new(std::hint::black_box(42.0)).unwrap();
        let rhs = GuardedF64::new(std::hint::black_box(2.0)).unwrap();
        b.iter(|| lhs == rhs)
    })
);

criterion_group!(benches, bench_cmp, bench_eq,);
criterion_main!(benches);
