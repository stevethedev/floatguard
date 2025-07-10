use criterion::{Criterion, criterion_group, criterion_main};
use floatguard::{GuardedF64, UnguardedF64};

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
    bench_abs,
    "Absolute Value",
    ("f64::abs", |b| {
        let value = std::hint::black_box(-42.0f64);
        b.iter(|| value.abs());
    }),
    ("GuardedF64::abs", |b| {
        let value = std::hint::black_box(GuardedF64::new(-42.0f64).unwrap());
        b.iter(|| value.abs());
    }),
    ("UnguardedF64::abs", |b| {
        let value = std::hint::black_box(UnguardedF64::new(-42.0f64));
        b.iter(|| value.abs());
    })
);

bench!(
    bench_signum,
    "Signum",
    ("f64::signum", |b| {
        let value = std::hint::black_box(-42.0f64);
        b.iter(|| value.signum());
    }),
    ("GuardedF64::signum", |b| {
        let value = std::hint::black_box(GuardedF64::new(-42.0f64).unwrap());
        b.iter(|| value.signum());
    }),
    ("UnguardedF64::signum", |b| {
        let value = std::hint::black_box(UnguardedF64::new(-42.0f64));
        b.iter(|| value.signum());
    })
);

bench!(
    bench_sqrt,
    "Square Root",
    ("f64::sqrt", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| value.sqrt());
    }),
    ("GuardedF64::sqrt", |b| {
        let value = std::hint::black_box(GuardedF64::new(42.0f64).unwrap());
        b.iter(|| value.sqrt());
    }),
    ("UnguardedF64::sqrt", |b| {
        let value = std::hint::black_box(UnguardedF64::new(42.0f64));
        b.iter(|| value.sqrt());
    })
);

bench!(
    bench_recip,
    "Reciprocal",
    ("f64::recip", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| value.recip());
    }),
    ("GuardedF64::recip", |b| {
        let value = std::hint::black_box(GuardedF64::new(42.0f64).unwrap());
        b.iter(|| value.recip());
    }),
    ("UnguardedF64::recip", |b| {
        let value = std::hint::black_box(UnguardedF64::new(42.0f64));
        b.iter(|| value.recip());
    })
);

bench!(
    bench_exp,
    "Exponential",
    ("f64::exp", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| value.exp());
    }),
    ("GuardedF64::exp", |b| {
        let value = std::hint::black_box(GuardedF64::new(42.0f64).unwrap());
        b.iter(|| value.exp());
    }),
    ("UnguardedF64::exp", |b| {
        let value = std::hint::black_box(UnguardedF64::new(42.0f64));
        b.iter(|| value.exp());
    })
);

bench!(
    bench_ln,
    "Natural Logarithm",
    ("f64::ln", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| value.ln());
    }),
    ("GuardedF64::ln", |b| {
        let value = std::hint::black_box(GuardedF64::new(42.0f64).unwrap());
        b.iter(|| value.ln());
    }),
    ("UnguardedF64::ln", |b| {
        let value = std::hint::black_box(UnguardedF64::new(42.0f64));
        b.iter(|| value.ln());
    })
);

bench!(
    bench_log2,
    "Base-2 Logarithm",
    ("f64::log2", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| value.log2());
    }),
    ("GuardedF64::log2", |b| {
        let value = std::hint::black_box(GuardedF64::new(42.0f64).unwrap());
        b.iter(|| value.log2());
    }),
    ("UnguardedF64::log2", |b| {
        let value = std::hint::black_box(UnguardedF64::new(42.0f64));
        b.iter(|| value.log2());
    })
);

bench!(
    bench_log10,
    "Base-10 Logarithm",
    ("f64::log10", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| value.log10());
    }),
    ("GuardedF64::log10", |b| {
        let value = std::hint::black_box(GuardedF64::new(42.0f64).unwrap());
        b.iter(|| value.log10());
    }),
    ("UnguardedF64::log10", |b| {
        let value = std::hint::black_box(UnguardedF64::new(42.0f64));
        b.iter(|| value.log10());
    })
);

bench!(
    bench_log,
    "Base-N Logarithm",
    ("f64::log", |b| {
        let value = std::hint::black_box(42.0f64);
        let base = std::hint::black_box(2.0);
        b.iter(|| value.log(base));
    }),
    ("GuardedF64::log", |b| {
        let value = std::hint::black_box(GuardedF64::new(42.0f64).unwrap());
        let base = std::hint::black_box(2.0);
        b.iter(|| value.log(base));
    }),
    ("UnguardedF64::log", |b| {
        let value = std::hint::black_box(UnguardedF64::new(42.0f64));
        let base = std::hint::black_box(2.0);
        b.iter(|| value.log(base));
    })
);

