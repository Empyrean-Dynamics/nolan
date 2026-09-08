use criterion::{Criterion, black_box, criterion_group, criterion_main};
use hyperjet::angles::{wrap_2pi, wrap_180, wrap_360, wrap_pi};
use hyperjet::grids::{linear_clamped, linspace, logspace};
use hyperjet::statistics::multivariate::{sample_statistics, sigma_points, split_gaussian};
use hyperjet::statistics::{normal_cdf, normal_cdf_difference, normal_pdf, normal_sf};

fn make_cov_6() -> [[f64; 6]; 6] {
    let mut a = [[0.0_f64; 6]; 6];
    for i in 0..6 {
        a[i][i] = (i + 2) as f64;
        for j in 0..6 {
            if i != j {
                a[i][j] = 0.1;
            }
        }
    }
    a
}

fn bench_sample_statistics(c: &mut Criterion) {
    // 50 samples is the typical size of a sigma-point cloud after
    // a non-linear propagation step (2N+1 = 13 for N=6, padded for
    // higher-order GH or unscented variants).
    let mean = [1.0_f64, 2.0, 3.0, 0.1, 0.2, 0.3];
    let cov = make_cov_6();
    let cloud = sigma_points::<6>(&mean, &cov).expect("sigma_points should succeed");
    // Replicate a 50-sample cloud by repeating the canonical sigma set.
    let mut samples: Vec<[f64; 6]> = Vec::with_capacity(50);
    for i in 0..50 {
        samples.push(cloud[i % cloud.len()]);
    }
    c.bench_function("sample_statistics_6_n50", |bench| {
        bench.iter(|| sample_statistics::<6>(black_box(&samples)))
    });
}

fn bench_sigma_points(c: &mut Criterion) {
    let mean = [1.0_f64, 2.0, 3.0, 0.1, 0.2, 0.3];
    let cov = make_cov_6();
    c.bench_function("sigma_points_6", |bench| {
        bench.iter(|| sigma_points::<6>(black_box(&mean), black_box(&cov)))
    });
}

fn bench_split_gaussian(c: &mut Criterion) {
    let mean = [1.0_f64, 2.0, 3.0, 0.1, 0.2, 0.3];
    let cov = make_cov_6();
    // Direction = first eigenvector approximation; bench just needs a
    // unit vector along an axis to exercise the split kernel.
    let dir = [1.0_f64, 0.0, 0.0, 0.0, 0.0, 0.0];
    c.bench_function("split_gaussian_6_k3", |bench| {
        bench.iter(|| {
            split_gaussian::<6>(
                black_box(&mean),
                black_box(&cov),
                black_box(&dir),
                black_box(3),
            )
        })
    });
    c.bench_function("split_gaussian_6_k5", |bench| {
        bench.iter(|| {
            split_gaussian::<6>(
                black_box(&mean),
                black_box(&cov),
                black_box(&dir),
                black_box(5),
            )
        })
    });
    // The widest split a caller is likely to afford: cost is linear in
    // the component count, since each component copies one N x N
    // sub-covariance.
    c.bench_function("split_gaussian_6_k7", |bench| {
        bench.iter(|| {
            split_gaussian::<6>(
                black_box(&mean),
                black_box(&cov),
                black_box(&dir),
                black_box(7),
            )
        })
    });
    c.bench_function("split_gaussian_6_k15", |bench| {
        bench.iter(|| {
            split_gaussian::<6>(
                black_box(&mean),
                black_box(&cov),
                black_box(&dir),
                black_box(15),
            )
        })
    });
}

fn bench_grids(c: &mut Criterion) {
    c.bench_function("linspace_64", |bench| {
        bench.iter(|| linspace(black_box(0.0), black_box(1.0), black_box(64)))
    });
    c.bench_function("logspace_64", |bench| {
        bench.iter(|| logspace(black_box(1e-3), black_box(1.0), black_box(64)))
    });

    let xs = linspace(0.0, 1.0, 64);
    let ys: Vec<f64> = xs.iter().map(|x| x.sin()).collect();
    c.bench_function("linear_clamped_64", |bench| {
        bench.iter(|| linear_clamped(black_box(&xs), black_box(&ys), black_box(0.5)))
    });
}

