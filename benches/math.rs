use criterion::{criterion_group, criterion_main, Criterion};
use checked_float::{CheckedF64, UncheckedF64};

fn bench_f64_abs(c: &mut Criterion) {
    c.bench_function("f64::abs", |b| {
        let value = std::hint::black_box(-42.0f64);
        b.iter(|| value.abs());
    });
}

fn bench_checked_abs(c: &mut Criterion) {
    c.bench_function("CheckedF64::abs", |b| {
        let value = std::hint::black_box(CheckedF64::new(-42.0f64).unwrap());
        b.iter(|| value.abs());
    });
}

fn bench_unchecked_abs(c: &mut Criterion) {
    c.bench_function("UncheckedF64::abs", |b| {
        let value = std::hint::black_box(UncheckedF64::new(-42.0f64));
        b.iter(|| value.abs());
    });
}

fn bench_f64_signum(c: &mut Criterion) {
    c.bench_function("f64::signum", |b| {
        let value = std::hint::black_box(-42.0f64);
        b.iter(|| value.signum());
    });
}

fn bench_checked_signum(c: &mut Criterion) {
    c.bench_function("CheckedF64::signum", |b| {
        let value = std::hint::black_box(CheckedF64::new(-42.0f64).unwrap());
        b.iter(|| value.signum());
    });
}

fn bench_unchecked_signum(c: &mut Criterion) {
    c.bench_function("UncheckedF64::signum", |b| {
        let value = std::hint::black_box(UncheckedF64::new(-42.0f64));
        b.iter(|| value.signum());
    });
}

fn bench_f64_sqrt(c: &mut Criterion) {
    c.bench_function("f64::sqrt", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| value.sqrt());
    });
}

fn bench_checked_sqrt(c: &mut Criterion) {
    c.bench_function("CheckedF64::sqrt", |b| {
        let value = std::hint::black_box(CheckedF64::new(42.0f64).unwrap());
        b.iter(|| value.sqrt());
    });
}

fn bench_unchecked_sqrt(c: &mut Criterion) {
    c.bench_function("UncheckedF64::sqrt", |b| {
        let value = std::hint::black_box(UncheckedF64::new(42.0f64));
        b.iter(|| value.sqrt());
    });
}

fn bench_f64_recip(c: &mut Criterion) {
    c.bench_function("f64::recip", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| value.recip());
    });
}

fn bench_checked_recip(c: &mut Criterion) {
    c.bench_function("CheckedF64::recip", |b| {
        let value = std::hint::black_box(CheckedF64::new(42.0f64).unwrap());
        b.iter(|| value.recip());
    });
}

fn bench_unchecked_recip(c: &mut Criterion) {
    c.bench_function("UncheckedF64::recip", |b| {
        let value = std::hint::black_box(UncheckedF64::new(42.0f64));
        b.iter(|| value.recip());
    });
}

fn bench_f64_exp(c: &mut Criterion) {
    c.bench_function("f64::exp", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| value.exp());
    });
}

fn bench_checked_exp(c: &mut Criterion) {
    c.bench_function("CheckedF64::exp", |b| {
        let value = std::hint::black_box(CheckedF64::new(42.0f64).unwrap());
        b.iter(|| value.exp());
    });
}

fn bench_unchecked_exp(c: &mut Criterion) {
    c.bench_function("UncheckedF64::exp", |b| {
        let value = std::hint::black_box(UncheckedF64::new(42.0f64));
        b.iter(|| value.exp());
    });
}

fn bench_f64_ln(c: &mut Criterion) {
    c.bench_function("f64::ln", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| value.ln());
    });
}

fn bench_checked_ln(c: &mut Criterion) {
    c.bench_function("CheckedF64::ln", |b| {
        let value = std::hint::black_box(CheckedF64::new(42.0f64).unwrap());
        b.iter(|| value.ln());
    });
}

fn bench_unchecked_ln(c: &mut Criterion) {
    c.bench_function("UncheckedF64::ln", |b| {
        let value = std::hint::black_box(UncheckedF64::new(42.0f64));
        b.iter(|| value.ln());
    });
}

fn bench_f64_log2(c: &mut Criterion) {
    c.bench_function("f64::log2", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| value.log2());
    });
}

fn bench_checked_log2(c: &mut Criterion) {
    c.bench_function("CheckedF64::log2", |b| {
        let value = std::hint::black_box(CheckedF64::new(42.0f64).unwrap());
        b.iter(|| value.log2());
    });
}

fn bench_unchecked_log2(c: &mut Criterion) {
    c.bench_function("UncheckedF64::log2", |b| {
        let value = std::hint::black_box(UncheckedF64::new(42.0f64));
        b.iter(|| value.log2());
    });
}

fn bench_f64_log10(c: &mut Criterion) {
    c.bench_function("f64::log10", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| value.log10());
    });
}

