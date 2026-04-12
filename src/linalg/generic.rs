use crate::traits::DifferentiableMath;

/// Solve \\(A \mathbf{x} = \mathbf{b}\\) for an \\(N \times N\\) system via Gauss–Jordan
/// elimination with partial pivoting.
///
/// Uses `Vec<Vec<T>>` for the augmented matrix since `N+1` cannot be expressed
/// as a const generic on stable Rust. Returns `None` if the matrix is singular
/// (pivot magnitude below \\(10^{-30}\\)).
#[allow(clippy::needless_range_loop)]
pub fn mat_solve<T: Copy + DifferentiableMath, const N: usize>(
    a: &[[T; N]; N],
    b: &[T; N],
) -> Option<[T; N]> {
    let zero = T::constant(0.0);
    // Augmented matrix [A | b]
    let mut m: Vec<Vec<T>> = (0..N)
        .map(|i| {
            let mut row = vec![zero; N + 1];
            row[..N].copy_from_slice(&a[i]);
            row[N] = b[i];
            row
        })
        .collect();

    for col in 0..N {
        let mut max_row = col;
        let mut max_val = m[col][col].value().abs();
        for row in (col + 1)..N {
            let v = m[row][col].value().abs();
            if v > max_val {
                max_val = v;
                max_row = row;
            }
        }
        if max_val < 1e-30 {
            return None;
        }
        if max_row != col {
            m.swap(col, max_row);
        }

        let pivot = m[col][col];
        for j in 0..N + 1 {
            m[col][j] = m[col][j] / pivot;
        }

        for row in 0..N {
            if row == col {
                continue;
            }
            let factor = m[row][col];
            for j in 0..N + 1 {
                m[row][j] = m[row][j] - factor * m[col][j];
            }
        }
    }

    let mut x = [zero; N];
    for i in 0..N {
        x[i] = m[i][N];
    }
    Some(x)
}

/// Invert an \\(N \times N\\) matrix via Gauss–Jordan elimination with partial pivoting.
///
/// Uses `Vec<Vec<T>>` for the augmented matrix since `2*N` cannot be expressed
/// as a const generic on stable Rust. Returns `None` if the matrix is singular
/// (pivot magnitude below \\(10^{-30}\\)).
#[allow(clippy::needless_range_loop)]
pub fn mat_inv<T: Copy + DifferentiableMath, const N: usize>(
    a: &[[T; N]; N],
) -> Option<[[T; N]; N]> {
    let zero = T::constant(0.0);
    let one = T::constant(1.0);
    // Augmented matrix [A | I]
    let mut m: Vec<Vec<T>> = (0..N)
        .map(|i| {
            let mut row = vec![zero; 2 * N];
            row[..N].copy_from_slice(&a[i]);
            row[N + i] = one;
            row
        })
        .collect();

    for col in 0..N {
        let mut max_row = col;
        let mut max_val = m[col][col].value().abs();
        for row in (col + 1)..N {
            let v = m[row][col].value().abs();
            if v > max_val {
                max_val = v;
                max_row = row;
            }
        }
        if max_val < 1e-30 {
            return None;
        }
        if max_row != col {
            m.swap(col, max_row);
        }

        let pivot = m[col][col];
        for j in 0..2 * N {
            m[col][j] = m[col][j] / pivot;
        }

        for row in 0..N {
            if row == col {
                continue;
            }
            let factor = m[row][col];
            for j in 0..2 * N {
                m[row][j] = m[row][j] - factor * m[col][j];
            }
        }
    }

    let mut inv = [[zero; N]; N];
    for i in 0..N {
        for j in 0..N {
            inv[i][j] = m[i][N + j];
        }
    }
    Some(inv)
}

/// Symmetrize an \\(N \times N\\) matrix: \\(S = \tfrac{1}{2}(A + A^\top)\\).
#[inline]
pub fn mat_symmetrize<T: Copy + DifferentiableMath, const N: usize>(
    a: &[[T; N]; N],
) -> [[T; N]; N] {
    let zero = T::constant(0.0);
    let mut s = [[zero; N]; N];
    for i in 0..N {
        for j in i..N {
            let avg = (a[i][j] + a[j][i]) * 0.5;
            s[i][j] = avg;
            s[j][i] = avg;
        }
    }
    s
}