fn bench_angles(c: &mut Criterion) {
    // Pre-build a small batch to amortize loop overhead; per-call
    // cost of wrap_* is sub-nanosecond and would be dominated by
    // criterion's measurement noise otherwise.
    let inputs: Vec<f64> = (0..64).map(|i| (i as f64 - 32.0) * 0.5).collect();
    c.bench_function("wrap_pi_x64", |bench| {
        bench.iter(|| {
            let mut s = 0.0;
            for &x in &inputs {
                s += wrap_pi(black_box(x));
            }
            s
        })
    });
    c.bench_function("wrap_2pi_x64", |bench| {
        bench.iter(|| {
            let mut s = 0.0;
            for &x in &inputs {
                s += wrap_2pi(black_box(x));
            }
            s
        })
    });
    c.bench_function("wrap_180_x64", |bench| {
        bench.iter(|| {
            let mut s = 0.0;
            for &x in &inputs {
                s += wrap_180(black_box(x));
            }
            s
        })
    });
    c.bench_function("wrap_360_x64", |bench| {
        bench.iter(|| {
            let mut s = 0.0;
            for &x in &inputs {
                s += wrap_360(black_box(x));
            }
            s
        })
    });
}

// ── Standard normal ─────────────────────────────────────────────────

/// 64 arguments spread over `[lo, hi]`, so one iteration exercises the
/// whole regime rather than one lucky point: the continued fraction's
/// depth follows its argument, so a single-point measurement of the tail
/// says nothing about the tail.
fn spread(lo: f64, hi: f64) -> Vec<f64> {
    (0..64).map(|i| lo + (hi - lo) * i as f64 / 63.0).collect()
}

/// The Abramowitz & Stegun 26.2.17 rational approximation that
/// `normal_cdf` used before it carried a relatively accurate tail. Kept
/// here as the cost baseline: it is what the accuracy is bought with.
fn abramowitz_stegun_26_2_17(x: f64) -> f64 {
    if x < -8.0 {
        return 0.0;
    }
    if x > 8.0 {
        return 1.0;
    }
    let sign = if x >= 0.0 { 1.0 } else { -1.0 };
    let ax = x.abs();
    let t = 1.0 / (1.0 + 0.231_641_9 * ax);
    let t2 = t * t;
    let t3 = t2 * t;
    let t4 = t3 * t;
    let t5 = t4 * t;
    let poly = 0.319_381_530 * t - 0.356_563_782 * t2 + 1.781_477_937 * t3 - 1.821_255_978 * t4
        + 1.330_274_429 * t5;
    let pdf = (-0.5 * ax * ax).exp() / (2.0 * std::f64::consts::PI).sqrt();
    let cdf_abs = 1.0 - pdf * poly;
    0.5 + sign * 0.5 * (2.0 * cdf_abs - 1.0)
}

fn bench_normal(c: &mut Criterion) {
    let bulk = spread(-1.7, 1.7);
    let near_tail = spread(1.75, 8.0);
    let deep_tail = spread(8.0, 37.0);

    // A free function rather than a closure, so it does not hold a
    // mutable borrow of `c` across the bracket benches below.
    fn sweep(c: &mut Criterion, name: &str, inputs: &[f64], f: fn(f64) -> f64) {
        let inputs = inputs.to_vec();
        c.bench_function(name, |bench| {
            bench.iter(|| {
                let mut total = 0.0;
                for &x in &inputs {
                    total += f(black_box(x));
                }
                total
            })
        });
    }

    sweep(c, "normal_cdf_bulk_x64", &bulk, normal_cdf);
    sweep(c, "normal_cdf_near_tail_x64", &near_tail, normal_cdf);
    sweep(c, "normal_cdf_deep_tail_x64", &deep_tail, normal_cdf);
    sweep(c, "normal_sf_near_tail_x64", &near_tail, normal_sf);
    sweep(c, "normal_pdf_bulk_x64", &bulk, normal_pdf);
    sweep(c, "normal_pdf_tail_x64", &near_tail, normal_pdf);
    // The bracket probability, on the two arms that cost differently:
    // a straddling bracket forms two central masses, a same-side one
    // two tails at the fraction's depth.
    let straddling: Vec<f64> = bulk.clone();
    c.bench_function("normal_cdf_difference_straddling_x64", |bench| {
        bench.iter(|| {
            let mut total = 0.0;
            for &x in &straddling {
                total += normal_cdf_difference(black_box(x.abs()), black_box(-x.abs()));
            }
            total
        })
    });
    let same_side: Vec<f64> = near_tail.clone();
    c.bench_function("normal_cdf_difference_same_side_x64", |bench| {
        bench.iter(|| {
            let mut total = 0.0;
            for &x in &same_side {
                total += normal_cdf_difference(black_box(x + 1.0), black_box(x));
            }
            total
        })
    });

    sweep(
        c,
        "normal_cdf_abramowitz_stegun_bulk_x64",
        &bulk,
        abramowitz_stegun_26_2_17,
    );
    sweep(
        c,
        "normal_cdf_abramowitz_stegun_near_tail_x64",
        &near_tail,
        abramowitz_stegun_26_2_17,
    );
}

criterion_group!(
    benches,
    bench_sample_statistics,
    bench_sigma_points,
    bench_split_gaussian,
    bench_grids,
    bench_angles,
    bench_normal,
);
criterion_main!(benches);