fn bench_checked_log10(c: &mut Criterion) {
    c.bench_function("CheckedF64::log10", |b| {
        let value = std::hint::black_box(CheckedF64::new(42.0f64).unwrap());
        b.iter(|| value.log10());
    });
}

fn bench_unchecked_log10(c: &mut Criterion) {
    c.bench_function("UncheckedF64::log10", |b| {
        let value = std::hint::black_box(UncheckedF64::new(42.0f64));
        b.iter(|| value.log10());
    });
}

fn bench_f64_log(c: &mut Criterion) {
    c.bench_function("f64::log", |b| {
        let value = std::hint::black_box(42.0f64);
        let base = std::hint::black_box(2.0);
        b.iter(|| value.log(base));
    });
}

fn bench_checked_log(c: &mut Criterion) {
    c.bench_function("CheckedF64::log", |b| {
        let value = std::hint::black_box(CheckedF64::new(42.0f64).unwrap());
        let base = std::hint::black_box(2.0);
        b.iter(|| value.log(base));
    });
}

fn bench_unchecked_log(c: &mut Criterion) {
    c.bench_function("UncheckedF64::log", |b| {
        let value = std::hint::black_box(UncheckedF64::new(42.0f64));
        let base = std::hint::black_box(2.0);
        b.iter(|| value.log(base));
    });
}

fn bench_f64_powi(c: &mut Criterion) {
    c.bench_function("f64::powi", |b| {
        let base = std::hint::black_box(42.0f64);
        let exp = std::hint::black_box(2);
        b.iter(|| base.powi(exp));
    });
}

fn bench_checked_powi(c: &mut Criterion) {
    c.bench_function("CheckedF64::powi", |b| {
        let base = std::hint::black_box(CheckedF64::new(42.0f64).unwrap());
        let exp = std::hint::black_box(2);
        b.iter(|| base.powi(exp));
    });
}

fn bench_unchecked_powi(c: &mut Criterion) {
    c.bench_function("UncheckedF64::powi", |b| {
        let base = std::hint::black_box(UncheckedF64::new(42.0f64));
        let exp = std::hint::black_box(2);
        b.iter(|| base.powi(exp));
    });
}

fn bench_f64_powf(c: &mut Criterion) {
    c.bench_function("f64::powf", |b| {
        let base = std::hint::black_box(42.0f64);
        let exp = std::hint::black_box(2.0);
        b.iter(|| base.powf(exp));
    });
}

fn bench_checked_powf(c: &mut Criterion) {
    c.bench_function("CheckedF64::powf", |b| {
        let base = std::hint::black_box(CheckedF64::new(42.0f64).unwrap());
        let exp = std::hint::black_box(2.0);
        b.iter(|| base.powf(exp));
    });
}

fn bench_unchecked_powf(c: &mut Criterion) {
    c.bench_function("UncheckedF64::powf", |b| {
        let base = std::hint::black_box(UncheckedF64::new(42.0f64));
        let exp = std::hint::black_box(2.0);
        b.iter(|| base.powf(exp));
    });
}

fn bench_f64_sin(c: &mut Criterion) {
    c.bench_function("f64::sin", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| value.sin());
    });
}

fn bench_checked_sin(c: &mut Criterion) {
    c.bench_function("CheckedF64::sin", |b| {
        let value = std::hint::black_box(CheckedF64::new(42.0f64).unwrap());
        b.iter(|| value.sin());
    });
}

fn bench_unchecked_sin(c: &mut Criterion) {
    c.bench_function("UncheckedF64::sin", |b| {
        let value = std::hint::black_box(UncheckedF64::new(42.0f64));
        b.iter(|| value.sin());
    });
}

fn bench_f64_asin(c: &mut Criterion) {
    c.bench_function("f64::asin", |b| {
        let value = std::hint::black_box(0.5f64);
        b.iter(|| value.asin());
    });
}

fn bench_checked_asin(c: &mut Criterion) {
    c.bench_function("CheckedF64::asin", |b| {
        let value = std::hint::black_box(CheckedF64::new(0.5f64).unwrap());
        b.iter(|| value.asin());
    });
}

fn bench_unchecked_asin(c: &mut Criterion) {
    c.bench_function("UncheckedF64::asin", |b| {
        let value = std::hint::black_box(UncheckedF64::new(0.5f64));
        b.iter(|| value.asin());
    });
}

fn bench_f64_sinh(c: &mut Criterion) {
    c.bench_function("f64::sinh", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| value.sinh());
    });
}

fn bench_checked_sinh(c: &mut Criterion) {
    c.bench_function("CheckedF64::sinh", |b| {
        let value = std::hint::black_box(CheckedF64::new(42.0f64).unwrap());
        b.iter(|| value.sinh());
    });
}

