use crate::linalg::NOLAN_REL_TOL;
use crate::traits::DifferentiableMath;

/// Multiply two \\(3 \times 3\\) matrices: \\(C = A \, B\\).
#[inline]
pub fn mat3_mul<T: Copy + DifferentiableMath>(a: &[[T; 3]; 3], b: &[[T; 3]; 3]) -> [[T; 3]; 3] {
    let zero = T::constant(0.0);
    let mut c = [[zero; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            c[i][j] = a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j];
        }
    }
    c
}

/// Transpose a \\(3 \times 3\\) matrix: \\(T_{ij} = A_{ji}\\).
#[inline]
pub fn mat3_transpose<T: Copy + DifferentiableMath>(a: &[[T; 3]; 3]) -> [[T; 3]; 3] {
    [
        [a[0][0], a[1][0], a[2][0]],
        [a[0][1], a[1][1], a[2][1]],
        [a[0][2], a[1][2], a[2][2]],
    ]
}

/// Multiply a \\(3 \times 3\\) matrix by a 3-vector: \\(\mathbf{y} = A \, \mathbf{x}\\).
#[inline]
pub fn mat3_vec_mul<T: Copy + DifferentiableMath>(a: &[[T; 3]; 3], x: &[T; 3]) -> [T; 3] {
    [
        a[0][0] * x[0] + a[0][1] * x[1] + a[0][2] * x[2],
        a[1][0] * x[0] + a[1][1] * x[1] + a[1][2] * x[2],
        a[2][0] * x[0] + a[2][1] * x[1] + a[2][2] * x[2],
    ]
}

/// Multiply \\(A^\top \mathbf{x}\\) without forming the transpose.
#[inline]
pub fn mat3_transpose_vec_mul<T: Copy + DifferentiableMath>(a: &[[T; 3]; 3], x: &[T; 3]) -> [T; 3] {
    [
        a[0][0] * x[0] + a[1][0] * x[1] + a[2][0] * x[2],
        a[0][1] * x[0] + a[1][1] * x[1] + a[2][1] * x[2],
        a[0][2] * x[0] + a[1][2] * x[1] + a[2][2] * x[2],
    ]
}

/// Invert a \\(3 \times 3\\) matrix via the adjugate (cofactor) method.
///
/// Returns `None` if the matrix is singular at scaled-relative
/// tolerance: \\(|\det A| < \text{REL\\_TOL} \cdot \max_{ij}|A_{ij}|^3\\).
/// The cubic scale follows from \\(\det A\\) being a sum of triple
/// products of entries.
///
/// # When to use this vs `mat_inv::<3>`
///
/// Cramer's rule (this function) is faster and bit-exact for
/// well-conditioned 3×3 inputs but accumulates roundoff proportional
/// to \\(\kappa(A)\\) without any pivot-based mitigation. For
/// marginally-conditioned matrices — covariances whose smallest
/// eigenvalue is within 4–6 orders of magnitude of the largest, or
/// matrices feeding into squared accumulators like
/// \\(H^\top W H\\) — route through [`crate::linalg::generic::mat_inv`]
/// with `N = 3` instead: scaled partial pivoting halves the worst-case
/// roundoff at the cost of one extra division per row.
pub fn mat3_inv<T: Copy + DifferentiableMath>(m: &[[T; 3]; 3]) -> Option<[[T; 3]; 3]> {
    let det = m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
        - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
        + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0]);
    let mut max_entry = 0.0_f64;
    for row in m {
        for v in row {
            let abs_v = v.value().abs();
            if abs_v > max_entry {
                max_entry = abs_v;
            }
        }
    }
    let scale = max_entry.powi(3).max(f64::MIN_POSITIVE);
    if det.value().abs() < NOLAN_REL_TOL * scale {
        return None;
    }
    let d = T::constant(1.0) / det;
    Some([
        [
            (m[1][1] * m[2][2] - m[1][2] * m[2][1]) * d,
            (m[0][2] * m[2][1] - m[0][1] * m[2][2]) * d,
            (m[0][1] * m[1][2] - m[0][2] * m[1][1]) * d,
        ],
        [
            (m[1][2] * m[2][0] - m[1][0] * m[2][2]) * d,
            (m[0][0] * m[2][2] - m[0][2] * m[2][0]) * d,
            (m[0][2] * m[1][0] - m[0][0] * m[1][2]) * d,
        ],
        [
            (m[1][0] * m[2][1] - m[1][1] * m[2][0]) * d,
            (m[0][1] * m[2][0] - m[0][0] * m[2][1]) * d,
            (m[0][0] * m[1][1] - m[0][1] * m[1][0]) * d,
        ],
    ])
}

