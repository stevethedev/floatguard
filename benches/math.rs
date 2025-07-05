use criterion::{criterion_group, criterion_main, Criterion};
use checked_float::CheckedF64;

fn bench_f64_abs(c: &mut Criterion) {
    c.bench_function("f64::abs", |b| {
        let value = std::hint::black_box(-42.0f64);
        b.iter(|| value.abs());
    });
}

fn bench_checked_f64_abs(c: &mut Criterion) {
    c.bench_function("CheckedF64::abs", |b| {
        let value = CheckedF64::new(std::hint::black_box(-42.0f64)).unwrap();
        b.iter(|| value.abs().unwrap());
    });
}

fn bench_checked_f64_result_abs(c: &mut Criterion) {
    c.bench_function("CheckedF64Result::abs", |b| {
        let value = CheckedF64::new(std::hint::black_box(-42.0f64));
        b.iter(|| value.abs());
    });
}

fn bench_f64_signum(c: &mut Criterion) {
    c.bench_function("f64::signum", |b| {
        let value = std::hint::black_box(-42.0f64);
        b.iter(|| value.signum());
    });
}

fn bench_checked_f64_signum(c: &mut Criterion) {
    c.bench_function("CheckedF64::signum", |b| {
        let value = CheckedF64::new(std::hint::black_box(-42.0f64)).unwrap();
        b.iter(|| value.signum().unwrap());
    });
}

fn bench_checked_f64_result_signum(c: &mut Criterion) {
    c.bench_function("CheckedF64Result::signum", |b| {
        let value = CheckedF64::new(std::hint::black_box(-42.0f64));
        b.iter(|| value.signum());
    });
}

fn bench_f64_sqrt(c: &mut Criterion) {
    c.bench_function("f64::sqrt", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| value.sqrt());
    });
}

fn bench_checked_f64_sqrt(c: &mut Criterion) {
    c.bench_function("CheckedF64::sqrt", |b| {
        let value = CheckedF64::new(std::hint::black_box(42.0f64)).unwrap();
        b.iter(|| value.sqrt().unwrap());
    });
}

fn bench_checked_f64_result_sqrt(c: &mut Criterion) {
    c.bench_function("CheckedF64Result::sqrt", |b| {
        let value = CheckedF64::new(std::hint::black_box(42.0f64));
        b.iter(|| value.sqrt());
    });
}

fn bench_f64_recip(c: &mut Criterion) {
    c.bench_function("f64::recip", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| value.recip());
    });
}

fn bench_checked_f64_recip(c: &mut Criterion) {
    c.bench_function("CheckedF64::recip", |b| {
        let value = CheckedF64::new(std::hint::black_box(42.0f64)).unwrap();
        b.iter(|| value.recip().unwrap());
    });
}

fn bench_checked_f64_result_recip(c: &mut Criterion) {
    c.bench_function("CheckedF64Result::recip", |b| {
        let value = CheckedF64::new(std::hint::black_box(42.0f64));
        b.iter(|| value.recip());
    });
}

fn bench_f64_exp(c: &mut Criterion) {
    c.bench_function("f64::exp", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| value.exp());
    });
}

fn bench_checked_f64_exp(c: &mut Criterion) {
    c.bench_function("CheckedF64::exp", |b| {
        let value = CheckedF64::new(std::hint::black_box(42.0f64)).unwrap();
        b.iter(|| value.exp().unwrap());
    });
}

fn bench_checked_f64_result_exp(c: &mut Criterion) {
    c.bench_function("CheckedF64Result::exp", |b| {
        let value = CheckedF64::new(std::hint::black_box(42.0f64));
        b.iter(|| value.exp());
    });
}

fn bench_f64_ln(c: &mut Criterion) {
    c.bench_function("f64::ln", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| value.ln());
    });
}

fn bench_checked_f64_ln(c: &mut Criterion) {
    c.bench_function("CheckedF64::ln", |b| {
        let value = CheckedF64::new(std::hint::black_box(42.0f64)).unwrap();
        b.iter(|| value.ln().unwrap());
    });
}

fn bench_checked_f64_result_ln(c: &mut Criterion) {
    c.bench_function("CheckedF64Result::ln", |b| {
        let value = CheckedF64::new(std::hint::black_box(42.0f64));
        b.iter(|| value.ln());
    });
}

fn bench_f64_log2(c: &mut Criterion) {
    c.bench_function("f64::log2", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| value.log2());
    });
}

fn bench_checked_f64_log2(c: &mut Criterion) {
    c.bench_function("CheckedF64::log2", |b| {
        let value = CheckedF64::new(std::hint::black_box(42.0f64)).unwrap();
        b.iter(|| value.log2().unwrap());
    });
}

