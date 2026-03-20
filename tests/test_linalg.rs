use nolan::jets::Jet1;
use nolan::linalg::*;
use nolan::traits::{Differentiable, FirstOrder};

/// Verify that `mat_solve::<_, 6>` and `mat6_solve` agree for f64.
#[test]
fn test_generic_vs_specialized_solve_6() {
    let mut a = [[0.0_f64; 6]; 6];
    for i in 0..6 {
        a[i][i] = (i + 2) as f64;
        for j in 0..6 {
            if i != j {
                a[i][j] = 0.1;
            }
        }
    }
    let b = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];

    let x_gen = mat_solve::<f64, 6>(&a, &b).unwrap();
    let x_spec = mat6_solve(&a, &b).unwrap();

    for i in 0..6 {
        assert!(
            (x_gen[i] - x_spec[i]).abs() < 1e-13,
            "solve6 mismatch at {}: {} vs {}",
            i,
            x_gen[i],
            x_spec[i]
        );
    }
}

/// Verify that `mat_solve::<_, 9>` and `mat9_solve` agree for f64.
#[test]
fn test_generic_vs_specialized_solve_9() {
    let mut a = [[0.0_f64; 9]; 9];
    for i in 0..9 {
        a[i][i] = (i + 2) as f64;
        for j in 0..9 {
            if i != j {
                a[i][j] = 0.1;
            }
        }
    }
    let b = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];

    let x_gen = mat_solve::<f64, 9>(&a, &b).unwrap();
    let x_spec = mat9_solve(&a, &b).unwrap();

    for i in 0..9 {
        assert!(
            (x_gen[i] - x_spec[i]).abs() < 1e-12,
            "solve9 mismatch at {}: {} vs {}",
            i,
            x_gen[i],
            x_spec[i]
        );
    }
}

/// Verify that `mat_inv::<_, 6>` and `mat6_inv` agree for f64.
#[test]
fn test_generic_vs_specialized_inv_6() {
    let mut a = [[0.0_f64; 6]; 6];
    for i in 0..6 {
        a[i][i] = (i + 2) as f64;
        for j in 0..6 {
            if i != j {
                a[i][j] = 0.1;
            }
        }
    }

    let inv_gen = mat_inv::<f64, 6>(&a).unwrap();
    let inv_spec = mat6_inv(&a).unwrap();

    for i in 0..6 {
        for j in 0..6 {
            assert!(
                (inv_gen[i][j] - inv_spec[i][j]).abs() < 1e-13,
                "inv6 mismatch at [{},{}]: {} vs {}",
                i,
                j,
                inv_gen[i][j],
                inv_spec[i][j]
            );
        }
    }
}

/// Verify that `mat_inv::<_, 9>` and `mat9_inv` agree for f64.
#[test]
fn test_generic_vs_specialized_inv_9() {
    let mut a = [[0.0_f64; 9]; 9];
    for i in 0..9 {
        a[i][i] = (i + 2) as f64;
        for j in 0..9 {
            if i != j {
                a[i][j] = 0.1;
            }
        }
    }

    let inv_gen = mat_inv::<f64, 9>(&a).unwrap();
    let inv_spec = mat9_inv(&a).unwrap();

    for i in 0..9 {
        for j in 0..9 {
            assert!(
                (inv_gen[i][j] - inv_spec[i][j]).abs() < 1e-12,
                "inv9 mismatch at [{},{}]: {} vs {}",
                i,
                j,
                inv_gen[i][j],
                inv_spec[i][j]
            );
        }
    }
}

/// Verify mat3_solve and mat3_inv agree.
#[test]
fn test_mat3_solve_vs_inv() {
    let a = [[2.0, 1.0, 0.0], [1.0, 3.0, 1.0], [0.0, 1.0, 2.0]];
    let b = [1.0, 2.0, 3.0];
    let x_solve = mat3_solve(&a, &b).unwrap();
    let inv = mat3_inv(&a).unwrap();
    let x_inv = mat3::mat3_vec_mul(&inv, &b);
    for i in 0..3 {
        assert!(
            (x_solve[i] - x_inv[i]).abs() < 1e-14,
            "solve vs inv mismatch at {}: {} vs {}",
            i,
            x_solve[i],
            x_inv[i]
        );
    }
}

/// Cross-module Jet1 test: solve a 6×6 system and verify derivatives.
#[test]
fn test_jet1_solve_6_gradient() {
    // Diagonal matrix with Jet variables
    let mut a = [[Jet1::<6>::constant(0.0); 6]; 6];
    for i in 0..6 {
        a[i][i] = Jet1::<6>::variable((i + 2) as f64, i);
    }
    let b: [Jet1<6>; 6] = std::array::from_fn(|i| Jet1::<6>::constant((i + 1) as f64));
    let x = mat6_solve(&a, &b).unwrap();

    // x[i] = b[i] / a[i][i]
    // d(x[i])/d(a[i][i]) = -b[i] / a[i][i]^2
    for i in 0..6 {
        let expected_val = (i + 1) as f64 / (i + 2) as f64;
        let expected_grad = -((i + 1) as f64) / ((i + 2) as f64 * (i + 2) as f64);
        assert!(
            (x[i].value() - expected_val).abs() < 1e-14,
            "x[{}] value: {} vs {}",
            i,
            x[i].value(),
            expected_val
        );
        assert!(
            (x[i].grad(i) - expected_grad).abs() < 1e-13,
            "x[{}] grad: {} vs {}",
            i,
            x[i].grad(i),
            expected_grad
        );
    }
}

/// Verify norm3/normalize3 roundtrip with Jet1.
#[test]
fn test_normalize3_norm_is_one() {
    let x = [
        Jet1::<3>::variable(3.0, 0),
        Jet1::<3>::variable(4.0, 1),
        Jet1::<3>::variable(5.0, 2),
    ];
    let n = normalize3(&x);
    let len = norm3(&n);
    assert!(
        (len.value() - 1.0).abs() < 1e-14,
        "normalized length = {}",
        len.value()
    );
}