bench!(
    bench_powi,
    "Integer Power",
    ("f64::powi", |b| {
        let base = std::hint::black_box(42.0f64);
        let exp = std::hint::black_box(2);
        b.iter(|| base.powi(exp));
    }),
    ("GuardedF64::powi", |b| {
        let base = std::hint::black_box(GuardedF64::new(42.0f64).unwrap());
        let exp = std::hint::black_box(2);
        b.iter(|| base.powi(exp));
    }),
    ("UnguardedF64::powi", |b| {
        let base = std::hint::black_box(UnguardedF64::new(42.0f64));
        let exp = std::hint::black_box(2);
        b.iter(|| base.powi(exp));
    })
);

bench!(
    bench_powf,
    "Floating Point Power",
    ("f64::powf", |b| {
        let base = std::hint::black_box(42.0f64);
        let exp = std::hint::black_box(2.0);
        b.iter(|| base.powf(exp));
    }),
    ("GuardedF64::powf", |b| {
        let base = std::hint::black_box(GuardedF64::new(42.0f64).unwrap());
        let exp = std::hint::black_box(2.0);
        b.iter(|| base.powf(exp));
    }),
    ("UnguardedF64::powf", |b| {
        let base = std::hint::black_box(UnguardedF64::new(42.0f64));
        let exp = std::hint::black_box(2.0);
        b.iter(|| base.powf(exp));
    })
);

bench!(
    bench_sin,
    "Sine",
    ("f64::sin", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| value.sin());
    }),
    ("GuardedF64::sin", |b| {
        let value = std::hint::black_box(GuardedF64::new(42.0f64).unwrap());
        b.iter(|| value.sin());
    }),
    ("UnguardedF64::sin", |b| {
        let value = std::hint::black_box(UnguardedF64::new(42.0f64));
        b.iter(|| value.sin());
    })
);

bench!(
    bench_asin,
    "Arcsine",
    ("f64::asin", |b| {
        let value = std::hint::black_box(0.5f64);
        b.iter(|| value.asin());
    }),
    ("GuardedF64::asin", |b| {
        let value = std::hint::black_box(GuardedF64::new(0.5f64).unwrap());
        b.iter(|| value.asin());
    }),
    ("UnguardedF64::asin", |b| {
        let value = std::hint::black_box(UnguardedF64::new(0.5f64));
        b.iter(|| value.asin());
    })
);

bench!(
    bench_sinh,
    "Hyperbolic Sine",
    ("f64::sinh", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| value.sinh());
    }),
    ("GuardedF64::sinh", |b| {
        let value = std::hint::black_box(GuardedF64::new(42.0f64).unwrap());
        b.iter(|| value.sinh());
    }),
    ("UnguardedF64::sinh", |b| {
        let value = std::hint::black_box(UnguardedF64::new(42.0f64));
        b.iter(|| value.sinh());
    })
);

bench!(
    bench_asinh,
    "Hyperbolic Arcsine",
    ("f64::asinh", |b| {
        let value = std::hint::black_box(0.5f64);
        b.iter(|| value.asinh());
    }),
    ("GuardedF64::asinh", |b| {
        let value = std::hint::black_box(GuardedF64::new(0.5f64).unwrap());
        b.iter(|| value.asinh());
    }),
    ("UnguardedF64::asinh", |b| {
        let value = std::hint::black_box(UnguardedF64::new(0.5f64));
        b.iter(|| value.asinh());
    })
);

bench!(
    bench_cos,
    "Cosine",
    ("f64::cos", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| value.cos());
    }),
    ("GuardedF64::cos", |b| {
        let value = std::hint::black_box(GuardedF64::new(42.0f64).unwrap());
        b.iter(|| value.cos());
    }),
    ("UnguardedF64::cos", |b| {
        let value = std::hint::black_box(UnguardedF64::new(42.0f64));
        b.iter(|| value.cos());
    })
);

bench!(
    bench_acos,
    "Arccosine",
    ("f64::acos", |b| {
        let value = std::hint::black_box(0.5f64);
        b.iter(|| value.acos());
    }),
    ("GuardedF64::acos", |b| {
        let value = std::hint::black_box(GuardedF64::new(0.5f64).unwrap());
        b.iter(|| value.acos());
    }),
    ("UnguardedF64::acos", |b| {
        let value = std::hint::black_box(UnguardedF64::new(0.5f64));
        b.iter(|| value.acos());
    })
);