fn bench_unchecked_sinh(c: &mut Criterion) {
    c.bench_function("UncheckedF64::sinh", |b| {
        let value = std::hint::black_box(UncheckedF64::new(42.0f64));
        b.iter(|| value.sinh());
    });
}

fn bench_f64_asinh(c: &mut Criterion) {
    c.bench_function("f64::asinh", |b| {
        let value = std::hint::black_box(0.5f64);
        b.iter(|| value.asinh());
    });
}

fn bench_checked_asinh(c: &mut Criterion) {
    c.bench_function("CheckedF64::asinh", |b| {
        let value = std::hint::black_box(CheckedF64::new(0.5f64).unwrap());
        b.iter(|| value.asinh());
    });
}

fn bench_unchecked_asinh(c: &mut Criterion) {
    c.bench_function("UncheckedF64::asinh", |b| {
        let value = std::hint::black_box(UncheckedF64::new(0.5f64));
        b.iter(|| value.asinh());
    });
}

fn bench_f64_cos(c: &mut Criterion) {
    c.bench_function("f64::cos", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| value.cos());
    });
}

fn bench_checked_cos(c: &mut Criterion) {
    c.bench_function("CheckedF64::cos", |b| {
        let value = std::hint::black_box(CheckedF64::new(42.0f64).unwrap());
        b.iter(|| value.cos());
    });
}

fn bench_unchecked_cos(c: &mut Criterion) {
    c.bench_function("UncheckedF64::cos", |b| {
        let value = std::hint::black_box(UncheckedF64::new(42.0f64));
        b.iter(|| value.cos());
    });
}

fn bench_f64_acos(c: &mut Criterion) {
    c.bench_function("f64::acos", |b| {
        let value = std::hint::black_box(0.5f64);
        b.iter(|| value.acos());
    });
}

fn bench_checked_acos(c: &mut Criterion) {
    c.bench_function("CheckedF64::acos", |b| {
        let value = std::hint::black_box(CheckedF64::new(0.5f64).unwrap());
        b.iter(|| value.acos());
    });
}

fn bench_unchecked_acos(c: &mut Criterion) {
    c.bench_function("UncheckedF64::acos", |b| {
        let value = std::hint::black_box(UncheckedF64::new(0.5f64));
        b.iter(|| value.acos());
    });
}

fn bench_f64_cosh(c: &mut Criterion) {
    c.bench_function("f64::cosh", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| value.cosh());
    });
}

fn bench_checked_cosh(c: &mut Criterion) {
    c.bench_function("CheckedF64::cosh", |b| {
        let value = std::hint::black_box(CheckedF64::new(42.0f64).unwrap());
        b.iter(|| value.cosh());
    });
}

fn bench_unchecked_cosh(c: &mut Criterion) {
    c.bench_function("UncheckedF64::cosh", |b| {
        let value = std::hint::black_box(UncheckedF64::new(42.0f64));
        b.iter(|| value.cosh());
    });
}

fn bench_f64_acosh(c: &mut Criterion) {
    c.bench_function("f64::acosh", |b| {
        let value = std::hint::black_box(1.5f64);
        b.iter(|| value.acosh());
    });
}

fn bench_checked_acosh(c: &mut Criterion) {
    c.bench_function("CheckedF64::acosh", |b| {
        let value = std::hint::black_box(CheckedF64::new(1.5f64).unwrap());
        b.iter(|| value.acosh());
    });
}

fn bench_unchecked_acosh(c: &mut Criterion) {
    c.bench_function("UncheckedF64::acosh", |b| {
        let value = std::hint::black_box(UncheckedF64::new(1.5f64));
        b.iter(|| value.acosh());
    });
}

fn bench_f64_tan(c: &mut Criterion) {
    c.bench_function("f64::tan", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| value.tan());
    });
}

fn bench_checked_tan(c: &mut Criterion) {
    c.bench_function("CheckedF64::tan", |b| {
        let value = std::hint::black_box(CheckedF64::new(42.0f64).unwrap());
        b.iter(|| value.tan());
    });
}

fn bench_unchecked_tan(c: &mut Criterion) {
    c.bench_function("UncheckedF64::tan", |b| {
        let value = std::hint::black_box(UncheckedF64::new(42.0f64));
        b.iter(|| value.tan());
    });
}

fn bench_f64_atan(c: &mut Criterion) {
    c.bench_function("f64::atan", |b| {
        let value = std::hint::black_box(0.5f64);
        b.iter(|| value.atan());
    });
}

fn bench_checked_atan(c: &mut Criterion) {
    c.bench_function("CheckedF64::atan", |b| {
        let value = std::hint::black_box(CheckedF64::new(0.5f64).unwrap());
        b.iter(|| value.atan());
    });
}

