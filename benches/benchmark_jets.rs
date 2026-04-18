use criterion::{Criterion, black_box, criterion_group, criterion_main};
use nolan::jets::{Jet1, Jet2, Jet3, hess_size, tens_size};
use nolan::traits::{FirstOrder, SecondOrder, ThirdOrder};

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

fn bench_jet2_variable(c: &mut Criterion) {
    c.bench_function("jet2_6_variable", |b| {
        b.iter(|| black_box(Jet2::<6, { hess_size(6) }>::variable(black_box(1.5), 0)))
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

fn bench_jet2_div(c: &mut Criterion) {
    let a = Jet2::<6, { hess_size(6) }>::variable(1.5, 0);
    let b = Jet2::<6, { hess_size(6) }>::variable(2.5, 1);
    c.bench_function("jet2_6_div", |bench| {
        bench.iter(|| black_box(black_box(a) / black_box(b)))
    });
}

fn bench_jet2_mul_scalar(c: &mut Criterion) {
    let a = Jet2::<6, { hess_size(6) }>::variable(1.5, 0);
    c.bench_function("jet2_6_mul_scalar", |bench| {
        bench.iter(|| black_box(black_box(a) * black_box(3.0)))
    });
}

fn bench_jet2_sin(c: &mut Criterion) {
    let a = Jet2::<6, { hess_size(6) }>::variable(1.5, 0);
    c.bench_function("jet2_6_sin", |bench| {
        bench.iter(|| black_box(black_box(a).sin()))
    });
}

fn bench_jet2_cos(c: &mut Criterion) {
    let a = Jet2::<6, { hess_size(6) }>::variable(1.5, 0);
    c.bench_function("jet2_6_cos", |bench| {
        bench.iter(|| black_box(black_box(a).cos()))
    });
}

fn bench_jet2_sqrt(c: &mut Criterion) {
    let a = Jet2::<6, { hess_size(6) }>::variable(1.5, 0);
    c.bench_function("jet2_6_sqrt", |bench| {
        bench.iter(|| black_box(black_box(a).sqrt()))
    });
}

fn bench_jet2_powi(c: &mut Criterion) {
    let a = Jet2::<6, { hess_size(6) }>::variable(1.5, 0);
    c.bench_function("jet2_6_powi_3", |bench| {
        bench.iter(|| black_box(black_box(a).powi(3)))
    });
}

fn bench_jet2_atan2(c: &mut Criterion) {
    let y = Jet2::<6, { hess_size(6) }>::variable(1.5, 0);
    let x = Jet2::<6, { hess_size(6) }>::variable(2.5, 1);
    c.bench_function("jet2_6_atan2", |bench| {
        bench.iter(|| black_box(black_box(y).atan2(black_box(x))))
    });
}

fn bench_jet2_gravity_pattern(c: &mut Criterion) {
    let x = Jet2::<6, { hess_size(6) }>::variable(1.0, 0);
    let y = Jet2::<6, { hess_size(6) }>::variable(0.5, 1);
    let z = Jet2::<6, { hess_size(6) }>::variable(0.1, 2);
    let mu = 1.327e11;

    c.bench_function("jet2_6_gravity_accel", |bench| {
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

// ─── extract_grad benchmarks ──────────────────────────────────

fn bench_jet1_extract_grad_6(c: &mut Criterion) {
    // Build a Jet1<6> with non-trivial gradients (gravity pattern output)
    let x = Jet1::<6>::variable(1.0, 0);
    let y = Jet1::<6>::variable(0.5, 1);
    let z = Jet1::<6>::variable(0.1, 2);
    let r2 = x * x + y * y + z * z;
    let r = r2.sqrt();
    let f = r.powi(-3) * x;

    c.bench_function("jet1_6_extract_grad", |bench| {
        bench.iter(|| black_box(black_box(f).extract_grad::<6>()))
    });
}

fn bench_jet1_extract_grad_9(c: &mut Criterion) {
    let x = Jet1::<9>::variable(1.0, 0);
    let y = Jet1::<9>::variable(0.5, 1);
    let z = Jet1::<9>::variable(0.1, 2);
    let r2 = x * x + y * y + z * z;
    let r = r2.sqrt();
    let f = r.powi(-3) * x;

    c.bench_function("jet1_9_extract_grad", |bench| {
        bench.iter(|| black_box(black_box(f).extract_grad::<9>()))
    });
}

// ─── extract_hess benchmarks ──────────────────────────────────

fn bench_jet2_extract_hess_6(c: &mut Criterion) {
    // Build a Jet2<6,21> with non-trivial Hessian (gravity pattern)
    let x = Jet2::<6, { hess_size(6) }>::variable(1.0, 0);
    let y = Jet2::<6, { hess_size(6) }>::variable(0.5, 1);
    let z = Jet2::<6, { hess_size(6) }>::variable(0.1, 2);
    let r2 = x * x + y * y + z * z;
    let r = r2.sqrt();
    let f = r.powi(-3) * x;

    c.bench_function("jet2_6_extract_hess", |bench| {
        bench.iter(|| black_box(black_box(f).extract_hess::<6>()))
    });
}

fn bench_jet2_extract_hess_9(c: &mut Criterion) {
    let x = Jet2::<9, { hess_size(9) }>::variable(1.0, 0);
    let y = Jet2::<9, { hess_size(9) }>::variable(0.5, 1);
    let z = Jet2::<9, { hess_size(9) }>::variable(0.1, 2);
    let r2 = x * x + y * y + z * z;
    let r = r2.sqrt();
    let f = r.powi(-3) * x;

    c.bench_function("jet2_9_extract_hess", |bench| {
        bench.iter(|| black_box(black_box(f).extract_hess::<9>()))
    });
}

// ─── Jet3 benchmarks ────────────────────────────────────────────

fn bench_jet3_constant(c: &mut Criterion) {
    c.bench_function("jet3_6_constant", |b| {
        b.iter(|| {
            black_box(Jet3::<6, { hess_size(6) }, { tens_size(6) }>::constant(
                black_box(1.5),
            ))
        })
    });
}

fn bench_jet3_variable(c: &mut Criterion) {
    c.bench_function("jet3_6_variable", |b| {
        b.iter(|| {
            black_box(Jet3::<6, { hess_size(6) }, { tens_size(6) }>::variable(
                black_box(1.5),
                0,
            ))
        })
    });
}

fn bench_jet3_add(c: &mut Criterion) {
    let a = Jet3::<6, { hess_size(6) }, { tens_size(6) }>::variable(1.5, 0);
    let b = Jet3::<6, { hess_size(6) }, { tens_size(6) }>::variable(2.5, 1);
    c.bench_function("jet3_6_add", |bench| {
        bench.iter(|| black_box(black_box(a) + black_box(b)))
    });
}

fn bench_jet3_mul(c: &mut Criterion) {
    let a = Jet3::<6, { hess_size(6) }, { tens_size(6) }>::variable(1.5, 0);
    let b = Jet3::<6, { hess_size(6) }, { tens_size(6) }>::variable(2.5, 1);
    c.bench_function("jet3_6_mul", |bench| {
        bench.iter(|| black_box(black_box(a) * black_box(b)))
    });
}

fn bench_jet3_div(c: &mut Criterion) {
    let a = Jet3::<6, { hess_size(6) }, { tens_size(6) }>::variable(1.5, 0);
    let b = Jet3::<6, { hess_size(6) }, { tens_size(6) }>::variable(2.5, 1);
    c.bench_function("jet3_6_div", |bench| {
        bench.iter(|| black_box(black_box(a) / black_box(b)))
    });
}

fn bench_jet3_mul_scalar(c: &mut Criterion) {
    let a = Jet3::<6, { hess_size(6) }, { tens_size(6) }>::variable(1.5, 0);
    c.bench_function("jet3_6_mul_scalar", |bench| {
        bench.iter(|| black_box(black_box(a) * black_box(3.0)))
    });
}

fn bench_jet3_sin(c: &mut Criterion) {
    let a = Jet3::<6, { hess_size(6) }, { tens_size(6) }>::variable(1.5, 0);
    c.bench_function("jet3_6_sin", |bench| {
        bench.iter(|| black_box(black_box(a).sin()))
    });
}

fn bench_jet3_cos(c: &mut Criterion) {
    let a = Jet3::<6, { hess_size(6) }, { tens_size(6) }>::variable(1.5, 0);
    c.bench_function("jet3_6_cos", |bench| {
        bench.iter(|| black_box(black_box(a).cos()))
    });
}

fn bench_jet3_sqrt(c: &mut Criterion) {
    let a = Jet3::<6, { hess_size(6) }, { tens_size(6) }>::variable(1.5, 0);
    c.bench_function("jet3_6_sqrt", |bench| {
        bench.iter(|| black_box(black_box(a).sqrt()))
    });
}

fn bench_jet3_powi(c: &mut Criterion) {
    let a = Jet3::<6, { hess_size(6) }, { tens_size(6) }>::variable(1.5, 0);
    c.bench_function("jet3_6_powi_3", |bench| {
        bench.iter(|| black_box(black_box(a).powi(3)))
    });
}

fn bench_jet3_atan2(c: &mut Criterion) {
    let y = Jet3::<6, { hess_size(6) }, { tens_size(6) }>::variable(1.5, 0);
    let x = Jet3::<6, { hess_size(6) }, { tens_size(6) }>::variable(2.5, 1);
    c.bench_function("jet3_6_atan2", |bench| {
        bench.iter(|| black_box(black_box(y).atan2(black_box(x))))
    });
}

fn bench_jet3_gravity_pattern(c: &mut Criterion) {
    let x = Jet3::<6, { hess_size(6) }, { tens_size(6) }>::variable(1.0, 0);
    let y = Jet3::<6, { hess_size(6) }, { tens_size(6) }>::variable(0.5, 1);
    let z = Jet3::<6, { hess_size(6) }, { tens_size(6) }>::variable(0.1, 2);
    let mu = 1.327e11;

    c.bench_function("jet3_6_gravity_accel", |bench| {
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

fn bench_jet3_extract_tens_6(c: &mut Criterion) {
    let x = Jet3::<6, { hess_size(6) }, { tens_size(6) }>::variable(1.0, 0);
    let y = Jet3::<6, { hess_size(6) }, { tens_size(6) }>::variable(0.5, 1);
    let z = Jet3::<6, { hess_size(6) }, { tens_size(6) }>::variable(0.1, 2);
    let r2 = x * x + y * y + z * z;
    let r = r2.sqrt();
    let f = r.powi(-3) * x;

    c.bench_function("jet3_6_extract_tens", |bench| {
        bench.iter(|| black_box(black_box(f).extract_tens::<6>()))
    });
}

fn bench_jet3_extract_tens_9(c: &mut Criterion) {
    let x = Jet3::<9, { hess_size(9) }, { tens_size(9) }>::variable(1.0, 0);
    let y = Jet3::<9, { hess_size(9) }, { tens_size(9) }>::variable(0.5, 1);
    let z = Jet3::<9, { hess_size(9) }, { tens_size(9) }>::variable(0.1, 2);
    let r2 = x * x + y * y + z * z;
    let r = r2.sqrt();
    let f = r.powi(-3) * x;

    c.bench_function("jet3_9_extract_tens", |bench| {
        bench.iter(|| black_box(black_box(f).extract_tens::<9>()))
    });
}

// ─── N=9 op benches (parity across Jet1/Jet2/Jet3 at 9-parameter size) ──

fn bench_jet1_9_add(c: &mut Criterion) {
    let a = Jet1::<9>::variable(1.5, 0);
    let b = Jet1::<9>::variable(2.5, 1);
    c.bench_function("jet1_9_add", |bench| {
        bench.iter(|| black_box(black_box(a) + black_box(b)))
    });
}

fn bench_jet1_9_mul(c: &mut Criterion) {
    let a = Jet1::<9>::variable(1.5, 0);
    let b = Jet1::<9>::variable(2.5, 1);
    c.bench_function("jet1_9_mul", |bench| {
        bench.iter(|| black_box(black_box(a) * black_box(b)))
    });
}

fn bench_jet1_9_sin(c: &mut Criterion) {
    let a = Jet1::<9>::variable(1.5, 0);
    c.bench_function("jet1_9_sin", |bench| {
        bench.iter(|| black_box(black_box(a).sin()))
    });
}

fn bench_jet1_9_gravity_pattern(c: &mut Criterion) {
    let x = Jet1::<9>::variable(1.0, 0);
    let y = Jet1::<9>::variable(0.5, 1);
    let z = Jet1::<9>::variable(0.1, 2);
    let mu = 1.327e11;

    c.bench_function("jet1_9_gravity_accel", |bench| {
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

fn bench_jet2_9_add(c: &mut Criterion) {
    let a = Jet2::<9, { hess_size(9) }>::variable(1.5, 0);
    let b = Jet2::<9, { hess_size(9) }>::variable(2.5, 1);
    c.bench_function("jet2_9_add", |bench| {
        bench.iter(|| black_box(black_box(a) + black_box(b)))
    });
}

fn bench_jet2_9_mul(c: &mut Criterion) {
    let a = Jet2::<9, { hess_size(9) }>::variable(1.5, 0);
    let b = Jet2::<9, { hess_size(9) }>::variable(2.5, 1);
    c.bench_function("jet2_9_mul", |bench| {
        bench.iter(|| black_box(black_box(a) * black_box(b)))
    });
}

fn bench_jet2_9_sin(c: &mut Criterion) {
    let a = Jet2::<9, { hess_size(9) }>::variable(1.5, 0);
    c.bench_function("jet2_9_sin", |bench| {
        bench.iter(|| black_box(black_box(a).sin()))
    });
}

fn bench_jet2_9_gravity_pattern(c: &mut Criterion) {
    let x = Jet2::<9, { hess_size(9) }>::variable(1.0, 0);
    let y = Jet2::<9, { hess_size(9) }>::variable(0.5, 1);
    let z = Jet2::<9, { hess_size(9) }>::variable(0.1, 2);
    let mu = 1.327e11;

    c.bench_function("jet2_9_gravity_accel", |bench| {
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

fn bench_jet3_9_add(c: &mut Criterion) {
    let a = Jet3::<9, { hess_size(9) }, { tens_size(9) }>::variable(1.5, 0);
    let b = Jet3::<9, { hess_size(9) }, { tens_size(9) }>::variable(2.5, 1);
    c.bench_function("jet3_9_add", |bench| {
        bench.iter(|| black_box(black_box(a) + black_box(b)))
    });
}

fn bench_jet3_9_mul(c: &mut Criterion) {
    let a = Jet3::<9, { hess_size(9) }, { tens_size(9) }>::variable(1.5, 0);
    let b = Jet3::<9, { hess_size(9) }, { tens_size(9) }>::variable(2.5, 1);
    c.bench_function("jet3_9_mul", |bench| {
        bench.iter(|| black_box(black_box(a) * black_box(b)))
    });
}

fn bench_jet3_9_sin(c: &mut Criterion) {
    let a = Jet3::<9, { hess_size(9) }, { tens_size(9) }>::variable(1.5, 0);
    c.bench_function("jet3_9_sin", |bench| {
        bench.iter(|| black_box(black_box(a).sin()))
    });
}

fn bench_jet3_9_gravity_pattern(c: &mut Criterion) {
    let x = Jet3::<9, { hess_size(9) }, { tens_size(9) }>::variable(1.0, 0);
    let y = Jet3::<9, { hess_size(9) }, { tens_size(9) }>::variable(0.5, 1);
    let z = Jet3::<9, { hess_size(9) }, { tens_size(9) }>::variable(0.1, 2);
    let mu = 1.327e11;

    c.bench_function("jet3_9_gravity_accel", |bench| {
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

// ─── differentiate convenience API ────────────────────────────────────
//
// Measure the full "seed + compute + extract" round-trip cost. Compared to
// the corresponding jet*_gravity_accel benches (compute-only, no seeding
// or extraction), these show the wrapper overhead.

use nolan::differentiate::{
    differentiate1, differentiate1_vec, differentiate2_6, differentiate3_6,
};

fn bench_differentiate1_6_gravity(c: &mut Criterion) {
    let state = [1.0, 0.5, 0.1, 0.0, 0.0, 0.0];
    let mu = 1.327e11_f64;

    c.bench_function("differentiate1_6_gravity_magnitude", |bench| {
        bench.iter(|| {
            let state = black_box(state);
            black_box(differentiate1(state, |[x, y, z, _vx, _vy, _vz]| {
                let r2 = x * x + y * y + z * z;
                let r = r2.sqrt();
                mu / (r * r2)
            }))
        })
    });
}

fn bench_differentiate2_6_gravity(c: &mut Criterion) {
    let state = [1.0, 0.5, 0.1, 0.0, 0.0, 0.0];
    let mu = 1.327e11_f64;

    c.bench_function("differentiate2_6_gravity_magnitude", |bench| {
        bench.iter(|| {
            let state = black_box(state);
            black_box(differentiate2_6(state, |[x, y, z, _vx, _vy, _vz]| {
                let r2 = x * x + y * y + z * z;
                let r = r2.sqrt();
                mu / (r * r2)
            }))
        })
    });
}

fn bench_differentiate3_6_gravity(c: &mut Criterion) {
    let state = [1.0, 0.5, 0.1, 0.0, 0.0, 0.0];
    let mu = 1.327e11_f64;

    c.bench_function("differentiate3_6_gravity_magnitude", |bench| {
        bench.iter(|| {
            let state = black_box(state);
            black_box(differentiate3_6(state, |[x, y, z, _vx, _vy, _vz]| {
                let r2 = x * x + y * y + z * z;
                let r = r2.sqrt();
                mu / (r * r2)
            }))
        })
    });
}

fn bench_differentiate1_vec_gravity_accel(c: &mut Criterion) {
    // Vector-valued: f: R^6 → R^3 returning the 3-component acceleration.
    let state = [1.0, 0.5, 0.1, 0.0, 0.0, 0.0];
    let mu = 1.327e11_f64;

    c.bench_function("differentiate1_6_3_gravity_accel", |bench| {
        bench.iter(|| {
            let state = black_box(state);
            black_box(differentiate1_vec::<6, 3, _>(
                state,
                |[x, y, z, _vx, _vy, _vz]| {
                    let r2 = x * x + y * y + z * z;
                    let r = r2.sqrt();
                    let r3_inv = r.powi(-3) * mu;
                    [x * r3_inv, y * r3_inv, z * r3_inv]
                },
            ))
        })
    });
}

criterion_group!(
    benches,
    // constants
    bench_jet1_constant,
    bench_jet2_constant,
    bench_jet3_constant,
    // variables
    bench_jet1_variable,
    bench_jet2_variable,
    bench_jet3_variable,
    // binary ops
    bench_jet1_add,
    bench_jet2_add,
    bench_jet3_add,
    bench_jet1_mul,
    bench_jet2_mul,
    bench_jet3_mul,
    bench_jet1_div,
    bench_jet2_div,
    bench_jet3_div,
    bench_jet1_mul_scalar,
    bench_jet2_mul_scalar,
    bench_jet3_mul_scalar,
    // unary math
    bench_jet1_sin,
    bench_jet2_sin,
    bench_jet3_sin,
    bench_jet1_cos,
    bench_jet2_cos,
    bench_jet3_cos,
    bench_jet1_sqrt,
    bench_jet2_sqrt,
    bench_jet3_sqrt,
    bench_jet1_powi,
    bench_jet2_powi,
    bench_jet3_powi,
    bench_jet1_atan2,
    bench_jet2_atan2,
    bench_jet3_atan2,
    // realistic pattern (N=6)
    bench_jet1_gravity_pattern,
    bench_jet2_gravity_pattern,
    bench_jet3_gravity_pattern,
    // N=9 ops (9-param: state + non-grav / 1 burn)
    bench_jet1_9_add,
    bench_jet2_9_add,
    bench_jet3_9_add,
    bench_jet1_9_mul,
    bench_jet2_9_mul,
    bench_jet3_9_mul,
    bench_jet1_9_sin,
    bench_jet2_9_sin,
    bench_jet3_9_sin,
    bench_jet1_9_gravity_pattern,
    bench_jet2_9_gravity_pattern,
    bench_jet3_9_gravity_pattern,
    // extraction
    bench_jet1_extract_grad_6,
    bench_jet1_extract_grad_9,
    bench_jet2_extract_hess_6,
    bench_jet2_extract_hess_9,
    bench_jet3_extract_tens_6,
    bench_jet3_extract_tens_9,
    // differentiate convenience wrapper (seed + compute + extract)
    bench_differentiate1_6_gravity,
    bench_differentiate2_6_gravity,
    bench_differentiate3_6_gravity,
    bench_differentiate1_vec_gravity_accel,
);
criterion_main!(benches);