/// Compute the quadratic form \\(\mathbf{x}^\top A \, \mathbf{x}\\).
#[inline]
pub fn mat_quadratic_form<T: Copy + DifferentiableMath, const N: usize>(
    x: &[T; N],
    a: &[[T; N]; N],
) -> T {
    let mut result = T::constant(0.0);
    for i in 0..N {
        for j in 0..N {
            result = result + x[i] * a[i][j] * x[j];
        }
    }
    result
}

/// Cholesky decomposition of an \\(N \times N\\) symmetric positive-definite matrix.
///
/// Returns the lower-triangular factor \\(L\\) such that \\(A = L L^\top\\).
/// Returns `None` if the matrix is not positive-definite.
#[allow(clippy::needless_range_loop)]
pub fn mat_cholesky<const N: usize>(a: &[[f64; N]; N]) -> Option<[[f64; N]; N]> {
    let mut l = [[0.0_f64; N]; N];
    for i in 0..N {
        for j in 0..=i {
            let mut sum = 0.0;
            for k in 0..j {
                sum += l[i][k] * l[j][k];
            }
            if i == j {
                let diag = a[i][i] - sum;
                if diag <= 0.0 {
                    return None;
                }
                l[i][j] = diag.sqrt();
            } else {
                l[i][j] = (a[i][j] - sum) / l[j][j];
            }
        }
    }
    Some(l)
}

/// Trace of an \\(N \times N\\) matrix: \\(\mathrm{Tr}(A) = \sum_i A_{ii}\\).
#[inline]
pub fn mat_trace<const N: usize>(a: &[[f64; N]; N]) -> f64 {
    let mut t = 0.0;
    for i in 0..N {
        t += a[i][i];
    }
    t
}

/// Trace of the product of two \\(N \times N\\) matrices:
/// \\(\mathrm{Tr}(AB) = \sum_{i,j} A_{ij} B_{ji}\\).
///
/// Computed without forming the full product matrix.
#[inline]
pub fn mat_trace_product<const N: usize>(a: &[[f64; N]; N], b: &[[f64; N]; N]) -> f64 {
    let mut trace = 0.0;
    for i in 0..N {
        for j in 0..N {
            trace += a[i][j] * b[j][i];
        }
    }
    trace
}

/// Log-determinant of an \\(N \times N\\) matrix via LU decomposition with
/// partial pivoting.
///
/// Returns \\(\ln |\det(A)|\\). Returns `NEG_INFINITY` if the matrix is singular.
#[allow(clippy::needless_range_loop)]
pub fn mat_log_det<const N: usize>(a: &[[f64; N]; N]) -> f64 {
    let mut lu = *a;
    let mut log_det = 0.0;

    for col in 0..N {
        let mut max_row = col;
        let mut max_val = lu[col][col].abs();
        for row in (col + 1)..N {
            let v = lu[row][col].abs();
            if v > max_val {
                max_val = v;
                max_row = row;
            }
        }
        if max_val < 1e-300 {
            return f64::NEG_INFINITY;
        }
        if max_row != col {
            lu.swap(col, max_row);
        }

        log_det += lu[col][col].abs().ln();

        for row in (col + 1)..N {
            let factor = lu[row][col] / lu[col][col];
            for j in (col + 1)..N {
                lu[row][j] -= factor * lu[col][j];
            }
        }
    }

    log_det
}

/// Euclidean norm of an N-vector: \\(\lVert\mathbf{x}\rVert = \sqrt{\sum_i x_i^2}\\).
#[inline]
pub fn vec_norm<const N: usize>(x: &[f64; N]) -> f64 {
    let mut sum = 0.0;
    for item in x {
        sum += item * item;
    }
    sum.sqrt()
}

