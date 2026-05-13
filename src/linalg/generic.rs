use crate::linalg::{NOLAN_MIN_SCALE, NOLAN_REL_TOL};
use crate::traits::DifferentiableMath;

/// Row-infinity norms of an \\(N \times N\\) matrix, taking `.value()` to
/// drop Jet partials before comparison.
#[inline]
#[allow(clippy::needless_range_loop)]
fn row_inf_norms<T: Copy + DifferentiableMath, const N: usize>(a: &[[T; N]; N]) -> [f64; N] {
    std::array::from_fn(|i| {
        let mut max = 0.0_f64;
        for j in 0..N {
            let v = a[i][j].value().abs();
            if v > max {
                max = v;
            }
        }
        max
    })
}

/// Solve \\(A \mathbf{x} = \mathbf{b}\\) for an \\(N \times N\\) system via Gauss–Jordan
/// elimination with **scaled partial pivoting**.
///
/// Uses `Vec<Vec<T>>` for the augmented matrix since `N+1` cannot be expressed
/// as a const generic on stable Rust.
///
/// Pivoting picks the row maximising the ratio
/// \\(|a_{ik}| / \max_j |a_{ij}|\\) over the active sub-matrix, which is
/// robust to mixed-scale rows where a plain absolute-value pivot would
/// pick a row whose pivot is only large because the entire row is large.
/// Returns `None` if the largest scaled pivot ratio falls below
/// [`NOLAN_REL_TOL`].
#[allow(clippy::needless_range_loop)]
pub fn mat_solve<T: Copy + DifferentiableMath, const N: usize>(
    a: &[[T; N]; N],
    b: &[T; N],
) -> Option<[T; N]> {
    let zero = T::constant(0.0);
    let mut s = row_inf_norms(a);
    if s.iter().all(|x| *x < NOLAN_MIN_SCALE) {
        return None;
    }

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
        let mut best_ratio = 0.0;
        let mut best_row = col;
        for i in col..N {
            if s[i] < NOLAN_MIN_SCALE {
                continue;
            }
            let ratio = m[i][col].value().abs() / s[i];
            if ratio > best_ratio {
                best_ratio = ratio;
                best_row = i;
            }
        }
        if best_ratio < NOLAN_REL_TOL {
            return None;
        }
        if best_row != col {
            m.swap(col, best_row);
            s.swap(col, best_row);
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

/// Invert an \\(N \times N\\) matrix via Gauss–Jordan elimination with
/// **scaled partial pivoting**.
///
/// See [`mat_solve`] for the pivoting strategy. Returns `None` if the
/// largest scaled pivot ratio falls below [`NOLAN_REL_TOL`].
#[allow(clippy::needless_range_loop)]
pub fn mat_inv<T: Copy + DifferentiableMath, const N: usize>(
    a: &[[T; N]; N],
) -> Option<[[T; N]; N]> {
    let zero = T::constant(0.0);
    let one = T::constant(1.0);
    let mut s = row_inf_norms(a);
    if s.iter().all(|x| *x < NOLAN_MIN_SCALE) {
        return None;
    }

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
        let mut best_ratio = 0.0;
        let mut best_row = col;
        for i in col..N {
            if s[i] < NOLAN_MIN_SCALE {
                continue;
            }
            let ratio = m[i][col].value().abs() / s[i];
            if ratio > best_ratio {
                best_ratio = ratio;
                best_row = i;
            }
        }
        if best_ratio < NOLAN_REL_TOL {
            return None;
        }
        if best_row != col {
            m.swap(col, best_row);
            s.swap(col, best_row);
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

/// Full symmetric eigendecomposition of an \\(N \times N\\) real
/// symmetric matrix via the **classical Jacobi rotation method**.
///
/// Returns `(eigenvalues, eigenvectors)` where `eigenvalues` is sorted
/// descending and `eigenvectors[i][k]` is element `i` of the
/// eigenvector corresponding to `eigenvalues[k]` (i.e., columns of the
/// returned matrix are eigenvectors). Returns `None` if the iteration
/// fails to converge within the sweep budget or produces non-finite
/// output.
///
/// # Why Jacobi
///
/// Jacobi is simple, robust, and numerically accurate for small-to-
/// moderate symmetric matrices (\\(N \lesssim 16\\)). It works by
/// repeatedly applying \\(2 \times 2\\) Givens rotations to zero off-
/// diagonal entries; convergence is guaranteed because each rotation
/// strictly decreases the Frobenius norm of the off-diagonal part. For
/// 6×6 well-conditioned inputs it converges in roughly 5 sweeps to
/// machine precision; highly anisotropic spectra (eigenvalues spanning
/// 20+ orders of magnitude) can take more, hence the generous
/// sweep budget.
///
/// # Tolerance
///
/// **The convergence criterion is RELATIVE to the matrix Frobenius
/// scale**, not absolute. Covariance matrices and similar physical-
/// units inputs can have natural scales spanning many orders of
/// magnitude — an absolute tolerance on the off-diagonal sum-of-
/// squares would fire spuriously on small-scale inputs (e.g., a
/// covariance with entries at \\(10^{-10}\\) would have an off-
/// diagonal sum-of-squares < \\(10^{-15}\\) even when far from
/// diagonalized), causing the loop to exit before performing any
/// rotation and returning the sorted input diagonal as "eigenvalues."
/// The same pattern applies inside the sweep: per-pair rotation skip
/// uses a relative threshold against the pair's diagonal scale.
///
/// # Algorithm reference
///
/// Golub & Van Loan, *Matrix Computations*, 4th ed., §8.5
/// ("The Symmetric QR Algorithm") — Algorithm 8.5.2 (Cyclic Jacobi
/// for Symmetric Eigenproblem). The implementation uses the
/// "tangent of the rotation angle" parametrization that avoids
/// trigonometric calls.
///
/// # Example
///
/// ```
/// use nolan::linalg::generic::mat_symmetric_eigen;
///
/// // Identity matrix: all eigenvalues are 1.
/// let mut id = [[0.0_f64; 6]; 6];
/// for i in 0..6 { id[i][i] = 1.0; }
/// let (eigs, _vecs) = mat_symmetric_eigen(&id).unwrap();
/// for &e in &eigs { assert!((e - 1.0).abs() < 1e-12); }
/// ```
#[allow(clippy::needless_range_loop)]
pub fn mat_symmetric_eigen<const N: usize>(a: &[[f64; N]; N]) -> Option<([f64; N], [[f64; N]; N])> {
    let mut m = *a;
    // Symmetrize defensively against tiny floating-point asymmetry.
    for i in 0..N {
        for j in (i + 1)..N {
            let avg = 0.5 * (m[i][j] + m[j][i]);
            m[i][j] = avg;
            m[j][i] = avg;
        }
    }
    let mut v = [[0.0_f64; N]; N];
    for i in 0..N {
        v[i][i] = 1.0;
    }
    // Frobenius norm² of the input — invariant under Jacobi rotations.
    let mut frob2_total = 0.0_f64;
    for i in 0..N {
        for j in 0..N {
            frob2_total += m[i][j] * m[i][j];
        }
    }
    // Sweep budget: 64 sweeps is overkill for well-conditioned 6×6
    // inputs (converges in ~5) but covers the highly-anisotropic case
    // where the spectrum spans many orders of magnitude.
    const MAX_SWEEPS: usize = 64;
    // Stop when sum of squared off-diagonals < REL_TOL² × ‖A‖_F².
    const REL_TOL: f64 = 1e-14;
    let tol_off2 = REL_TOL * REL_TOL * frob2_total.max(f64::MIN_POSITIVE);
    for _ in 0..MAX_SWEEPS {
        let mut off = 0.0_f64;
        for i in 0..N {
            for j in (i + 1)..N {
                off += m[i][j] * m[i][j];
            }
        }
        if off < tol_off2 {
            break;
        }
        // Sweep all (p, q) pairs with p < q.
        for p in 0..(N - 1) {
            for q in (p + 1)..N {
                let apq = m[p][q];
                // Skip the rotation when apq is negligible RELATIVE to
                // the diagonal scale at this pair. Absolute thresholds
                // miss tiny-but-significant entries on small-magnitude
                // matrices.
                let scale = m[p][p].abs().max(m[q][q].abs()).max(f64::MIN_POSITIVE);
                if apq.abs() < REL_TOL * scale {
                    continue;
                }
                let app = m[p][p];
                let aqq = m[q][q];
                let theta = 0.5 * (aqq - app) / apq;
                // Tangent of the rotation angle; the conditional
                // protects against overflow when theta is huge.
                let t = if theta.abs() > 1e150 {
                    0.5 / theta
                } else {
                    let sign = if theta >= 0.0 { 1.0 } else { -1.0 };
                    sign / (theta.abs() + (theta * theta + 1.0).sqrt())
                };
                let cos_phi = 1.0 / (t * t + 1.0).sqrt();
                let sin_phi = t * cos_phi;
                // Update the eigenvalue carriers on the (p, q) pair.
                m[p][p] = app - t * apq;
                m[q][q] = aqq + t * apq;
                m[p][q] = 0.0;
                m[q][p] = 0.0;
                // Update off-pair rows/columns.
                for r in 0..N {
                    if r == p || r == q {
                        continue;
                    }
                    let arp = m[r][p];
                    let arq = m[r][q];
                    m[r][p] = cos_phi * arp - sin_phi * arq;
                    m[p][r] = m[r][p];
                    m[r][q] = sin_phi * arp + cos_phi * arq;
                    m[q][r] = m[r][q];
                }
                // Accumulate the rotation in the eigenvector matrix.
                for r in 0..N {
                    let vrp = v[r][p];
                    let vrq = v[r][q];
                    v[r][p] = cos_phi * vrp - sin_phi * vrq;
                    v[r][q] = sin_phi * vrp + cos_phi * vrq;
                }
            }
        }
    }
    let mut eigs = [0.0_f64; N];
    for i in 0..N {
        eigs[i] = m[i][i];
    }
    // Build a permutation that sorts eigenvalues descending. Cannot
    // sort `[usize; N]` with `.sort_by` directly when N is generic and
    // we want it stable, so collect into a Vec briefly.
    let mut idx: [usize; N] = [0; N];
    for i in 0..N {
        idx[i] = i;
    }
    // Insertion sort — N is small.
    for i in 1..N {
        let key = idx[i];
        let key_val = eigs[key];
        let mut j = i;
        while j > 0 && eigs[idx[j - 1]] < key_val {
            idx[j] = idx[j - 1];
            j -= 1;
        }
        idx[j] = key;
    }
    let mut sorted_eigs = [0.0_f64; N];
    let mut sorted_v = [[0.0_f64; N]; N];
    for k in 0..N {
        sorted_eigs[k] = eigs[idx[k]];
        for i in 0..N {
            sorted_v[i][k] = v[i][idx[k]];
        }
    }
    if !sorted_eigs.iter().all(|x| x.is_finite()) {
        return None;
    }
    Some((sorted_eigs, sorted_v))
}

// ── Phase 1D additions: determinant, trace-cube, Frobenius, σ_max ───

/// Signed determinant of an \\(N \times N\\) matrix via LU decomposition
/// with partial pivoting.
///
/// Returns `0.0` if the matrix is singular (the LU step encounters a
/// zero pivot at scaled-relative tolerance).
///
/// Sign comes from the parity of row swaps; magnitude from the product
/// of diagonal U entries.
#[allow(clippy::needless_range_loop)]
pub fn mat_det<const N: usize>(a: &[[f64; N]; N]) -> f64 {
    let mut lu = *a;
    let mut s = row_inf_norms(a);
    if s.iter().all(|x| *x < NOLAN_MIN_SCALE) {
        return 0.0;
    }
    let mut sign = 1.0;
    let mut det = 1.0;

    for col in 0..N {
        let mut best_ratio = 0.0;
        let mut best_row = col;
        for i in col..N {
            if s[i] < NOLAN_MIN_SCALE {
                continue;
            }
            let ratio = lu[i][col].abs() / s[i];
            if ratio > best_ratio {
                best_ratio = ratio;
                best_row = i;
            }
        }
        if best_ratio < NOLAN_REL_TOL {
            return 0.0;
        }
        if best_row != col {
            lu.swap(col, best_row);
            s.swap(col, best_row);
            sign = -sign;
        }

        det *= lu[col][col];

        for row in (col + 1)..N {
            let factor = lu[row][col] / lu[col][col];
            for j in (col + 1)..N {
                lu[row][j] -= factor * lu[col][j];
            }
        }
    }

    sign * det
}

/// Trace of the cube of the product \\(M = AB\\):
/// \\(\mathrm{Tr}(M^3) = \sum_{i,j,k} M_{ij}\,M_{jk}\,M_{ki}\\),
/// computed without forming \\(M^3\\) explicitly.
#[allow(clippy::needless_range_loop)]
pub fn mat_trace_cube<const N: usize>(a: &[[f64; N]; N], b: &[[f64; N]; N]) -> f64 {
    let m = mat_mul::<N, N, N>(a, b);
    let mut trace = 0.0;
    for i in 0..N {
        for j in 0..N {
            for k in 0..N {
                trace += m[i][j] * m[j][k] * m[k][i];
            }
        }
    }
    trace
}

/// Frobenius norm of an \\(M \times N\\) matrix:
/// \\(\lVert A \rVert_F = \sqrt{\sum_{i,j} A_{ij}^2}\\).
#[inline]
pub fn mat_frobenius<const M: usize, const N: usize>(a: &[[f64; N]; M]) -> f64 {
    let mut sum = 0.0;
    for row in a.iter() {
        for &val in row {
            sum += val * val;
        }
    }
    sum.sqrt()
}

/// Largest singular value of an \\(N \times N\\) matrix via power iteration
/// on \\(A^\top A\\): \\(\sigma_{\max} = \sqrt{\lambda_{\max}(A^\top A)}\\).
///
/// Returns `0.0` when the dominant eigenvalue of \\(A^\top A\\) is
/// non-positive (degenerate matrix; this should not occur for any
/// well-formed real matrix since \\(A^\top A\\) is always PSD).
pub fn mat_largest_singular_value<const N: usize>(
    a: &[[f64; N]; N],
    max_iter: usize,
    tol: f64,
) -> f64 {
    let ata = mat_ata::<N, N>(a);
    let (_v, lambda) = mat_eigenvector_max(&ata, max_iter, tol);
    if lambda <= 0.0 { 0.0 } else { lambda.sqrt() }
}

// ── Phase 1E additions: rectangular mat_mul, transpose, AᵀA ─────────

/// Rectangular matrix multiplication: \\(C = A B\\) where
/// \\(A\\) is \\(M \times K\\), \\(B\\) is \\(K \times N\\),
/// \\(C\\) is \\(M \times N\\).
#[allow(clippy::needless_range_loop)]
pub fn mat_mul<const M: usize, const K: usize, const N: usize>(
    a: &[[f64; K]; M],
    b: &[[f64; N]; K],
) -> [[f64; N]; M] {
    let mut c = [[0.0_f64; N]; M];
    for i in 0..M {
        for j in 0..N {
            let mut s = 0.0;
            for k in 0..K {
                s += a[i][k] * b[k][j];
            }
            c[i][j] = s;
        }
    }
    c
}

/// Transpose of an \\(M \times N\\) matrix: returns \\(A^\top\\) of shape
/// \\(N \times M\\).
#[allow(clippy::needless_range_loop)]
pub fn mat_transpose<const M: usize, const N: usize>(a: &[[f64; N]; M]) -> [[f64; M]; N] {
    let mut t = [[0.0_f64; M]; N];
    for i in 0..M {
        for j in 0..N {
            t[j][i] = a[i][j];
        }
    }
    t
}

/// Symmetric product \\(A^\top A\\) for an \\(M \times N\\) matrix.
/// Returns an \\(N \times N\\) symmetric matrix.
///
/// Slightly cheaper than `mat_mul(&mat_transpose(a), a)` because only
/// the lower triangle is computed and mirrored.
#[allow(clippy::needless_range_loop)]
pub fn mat_ata<const M: usize, const N: usize>(a: &[[f64; N]; M]) -> [[f64; N]; N] {
    let mut c = [[0.0_f64; N]; N];
    for i in 0..N {
        for j in 0..=i {
            let mut s = 0.0;
            for k in 0..M {
                s += a[k][i] * a[k][j];
            }
            c[i][j] = s;
            c[j][i] = s;
        }
    }
    c
}

#[cfg(test)]
#[allow(clippy::needless_range_loop)]
#[allow(clippy::assign_op_pattern)]
mod tests {
    use super::*;
    use crate::jets::Jet1;
    use crate::traits::{Differentiable, FirstOrder};

    // ── mat_symmetric_eigen tests ────────────────────────────────────

    #[test]
    fn test_symmetric_eigen_identity_6x6() {
        let mut id = [[0.0_f64; 6]; 6];
        for i in 0..6 {
            id[i][i] = 1.0;
        }
        let (eigs, vecs) = mat_symmetric_eigen(&id).expect("converged");
        for &e in &eigs {
            assert!((e - 1.0).abs() < 1e-12);
        }
        // Eigenvectors are columns of identity (up to sign).
        for k in 0..6 {
            for i in 0..6 {
                let v = vecs[i][k];
                if i == k {
                    assert!((v.abs() - 1.0).abs() < 1e-12);
                } else {
                    assert!(v.abs() < 1e-12);
                }
            }
        }
    }

    #[test]
    fn test_symmetric_eigen_descending_diagonal_3x3() {
        let mut m = [[0.0_f64; 3]; 3];
        m[0][0] = 5.0;
        m[1][1] = 2.0;
        m[2][2] = 0.5;
        let (eigs, _v) = mat_symmetric_eigen(&m).expect("converged");
        assert!((eigs[0] - 5.0).abs() < 1e-12);
        assert!((eigs[1] - 2.0).abs() < 1e-12);
        assert!((eigs[2] - 0.5).abs() < 1e-12);
    }

    #[test]
    fn test_symmetric_eigen_known_2x2_block_4x4() {
        // A = R diag(10, 1) R^T in (0,1) block at angle 30°; diag(0.5,
        // 0.1) in (2,3). Eigenvalues should be 10, 1, 0.5, 0.1, and
        // the first eigenvector should be (cos 30°, sin 30°, 0, 0).
        let theta = 30.0_f64.to_radians();
        let c = theta.cos();
        let s = theta.sin();
        let l1 = 10.0_f64;
        let l2 = 1.0_f64;
        let mut m = [[0.0_f64; 4]; 4];
        m[0][0] = c * c * l1 + s * s * l2;
        m[1][1] = s * s * l1 + c * c * l2;
        m[0][1] = c * s * (l1 - l2);
        m[1][0] = m[0][1];
        m[2][2] = 0.5;
        m[3][3] = 0.1;
        let (eigs, vecs) = mat_symmetric_eigen(&m).expect("converged");
        assert!((eigs[0] - 10.0).abs() < 1e-10);
        assert!((eigs[1] - 1.0).abs() < 1e-10);
        assert!((eigs[2] - 0.5).abs() < 1e-10);
        assert!((eigs[3] - 0.1).abs() < 1e-10);
        let v1 = [vecs[0][0], vecs[1][0], vecs[2][0], vecs[3][0]];
        let sgn = v1[0].signum();
        assert!((sgn * v1[0] - c).abs() < 1e-10);
        assert!((sgn * v1[1] - s).abs() < 1e-10);
    }

    #[test]
    fn test_symmetric_eigen_small_scale_covariance() {
        // Apophis-flavored Keplerian covariance (entries 10⁻¹⁹ to
        // 10⁻¹⁰). Absolute-tolerance Jacobi would falsely converge
        // immediately on this matrix. Expected eigenvalues from numpy:
        // (6.84e-10, 1.12e-10, 1.59e-12, 3.80e-13, 2.62e-16, 2.01e-20).
        let m: [[f64; 6]; 6] = [
            [
                2.748474e-19,
                -1.658782e-17,
                -3.840071e-16,
                -1.187933e-16,
                -4.219032e-17,
                -9.201042e-16,
            ],
            [
                -1.658782e-17,
                1.209197e-15,
                3.018236e-14,
                -4.234606e-14,
                8.170545e-15,
                1.229598e-13,
            ],
            [
                -3.840071e-16,
                3.018236e-14,
                1.659841e-12,
                -1.381882e-11,
                1.108650e-11,
                5.972759e-12,
            ],
            [
                -1.187933e-16,
                -4.234606e-14,
                -1.381882e-11,
                3.687986e-10,
                -3.301292e-10,
                -6.133320e-11,
            ],
            [
                -4.219032e-17,
                8.170545e-15,
                1.108650e-11,
                -3.301292e-10,
                3.300814e-10,
                1.691263e-12,
            ],
            [
                -9.201042e-16,
                1.229598e-13,
                5.972759e-12,
                -6.133320e-11,
                1.691263e-12,
                9.808602e-11,
            ],
        ];
        let (eigs, _v) = mat_symmetric_eigen(&m).expect("converged");
        let expected = [
            6.8421e-10_f64,
            1.1244e-10,
            1.5942e-12,
            3.7988e-13,
            2.6205e-16,
            2.0118e-20,
        ];
        for k in 0..4 {
            let rel = (eigs[k] - expected[k]).abs() / expected[k];
            assert!(
                rel < 1e-4,
                "λ_{} = {:.4e}, expected {:.4e} (rel {:.2e})",
                k + 1,
                eigs[k],
                expected[k],
                rel,
            );
        }
    }

    #[test]
    fn test_symmetric_eigen_reconstructs_matrix() {
        // For any returned (eigs, V): V · diag(eigs) · V^T should
        // reconstruct A. Use a synthetic random-ish symmetric matrix.
        let m = [[4.0_f64, 1.0, 2.0], [1.0, 3.0, 0.5], [2.0, 0.5, 2.0]];
        let (eigs, vecs) = mat_symmetric_eigen(&m).expect("converged");
        // V · diag(eigs) · V^T
        let mut reconstructed = [[0.0_f64; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                let mut s = 0.0;
                for k in 0..3 {
                    s += vecs[i][k] * eigs[k] * vecs[j][k];
                }
                reconstructed[i][j] = s;
            }
        }
        for i in 0..3 {
            for j in 0..3 {
                assert!(
                    (reconstructed[i][j] - m[i][j]).abs() < 1e-12,
                    "reconstruction mismatch at ({i},{j}): {} vs {}",
                    reconstructed[i][j],
                    m[i][j],
                );
            }
        }
    }

    #[test]
    fn test_symmetric_eigen_preserves_trace() {
        let m = [
            [1.0_f64, 0.3, -0.1, 0.05],
            [0.3, 2.0, 0.15, 0.02],
            [-0.1, 0.15, 3.0, -0.2],
            [0.05, 0.02, -0.2, 4.0],
        ];
        let trace_in: f64 = (0..4).map(|i| m[i][i]).sum();
        let (eigs, _v) = mat_symmetric_eigen(&m).expect("converged");
        let trace_out: f64 = eigs.iter().sum();
        assert!((trace_in - trace_out).abs() < 1e-12);
    }

    // ── existing mat_solve tests ─────────────────────────────────────

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

    // ── Phase 1D regression: mat_cholesky on 2×2 ill-conditioned input ──

    #[test]
    fn test_mat_cholesky_2x2_diagonal_tiny() {
        // Pre-cancellation diag input: [[ε, 0], [0, 1]]. L[1][0] = 0 so
        // sqrt(1 - L[1][0]²) does not cancel; this exercises the well-behaved
        // path for tiny diagonal entries.
        let eps = 1e-300_f64;
        let a = [[eps, 0.0], [0.0, 1.0]];
        let l = mat_cholesky::<2>(&a).expect("Cholesky should succeed");
        assert!((l[0][0] - eps.sqrt()).abs() / eps.sqrt() < 1e-10);
        assert_eq!(l[0][1], 0.0);
        assert!((l[1][1] - 1.0).abs() < 1e-15);
    }

    #[test]
    fn test_mat_cholesky_2x2_diagonal_epsilon() {
        let eps = f64::EPSILON;
        let a = [[eps, 0.0], [0.0, 1.0]];
        let l = mat_cholesky::<2>(&a).expect("Cholesky should succeed");
        let recon = [
            [l[0][0] * l[0][0], l[0][0] * l[1][0]],
            [l[0][0] * l[1][0], l[1][0] * l[1][0] + l[1][1] * l[1][1]],
        ];
        assert!((recon[0][0] - a[0][0]).abs() / a[0][0] < 1e-10);
        assert!((recon[1][1] - a[1][1]).abs() < 1e-15);
    }

    // ── Phase 1D: mat_det ───────────────────────────────────────

    #[test]
    fn test_mat_det_identity() {
        let i: [[f64; 4]; 4] =
            std::array::from_fn(|i| std::array::from_fn(|j| if i == j { 1.0 } else { 0.0 }));
        assert!((mat_det(&i) - 1.0).abs() < 1e-14);
    }

    #[test]
    fn test_mat_det_diagonal() {
        let a = [[2.0, 0.0, 0.0], [0.0, 3.0, 0.0], [0.0, 0.0, 5.0]];
        assert!((mat_det(&a) - 30.0).abs() < 1e-12);
    }

    #[test]
    fn test_mat_det_3x3_known() {
        let a = [[1.0, 2.0, 3.0], [0.0, 4.0, 5.0], [1.0, 0.0, 6.0]];
        // 1·(4·6 - 5·0) - 2·(0·6 - 5·1) + 3·(0·0 - 4·1) = 24 + 10 - 12 = 22
        assert!((mat_det(&a) - 22.0).abs() < 1e-12);
    }

    #[test]
    fn test_mat_det_singular() {
        let a = [[1.0, 2.0, 3.0], [2.0, 4.0, 6.0], [0.0, 1.0, 1.0]];
        assert!(mat_det(&a).abs() < 1e-12);
    }

    #[test]
    fn test_mat_det_sign_flip_on_row_swap() {
        let a = [[1.0, 2.0, 0.0], [0.0, 1.0, 3.0], [4.0, 0.0, 1.0]];
        let mut b = a;
        b.swap(0, 2);
        assert!((mat_det(&a) + mat_det(&b)).abs() / mat_det(&a).abs() < 1e-13);
    }

    #[test]
    fn test_mat_det_matches_log_det() {
        // Compare against existing mat_log_det on a strictly positive-det case.
        let a = [
            [3.0, 1.0, 0.0, 0.0],
            [1.0, 3.0, 1.0, 0.0],
            [0.0, 1.0, 3.0, 1.0],
            [0.0, 0.0, 1.0, 3.0],
        ];
        let d = mat_det(&a);
        assert!(d > 0.0);
        assert!((d.ln() - mat_log_det(&a)).abs() < 1e-12);
    }

    // ── Phase 1D: mat_trace_cube ────────────────────────────────

    #[test]
    fn test_mat_trace_cube_identity() {
        let i: [[f64; 3]; 3] =
            std::array::from_fn(|i| std::array::from_fn(|j| if i == j { 1.0 } else { 0.0 }));
        // Tr((I·I)³) = Tr(I) = N
        assert!((mat_trace_cube(&i, &i) - 3.0).abs() < 1e-12);
    }

    #[test]
    fn test_mat_trace_cube_scalar_matrix() {
        // A = αI, B = βI → M = AB = αβI → M³ = (αβ)³ I → Tr = N(αβ)³
        let a = [[2.0, 0.0], [0.0, 2.0]];
        let b = [[3.0, 0.0], [0.0, 3.0]];
        let n = 2.0;
        let expected = n * (2.0 * 3.0_f64).powi(3);
        assert!((mat_trace_cube(&a, &b) - expected).abs() < 1e-12);
    }

    // ── Phase 1D: mat_frobenius ─────────────────────────────────

    #[test]
    fn test_mat_frobenius_zero() {
        let a = [[0.0_f64; 4]; 3];
        assert_eq!(mat_frobenius(&a), 0.0);
    }

    #[test]
    fn test_mat_frobenius_identity() {
        let i: [[f64; 5]; 5] =
            std::array::from_fn(|i| std::array::from_fn(|j| if i == j { 1.0 } else { 0.0 }));
        // ||I_5||_F = sqrt(5)
        assert!((mat_frobenius(&i) - 5.0_f64.sqrt()).abs() < 1e-13);
    }

    #[test]
    fn test_mat_frobenius_known_2x3() {
        let a = [[1.0_f64, 2.0, 3.0], [4.0, 5.0, 6.0]];
        // sqrt(1+4+9+16+25+36) = sqrt(91)
        assert!((mat_frobenius(&a) - 91.0_f64.sqrt()).abs() < 1e-12);
    }

    // ── Phase 1D: mat_largest_singular_value ─────────────────────

    #[test]
    fn test_mat_largest_singular_value_diagonal() {
        // Singular values of a diagonal matrix are |diag|. Max here is 7.
        let a = [[3.0_f64, 0.0], [0.0, 7.0]];
        let s = mat_largest_singular_value(&a, 200, 1e-12);
        assert!((s - 7.0).abs() < 1e-7, "got {s}");
    }

    #[test]
    fn test_mat_largest_singular_value_orthogonal_is_one() {
        // Rotation matrix in 2D has σ_max = 1.
        let theta = 0.7_f64;
        let a = [[theta.cos(), -theta.sin()], [theta.sin(), theta.cos()]];
        let s = mat_largest_singular_value(&a, 200, 1e-12);
        assert!((s - 1.0).abs() < 1e-7, "got {s}");
    }

    // ── Phase 1E: mat_mul ───────────────────────────────────────

    #[test]
    fn test_mat_mul_identity() {
        let i: [[f64; 3]; 3] =
            std::array::from_fn(|i| std::array::from_fn(|j| if i == j { 1.0 } else { 0.0 }));
        let a = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];
        let c = mat_mul::<3, 3, 3>(&a, &i);
        for row in 0..3 {
            for col in 0..3 {
                assert_eq!(c[row][col], a[row][col]);
            }
        }
    }

    #[test]
    fn test_mat_mul_rectangular_2x3_times_3x4() {
        let a: [[f64; 3]; 2] = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]];
        let b: [[f64; 4]; 3] = [
            [1.0, 0.0, 0.0, 1.0],
            [0.0, 1.0, 0.0, 1.0],
            [0.0, 0.0, 1.0, 1.0],
        ];
        let c = mat_mul::<2, 3, 4>(&a, &b);
        let expected: [[f64; 4]; 2] = [[1.0, 2.0, 3.0, 6.0], [4.0, 5.0, 6.0, 15.0]];
        for row in 0..2 {
            for col in 0..4 {
                assert!((c[row][col] - expected[row][col]).abs() < 1e-14);
            }
        }
    }

    #[test]
    fn test_mat_mul_matches_mat3_mul() {
        // Byte-equality with the existing 3×3 specialization.
        use crate::linalg::mat3::mat3_mul;
        let a = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];
        let b = [[9.0, 8.0, 7.0], [6.0, 5.0, 4.0], [3.0, 2.0, 1.0]];
        let c_specialized = mat3_mul(&a, &b);
        let c_generic = mat_mul::<3, 3, 3>(&a, &b);
        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(c_specialized[i][j], c_generic[i][j]);
            }
        }
    }

    // ── Phase 1E: mat_transpose ─────────────────────────────────

    #[test]
    fn test_mat_transpose_square() {
        let a = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];
        let t = mat_transpose::<3, 3>(&a);
        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(t[j][i], a[i][j]);
            }
        }
    }

    #[test]
    fn test_mat_transpose_rectangular() {
        let a: [[f64; 4]; 2] = [[1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0]];
        let t: [[f64; 2]; 4] = mat_transpose::<2, 4>(&a);
        for i in 0..2 {
            for j in 0..4 {
                assert_eq!(t[j][i], a[i][j]);
            }
        }
    }

    #[test]
    fn test_mat_transpose_involutive() {
        let a: [[f64; 5]; 3] = std::array::from_fn(|i| std::array::from_fn(|j| (3 * i + j) as f64));
        let t = mat_transpose::<3, 5>(&a);
        let tt = mat_transpose::<5, 3>(&t);
        for i in 0..3 {
            for j in 0..5 {
                assert_eq!(tt[i][j], a[i][j]);
            }
        }
    }

    // ── Phase 1E: mat_ata ───────────────────────────────────────

    #[test]
    fn test_mat_ata_identity() {
        let i: [[f64; 3]; 3] =
            std::array::from_fn(|i| std::array::from_fn(|j| if i == j { 1.0 } else { 0.0 }));
        let c = mat_ata::<3, 3>(&i);
        for row in 0..3 {
            for col in 0..3 {
                assert_eq!(c[row][col], if row == col { 1.0 } else { 0.0 });
            }
        }
    }

    #[test]
    fn test_mat_ata_matches_explicit() {
        let a: [[f64; 3]; 4] = [
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0],
            [-1.0, 0.5, 0.25],
        ];
        let at = mat_transpose::<4, 3>(&a);
        let expected = mat_mul::<3, 4, 3>(&at, &a);
        let computed = mat_ata::<4, 3>(&a);
        for i in 0..3 {
            for j in 0..3 {
                assert!(
                    (computed[i][j] - expected[i][j]).abs() < 1e-12,
                    "({i},{j}): {} vs {}",
                    computed[i][j],
                    expected[i][j]
                );
            }
        }
    }

    #[test]
    fn test_mat_ata_symmetric() {
        let a: [[f64; 4]; 3] = std::array::from_fn(|i| std::array::from_fn(|j| (i + j) as f64));
        let c = mat_ata::<3, 4>(&a);
        for i in 0..4 {
            for j in 0..4 {
                assert_eq!(c[i][j], c[j][i]);
            }
        }
    }

    // ── Phase 1.5A-Gauss: scaled partial pivoting ──────────────────

    /// Canonical Numerical Recipes §2.5 scaled-pivoting test case.
    /// Plain partial pivoting picks row 0 (|a[0][0]| = 1) because |a[0][0]|
    /// happens to equal |a[1][0]| = 1. The row-0 elimination introduces
    /// a 1e10-scaled subtraction that destroys digits in row 1. Scaled
    /// pivoting picks row 1 (scaled by 1 vs scaled by 1e10), preserving
    /// precision.
    #[test]
    fn test_mat_solve_2x2_scaled_pivot_canonical() {
        let a: [[f64; 2]; 2] = [[1.0, 1e10], [1.0, 1.0]];
        let b: [f64; 2] = [1e10, 2.0];
        let x = mat_solve::<f64, 2>(&a, &b).expect("solvable");
        // True solution: x₂ = (2·1 − 1e10·1)/(1·1 − 1e10·1) ≈ 1, x₁ = 2 − x₂ ≈ 1.
        assert!((x[0] - 1.0).abs() < 1e-9, "x[0] = {}", x[0]);
        assert!((x[1] - 1.0).abs() < 1e-9, "x[1] = {}", x[1]);
    }

    #[test]
    fn test_mat_solve_round_trip_wide_dynamic_range() {
        // For each scale α ∈ {1e-20, 1e-10, 1, 1e5, 1e10}, solve a 3×3
        // well-conditioned scaled identity and verify A·x ≈ b.
        let scales = [1e-20_f64, 1e-10, 1.0, 1e5, 1e10];
        for &alpha in &scales {
            let a = [
                [2.0 * alpha, 0.5 * alpha, 0.0],
                [0.5 * alpha, 3.0 * alpha, 0.2 * alpha],
                [0.0, 0.2 * alpha, 1.0 * alpha],
            ];
            let b = [1.0_f64 * alpha, 0.0, -1.0 * alpha];
            let x = mat_solve::<f64, 3>(&a, &b).expect("solvable");
            // A·x ≈ b
            for i in 0..3 {
                let mut s = 0.0;
                for j in 0..3 {
                    s += a[i][j] * x[j];
                }
                let rel_err = (s - b[i]).abs() / b[i].abs().max(alpha);
                assert!(
                    rel_err < 1e-12,
                    "α={alpha}: A·x = {s}, b = {} (rel = {rel_err})",
                    b[i]
                );
            }
        }
    }

    #[test]
    fn test_mat_inv_round_trip_wide_dynamic_range() {
        let scales = [1e-20_f64, 1.0, 1e10];
        for &alpha in &scales {
            let a = [[2.0 * alpha, 0.5 * alpha], [0.5 * alpha, 3.0 * alpha]];
            let inv = mat_inv::<f64, 2>(&a).expect("invertible");
            // A·A⁻¹ ≈ I
            let product = mat_mul::<2, 2, 2>(&a, &inv);
            for i in 0..2 {
                for j in 0..2 {
                    let target = if i == j { 1.0 } else { 0.0 };
                    assert!(
                        (product[i][j] - target).abs() < 1e-12,
                        "α={alpha}: (A·A⁻¹)[{i}][{j}] = {} vs {target}",
                        product[i][j]
                    );
                }
            }
        }
    }

    #[test]
    fn test_mat_solve_singular_returns_none() {
        // Rank-1 matrix [[1,1],[1,1]] is singular.
        let a: [[f64; 2]; 2] = [[1.0, 1.0], [1.0, 1.0]];
        let b: [f64; 2] = [1.0, 1.0];
        assert!(mat_solve::<f64, 2>(&a, &b).is_none());
    }

    #[test]
    fn test_mat_solve_mixed_scale_rows() {
        // Row 0 at scale 1e10, row 1 at scale 1e-15. Scaled pivoting
        // prevents a 1e25 amplification of roundoff that plain partial
        // pivoting would suffer.
        let a: [[f64; 2]; 2] = [[1e10, 1e10], [1e-15, 2e-15]];
        let b: [f64; 2] = [2e10, 1.5e-15];
        let x = mat_solve::<f64, 2>(&a, &b).expect("solvable");
        // True solution: 1e-15·x₀ + 2e-15·x₁ = 1.5e-15  =>  x₀ + 2x₁ = 1.5
        //                1e10·x₀  + 1e10·x₁  = 2e10     =>  x₀ +  x₁ = 2
        // Subtract: x₁ = -0.5, x₀ = 2.5.
        assert!((x[0] - 2.5).abs() < 1e-9, "x[0] = {}", x[0]);
        assert!((x[1] - (-0.5)).abs() < 1e-9, "x[1] = {}", x[1]);
    }

    #[test]
    fn test_mat_solve_all_zero_returns_none() {
        let a: [[f64; 3]; 3] = [[0.0; 3]; 3];
        let b: [f64; 3] = [1.0, 2.0, 3.0];
        assert!(mat_solve::<f64, 3>(&a, &b).is_none());
    }
}
