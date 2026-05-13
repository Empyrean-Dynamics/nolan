//! Covariance regularization: enforce positive-definiteness, floor
//! eigenvalues, apply Tikhonov damping.
//!
//! All routines work on \\(N \times N\\) symmetric matrices and return
//! a [`RegularizationReport`] (or, for [`tikhonov`], a bare matrix
//! since the operation is always exact and never "silently
//! substitutes" anything).
//!
//! The [`RegularizationReport`] carries observability fields
//! (`n_clipped`, `max_clip_magnitude`, `clipped_indices`) so callers
//! can decide whether the regularization was a no-op or a meaningful
//! modification — supporting the "no hidden fallbacks" contract.
//!
//! # References
//!
//! - Higham, N. J. (1988). *Computing a nearest symmetric positive
//!   semidefinite matrix.* Linear Algebra Appl. 103: 103–118.

use crate::linalg::generic::mat_symmetric_eigen;

/// Outcome of an eigenvalue regularization step.
///
/// `#[must_use]` enforces the no-hidden-fallback contract: callers
/// cannot silently drop the report and lose visibility into whether
/// regularization actually modified anything.
#[must_use]
#[derive(Clone, Copy, Debug)]
pub struct RegularizationReport<const N: usize> {
    /// The regularized covariance matrix.
    pub cov: [[f64; N]; N],
    /// Number of eigenvalues that were clipped to the floor.
    pub n_clipped: usize,
    /// Largest single-eigenvalue clip magnitude: \\(\max_i (\text{floor} - \lambda_i)\\)
    /// over clipped indices, or `0.0` if no eigenvalue was clipped.
    pub max_clip_magnitude: f64,
    /// Boolean mask: `true` at position `i` if eigenvalue `i` of the
    /// input matrix was below the floor (and therefore replaced).
    pub clipped_indices: [bool; N],
}

/// Error returned by [`nearest_psd`] and [`eigenvalue_floor`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegularizationError {
    /// The Jacobi eigendecomposition failed to converge or produced
    /// non-finite eigenvalues / eigenvectors.
    EigendecompositionFailed,
    /// Input matrix contains NaN or Inf entries.
    NonFiniteInput,
}

impl std::fmt::Display for RegularizationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EigendecompositionFailed => write!(f, "eigendecomposition failed"),
            Self::NonFiniteInput => write!(f, "input matrix contains NaN/Inf"),
        }
    }
}

impl std::error::Error for RegularizationError {}

/// Project a symmetric matrix onto the cone of positive-semidefinite
/// matrices with eigenvalues \\(\geq \text{min\\_eig}\\).
///
/// Eigenvalues of the input below `min_eig` are replaced with `min_eig`;
/// eigenvalues above are left untouched. The eigenvectors are preserved
/// (only the spectrum is modified), so the result has the same principal
/// axes as the input.
///
/// For `min_eig == 0.0`, this implements Higham (1988)'s "nearest PSD
/// matrix" projection.
///
/// # Errors
///
/// - [`RegularizationError::NonFiniteInput`] if `cov` contains NaN/Inf.
/// - [`RegularizationError::EigendecompositionFailed`] if the Jacobi
///   eigensolver fails to converge.
#[allow(clippy::needless_range_loop)]
pub fn nearest_psd<const N: usize>(
    cov: &[[f64; N]; N],
    min_eig: f64,
) -> Result<RegularizationReport<N>, RegularizationError> {
    if cov.iter().flatten().any(|x| !x.is_finite()) {
        return Err(RegularizationError::NonFiniteInput);
    }

    let (eigs, v) =
        mat_symmetric_eigen(cov).ok_or(RegularizationError::EigendecompositionFailed)?;

    let mut clipped_indices = [false; N];
    let mut n_clipped = 0;
    let mut max_clip_magnitude = 0.0_f64;
    let mut new_eigs = [0.0_f64; N];
    for i in 0..N {
        if eigs[i] < min_eig {
            new_eigs[i] = min_eig;
            clipped_indices[i] = true;
            n_clipped += 1;
            let clip_mag = min_eig - eigs[i];
            if clip_mag > max_clip_magnitude {
                max_clip_magnitude = clip_mag;
            }
        } else {
            new_eigs[i] = eigs[i];
        }
    }

    // Reconstruct: A_new = V · diag(new_eigs) · V^T.
    // mat_symmetric_eigen returns eigenvectors as COLUMNS of v: v[i][k] is
    // element i of the eigenvector corresponding to eigs[k].
    let mut new_cov = [[0.0_f64; N]; N];
    for i in 0..N {
        for j in 0..N {
            let mut s = 0.0;
            for k in 0..N {
                s += v[i][k] * new_eigs[k] * v[j][k];
            }
            new_cov[i][j] = s;
        }
    }

    Ok(RegularizationReport {
        cov: new_cov,
        n_clipped,
        max_clip_magnitude,
        clipped_indices,
    })
}