/// Matrix-vector product \\(\mathbf{y} = A\mathbf{x}\\) for \\(N \times N\\) matrices.
#[inline]
#[allow(clippy::needless_range_loop)]
pub fn mat_vec_mul<const N: usize>(a: &[[f64; N]; N], x: &[f64; N]) -> [f64; N] {
    let mut y = [0.0; N];
    for i in 0..N {
        for j in 0..N {
            y[i] += a[i][j] * x[j];
        }
    }
    y
}

/// Squared Mahalanobis distance:
/// \\(d^2 = (\mathbf{x} - \boldsymbol{\mu})^\top \Sigma^{-1} (\mathbf{x} - \boldsymbol{\mu})\\).
///
/// Returns `None` if the covariance matrix is singular.
pub fn mahalanobis_distance_squared<const N: usize>(
    x: &[f64; N],
    mu: &[f64; N],
    cov: &[[f64; N]; N],
) -> Option<f64> {
    let cov_inv = mat_inv(cov)?;
    let mut delta = [0.0; N];
    for i in 0..N {
        delta[i] = x[i] - mu[i];
    }
    Some(mat_quadratic_form(&delta, &cov_inv))
}

/// Squared Mahalanobis distance using a pre-computed inverse covariance.
#[inline]
pub fn mahalanobis_distance_squared_with_inv<const N: usize>(
    x: &[f64; N],
    mu: &[f64; N],
    cov_inv: &[[f64; N]; N],
) -> f64 {
    let mut delta = [0.0; N];
    for i in 0..N {
        delta[i] = x[i] - mu[i];
    }
    mat_quadratic_form(&delta, cov_inv)
}