fn bench_checked_f64_result_log2(c: &mut Criterion) {
    c.bench_function("CheckedF64Result::log2", |b| {
        let value = CheckedF64::new(std::hint::black_box(42.0f64));
        b.iter(|| value.log2());
    });
}

fn bench_f64_log10(c: &mut Criterion) {
    c.bench_function("f64::log10", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| value.log10());
    });
}

fn bench_checked_f64_log10(c: &mut Criterion) {
    c.bench_function("CheckedF64::log10", |b| {
        let value = CheckedF64::new(std::hint::black_box(42.0f64)).unwrap();
        b.iter(|| value.log10().unwrap());
    });
}

fn bench_checked_f64_result_log10(c: &mut Criterion) {
    c.bench_function("CheckedF64Result::log10", |b| {
        let value = CheckedF64::new(std::hint::black_box(42.0f64));
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

fn bench_checked_f64_log(c: &mut Criterion) {
    c.bench_function("CheckedF64::log", |b| {
        let value = CheckedF64::new(std::hint::black_box(42.0f64)).unwrap();
        let base = std::hint::black_box(2.0);
        b.iter(|| value.log(base).unwrap());
    });
}

fn bench_checked_f64_result_log(c: &mut Criterion) {
    c.bench_function("CheckedF64Result::log", |b| {
        let value = CheckedF64::new(std::hint::black_box(42.0f64));
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

fn bench_checked_f64_powi(c: &mut Criterion) {
    c.bench_function("CheckedF64::powi", |b| {
        let base = CheckedF64::new(std::hint::black_box(42.0f64)).unwrap();
        let exp = std::hint::black_box(2);
        b.iter(|| base.powi(exp).unwrap());
    });
}

fn bench_checked_f64_result_powi(c: &mut Criterion) {
    c.bench_function("CheckedF64Result::powi", |b| {
        let base = CheckedF64::new(std::hint::black_box(42.0f64));
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

fn bench_checked_f64_powf(c: &mut Criterion) {
    c.bench_function("CheckedF64::powf", |b| {
        let base = CheckedF64::new(std::hint::black_box(42.0f64)).unwrap();
        let exp = std::hint::black_box(2.0);
        b.iter(|| base.powf(exp).unwrap());
    });
}

fn bench_checked_f64_result_powf(c: &mut Criterion) {
    c.bench_function("CheckedF64Result::powf", |b| {
        let base = CheckedF64::new(std::hint::black_box(42.0f64));
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

fn bench_checked_f64_sin(c: &mut Criterion) {
    c.bench_function("CheckedF64::sin", |b| {
        let value = CheckedF64::new(std::hint::black_box(42.0f64)).unwrap();
        b.iter(|| value.sin().unwrap());
    });
}

fn bench_checked_f64_result_sin(c: &mut Criterion) {
    c.bench_function("CheckedF64Result::sin", |b| {
        let value = CheckedF64::new(std::hint::black_box(42.0f64));
        b.iter(|| value.sin());
    });
}

fn bench_f64_asin(c: &mut Criterion) {
    c.bench_function("f64::asin", |b| {
        let value = std::hint::black_box(0.5f64);
        b.iter(|| value.asin());
    });
}

fn bench_checked_f64_asin(c: &mut Criterion) {
    c.bench_function("CheckedF64::asin", |b| {
        let value = CheckedF64::new(std::hint::black_box(0.5f64)).unwrap();
        b.iter(|| value.asin().unwrap());
    });
}

fn bench_checked_f64_result_asin(c: &mut Criterion) {
    c.bench_function("CheckedF64Result::asin", |b| {
        let value = CheckedF64::new(std::hint::black_box(0.5f64));
        b.iter(|| value.asin());
    });
}

fn bench_f64_sinh(c: &mut Criterion) {
    c.bench_function("f64::sinh", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| value.sinh());
    });
}

fn bench_checked_f64_sinh(c: &mut Criterion) {
    c.bench_function("CheckedF64::sinh", |b| {
        let value = CheckedF64::new(std::hint::black_box(42.0f64)).unwrap();
        b.iter(|| value.sinh().unwrap());
    });
}

fn bench_checked_f64_result_sinh(c: &mut Criterion) {
    c.bench_function("CheckedF64Result::sinh", |b| {
        let value = CheckedF64::new(std::hint::black_box(42.0f64));
        b.iter(|| value.sinh());
    });
}

fn bench_f64_asinh(c: &mut Criterion) {
    c.bench_function("f64::asinh", |b| {
        let value = std::hint::black_box(0.5f64);
        b.iter(|| value.asinh());
    });
}

fn bench_checked_f64_asinh(c: &mut Criterion) {
    c.bench_function("CheckedF64::asinh", |b| {
        let value = CheckedF64::new(std::hint::black_box(0.5f64)).unwrap();
        b.iter(|| value.asinh().unwrap());
    });
}

fn bench_checked_f64_result_asinh(c: &mut Criterion) {
    c.bench_function("CheckedF64Result::asinh", |b| {
        let value = CheckedF64::new(std::hint::black_box(0.5f64));
        b.iter(|| value.asinh());
    });
}

fn bench_f64_cos(c: &mut Criterion) {
    c.bench_function("f64::cos", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| value.cos());
    });
}

fn bench_checked_f64_cos(c: &mut Criterion) {
    c.bench_function("CheckedF64::cos", |b| {
        let value = CheckedF64::new(std::hint::black_box(42.0f64)).unwrap();
        b.iter(|| value.cos().unwrap());
    });
}

fn bench_checked_f64_result_cos(c: &mut Criterion) {
    c.bench_function("CheckedF64Result::cos", |b| {
        let value = CheckedF64::new(std::hint::black_box(42.0f64));
        b.iter(|| value.cos());
    });
}

fn bench_f64_acos(c: &mut Criterion) {
    c.bench_function("f64::acos", |b| {
        let value = std::hint::black_box(0.5f64);
        b.iter(|| value.acos());
    });
}

fn bench_checked_f64_acos(c: &mut Criterion) {
    c.bench_function("CheckedF64::acos", |b| {
        let value = CheckedF64::new(std::hint::black_box(0.5f64)).unwrap();
        b.iter(|| value.acos().unwrap());
    });
}

fn bench_checked_f64_result_acos(c: &mut Criterion) {
    c.bench_function("CheckedF64Result::acos", |b| {
        let value = CheckedF64::new(std::hint::black_box(0.5f64));
        b.iter(|| value.acos());
    });
}

fn bench_f64_cosh(c: &mut Criterion) {
    c.bench_function("f64::cosh", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| value.cosh());
    });
}

fn bench_checked_f64_cosh(c: &mut Criterion) {
    c.bench_function("CheckedF64::cosh", |b| {
        let value = CheckedF64::new(std::hint::black_box(42.0f64)).unwrap();
        b.iter(|| value.cosh().unwrap());
    });
}

fn bench_checked_f64_result_cosh(c: &mut Criterion) {
    c.bench_function("CheckedF64Result::cosh", |b| {
        let value = CheckedF64::new(std::hint::black_box(42.0f64));
        b.iter(|| value.cosh());
    });
}

fn bench_f64_acosh(c: &mut Criterion) {
    c.bench_function("f64::acosh", |b| {
        let value = std::hint::black_box(1.5f64);
        b.iter(|| value.acosh());
    });
}

fn bench_checked_f64_acosh(c: &mut Criterion) {
    c.bench_function("CheckedF64::acosh", |b| {
        let value = CheckedF64::new(std::hint::black_box(1.5f64)).unwrap();
        b.iter(|| value.acosh().unwrap());
    });
}

fn bench_checked_f64_result_acosh(c: &mut Criterion) {
    c.bench_function("CheckedF64Result::acosh", |b| {
        let value = CheckedF64::new(std::hint::black_box(1.5f64));
        b.iter(|| value.acosh());
    });
}

fn bench_f64_tan(c: &mut Criterion) {
    c.bench_function("f64::tan", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| value.tan());
    });
}

fn bench_checked_f64_tan(c: &mut Criterion) {
    c.bench_function("CheckedF64::tan", |b| {
        let value = CheckedF64::new(std::hint::black_box(42.0f64)).unwrap();
        b.iter(|| value.tan().unwrap());
    });
}

fn bench_checked_f64_result_tan(c: &mut Criterion) {
    c.bench_function("CheckedF64Result::tan", |b| {
        let value = CheckedF64::new(std::hint::black_box(42.0f64));
        b.iter(|| value.tan());
    });
}

fn bench_f64_atan(c: &mut Criterion) {
    c.bench_function("f64::atan", |b| {
        let value = std::hint::black_box(0.5f64);
        b.iter(|| value.atan());
    });
}

fn bench_checked_f64_atan(c: &mut Criterion) {
    c.bench_function("CheckedF64::atan", |b| {
        let value = CheckedF64::new(std::hint::black_box(0.5f64)).unwrap();
        b.iter(|| value.atan().unwrap());
    });
}

fn bench_checked_f64_result_atan(c: &mut Criterion) {
    c.bench_function("CheckedF64Result::atan", |b| {
        let value = CheckedF64::new(std::hint::black_box(0.5f64));
        b.iter(|| value.atan());
    });
}

fn bench_f64_tanh(c: &mut Criterion) {
    c.bench_function("f64::tanh", |b| {
        let value = std::hint::black_box(42.0f64);
        b.iter(|| value.tanh());
    });
}

fn bench_checked_f64_tanh(c: &mut Criterion) {
    c.bench_function("CheckedF64::tanh", |b| {
        let value = CheckedF64::new(std::hint::black_box(42.0f64)).unwrap();
        b.iter(|| value.tanh().unwrap());
    });
}

fn bench_checked_f64_result_tanh(c: &mut Criterion) {
    c.bench_function("CheckedF64Result::tanh", |b| {
        let value = CheckedF64::new(std::hint::black_box(42.0f64));
        b.iter(|| value.tanh());
    });
}

fn bench_f64_atanh(c: &mut Criterion) {
    c.bench_function("f64::atanh", |b| {
        let value = std::hint::black_box(0.5f64);
        b.iter(|| value.atanh());
    });
}

fn bench_checked_f64_atanh(c: &mut Criterion) {
    c.bench_function("CheckedF64::atanh", |b| {
        let value = CheckedF64::new(std::hint::black_box(0.5f64)).unwrap();
        b.iter(|| value.atanh().unwrap());
    });
}

fn bench_checked_f64_result_atanh(c: &mut Criterion) {
    c.bench_function("CheckedF64Result::atanh", |b| {
        let value = CheckedF64::new(std::hint::black_box(0.5f64));
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

fn bench_checked_f64_atan2(c: &mut Criterion) {
    c.bench_function("CheckedF64::atan2", |b| {
        let y = CheckedF64::new(std::hint::black_box(1.0f64)).unwrap();
        let x = CheckedF64::new(std::hint::black_box(0.5f64)).unwrap();
        b.iter(|| y.atan2(x).unwrap());
    });
}

fn bench_checked_f64_result_atan2(c: &mut Criterion) {
    c.bench_function("CheckedF64Result::atan2", |b| {
        let y = CheckedF64::new(std::hint::black_box(1.0f64));
        let x = CheckedF64::new(std::hint::black_box(0.5f64));
        b.iter(|| y.atan2(x));
    });
}



criterion_group!(
    benches,
    bench_f64_abs,
    bench_checked_f64_abs,
    bench_checked_f64_result_abs,
    bench_f64_signum,
    bench_checked_f64_signum,
    bench_checked_f64_result_signum,
    bench_f64_sqrt,
    bench_checked_f64_sqrt,
    bench_checked_f64_result_sqrt,
    bench_f64_recip,
    bench_checked_f64_recip,
    bench_checked_f64_result_recip,
    bench_f64_exp,
    bench_checked_f64_exp,
    bench_checked_f64_result_exp,
    bench_f64_ln,
    bench_checked_f64_ln,
    bench_checked_f64_result_ln,
    bench_f64_log2,
    bench_checked_f64_log2,
    bench_checked_f64_result_log2,
    bench_f64_log10,
    bench_checked_f64_log10,
    bench_checked_f64_result_log10,
    bench_f64_log,
    bench_checked_f64_log,
    bench_checked_f64_result_log,
    bench_f64_powi,
    bench_checked_f64_powi,
    bench_checked_f64_result_powi,
    bench_f64_powf,
    bench_checked_f64_powf,
    bench_checked_f64_result_powf,
    bench_f64_sin,
    bench_checked_f64_sin,
    bench_checked_f64_result_sin,
    bench_f64_asin,
    bench_checked_f64_asin,
    bench_checked_f64_result_asin,
    bench_f64_sinh,
    bench_checked_f64_sinh,
    bench_checked_f64_result_sinh,
    bench_f64_asinh,
    bench_checked_f64_asinh,
    bench_checked_f64_result_asinh,
    bench_f64_cos,
    bench_checked_f64_cos,
    bench_checked_f64_result_cos,
    bench_f64_acos,
    bench_checked_f64_acos,
    bench_checked_f64_result_acos,
    bench_f64_cosh,
    bench_checked_f64_cosh,
    bench_checked_f64_result_cosh,
    bench_f64_acosh,
    bench_checked_f64_acosh,
    bench_checked_f64_result_acosh,
    bench_f64_tan,
    bench_checked_f64_tan,
    bench_checked_f64_result_tan,
    bench_f64_atan,
    bench_checked_f64_atan,
    bench_checked_f64_result_atan,
    bench_f64_tanh,
    bench_checked_f64_tanh,
    bench_checked_f64_result_tanh,
    bench_f64_atanh,
    bench_checked_f64_atanh,
    bench_checked_f64_result_atanh,
    bench_f64_atan2,
    bench_checked_f64_atan2,
    bench_checked_f64_result_atan2,
);
criterion_main!(benches);