/// Solve \\(A \mathbf{x} = \mathbf{b}\\) for a \\(3 \times 3\\) system via Cramer's rule.
///
/// Returns `None` if the matrix is singular at scaled-relative
/// tolerance: \\(|\det A| < \text{REL\\_TOL} \cdot \max_{ij}|A_{ij}|^3\\).
pub fn mat3_solve<T: Copy + DifferentiableMath>(a: &[[T; 3]; 3], b: &[T; 3]) -> Option<[T; 3]> {
    let det_a = a[0][0] * (a[1][1] * a[2][2] - a[1][2] * a[2][1])
        - a[0][1] * (a[1][0] * a[2][2] - a[1][2] * a[2][0])
        + a[0][2] * (a[1][0] * a[2][1] - a[1][1] * a[2][0]);
    let mut max_entry = 0.0_f64;
    for row in a {
        for v in row {
            let abs_v = v.value().abs();
            if abs_v > max_entry {
                max_entry = abs_v;
            }
        }
    }
    let scale = max_entry.powi(3).max(f64::MIN_POSITIVE);
    if det_a.value().abs() < NOLAN_REL_TOL * scale {
        return None;
    }
    let inv_det = T::constant(1.0) / det_a;

    // Replace column 0 with b
    let det_x0 = b[0] * (a[1][1] * a[2][2] - a[1][2] * a[2][1])
        - a[0][1] * (b[1] * a[2][2] - a[1][2] * b[2])
        + a[0][2] * (b[1] * a[2][1] - a[1][1] * b[2]);

    // Replace column 1 with b
    let det_x1 = a[0][0] * (b[1] * a[2][2] - a[1][2] * b[2])
        - b[0] * (a[1][0] * a[2][2] - a[1][2] * a[2][0])
        + a[0][2] * (a[1][0] * b[2] - b[1] * a[2][0]);

    // Replace column 2 with b
    let det_x2 = a[0][0] * (a[1][1] * b[2] - b[1] * a[2][1])
        - a[0][1] * (a[1][0] * b[2] - b[1] * a[2][0])
        + b[0] * (a[1][0] * a[2][1] - a[1][1] * a[2][0]);

    Some([det_x0 * inv_det, det_x1 * inv_det, det_x2 * inv_det])
}

/// Determinant of a \\(3 \times 3\\) matrix via cofactor expansion.
///
/// Generic over `T: DifferentiableMath`, so this can be called on
/// `Jet1` / `Jet2` matrices to get partials of the determinant.
#[inline]
pub fn mat3_det<T: Copy + DifferentiableMath>(m: &[[T; 3]; 3]) -> T {
    m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
        - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
        + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
}