/// Find the eigenvector corresponding to the largest absolute eigenvalue
/// via power iteration.
///
/// Returns `(eigenvector, eigenvalue)` where the eigenvector is unit-normalized.
/// Works for any real \\(N \times N\\) matrix (need not be symmetric).
#[allow(clippy::needless_range_loop)]
pub fn mat_eigenvector_max<const N: usize>(
    a: &[[f64; N]; N],
    max_iter: usize,
    tol: f64,
) -> ([f64; N], f64) {
    // Initial vector: normalised [1, 1, ..., 1]
    let inv_sqrt_n = 1.0 / (N as f64).sqrt();
    let mut v = [inv_sqrt_n; N];
    let mut lambda = 0.0_f64;

    for _ in 0..max_iter {
        let w = mat_vec_mul(a, &v);
        let w_norm = vec_norm(&w);

        if w_norm < 1e-30 {
            return (v, 0.0);
        }

        // Sign of eigenvalue: dot(w, v_old)
        let mut dot = 0.0;
        for i in 0..N {
            dot += w[i] * v[i];
        }
        let sign = if dot >= 0.0 { 1.0 } else { -1.0 };

        // Update eigenvector
        for i in 0..N {
            v[i] = w[i] / w_norm;
        }
        lambda = sign * w_norm;

        // Check convergence: ||Av - λv|| / |λ|
        let av = mat_vec_mul(a, &v);
        let mut residual_sq = 0.0;
        for i in 0..N {
            let diff = av[i] - lambda * v[i];
            residual_sq += diff * diff;
        }
        if lambda.abs() > 1e-30 && residual_sq.sqrt() / lambda.abs() < tol {
            break;
        }
    }

    // Rayleigh quotient refinement
    let av = mat_vec_mul(a, &v);
    let mut numerator = 0.0;
    for i in 0..N {
        numerator += v[i] * av[i];
    }
    lambda = numerator; // v^T A v (v is unit)

    (v, lambda)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jets::Jet1;
    use crate::traits::{Differentiable, FirstOrder};

    #[test]
    fn test_mat_solve_3x3() {
        let a = [[2.0, 1.0, 0.0], [1.0, 3.0, 1.0], [0.0, 1.0, 2.0]];
        let b = [1.0, 2.0, 3.0];
        let x = mat_solve::<f64, 3>(&a, &b).unwrap();
        // Verify A*x = b
        for i in 0..3 {
            let mut sum = 0.0;
            for j in 0..3 {
                sum += a[i][j] * x[j];
            }
            assert!((sum - b[i]).abs() < 1e-14);
        }
    }

    #[test]
    fn test_mat_inv_3x3() {
        let a = [[2.0, 1.0, 0.0], [1.0, 3.0, 1.0], [0.0, 1.0, 2.0]];
        let inv = mat_inv::<f64, 3>(&a).unwrap();
        // Verify A * A^-1 = I
        for i in 0..3 {
            for j in 0..3 {
                let mut sum = 0.0;
                for k in 0..3 {
                    sum += a[i][k] * inv[k][j];
                }
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((sum - expected).abs() < 1e-14);
            }
        }
    }

    #[test]
    fn test_mat_solve_6x6_vs_specialized() {
        // Create a diagonally-dominant 6×6 matrix
        let mut a = [[0.0; 6]; 6];
        for i in 0..6 {
            a[i][i] = (i + 2) as f64;
            for j in 0..6 {
                if i != j {
                    a[i][j] = 0.1;
                }
            }
        }
        let b = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];

        let x_generic = mat_solve::<f64, 6>(&a, &b).unwrap();
        let x_specialized = crate::linalg::mat6::mat6_solve(&a, &b).unwrap();

        for i in 0..6 {
            assert!(
                (x_generic[i] - x_specialized[i]).abs() < 1e-13,
                "x_generic[{}] = {}, x_specialized[{}] = {}",
                i,
                x_generic[i],
                i,
                x_specialized[i]
            );
        }
    }

    #[test]
    fn test_mat_solve_9x9_vs_specialized() {
        let mut a = [[0.0; 9]; 9];
        for i in 0..9 {
            a[i][i] = (i + 2) as f64;
            for j in 0..9 {
                if i != j {
                    a[i][j] = 0.1;
                }
            }
        }
        let b = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];

        let x_generic = mat_solve::<f64, 9>(&a, &b).unwrap();
        let x_specialized = crate::linalg::mat9::mat9_solve(&a, &b).unwrap();

        for i in 0..9 {
            assert!(
                (x_generic[i] - x_specialized[i]).abs() < 1e-12,
                "x_generic[{}] = {}, x_specialized[{}] = {}",
                i,
                x_generic[i],
                i,
                x_specialized[i]
            );
        }
    }

    #[test]
    fn test_mat_symmetrize() {
        let mut a = [[0.0; 4]; 4];
        a[0][1] = 2.0;
        a[1][0] = 4.0;
        a[2][3] = 6.0;
        a[3][2] = 8.0;
        let s = mat_symmetrize(&a);
        assert!((s[0][1] - 3.0).abs() < 1e-15);
        assert!((s[1][0] - 3.0).abs() < 1e-15);
        assert!((s[2][3] - 7.0).abs() < 1e-15);
        assert!((s[3][2] - 7.0).abs() < 1e-15);
    }

    #[test]
    fn test_mat_quadratic_form() {
        // x^T I x = ||x||^2
        let mut id = [[0.0; 3]; 3];
        for i in 0..3 {
            id[i][i] = 1.0;
        }
        let x = [1.0, 2.0, 3.0];
        let q = mat_quadratic_form(&x, &id);
        assert!((q - 14.0).abs() < 1e-15);
    }

    #[test]
    fn test_mat_quadratic_form_jet1() {
        let mut id = [[Jet1::<3>::constant(0.0); 3]; 3];
        for i in 0..3 {
            id[i][i] = Jet1::<3>::constant(1.0);
        }
        let x = [
            Jet1::<3>::variable(1.0, 0),
            Jet1::<3>::variable(2.0, 1),
            Jet1::<3>::variable(3.0, 2),
        ];
        let q = mat_quadratic_form(&x, &id);
        // x^T I x = x0^2 + x1^2 + x2^2 = 14
        assert!((q.value() - 14.0).abs() < 1e-15);
        // d/dx0 = 2*x0 = 2
        assert!((q.grad(0) - 2.0).abs() < 1e-14);
        // d/dx1 = 2*x1 = 4
        assert!((q.grad(1) - 4.0).abs() < 1e-14);
        // d/dx2 = 2*x2 = 6
        assert!((q.grad(2) - 6.0).abs() < 1e-14);
    }

    #[test]
    fn test_mat_solve_jet1() {
        let mut a = [[Jet1::<3>::constant(0.0); 3]; 3];
        a[0][0] = Jet1::<3>::variable(2.0, 0);
        a[0][1] = Jet1::<3>::constant(1.0);
        a[1][0] = Jet1::<3>::constant(1.0);
        a[1][1] = Jet1::<3>::variable(3.0, 1);
        a[1][2] = Jet1::<3>::constant(1.0);
        a[2][1] = Jet1::<3>::constant(1.0);
        a[2][2] = Jet1::<3>::variable(2.0, 2);
        let b = [
            Jet1::<3>::constant(1.0),
            Jet1::<3>::constant(2.0),
            Jet1::<3>::constant(3.0),
        ];
        let x = mat_solve::<Jet1<3>, 3>(&a, &b).unwrap();
        // Verify A*x = b (values)
        for i in 0..3 {
            let mut sum = Jet1::<3>::constant(0.0);
            for j in 0..3 {
                sum = sum + a[i][j] * x[j];
            }
            assert!(
                (sum.value() - b[i].value()).abs() < 1e-13,
                "row {} residual: {}",
                i,
                sum.value() - b[i].value()
            );
        }
    }

    #[test]
    fn test_mat_inv_singular() {
        // Exactly singular matrix (row 3 = row 1)
        let a = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [1.0, 2.0, 3.0]];
        assert!(mat_inv::<f64, 3>(&a).is_none());
    }

    #[test]
    fn test_mat_cholesky_3x3() {
        let a = [[4.0, 2.0, 0.0], [2.0, 5.0, 1.0], [0.0, 1.0, 3.0]];
        let l = mat_cholesky(&a).unwrap();
        for i in 0..3 {
            for j in 0..3 {
                let mut sum = 0.0;
                for k in 0..3 {
                    sum += l[i][k] * l[j][k];
                }
                assert!((sum - a[i][j]).abs() < 1e-12);
            }
        }
        assert!(l[0][1].abs() < 1e-15);
    }

    #[test]
    fn test_mat_cholesky_not_spd() {
        let a = [[1.0, 0.0], [0.0, -1.0]];
        assert!(mat_cholesky(&a).is_none());
    }

    #[test]
    fn test_mat_cholesky_6x6_identity() {
        let mut a = [[0.0_f64; 6]; 6];
        for i in 0..6 {
            a[i][i] = 1.0;
        }
        let l = mat_cholesky(&a).unwrap();
        for i in 0..6 {
            assert!((l[i][i] - 1.0).abs() < 1e-15);
        }
    }

    #[test]
    fn test_mat_trace() {
        let a = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];
        assert!((mat_trace(&a) - 15.0).abs() < 1e-15);
    }

    #[test]
    fn test_mat_log_det() {
        let a = [[2.0, 0.0], [0.0, 3.0]];
        assert!((mat_log_det(&a) - 6.0_f64.ln()).abs() < 1e-12);
    }

    #[test]
    fn test_vec_norm() {
        let x = [3.0, 4.0];
        assert!((vec_norm(&x) - 5.0).abs() < 1e-15);
    }

    #[test]
    fn test_mahalanobis_identity_cov() {
        let x = [3.0, 4.0];
        let mu = [0.0, 0.0];
        let cov = [[1.0, 0.0], [0.0, 1.0]];
        let d2 = mahalanobis_distance_squared(&x, &mu, &cov).unwrap();
        assert!((d2 - 25.0).abs() < 1e-10);
    }

    #[test]
    fn test_mat_eigenvector_max_diagonal() {
        let a = [[3.0, 0.0], [0.0, 7.0]];
        let (_v, lambda) = mat_eigenvector_max(&a, 100, 1e-12);
        assert!((lambda - 7.0).abs() < 1e-8, "lambda={lambda}");
    }
}