fn bench_unchecked_atan(c: &mut Criterion) {
    c.bench_function("UncheckedF64::atan", |b| {
        let value = std::hint::black_box(UncheckedF64::new(0.5f64));
        b.iter(|| value.atan());
    });
}

fn bench_f64_tanh(c: &mut Criterion) {
    c.bench_function("f64::tanh", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| value.tanh());
    });
}

fn bench_checked_tanh(c: &mut Criterion) {
    c.bench_function("CheckedF64::tanh", |b| {
        let value = std::hint::black_box(CheckedF64::new(42.0f64).unwrap());
        b.iter(|| value.tanh());
    });
}

fn bench_unchecked_tanh(c: &mut Criterion) {
    c.bench_function("UncheckedF64::tanh", |b| {
        let value = std::hint::black_box(UncheckedF64::new(42.0f64));
        b.iter(|| value.tanh());
    });
}

fn bench_f64_atanh(c: &mut Criterion) {
    c.bench_function("f64::atanh", |b| {
        let value = std::hint::black_box(0.5f64);
        b.iter(|| value.atanh());
    });
}

fn bench_checked_atanh(c: &mut Criterion) {
    c.bench_function("CheckedF64::atanh", |b| {
        let value = std::hint::black_box(CheckedF64::new(0.5f64).unwrap());
        b.iter(|| value.atanh());
    });
}

fn bench_unchecked_atanh(c: &mut Criterion) {
    c.bench_function("UncheckedF64::atanh", |b| {
        let value = std::hint::black_box(UncheckedF64::new(0.5f64));
        b.iter(|| value.atanh());
    });
}

fn bench_f64_atan2(c: &mut Criterion) {
    c.bench_function("f64::atan2", |b| {
        let y = std::hint::black_box(1.0f64);
        let x = std::hint::black_box(0.5f64);
        b.iter(|| y.atan2(x));
    });
}

fn bench_checked_atan2(c: &mut Criterion) {
    c.bench_function("CheckedF64::atan2", |b| {
        let y = std::hint::black_box(CheckedF64::new(1.0f64).unwrap());
        let x = std::hint::black_box(CheckedF64::new(0.5f64).unwrap());
        b.iter(|| y.atan2(x));
    });
}

fn bench_unchecked_atan2(c: &mut Criterion) {
    c.bench_function("UncheckedF64::atan2", |b| {
        let y = std::hint::black_box(UncheckedF64::new(1.0f64));
        let x = std::hint::black_box(UncheckedF64::new(0.5f64));
        b.iter(|| y.atan2(x));
    });
}



criterion_group!(
    benches,
    bench_f64_abs,
    bench_checked_abs,
    bench_unchecked_abs,
    bench_f64_signum,
    bench_checked_signum,
    bench_unchecked_signum,
    bench_f64_sqrt,
    bench_checked_sqrt,
    bench_unchecked_sqrt,
    bench_f64_recip,
    bench_checked_recip,
    bench_unchecked_recip,
    bench_f64_exp,
    bench_checked_exp,
    bench_unchecked_exp,
    bench_f64_ln,
    bench_checked_ln,
    bench_unchecked_ln,
    bench_f64_log2,
    bench_checked_log2,
    bench_unchecked_log2,
    bench_f64_log10,
    bench_checked_log10,
    bench_unchecked_log10,
    bench_f64_log,
    bench_checked_log,
    bench_unchecked_log,
    bench_f64_powi,
    bench_checked_powi,
    bench_unchecked_powi,
    bench_f64_powf,
    bench_checked_powf,
    bench_unchecked_powf,
    bench_f64_sin,
    bench_checked_sin,
    bench_unchecked_sin,
    bench_f64_asin,
    bench_checked_asin,
    bench_unchecked_asin,
    bench_f64_sinh,
    bench_checked_sinh,
    bench_unchecked_sinh,
    bench_f64_asinh,
    bench_checked_asinh,
    bench_unchecked_asinh,
    bench_f64_cos,
    bench_checked_cos,
    bench_unchecked_cos,
    bench_f64_acos,
    bench_checked_acos,
    bench_unchecked_acos,
    bench_f64_cosh,
    bench_checked_cosh,
    bench_unchecked_cosh,
    bench_f64_acosh,
    bench_checked_acosh,
    bench_unchecked_acosh,
    bench_f64_tan,
    bench_checked_tan,
    bench_unchecked_tan,
    bench_f64_atan,
    bench_checked_atan,
    bench_unchecked_atan,
    bench_f64_tanh,
    bench_checked_tanh,
    bench_unchecked_tanh,
    bench_f64_atanh,
    bench_checked_atanh,
    bench_unchecked_atanh,
    bench_f64_atan2,
    bench_checked_atan2,
    bench_unchecked_atan2,
);
criterion_main!(benches);