/// Floor every eigenvalue of `cov` at `min_eig`. Semantically identical
/// to [`nearest_psd`] — the two names exist as documentation hints for
/// the caller's intent: `eigenvalue_floor` emphasises "I want a minimum
/// scale on the spectrum"; `nearest_psd` emphasises "I want positive-
/// definiteness".
pub fn eigenvalue_floor<const N: usize>(
    cov: &[[f64; N]; N],
    min_eig: f64,
) -> Result<RegularizationReport<N>, RegularizationError> {
    nearest_psd(cov, min_eig)
}

/// Tikhonov damping: \\(A_{\text{reg}} = A + \alpha \, I\\).
///
/// Always succeeds (no failure mode — pure arithmetic). Returns the
/// damped matrix directly; no report is produced because every diagonal
/// entry is modified by exactly the same `alpha` (no caller-relevant
/// decision about which entries got touched).
///
/// Callers who need to know whether the damping actually *rescued the
/// conditioning* should use [`tikhonov_with_report`] instead.
#[inline]
#[allow(clippy::needless_range_loop)]
pub fn tikhonov<const N: usize>(cov: &[[f64; N]; N], alpha: f64) -> [[f64; N]; N] {
    let mut out = *cov;
    for i in 0..N {
        out[i][i] += alpha;
    }
    out
}

/// Outcome of a [`tikhonov_with_report`] call.
///
/// `#[must_use]` enforces the no-hidden-fallback contract — a caller
/// who damps a marginal covariance should not silently drop the
/// before/after conditioning numbers and assume the damping was
/// "enough".
#[must_use]
#[derive(Clone, Copy, Debug)]
pub struct TikhonovReport<const N: usize> {
    /// The damped matrix \\(A + \alpha I\\).
    pub cov: [[f64; N]; N],
    /// The damping coefficient that was applied.
    pub alpha: f64,
    /// 2-norm condition number of the input matrix
    /// (`f64::INFINITY` if the input was scaled-relative singular).
    pub condition_number_before: f64,
    /// 2-norm condition number of the damped matrix. Compare against
    /// [`Self::condition_number_before`] to decide whether `alpha` was
    /// large enough to rescue the conditioning.
    pub condition_number_after: f64,
}

/// Tikhonov damping with a before/after condition-number report.
///
/// Computes `condition_number(cov)` and `condition_number(cov + αI)`
/// alongside the damped matrix, so the caller can decide whether
/// `alpha` was large enough to rescue an ill-conditioned input. The
/// returned report carries `#[must_use]` to surface the conditioning
/// numbers at the call site rather than silently absorbing them.
///
/// Two power iterations are run (one before and one after damping),
/// each up to [`crate::linalg::condition_number`]'s budget — so this
/// is materially more expensive than the bare [`tikhonov`]. Use the
/// bare form when you only need the damped matrix.
#[allow(clippy::needless_range_loop)]
pub fn tikhonov_with_report<const N: usize>(cov: &[[f64; N]; N], alpha: f64) -> TikhonovReport<N> {
    let condition_number_before = crate::linalg::generic::condition_number(cov);
    let damped = tikhonov(cov, alpha);
    let condition_number_after = crate::linalg::generic::condition_number(&damped);
    TikhonovReport {
        cov: damped,
        alpha,
        condition_number_before,
        condition_number_after,
    }
}