/// Eigenvalues of a \\(3 \times 3\\) real symmetric matrix via the
/// analytical method of Kopp (2008).
///
/// Shifts the matrix to zero trace, then solves the depressed cubic
/// via trigonometric identity. Returns eigenvalues sorted by descending
/// absolute value: \\(|\lambda_0| \geq |\lambda_1| \geq |\lambda_2|\\).
///
/// Closed-form. Faster than the general iterative Jacobi method
/// ([`crate::linalg::generic::mat_symmetric_eigen`]) for the 3×3 case,
/// but only returns eigenvalues — not eigenvectors.
///
/// # Reference
///
/// Kopp, J. (2008). *Efficient numerical diagonalization of hermitian
/// 3×3 matrices.* Int. J. Mod. Phys. C, 19(3), 523–548.
pub fn sym_eigenvalues_3(a: &[[f64; 3]; 3]) -> [f64; 3] {
    use std::f64::consts::FRAC_PI_3;

    let q = (a[0][0] + a[1][1] + a[2][2]) / 3.0;

    let b00 = a[0][0] - q;
    let b11 = a[1][1] - q;
    let b22 = a[2][2] - q;
    let b01 = a[0][1];
    let b02 = a[0][2];
    let b12 = a[1][2];

    let p_sq =
        (b00 * b00 + b11 * b11 + b22 * b22 + 2.0 * (b01 * b01 + b02 * b02 + b12 * b12)) / 6.0;
    let p = p_sq.sqrt();

    if p < 1e-30 {
        return [q, q, q];
    }

    let inv_p = 1.0 / p;
    let n00 = b00 * inv_p;
    let n11 = b11 * inv_p;
    let n22 = b22 * inv_p;
    let n01 = b01 * inv_p;
    let n02 = b02 * inv_p;
    let n12 = b12 * inv_p;
    let r = 0.5
        * (n00 * (n11 * n22 - n12 * n12) - n01 * (n01 * n22 - n12 * n02)
            + n02 * (n01 * n12 - n11 * n02));

    let phi = if r <= -1.0 {
        FRAC_PI_3
    } else if r >= 1.0 {
        0.0
    } else {
        r.acos() / 3.0
    };

    let eig1 = q + 2.0 * p * phi.cos();
    let eig3 = q + 2.0 * p * (phi + 2.0 * FRAC_PI_3).cos();
    let eig2 = 3.0 * q - eig1 - eig3;

    let mut eig = [eig1, eig2, eig3];
    eig.sort_by(|a, b| {
        b.abs()
            .partial_cmp(&a.abs())
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    eig
}

#[cfg(test)]
#[allow(clippy::needless_range_loop)]
#[allow(clippy::assign_op_pattern)]
mod tests {
    use super::*;
    use crate::jets::Jet1;
    use crate::traits::{Differentiable, FirstOrder};

    fn identity3<T: Copy + DifferentiableMath>() -> [[T; 3]; 3] {
        let zero = T::constant(0.0);
        let one = T::constant(1.0);
        [[one, zero, zero], [zero, one, zero], [zero, zero, one]]
    }

    #[test]
    fn test_mat3_mul_identity() {
        let id = identity3::<f64>();
        let a = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];
        let c = mat3_mul(&a, &id);
        for i in 0..3 {
            for j in 0..3 {
                assert!((c[i][j] - a[i][j]).abs() < 1e-15);
            }
        }
    }

    #[test]
    fn test_mat3_transpose() {
        let a = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];
        let t = mat3_transpose(&a);
        for i in 0..3 {
            for j in 0..3 {
                assert!((t[i][j] - a[j][i]).abs() < 1e-15);
            }
        }
    }

    #[test]
    fn test_mat3_vec_mul_identity() {
        let id = identity3::<f64>();
        let x = [1.0, 2.0, 3.0];
        let y = mat3_vec_mul(&id, &x);
        for i in 0..3 {
            assert!((y[i] - x[i]).abs() < 1e-15);
        }
    }

    #[test]
    fn test_mat3_transpose_vec_mul() {
        let a = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];
        let x = [1.0, 1.0, 1.0];
        let y = mat3_transpose_vec_mul(&a, &x);
        // A^T * [1,1,1] = column sums = [1+4+7, 2+5+8, 3+6+9] = [12, 15, 18]
        assert!((y[0] - 12.0).abs() < 1e-14);
        assert!((y[1] - 15.0).abs() < 1e-14);
        assert!((y[2] - 18.0).abs() < 1e-14);
    }

    #[test]
    fn test_mat3_inv_roundtrip() {
        let a = [[2.0, 1.0, 0.0], [1.0, 3.0, 1.0], [0.0, 1.0, 2.0]];
        let inv = mat3_inv(&a).unwrap();
        let prod = mat3_mul(&a, &inv);
        let id = identity3::<f64>();
        for i in 0..3 {
            for j in 0..3 {
                assert!((prod[i][j] - id[i][j]).abs() < 1e-14);
            }
        }
    }

    #[test]
    fn test_mat3_inv_singular() {
        let a = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];
        assert!(mat3_inv(&a).is_none());
    }

    #[test]
    fn test_mat3_solve_f64() {
        let a = [[2.0, 1.0, 0.0], [1.0, 3.0, 1.0], [0.0, 1.0, 2.0]];
        let b = [1.0, 2.0, 3.0];
        let x = mat3_solve(&a, &b).unwrap();
        // Verify A*x = b
        let y = mat3_vec_mul(&a, &x);
        for i in 0..3 {
            assert!((y[i] - b[i]).abs() < 1e-14);
        }
    }

    #[test]
    fn test_mat3_solve_singular() {
        let a = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];
        let b = [1.0, 2.0, 3.0];
        assert!(mat3_solve(&a, &b).is_none());
    }

    #[test]
    fn test_mat3_inv_jet1() {
        let a: [[Jet1<3>; 3]; 3] = [
            [
                Jet1::<3>::variable(2.0, 0),
                Jet1::<3>::constant(1.0),
                Jet1::<3>::constant(0.0),
            ],
            [
                Jet1::<3>::constant(1.0),
                Jet1::<3>::variable(3.0, 1),
                Jet1::<3>::constant(1.0),
            ],
            [
                Jet1::<3>::constant(0.0),
                Jet1::<3>::constant(1.0),
                Jet1::<3>::variable(2.0, 2),
            ],
        ];
        let inv = mat3_inv(&a).unwrap();
        // Verify A * A^-1 = I (values)
        let prod = mat3_mul(&a, &inv);
        for i in 0..3 {
            for j in 0..3 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!(
                    (prod[i][j].value() - expected).abs() < 1e-13,
                    "prod[{}][{}] = {}, expected {}",
                    i,
                    j,
                    prod[i][j].value(),
                    expected
                );
            }
        }
    }

    #[test]
    fn test_mat3_solve_jet1() {
        // Solve A*x = b with A having jet variables on diagonal
        let a: [[Jet1<3>; 3]; 3] = [
            [
                Jet1::<3>::variable(2.0, 0),
                Jet1::<3>::constant(1.0),
                Jet1::<3>::constant(0.0),
            ],
            [
                Jet1::<3>::constant(1.0),
                Jet1::<3>::variable(3.0, 1),
                Jet1::<3>::constant(1.0),
            ],
            [
                Jet1::<3>::constant(0.0),
                Jet1::<3>::constant(1.0),
                Jet1::<3>::variable(2.0, 2),
            ],
        ];
        let b = [
            Jet1::<3>::constant(1.0),
            Jet1::<3>::constant(2.0),
            Jet1::<3>::constant(3.0),
        ];
        let x = mat3_solve(&a, &b).unwrap();
        // Verify A*x = b (values)
        let y = mat3_vec_mul(&a, &x);
        for i in 0..3 {
            assert!(
                (y[i].value() - b[i].value()).abs() < 1e-13,
                "y[{}] = {}, b[{}] = {}",
                i,
                y[i].value(),
                i,
                b[i].value()
            );
        }
    }

    #[test]
    fn test_mat3_det_identity_f64() {
        let id = identity3::<f64>();
        assert!((mat3_det(&id) - 1.0).abs() < 1e-15);
    }

    #[test]
    fn test_mat3_det_diagonal_f64() {
        let m = [[2.0, 0.0, 0.0], [0.0, 3.0, 0.0], [0.0, 0.0, 4.0]];
        assert!((mat3_det(&m) - 24.0).abs() < 1e-15);
    }

    #[test]
    fn test_mat3_det_singular_f64() {
        let m = [[1.0, 2.0, 3.0], [2.0, 4.0, 6.0], [0.0, 1.0, 0.0]];
        assert!(mat3_det(&m).abs() < 1e-15);
    }

    #[test]
    fn test_mat3_det_sign_flip_on_row_swap() {
        let a = [[1.0, 2.0, 0.0], [0.0, 1.0, 3.0], [4.0, 0.0, 1.0]];
        let mut b = a;
        b.swap(0, 1);
        assert!((mat3_det(&a) + mat3_det(&b)).abs() < 1e-13);
    }

    #[test]
    fn test_mat3_det_jet1_partials() {
        // d(det A) / dA[i][j] = cofactor(A)[i][j].
        // Use A = diag(1, 2, 3); det = 6. Differentiate wrt A[0][0] -> cofactor = 2*3 = 6.
        let a = [
            [
                Jet1::<1>::variable(1.0, 0),
                Jet1::<1>::constant(0.0),
                Jet1::<1>::constant(0.0),
            ],
            [
                Jet1::<1>::constant(0.0),
                Jet1::<1>::constant(2.0),
                Jet1::<1>::constant(0.0),
            ],
            [
                Jet1::<1>::constant(0.0),
                Jet1::<1>::constant(0.0),
                Jet1::<1>::constant(3.0),
            ],
        ];
        let d = mat3_det(&a);
        assert!((d.value() - 6.0).abs() < 1e-13);
        assert!((d.grad(0) - 6.0).abs() < 1e-13);
    }

    #[test]
    fn test_sym_eigenvalues_3_diagonal() {
        let a = [[3.0, 0.0, 0.0], [0.0, -2.0, 0.0], [0.0, 0.0, 1.0]];
        let e = sym_eigenvalues_3(&a);
        // Sorted by descending |λ|: [3.0, -2.0, 1.0]
        assert!((e[0] - 3.0).abs() < 1e-12);
        assert!((e[1] - (-2.0)).abs() < 1e-12);
        assert!((e[2] - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_sym_eigenvalues_3_scalar_matrix() {
        let a = [[2.5, 0.0, 0.0], [0.0, 2.5, 0.0], [0.0, 0.0, 2.5]];
        let e = sym_eigenvalues_3(&a);
        for v in e {
            assert!((v - 2.5).abs() < 1e-12);
        }
    }

    #[test]
    fn test_sym_eigenvalues_3_known_2x2_extended() {
        // Eigenvalues of [[2, 1, 0], [1, 2, 0], [0, 0, 4]] are {1, 3, 4}.
        let a = [[2.0, 1.0, 0.0], [1.0, 2.0, 0.0], [0.0, 0.0, 4.0]];
        let e = sym_eigenvalues_3(&a);
        // Sorted by |λ|: [4, 3, 1]
        assert!((e[0] - 4.0).abs() < 1e-12);
        assert!((e[1] - 3.0).abs() < 1e-12);
        assert!((e[2] - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_sym_eigenvalues_3_trace_preserved() {
        let a = [[1.5, 0.3, -0.1], [0.3, 2.7, 0.5], [-0.1, 0.5, 0.8]];
        let trace = a[0][0] + a[1][1] + a[2][2];
        let e = sym_eigenvalues_3(&a);
        let sum: f64 = e.iter().sum();
        assert!((sum - trace).abs() < 1e-12);
    }

    // ── Phase 1.5A-Cramer: relative determinant guard ──────────────

    #[test]
    fn test_mat3_inv_small_scale_roundtrip() {
        // A = 1e-10 · I → A⁻¹ = 1e10 · I. Passes today's 1e-100 absolute
        // threshold trivially but exercises the new relative-scaled guard.
        let alpha = 1e-10_f64;
        let a = [[alpha, 0.0, 0.0], [0.0, alpha, 0.0], [0.0, 0.0, alpha]];
        let inv = mat3_inv(&a).expect("invertible");
        let expected = 1.0 / alpha;
        for i in 0..3 {
            for j in 0..3 {
                let target = if i == j { expected } else { 0.0 };
                let rel = if target != 0.0 {
                    (inv[i][j] - target).abs() / target.abs()
                } else {
                    inv[i][j].abs()
                };
                assert!(rel < 1e-12, "({i},{j}): {} vs {target}", inv[i][j]);
            }
        }
    }

    #[test]
    fn test_mat3_inv_rank_deficient_returns_none() {
        // Row 2 = 2*row 0 → rank-deficient → singular.
        let a = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [2.0, 4.0, 6.0]];
        assert!(mat3_inv(&a).is_none());
    }

    #[test]
    fn test_mat3_solve_rank_deficient_returns_none() {
        let a = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [2.0, 4.0, 6.0]];
        let b = [1.0, 2.0, 3.0];
        assert!(mat3_solve(&a, &b).is_none());
    }

    #[test]
    fn test_mat3_solve_round_trip_wide_dynamic_range() {
        // Verify Cramer's rule still works across [1e-20, 1e10] scales.
        let scales = [1e-20_f64, 1.0, 1e10];
        for &alpha in &scales {
            let a = [
                [2.0 * alpha, 0.5 * alpha, 0.0],
                [0.5 * alpha, 3.0 * alpha, 0.2 * alpha],
                [0.0, 0.2 * alpha, 1.0 * alpha],
            ];
            let b = [1.0_f64 * alpha, 0.0, -1.0 * alpha];
            let x = mat3_solve(&a, &b).expect("solvable");
            for i in 0..3 {
                let mut s = 0.0;
                for j in 0..3 {
                    s += a[i][j] * x[j];
                }
                let denom = b[i].abs().max(alpha);
                let rel_err = (s - b[i]).abs() / denom;
                assert!(rel_err < 1e-12, "α={alpha}, i={i}: rel = {rel_err}");
            }
        }
    }

    #[test]
    fn test_mat3_inv_extremely_small_scale_still_invertible() {
        // A = 1e-100 · I → |det| = 1e-300, max_entry^3 = 1e-300.
        // Ratio = 1, which is well above NOLAN_REL_TOL — but max_entry^3
        // floors to MIN_POSITIVE, so this still passes the guard.
        let alpha = 1e-100_f64;
        let a = [[alpha, 0.0, 0.0], [0.0, alpha, 0.0], [0.0, 0.0, alpha]];
        // Should succeed: relative-singularity guard does not over-reject.
        let inv = mat3_inv(&a).expect("invertible");
        assert!((inv[0][0] - 1.0 / alpha).abs() / (1.0 / alpha) < 1e-12);
    }
}