bench!(
    bench_cosh,
    "Hyperbolic Cosine",
    ("f64::cosh", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| value.cosh());
    }),
    ("GuardedF64::cosh", |b| {
        let value = std::hint::black_box(GuardedF64::new(42.0f64).unwrap());
        b.iter(|| value.cosh());
    }),
    ("UnguardedF64::cosh", |b| {
        let value = std::hint::black_box(UnguardedF64::new(42.0f64));
        b.iter(|| value.cosh());
    })
);

bench!(
    bench_acosh,
    "Hyperbolic Arccosine",
    ("f64::acosh", |b| {
        let value = std::hint::black_box(1.5f64);
        b.iter(|| value.acosh());
    }),
    ("GuardedF64::acosh", |b| {
        let value = std::hint::black_box(GuardedF64::new(1.5f64).unwrap());
        b.iter(|| value.acosh());
    }),
    ("UnguardedF64::acosh", |b| {
        let value = std::hint::black_box(UnguardedF64::new(1.5f64));
        b.iter(|| value.acosh());
    })
);

bench!(
    bench_tan,
    "Tangent",
    ("f64::tan", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| value.tan());
    }),
    ("GuardedF64::tan", |b| {
        let value = std::hint::black_box(GuardedF64::new(42.0f64).unwrap());
        b.iter(|| value.tan());
    }),
    ("UnguardedF64::tan", |b| {
        let value = std::hint::black_box(UnguardedF64::new(42.0f64));
        b.iter(|| value.tan());
    })
);

bench!(
    bench_atan,
    "Arctangent",
    ("f64::atan", |b| {
        let value = std::hint::black_box(0.5f64);
        b.iter(|| value.atan());
    }),
    ("GuardedF64::atan", |b| {
        let value = std::hint::black_box(GuardedF64::new(0.5f64).unwrap());
        b.iter(|| value.atan());
    }),
    ("UnguardedF64::atan", |b| {
        let value = std::hint::black_box(UnguardedF64::new(0.5f64));
        b.iter(|| value.atan());
    })
);

bench!(
    bench_tanh,
    "Hyperbolic Tangent",
    ("f64::tanh", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| value.tanh());
    }),
    ("GuardedF64::tanh", |b| {
        let value = std::hint::black_box(GuardedF64::new(42.0f64).unwrap());
        b.iter(|| value.tanh());
    }),
    ("UnguardedF64::tanh", |b| {
        let value = std::hint::black_box(UnguardedF64::new(42.0f64));
        b.iter(|| value.tanh());
    })
);

bench!(
    bench_atanh,
    "Hyperbolic Arctangent",
    ("f64::atanh", |b| {
        let value = std::hint::black_box(0.5f64);
        b.iter(|| value.atanh());
    }),
    ("GuardedF64::atanh", |b| {
        let value = std::hint::black_box(GuardedF64::new(0.5f64).unwrap());
        b.iter(|| value.atanh());
    }),
    ("UnguardedF64::atanh", |b| {
        let value = std::hint::black_box(UnguardedF64::new(0.5f64));
        b.iter(|| value.atanh());
    })
);

bench!(
    bench_atan2,
    "Arctangent2",
    ("f64::atan2", |b| {
        let y = std::hint::black_box(1.0f64);
        let x = std::hint::black_box(0.5f64);
        b.iter(|| y.atan2(x));
    }),
    ("GuardedF64::atan2", |b| {
        let y = std::hint::black_box(GuardedF64::new(1.0f64).unwrap());
        let x = std::hint::black_box(GuardedF64::new(0.5f64).unwrap());
        b.iter(|| y.atan2(x));
    }),
    ("UnguardedF64::atan2", |b| {
        let y = std::hint::black_box(UnguardedF64::new(1.0f64));
        let x = std::hint::black_box(UnguardedF64::new(0.5f64));
        b.iter(|| y.atan2(x));
    })
);

criterion_group!(
    benches,
    bench_abs,
    bench_signum,
    bench_sqrt,
    bench_recip,
    bench_exp,
    bench_ln,
    bench_log2,
    bench_log10,
    bench_log,
    bench_powi,
    bench_powf,
    bench_sin,
    bench_asin,
    bench_sinh,
    bench_asinh,
    bench_cos,
    bench_acos,
    bench_cosh,
    bench_acosh,
    bench_tan,
    bench_atan,
    bench_tanh,
    bench_atanh,
    bench_atan2,
);
criterion_main!(benches);