#[cfg(test)]
#[allow(clippy::needless_range_loop)]
mod tests {
    use super::*;

    /// Asserts a matrix is approximately PSD by computing its eigenvalues
    /// via `mat_symmetric_eigen` and checking they are all `>= -tol`.
    fn assert_approximately_psd<const N: usize>(a: &[[f64; N]; N], tol: f64) {
        let (eigs, _) = mat_symmetric_eigen(a).expect("eigendecomp");
        for &lam in &eigs {
            assert!(lam >= -tol, "eigenvalue {lam} is below -{tol}");
        }
    }

    // ── nearest_psd ─────────────────────────────────────────────

    #[test]
    fn nearest_psd_indefinite_input_yields_psd() {
        // Eigenvalues 2 and -1 (indefinite); should be clipped to 2 and 0.
        let a = [[1.5, 1.5], [1.5, 0.5]];
        let report = nearest_psd::<2>(&a, 0.0).unwrap();
        assert!(report.n_clipped >= 1);
        assert!(report.max_clip_magnitude > 0.0);
        assert_approximately_psd(&report.cov, 1e-12);
    }

    #[test]
    fn nearest_psd_clipped_indices_match() {
        // Diagonal matrix [3, -2, 1]: eigenvalues sorted desc -> [3, 1, -2].
        // Only the last (smallest in the returned ordering) is < 0.
        let a = [[3.0, 0.0, 0.0], [0.0, -2.0, 0.0], [0.0, 0.0, 1.0]];
        let report = nearest_psd::<3>(&a, 0.0).unwrap();
        assert_eq!(report.n_clipped, 1);
        let mut found_clip = false;
        for i in 0..3 {
            if report.clipped_indices[i] {
                found_clip = true;
            }
        }
        assert!(found_clip);
        assert_approximately_psd(&report.cov, 1e-12);
    }

    #[test]
    fn nearest_psd_already_psd_min_eig_zero_passthrough() {
        // PSD-but-singular (λ_min = 0) at min_eig = 0 → identity passthrough.
        let a = [[2.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 0.0]];
        let report = nearest_psd::<3>(&a, 0.0).unwrap();
        assert_eq!(report.n_clipped, 0);
        assert_eq!(report.max_clip_magnitude, 0.0);
        for i in 0..3 {
            assert!(!report.clipped_indices[i]);
            for j in 0..3 {
                assert!(
                    (report.cov[i][j] - a[i][j]).abs() < 1e-12,
                    "({i},{j}): {} vs {}",
                    report.cov[i][j],
                    a[i][j]
                );
            }
        }
    }

