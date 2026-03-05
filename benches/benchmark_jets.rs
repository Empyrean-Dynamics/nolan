use criterion::{Criterion, black_box, criterion_group, criterion_main};
use nolan::jets::{Jet1, Jet2, hess_size};

fn bench_jet1_constant(c: &mut Criterion) {
    c.bench_function("jet1_6_constant", |b| {
        b.iter(|| black_box(Jet1::<6>::constant(black_box(1.5))))
    });
}

fn bench_jet2_constant(c: &mut Criterion) {
    c.bench_function("jet2_6_constant", |b| {
        b.iter(|| black_box(Jet2::<6, { hess_size(6) }>::constant(black_box(1.5))))
    });
}

fn bench_jet1_variable(c: &mut Criterion) {
    c.bench_function("jet1_6_variable", |b| {
        b.iter(|| black_box(Jet1::<6>::variable(black_box(1.5), 0)))
    });
}

fn bench_jet1_add(c: &mut Criterion) {
    let a = Jet1::<6>::variable(1.5, 0);
    let b = Jet1::<6>::variable(2.5, 1);
    c.bench_function("jet1_6_add", |bench| {
        bench.iter(|| black_box(black_box(a) + black_box(b)))
    });
}

fn bench_jet1_mul(c: &mut Criterion) {
    let a = Jet1::<6>::variable(1.5, 0);
    let b = Jet1::<6>::variable(2.5, 1);
    c.bench_function("jet1_6_mul", |bench| {
        bench.iter(|| black_box(black_box(a) * black_box(b)))
    });
}

fn bench_jet1_div(c: &mut Criterion) {
    let a = Jet1::<6>::variable(1.5, 0);
    let b = Jet1::<6>::variable(2.5, 1);
    c.bench_function("jet1_6_div", |bench| {
        bench.iter(|| black_box(black_box(a) / black_box(b)))
    });
}

fn bench_jet1_mul_scalar(c: &mut Criterion) {
    let a = Jet1::<6>::variable(1.5, 0);
    c.bench_function("jet1_6_mul_scalar", |bench| {
        bench.iter(|| black_box(black_box(a) * black_box(3.0)))
    });
}

fn bench_jet1_sin(c: &mut Criterion) {
    let a = Jet1::<6>::variable(1.5, 0);
    c.bench_function("jet1_6_sin", |bench| {
        bench.iter(|| black_box(black_box(a).sin()))
    });
}

fn bench_jet1_cos(c: &mut Criterion) {
    let a = Jet1::<6>::variable(1.5, 0);
    c.bench_function("jet1_6_cos", |bench| {
        bench.iter(|| black_box(black_box(a).cos()))
    });
}

fn bench_jet1_sqrt(c: &mut Criterion) {
    let a = Jet1::<6>::variable(1.5, 0);
    c.bench_function("jet1_6_sqrt", |bench| {
        bench.iter(|| black_box(black_box(a).sqrt()))
    });
}

fn bench_jet1_powi(c: &mut Criterion) {
    let a = Jet1::<6>::variable(1.5, 0);
    c.bench_function("jet1_6_powi_3", |bench| {
        bench.iter(|| black_box(black_box(a).powi(3)))
    });
}

fn bench_jet1_atan2(c: &mut Criterion) {
    let y = Jet1::<6>::variable(1.5, 0);
    let x = Jet1::<6>::variable(2.5, 1);
    c.bench_function("jet1_6_atan2", |bench| {
        bench.iter(|| black_box(black_box(y).atan2(black_box(x))))
    });
}

fn bench_jet2_add(c: &mut Criterion) {
    let a = Jet2::<6, { hess_size(6) }>::variable(1.5, 0);
    let b = Jet2::<6, { hess_size(6) }>::variable(2.5, 1);
    c.bench_function("jet2_6_add", |bench| {
        bench.iter(|| black_box(black_box(a) + black_box(b)))
    });
}

fn bench_jet2_mul(c: &mut Criterion) {
    let a = Jet2::<6, { hess_size(6) }>::variable(1.5, 0);
    let b = Jet2::<6, { hess_size(6) }>::variable(2.5, 1);
    c.bench_function("jet2_6_mul", |bench| {
        bench.iter(|| black_box(black_box(a) * black_box(b)))
    });
}

fn bench_jet2_sin(c: &mut Criterion) {
    let a = Jet2::<6, { hess_size(6) }>::variable(1.5, 0);
    c.bench_function("jet2_6_sin", |bench| {
        bench.iter(|| black_box(black_box(a).sin()))
    });
}

/// Simulate a typical acceleration computation pattern:
/// r = sqrt(x^2 + y^2 + z^2), then a = -mu/r^3 * [x, y, z]
fn bench_jet1_gravity_pattern(c: &mut Criterion) {
    let x = Jet1::<6>::variable(1.0, 0);
    let y = Jet1::<6>::variable(0.5, 1);
    let z = Jet1::<6>::variable(0.1, 2);
    let mu = 1.327e11; // GM_sun in km^3/s^2

    c.bench_function("jet1_6_gravity_accel", |bench| {
        bench.iter(|| {
            let x = black_box(x);
            let y = black_box(y);
            let z = black_box(z);
            let r2 = x * x + y * y + z * z;
            let r = r2.sqrt();
            let r3_inv = r.powi(-3) * mu;
            let _ax = x * r3_inv;
            let _ay = y * r3_inv;
            let _az = z * r3_inv;
            black_box((_ax, _ay, _az))
        })
    });
}

criterion_group!(
    benches,
    bench_jet1_constant,
    bench_jet2_constant,
    bench_jet1_variable,
    bench_jet1_add,
    bench_jet1_mul,
    bench_jet1_div,
    bench_jet1_mul_scalar,
    bench_jet1_sin,
    bench_jet1_cos,
    bench_jet1_sqrt,
    bench_jet1_powi,
    bench_jet1_atan2,
    bench_jet2_add,
    bench_jet2_mul,
    bench_jet2_sin,
    bench_jet1_gravity_pattern,
);
criterion_main!(benches);
