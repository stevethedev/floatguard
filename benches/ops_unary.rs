use criterion::{Criterion, criterion_group, criterion_main};
use floatguard::{GuardedF32, GuardedF64, UnguardedF32, UnguardedF64};

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
    bench_neg,
    "Negation",
    ("f64::neg", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| -value)
    }),
    ("GuardedF64::neg", |b| {
        let value = GuardedF64::new(std::hint::black_box(42.0f64)).unwrap();
        b.iter(|| -value)
    }),
    ("UnguardedF64::neg", |b| {
        let value = UnguardedF64::new(std::hint::black_box(42.0f64));
        b.iter(|| -value)
    }),
    ("f32::neg", |b| {
        let value = std::hint::black_box(42.0f32);
        b.iter(|| -value)
    }),
    ("GuardedF32::neg", |b| {
        let value = GuardedF32::new(std::hint::black_box(42.0f32)).unwrap();
        b.iter(|| -value)
    }),
    ("UnguardedF32::neg", |b| {
        let value = UnguardedF32::new(std::hint::black_box(42.0f32));
        b.iter(|| -value)
    })
);

criterion_group!(benches, bench_neg,);
criterion_main!(benches);
