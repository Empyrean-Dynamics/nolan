use criterion::{Criterion, black_box, criterion_group, criterion_main};
use hyperjet::jets::Jet1;
use hyperjet::linalg::regularize::{nearest_psd, tikhonov_with_report};
use hyperjet::linalg::*;

fn make_dd6_f64() -> [[f64; 6]; 6] {
    let mut a = [[0.0; 6]; 6];
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

fn make_dd9_f64() -> [[f64; 9]; 9] {
    let mut a = [[0.0; 9]; 9];
    for i in 0..9 {
        a[i][i] = (i + 2) as f64;
        for j in 0..9 {
            if i != j {
                a[i][j] = 0.1;
            }
        }
    }
    a
}

fn make_dd6_jet() -> [[Jet1<6>; 6]; 6] {
    let mut a = [[Jet1::<6>::constant(0.0); 6]; 6];
    for i in 0..6 {
        a[i][i] = Jet1::<6>::variable((i + 2) as f64, i);
        for j in 0..6 {
            if i != j {
                a[i][j] = Jet1::<6>::constant(0.1);
            }
        }
    }
    a
}

fn make_dd9_jet() -> [[Jet1<9>; 9]; 9] {
    let mut a = [[Jet1::<9>::constant(0.0); 9]; 9];
    for i in 0..9 {
        a[i][i] = Jet1::<9>::variable((i + 2) as f64, i);
        for j in 0..9 {
            if i != j {
                a[i][j] = Jet1::<9>::constant(0.1);
            }
        }
    }
    a
}

fn bench_vec3(c: &mut Criterion) {
    let a = [1.0_f64, 2.0, 3.0];
    let b = [4.0_f64, 5.0, 6.0];
    c.bench_function("dot3_f64", |bench| {
        bench.iter(|| dot3(black_box(&a), black_box(&b)))
    });
    c.bench_function("cross3_f64", |bench| {
        bench.iter(|| cross3(black_box(&a), black_box(&b)))
    });
    c.bench_function("norm3_f64", |bench| bench.iter(|| norm3(black_box(&a))));

    let aj: [Jet1<6>; 3] = [
        Jet1::<6>::variable(1.0, 0),
        Jet1::<6>::variable(2.0, 1),
        Jet1::<6>::variable(3.0, 2),
    ];
    let bj: [Jet1<6>; 3] = [
        Jet1::<6>::constant(4.0),
        Jet1::<6>::constant(5.0),
        Jet1::<6>::constant(6.0),
    ];
    c.bench_function("dot3_jet1_6", |bench| {
        bench.iter(|| dot3(black_box(&aj), black_box(&bj)))
    });
    c.bench_function("cross3_jet1_6", |bench| {
        bench.iter(|| cross3(black_box(&aj), black_box(&bj)))
    });
    c.bench_function("norm3_jet1_6", |bench| bench.iter(|| norm3(black_box(&aj))));
}

fn bench_mat6(c: &mut Criterion) {
    let a = make_dd6_f64();
    let b = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    c.bench_function("mat6_solve_f64", |bench| {
        bench.iter(|| mat6_solve(black_box(&a), black_box(&b)))
    });
    c.bench_function("mat6_inv_f64", |bench| {
        bench.iter(|| mat6_inv(black_box(&a)))
    });

    let aj = make_dd6_jet();
    let bj: [Jet1<6>; 6] = std::array::from_fn(|i| Jet1::<6>::constant((i + 1) as f64));
    c.bench_function("mat6_solve_jet1_6", |bench| {
        bench.iter(|| mat6_solve(black_box(&aj), black_box(&bj)))
    });
    c.bench_function("mat6_inv_jet1_6", |bench| {
        bench.iter(|| mat6_inv(black_box(&aj)))
    });
}

fn bench_mat9(c: &mut Criterion) {
    let a = make_dd9_f64();
    let b = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    c.bench_function("mat9_solve_f64", |bench| {
        bench.iter(|| mat9_solve(black_box(&a), black_box(&b)))
    });
    c.bench_function("mat9_inv_f64", |bench| {
        bench.iter(|| mat9_inv(black_box(&a)))
    });

    let aj = make_dd9_jet();
    let bj: [Jet1<9>; 9] = std::array::from_fn(|i| Jet1::<9>::constant((i + 1) as f64));
    c.bench_function("mat9_solve_jet1_9", |bench| {
        bench.iter(|| mat9_solve(black_box(&aj), black_box(&bj)))
    });
    c.bench_function("mat9_inv_jet1_9", |bench| {
        bench.iter(|| mat9_inv(black_box(&aj)))
    });
}

fn bench_generic(c: &mut Criterion) {
    let a6 = make_dd6_f64();
    let b6 = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    c.bench_function("mat_solve_6_f64", |bench| {
        bench.iter(|| mat_solve::<f64, 6>(black_box(&a6), black_box(&b6)))
    });

    let a9 = make_dd9_f64();
    let b9 = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    c.bench_function("mat_solve_9_f64", |bench| {
        bench.iter(|| mat_solve::<f64, 9>(black_box(&a9), black_box(&b9)))
    });
}

fn bench_rectangular(c: &mut Criterion) {
    let h_2x6: [[f64; 6]; 2] = [
        [1.0, 0.5, 0.25, 0.1, 0.05, 0.02],
        [0.0, 1.0, 0.5, 0.25, 0.1, 0.05],
    ];
    let w_2x2: [[f64; 2]; 2] = [[100.0, 1.0], [1.0, 90.0]];

    c.bench_function("mat_transpose_2x6", |bench| {
        bench.iter(|| mat_transpose::<2, 6>(black_box(&h_2x6)))
    });
    c.bench_function("mat_mul_2x2x6", |bench| {
        bench.iter(|| mat_mul::<2, 2, 6>(black_box(&w_2x2), black_box(&h_2x6)))
    });

    let h_t = mat_transpose::<2, 6>(&h_2x6);
    let wh = mat_mul::<2, 2, 6>(&w_2x2, &h_2x6);
    c.bench_function("mat_mul_6x2x6", |bench| {
        bench.iter(|| mat_mul::<6, 2, 6>(black_box(&h_t), black_box(&wh)))
    });

    c.bench_function("mat_ata_2x6", |bench| {
        bench.iter(|| mat_ata::<2, 6>(black_box(&h_2x6)))
    });

    let r = [1e-6_f64, 2e-6];
    c.bench_function("mat_vec_mul_2", |bench| {
        bench.iter(|| mat_vec_mul::<2>(black_box(&w_2x2), black_box(&r)))
    });
}

fn bench_scalar_summaries(c: &mut Criterion) {
    let a6 = make_dd6_f64();
    let b6 = make_dd6_f64();

    c.bench_function("mat_det_6", |bench| {
        bench.iter(|| mat_det::<6>(black_box(&a6)))
    });
    c.bench_function("mat_trace_cube_6", |bench| {
        bench.iter(|| mat_trace_cube::<6>(black_box(&a6), black_box(&b6)))
    });
    c.bench_function("mat_frobenius_6x6", |bench| {
        bench.iter(|| mat_frobenius::<6, 6>(black_box(&a6)))
    });
    c.bench_function("mat_largest_singular_value_6", |bench| {
        bench.iter(|| mat_largest_singular_value::<6>(black_box(&a6), 64, 1e-12))
    });
    c.bench_function("condition_number_6", |bench| {
        bench.iter(|| condition_number::<6>(black_box(&a6)))
    });
}

fn bench_eigen(c: &mut Criterion) {
    let cov_3: [[f64; 3]; 3] = [[1.0, 0.2, 0.1], [0.2, 0.8, 0.05], [0.1, 0.05, 0.6]];
    c.bench_function("sym_eigenvalues_3", |bench| {
        bench.iter(|| sym_eigenvalues_3(black_box(&cov_3)))
    });

    let cov_6 = make_dd6_f64();
    c.bench_function("mat_symmetric_eigen_6", |bench| {
        bench.iter(|| mat_symmetric_eigen::<6>(black_box(&cov_6)))
    });
}

fn bench_regularize(c: &mut Criterion) {
    let mut cov_6 = make_dd6_f64();
    cov_6[5][5] = 1e-18;
    c.bench_function("nearest_psd_6", |bench| {
        bench.iter(|| nearest_psd::<6>(black_box(&cov_6), 1e-12))
    });
    c.bench_function("tikhonov_with_report_6", |bench| {
        bench.iter(|| tikhonov_with_report::<6>(black_box(&cov_6), 1e-10))
    });
}

criterion_group!(
    benches,
    bench_vec3,
    bench_mat6,
    bench_mat9,
    bench_generic,
    bench_rectangular,
    bench_scalar_summaries,
    bench_eigen,
    bench_regularize,
);
criterion_main!(benches);