    #[test]
    fn nearest_psd_psd_near_singular_eigenvectors_preserved() {
        // PSD-near-singular (λ_min ≈ 1e-20) at min_eig = 1e-15 → clipped.
        // Eigenvectors of the result should match the input (only spectrum changes).
        let a = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1e-20]];
        let report = nearest_psd::<3>(&a, 1e-15).unwrap();
        assert!(report.n_clipped == 1);
        assert!(report.max_clip_magnitude > 0.0);

        // Original eigenvectors are the standard basis. Verify same of the result.
        let (_eigs_out, v_out) = mat_symmetric_eigen(&report.cov).unwrap();
        // Eigenvector columns should still be axis-aligned (up to sign / row order).
        for k in 0..3 {
            let mut max = 0.0_f64;
            let mut max_count = 0;
            for i in 0..3 {
                let abs_v = v_out[i][k].abs();
                if abs_v > 0.99 {
                    max_count += 1;
                    if abs_v > max {
                        max = abs_v;
                    }
                }
            }
            assert_eq!(max_count, 1, "eigenvector {k} not axis-aligned");
            assert!(max > 0.99);
        }
    }

    #[test]
    fn nearest_psd_non_finite_input() {
        let a = [[1.0, f64::NAN], [f64::NAN, 1.0]];
        assert_eq!(
            nearest_psd::<2>(&a, 0.0).err(),
            Some(RegularizationError::NonFiniteInput)
        );
    }

    #[test]
    fn nearest_psd_floor_positive_clips_all_smaller() {
        let a = [[0.5, 0.0], [0.0, 0.1]];
        let report = nearest_psd::<2>(&a, 1.0).unwrap();
        assert_eq!(report.n_clipped, 2);
        // Both eigenvalues should now be ≥ 1.0.
        let (eigs, _) = mat_symmetric_eigen(&report.cov).unwrap();
        for &lam in &eigs {
            assert!(lam >= 1.0 - 1e-12, "lambda = {lam}");
        }
    }

    #[test]
    fn nearest_psd_preserves_symmetry() {
        let a = [[4.0, 1.0, 0.5], [1.0, 3.0, 0.2], [0.5, 0.2, -0.5]];
        let report = nearest_psd::<3>(&a, 0.0).unwrap();
        for i in 0..3 {
            for j in 0..3 {
                assert!(
                    (report.cov[i][j] - report.cov[j][i]).abs() < 1e-12,
                    "({i},{j}) vs ({j},{i}): {} vs {}",
                    report.cov[i][j],
                    report.cov[j][i]
                );
            }
        }
    }

    // ── eigenvalue_floor ─────────────────────────────────────────

    #[test]
    fn eigenvalue_floor_alias_matches_nearest_psd() {
        let a = [[2.0, 0.5], [0.5, 0.1]];
        let r1 = nearest_psd::<2>(&a, 0.05).unwrap();
        let r2 = eigenvalue_floor::<2>(&a, 0.05).unwrap();
        for i in 0..2 {
            for j in 0..2 {
                assert_eq!(r1.cov[i][j], r2.cov[i][j]);
            }
        }
        assert_eq!(r1.n_clipped, r2.n_clipped);
    }

    // ── tikhonov ────────────────────────────────────────────────

    #[test]
    fn tikhonov_adds_alpha_to_diagonal() {
        let a = [[1.0, 0.5], [0.5, 2.0]];
        let out = tikhonov::<2>(&a, 0.1);
        assert!((out[0][0] - 1.1).abs() < 1e-15);
        assert!((out[1][1] - 2.1).abs() < 1e-15);
        // Off-diagonals untouched.
        assert_eq!(out[0][1], a[0][1]);
        assert_eq!(out[1][0], a[1][0]);
    }

    #[test]
    fn tikhonov_alpha_zero_is_identity() {
        let a = [[1.0, 0.5, 0.0], [0.5, 2.0, 0.3], [0.0, 0.3, 1.5]];
        let out = tikhonov::<3>(&a, 0.0);
        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(out[i][j], a[i][j]);
            }
        }
    }

    #[test]
    fn tikhonov_makes_indefinite_matrix_pd_when_alpha_large_enough() {
        // Eigenvalues of [[1, 2], [2, 1]] are 3, -1. Adding α=2 shifts to 5, 1.
        let a = [[1.0, 2.0], [2.0, 1.0]];
        let out = tikhonov::<2>(&a, 2.0);
        let (eigs, _) = mat_symmetric_eigen(&out).unwrap();
        for &lam in &eigs {
            assert!(lam > 0.0, "eigenvalue {lam} not positive");
        }
    }

    // ── tikhonov_with_report ────────────────────────────────────

    #[test]
    fn tikhonov_with_report_well_conditioned_input_alpha_zero_passthrough() {
        // For a well-conditioned matrix and α=0, before == after.
        let a = [[2.0, 0.5], [0.5, 3.0]];
        let report = tikhonov_with_report::<2>(&a, 0.0);
        assert_eq!(report.alpha, 0.0);
        assert!(report.condition_number_before.is_finite());
        assert!(report.condition_number_after.is_finite());
        assert!(
            (report.condition_number_after - report.condition_number_before).abs() < 1e-9,
            "κ_before={}, κ_after={}",
            report.condition_number_before,
            report.condition_number_after
        );
    }

    #[test]
    fn tikhonov_with_report_singular_input_becomes_finite_after_damping() {
        // Rank-1 [[1,1],[1,1]] → κ_before = ∞. After α=1, eigenvalues are
        // (2+1, 0+1) = (3, 1) → κ_after = 3.
        let a = [[1.0, 1.0], [1.0, 1.0]];
        let report = tikhonov_with_report::<2>(&a, 1.0);
        assert!(report.condition_number_before.is_infinite());
        assert!(report.condition_number_after.is_finite());
        assert!(
            (report.condition_number_after - 3.0).abs() < 1e-6,
            "κ_after = {}",
            report.condition_number_after
        );
    }

    #[test]
    fn tikhonov_with_report_alpha_too_small_doesnt_help() {
        // α = 1e-20 on a rank-1 matrix leaves κ near ∞ — the report
        // surfaces this so a caller can detect under-damping.
        let a = [[1.0_f64, 1.0], [1.0, 1.0]];
        let report = tikhonov_with_report::<2>(&a, 1e-20);
        // Even tiny α makes the matrix nominally invertible, but the
        // resulting κ is astronomical (~4e20) — well above any
        // reasonable conditioning gate. Caller can detect this.
        assert!(report.condition_number_after > 1e15);
    }

    // ── nearest_psd Higham property ─────────────────────────────

    /// Higham (1988) Theorem 2.1: the nearest PSD matrix to a symmetric
    /// indefinite matrix \\(A\\) in the Frobenius norm is
    /// \\(A_+ = V \, \mathrm{diag}(\max(\lambda_i, 0)) \, V^\top\\), with
    /// distance
    /// \\(\lVert A - A_+ \rVert_F^2 = \sum_{\lambda_i < 0} \lambda_i^2\\).
    /// For `min_eig > 0`, the analogous formula uses
    /// \\(\sum_{\lambda_i < \text{min\_eig}} (\text{min\_eig} - \lambda_i)^2\\).
    #[test]
    fn nearest_psd_minimizes_frobenius_distance() {
        // 3×3 indefinite matrix. Eigenvalues computed via Jacobi reference.
        let a = [[1.0, 2.0, 0.5], [2.0, -1.0, 0.3], [0.5, 0.3, 0.5]];
        let (eigs, _) = mat_symmetric_eigen(&a).unwrap();

        let report = nearest_psd::<3>(&a, 0.0).unwrap();

        // Predicted Frobenius distance squared (Higham 1988 Eq. 2.5):
        let predicted_dist_sq: f64 = eigs.iter().filter(|&&l| l < 0.0).map(|&l| l * l).sum();

        let mut actual_dist_sq = 0.0;
        for i in 0..3 {
            for j in 0..3 {
                let d = a[i][j] - report.cov[i][j];
                actual_dist_sq += d * d;
            }
        }

        assert!(
            (actual_dist_sq - predicted_dist_sq).abs() / predicted_dist_sq < 1e-10,
            "Frobenius distance²: actual {actual_dist_sq}, predicted {predicted_dist_sq}"
        );
    }

    #[test]
    fn nearest_psd_min_eig_positive_minimizes_clipped_squared_sum() {
        // For min_eig > 0, distance² = Σ_{λ < min_eig} (min_eig − λ)².
        let a = [[2.0, 0.0, 0.0], [0.0, 0.5, 0.0], [0.0, 0.0, 0.1]];
        let min_eig = 1.0_f64;
        let (eigs, _) = mat_symmetric_eigen(&a).unwrap();

        let report = nearest_psd::<3>(&a, min_eig).unwrap();
        let predicted_dist_sq: f64 = eigs
            .iter()
            .filter(|&&l| l < min_eig)
            .map(|&l| (min_eig - l).powi(2))
            .sum();
        let mut actual_dist_sq = 0.0;
        for i in 0..3 {
            for j in 0..3 {
                let d = a[i][j] - report.cov[i][j];
                actual_dist_sq += d * d;
            }
        }
        assert!(
            (actual_dist_sq - predicted_dist_sq).abs() / predicted_dist_sq < 1e-10,
            "actual {actual_dist_sq} vs predicted {predicted_dist_sq}"
        );
    }
}
