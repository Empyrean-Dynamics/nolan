use criterion::{Criterion, black_box, criterion_group, criterion_main};
use nolan::angles::{wrap_2pi, wrap_180, wrap_360, wrap_pi};
use nolan::grids::{linear_clamped, linspace, logspace};
use nolan::statistics::multivariate::{sample_statistics, sigma_points, split_gaussian};

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

criterion_group!(
    benches,
    bench_sample_statistics,
    bench_sigma_points,
    bench_split_gaussian,
    bench_grids,
    bench_angles,
);
criterion_main!(benches);
