//! Multivariate Gaussian primitives: mixture splitting, sigma-point
//! ensembles, sample moments.
//!
//! All routines are generic over the state dimension `N`. They are pure
//! mathematical primitives with no domain coupling.
//!
//! # References
//!
//! - DeMars, K. J., Bishop, R. H., & Jah, M. K. (2013). *Entropy-based
//!   approach for uncertainty propagation of nonlinear dynamical
//!   systems.* J. Guid. Control Dyn. 36(4): 1047–1057. (univariate
//!   Gaussian splitting library used by [`split_gaussian`])
//! - Julier, S. J., & Uhlmann, J. K. (1997). *A new extension of the
//!   Kalman filter to nonlinear systems.* SPIE 3068: 182–193. (canonical
//!   \\(2N+1\\) sigma-point construction used by [`sigma_points`])

use crate::linalg::generic::{mat_cholesky, mat_quadratic_form, vec_norm};

// ─── Gaussian mixture splitting ─────────────────────────────────────

/// Error returned by [`split_gaussian`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GaussianSplitError {
    /// `k == 0`. A split must produce at least one component.
    InvalidK,
    /// Direction vector is the zero vector or contains NaN/Inf.
    InvalidDirection,
    /// Mean or covariance contains NaN/Inf.
    NonFiniteInput,
    /// Covariance is not positive-semidefinite along the requested
    /// direction: \\(\mathbf{e}^\top \Sigma \, \mathbf{e} < 0\\).
    NegativeVarianceAlongDirection,
}

impl std::fmt::Display for GaussianSplitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidK => write!(f, "k must be at least 1"),
            Self::InvalidDirection => write!(f, "direction must be non-zero and finite"),
            Self::NonFiniteInput => write!(f, "mean or covariance contains NaN/Inf"),
            Self::NegativeVarianceAlongDirection => {
                write!(f, "covariance is not PSD along the split direction")
            }
        }
    }
}

impl std::error::Error for GaussianSplitError {}

/// Decompose an N-dimensional Gaussian \\(N(\boldsymbol{\mu}, \Sigma)\\) into
/// `k` equal-weight sub-Gaussians along a given direction \\(\mathbf{e}\\),
/// preserving the mixture mean and total covariance.
///
/// Returns a `Vec` of `(weight, mean, covariance)` tuples with
/// `weight = 1/k`.
///
/// # Algorithm
///
/// 1. Variance along **e**: \\(\sigma_v^2 = \mathbf{e}^\top \Sigma \, \mathbf{e}\\).
/// 2. Shared sub-covariance: \\(\Sigma_k = \Sigma - \frac{K-1}{K}\,\sigma_v^2\,\mathbf{e}\mathbf{e}^\top\\).
/// 3. Component means: \\(\mu_k = \mu + c_k \, d \, \sigma_v \, \mathbf{e}\\)
///    where \\(d = \sqrt{(K-1)/S}\\), \\(S = \sum c_k^2\\), and the offsets
///    \\(c_k\\) are symmetrically spaced integers (odd \\(K\\)) or
///    half-integers (even \\(K\\)).
/// 4. Weights: \\(w_k = 1/K\\).
///
/// This is DeMars-style equal-weight splitting with **uniform spacing**,
/// not the DeMars (2013) optimized splitting library that tabulates
/// per-\\(K\\) \\((m_k, \sigma_k/\sigma)\\) pairs minimizing the
/// \\(L^2\\) error between the source Gaussian and its mixture
/// approximation. Uniform spacing preserves the first and second moments
/// exactly (by construction) but allows non-zero higher-moment error
/// that grows with \\(K\\); the optimized library reduces that
/// higher-moment error at the cost of needing tabulated coefficients.
/// For \\(K = 3\\) the two approaches are essentially identical; for
/// \\(K \geq 5\\) they diverge slightly.
///
/// # Errors
///
/// - [`GaussianSplitError::InvalidK`] if `k == 0`.
/// - [`GaussianSplitError::InvalidDirection`] if `direction` is zero or
///   contains non-finite components.
/// - [`GaussianSplitError::NonFiniteInput`] if `mean` or `cov` contains
///   non-finite components.
/// - [`GaussianSplitError::NegativeVarianceAlongDirection`] if
///   \\(\mathbf{e}^\top \Sigma \, \mathbf{e} < 0\\).
#[allow(clippy::type_complexity)]
#[allow(clippy::needless_range_loop)]
pub fn split_gaussian<const N: usize>(
    mean: &[f64; N],
    cov: &[[f64; N]; N],
    direction: &[f64; N],
    k: usize,
) -> Result<Vec<(f64, [f64; N], [[f64; N]; N])>, GaussianSplitError> {
    if k == 0 {
        return Err(GaussianSplitError::InvalidK);
    }
    if mean.iter().any(|x| !x.is_finite()) || cov.iter().flatten().any(|x| !x.is_finite()) {
        return Err(GaussianSplitError::NonFiniteInput);
    }
    if direction.iter().any(|x| !x.is_finite()) {
        return Err(GaussianSplitError::InvalidDirection);
    }

    let dir_norm = vec_norm(direction);
    if dir_norm == 0.0 || !dir_norm.is_finite() {
        return Err(GaussianSplitError::InvalidDirection);
    }

    let mut e = [0.0_f64; N];
    for i in 0..N {
        e[i] = direction[i] / dir_norm;
    }

    if k == 1 {
        return Ok(vec![(1.0, *mean, *cov)]);
    }

    let weight = 1.0 / k as f64;

    let sigma_v_sq = mat_quadratic_form(&e, cov);
    if sigma_v_sq < 0.0 {
        return Err(GaussianSplitError::NegativeVarianceAlongDirection);
    }
    let sigma_v = sigma_v_sq.sqrt();

    let scale = sigma_v_sq * (k - 1) as f64 / k as f64;
    let mut sub_cov = *cov;
    for i in 0..N {
        for j in 0..N {
            sub_cov[i][j] -= scale * e[i] * e[j];
        }
    }

    let offsets: Vec<f64> = (0..k).map(|i| i as f64 - (k - 1) as f64 / 2.0).collect();
    let s: f64 = offsets.iter().map(|c| c * c).sum();
    let d = ((k - 1) as f64 / s).sqrt();

    let mut components = Vec::with_capacity(k);
    for offset in &offsets {
        let shift = offset * d * sigma_v;
        let mut mean_k = *mean;
        for i in 0..N {
            mean_k[i] += shift * e[i];
        }
        components.push((weight, mean_k, sub_cov));
    }

    Ok(components)
}

// ─── Sigma points ──────────────────────────────────────────────────

/// Error returned by [`sigma_points`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SigmaPointsError {
    /// Covariance is not positive-definite (Cholesky failed).
    NotPositiveDefinite,
    /// Mean or covariance contains NaN/Inf.
    NonFiniteInput,
    /// The scaled unscented transform has a non-positive scaling
    /// \\(N + \lambda = \alpha^2 (N + \kappa) \le 0\\), so the spread
    /// \\(\sqrt{N+\lambda}\\) is undefined. Choose \\(\kappa > -N\\)
    /// (e.g. `kappa = 0` or `3 - N`) with `alpha != 0`.
    InvalidScaling,
}

impl std::fmt::Display for SigmaPointsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotPositiveDefinite => write!(f, "covariance is not positive-definite"),
            Self::NonFiniteInput => write!(f, "mean or covariance contains NaN/Inf"),
            Self::InvalidScaling => {
                write!(f, "scaled unscented transform has non-positive N + lambda")
            }
        }
    }
}

impl std::error::Error for SigmaPointsError {}

/// Canonical Julier–Uhlmann unscaled sigma-point set: \\(2N+1\\) points
/// distributed symmetrically about the mean along the Cholesky columns of
/// the covariance.
///
/// The points are:
/// \\[
/// \chi_0 = \boldsymbol{\mu}, \quad
/// \chi_{2i-1} = \boldsymbol{\mu} + \sqrt{N}\,\mathbf{L}_{:,i-1}, \quad
/// \chi_{2i}   = \boldsymbol{\mu} - \sqrt{N}\,\mathbf{L}_{:,i-1}
/// \quad (i = 1, \ldots, N)
/// \\]
///
/// where \\(\mathbf{L}\\) is the lower-triangular Cholesky factor of
/// \\(\Sigma\\): \\(\Sigma = \mathbf{L}\mathbf{L}^\top\\).
///
/// With the unweighted scaling \\(\sqrt{N}\\), the empirical mean and
/// covariance of the \\(2N+1\\) points (using the standard \\(1/(2N)\\)
/// denominator, i.e. [`sample_statistics`]) reproduce
/// \\((\boldsymbol{\mu}, \Sigma)\\) exactly.
///
/// # Errors
///
/// - [`SigmaPointsError::NonFiniteInput`] if `mean` or `cov` contains
///   non-finite components.
/// - [`SigmaPointsError::NotPositiveDefinite`] if `cov` is not
///   positive-definite.
#[allow(clippy::needless_range_loop)]
pub fn sigma_points<const N: usize>(
    mean: &[f64; N],
    cov: &[[f64; N]; N],
) -> Result<Vec<[f64; N]>, SigmaPointsError> {
    if mean.iter().any(|x| !x.is_finite()) || cov.iter().flatten().any(|x| !x.is_finite()) {
        return Err(SigmaPointsError::NonFiniteInput);
    }

    let l = mat_cholesky(cov).ok_or(SigmaPointsError::NotPositiveDefinite)?;
    let scale = (N as f64).sqrt();

    let mut points = Vec::with_capacity(2 * N + 1);
    points.push(*mean);
    for i in 0..N {
        let mut plus = *mean;
        let mut minus = *mean;
        // Column i of L: l[k][i] is nonzero for k >= i (L is lower triangular).
        for k in i..N {
            let delta = scale * l[k][i];
            plus[k] += delta;
            minus[k] -= delta;
        }
        points.push(plus);
        points.push(minus);
    }

    Ok(points)
}

// ─── Scaled (Merwe) sigma points ────────────────────────────────────

/// Tuning parameters for the Merwe scaled unscented transform.
///
/// - `alpha` controls the spread of the sigma points around the mean
///   (typically small, e.g. `1e-3`, to keep them close and reduce
///   higher-order sampling error).
/// - `beta` incorporates prior knowledge of the distribution; `beta = 2`
///   is optimal for a Gaussian (it only affects the covariance weight of
///   the center point).
/// - `kappa` is a secondary scaling, commonly `0` (Merwe) or `3 - N`
///   (Julier).
///
/// The composite scaling is \\(\lambda = \alpha^2 (N + \kappa) - N\\), and
/// the spread is \\(\sqrt{N + \lambda} = \alpha\sqrt{N + \kappa}\\).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SigmaPointScaling {
    /// Spread parameter \\(\alpha\\).
    pub alpha: f64,
    /// Prior-knowledge parameter \\(\beta\\) (2 is optimal for Gaussians).
    pub beta: f64,
    /// Secondary scaling \\(\kappa\\).
    pub kappa: f64,
}

impl SigmaPointScaling {
    /// Construct from explicit \\((\alpha, \beta, \kappa)\\).
    pub fn new(alpha: f64, beta: f64, kappa: f64) -> Self {
        Self { alpha, beta, kappa }
    }

    /// The common Merwe default for Gaussian state estimation:
    /// \\(\alpha = 10^{-3}, \beta = 2, \kappa = 0\\).
    pub fn merwe() -> Self {
        Self {
            alpha: 1e-3,
            beta: 2.0,
            kappa: 0.0,
        }
    }
}

impl Default for SigmaPointScaling {
    fn default() -> Self {
        Self::merwe()
    }
}

/// A Merwe scaled sigma-point set: the \\(2N+1\\) points together with
/// their separate mean and covariance weights.
///
/// `points`, `weights_mean`, and `weights_cov` are positionally aligned,
/// all of length \\(2N+1\\). Reconstruct the propagated moments with
/// [`weighted_sample_statistics`].
#[derive(Clone, Debug, PartialEq)]
pub struct ScaledSigmaPoints<const N: usize> {
    /// The \\(2N+1\\) sigma points (index 0 is the center).
    pub points: Vec<[f64; N]>,
    /// Mean-reconstruction weights \\(W_i^m\\) (sum to 1).
    pub weights_mean: Vec<f64>,
    /// Covariance-reconstruction weights \\(W_i^c\\) (the center weight
    /// may be negative — this is expected for the scaled UT).
    pub weights_cov: Vec<f64>,
}

/// Merwe scaled unscented sigma points and weights for a Gaussian
/// \\(\mathcal{N}(\boldsymbol{\mu}, \Sigma)\\).
///
/// Generates the \\(2N+1\\) points
/// \\[
/// \chi_0 = \boldsymbol{\mu}, \quad
/// \chi_i = \boldsymbol{\mu} + \bigl(\sqrt{(N+\lambda)\,\Sigma}\bigr)_{:,i},
/// \quad
/// \chi_{i+N} = \boldsymbol{\mu} - \bigl(\sqrt{(N+\lambda)\,\Sigma}\bigr)_{:,i}
/// \\]
/// with \\(\lambda = \alpha^2 (N + \kappa) - N\\) (the matrix square root
/// is the lower Cholesky factor), and the weights
/// \\[
/// W_0^m = \frac{\lambda}{N+\lambda}, \quad
/// W_0^c = \frac{\lambda}{N+\lambda} + (1 - \alpha^2 + \beta), \quad
/// W_i^m = W_i^c = \frac{1}{2(N+\lambda)}.
/// \\]
///
/// Propagating the points through a map and reconstructing with
/// [`weighted_sample_statistics`] gives the unscented mean and
/// covariance: exact for affine maps, second-order accurate (with the
/// \\(\beta\\) correction) for nonlinear maps. See Wan & Van der Merwe
/// (2000).
///
/// # Errors
///
/// - [`SigmaPointsError::NonFiniteInput`] if `mean`, `cov`, or the
///   scaling parameters contain non-finite components.
/// - [`SigmaPointsError::InvalidScaling`] if \\(N + \lambda \le 0\\).
/// - [`SigmaPointsError::NotPositiveDefinite`] if `cov` is not
///   positive-definite.
#[allow(clippy::needless_range_loop)]
pub fn sigma_points_scaled<const N: usize>(
    mean: &[f64; N],
    cov: &[[f64; N]; N],
    scaling: &SigmaPointScaling,
) -> Result<ScaledSigmaPoints<N>, SigmaPointsError> {
    if mean.iter().any(|x| !x.is_finite())
        || cov.iter().flatten().any(|x| !x.is_finite())
        || !scaling.alpha.is_finite()
        || !scaling.beta.is_finite()
        || !scaling.kappa.is_finite()
    {
        return Err(SigmaPointsError::NonFiniteInput);
    }

    let n = N as f64;
    let lambda = scaling.alpha * scaling.alpha * (n + scaling.kappa) - n;
    let n_plus_lambda = n + lambda; // = alpha^2 (N + kappa)
    if n_plus_lambda <= 0.0 {
        return Err(SigmaPointsError::InvalidScaling);
    }

    let l = mat_cholesky(cov).ok_or(SigmaPointsError::NotPositiveDefinite)?;
    let scale = n_plus_lambda.sqrt();

    let mut points = Vec::with_capacity(2 * N + 1);
    points.push(*mean);
    for i in 0..N {
        let mut plus = *mean;
        let mut minus = *mean;
        // Column i of L: l[k][i] is nonzero for k >= i (L is lower triangular).
        for k in i..N {
            let delta = scale * l[k][i];
            plus[k] += delta;
            minus[k] -= delta;
        }
        points.push(plus);
        points.push(minus);
    }

    let w0_m = lambda / n_plus_lambda;
    let w0_c = w0_m + (1.0 - scaling.alpha * scaling.alpha + scaling.beta);
    let wi = 1.0 / (2.0 * n_plus_lambda);

    let mut weights_mean = Vec::with_capacity(2 * N + 1);
    let mut weights_cov = Vec::with_capacity(2 * N + 1);
    weights_mean.push(w0_m);
    weights_cov.push(w0_c);
    for _ in 0..(2 * N) {
        weights_mean.push(wi);
        weights_cov.push(wi);
    }

    Ok(ScaledSigmaPoints {
        points,
        weights_mean,
        weights_cov,
    })
}

// ─── Sample statistics ─────────────────────────────────────────────

/// Mean and unbiased sample covariance (denominator \\(n-1\\)) of a
/// collection of N-dimensional samples.
///
/// Returns `None` if `samples` is empty. For `samples.len() == 1` the
/// covariance is the zero matrix (denominator clamped to 1 to avoid
/// division by zero — single-sample covariance is undefined).
#[allow(clippy::needless_range_loop)]
pub fn sample_statistics<const N: usize>(
    samples: &[[f64; N]],
) -> Option<([f64; N], [[f64; N]; N])> {
    let n = samples.len();
    if n == 0 {
        return None;
    }

    let mut mean = [0.0_f64; N];
    for s in samples {
        for i in 0..N {
            mean[i] += s[i];
        }
    }
    for m in &mut mean {
        *m /= n as f64;
    }

    let mut cov = [[0.0_f64; N]; N];
    for s in samples {
        for i in 0..N {
            let di = s[i] - mean[i];
            for j in 0..=i {
                cov[i][j] += di * (s[j] - mean[j]);
            }
        }
    }
    let denom = if n > 1 { (n - 1) as f64 } else { 1.0 };
    for i in 0..N {
        for j in 0..=i {
            cov[i][j] /= denom;
            cov[j][i] = cov[i][j];
        }
    }

    Some((mean, cov))
}

/// Weighted mean and covariance of a sigma-point ensemble.
///
/// Computes \\(\boldsymbol{\mu} = \sum_i W_i^m \chi_i\\) and
/// \\(\Sigma = \sum_i W_i^c (\chi_i - \boldsymbol{\mu})(\chi_i - \boldsymbol{\mu})^\top\\)
/// — the reconstruction half of the scaled unscented transform (pair with
/// [`sigma_points_scaled`]). Unlike [`sample_statistics`], the points are
/// weighted, so the covariance weights may be negative (as the scaled UT's
/// center weight typically is); the result is symmetric by construction
/// but is not guaranteed positive-definite.
///
/// Returns `None` if `points` is empty or the weight slices' lengths do
/// not match `points`.
#[allow(clippy::needless_range_loop)]
pub fn weighted_sample_statistics<const N: usize>(
    points: &[[f64; N]],
    weights_mean: &[f64],
    weights_cov: &[f64],
) -> Option<([f64; N], [[f64; N]; N])> {
    if points.is_empty() || weights_mean.len() != points.len() || weights_cov.len() != points.len()
    {
        return None;
    }

    let mut mean = [0.0_f64; N];
    for (p, &w) in points.iter().zip(weights_mean) {
        for i in 0..N {
            mean[i] += w * p[i];
        }
    }

    let mut cov = [[0.0_f64; N]; N];
    for (p, &w) in points.iter().zip(weights_cov) {
        let mut d = [0.0_f64; N];
        for i in 0..N {
            d[i] = p[i] - mean[i];
        }
        for i in 0..N {
            for j in 0..N {
                cov[i][j] += w * d[i] * d[j];
            }
        }
    }

    Some((mean, cov))
}

#[cfg(test)]
#[allow(clippy::needless_range_loop)]
#[allow(clippy::type_complexity)]
mod tests {
    use super::*;

    // ── scaled (Merwe) sigma points ───────────────────────────────

    fn spd_3x3() -> [[f64; 3]; 3] {
        // A symmetric positive-definite covariance with cross terms.
        [[4.0, 1.0, 0.5], [1.0, 3.0, -0.8], [0.5, -0.8, 2.0]]
    }

    #[test]
    fn scaled_sigma_points_count_and_center() {
        let mean = [1.0, -2.0, 3.0];
        let sp = sigma_points_scaled::<3>(&mean, &spd_3x3(), &SigmaPointScaling::merwe()).unwrap();
        assert_eq!(sp.points.len(), 2 * 3 + 1);
        assert_eq!(sp.weights_mean.len(), 7);
        assert_eq!(sp.weights_cov.len(), 7);
        // Center point is the mean.
        assert_eq!(sp.points[0], mean);
    }

    #[test]
    fn scaled_mean_weights_sum_to_one() {
        let sp =
            sigma_points_scaled::<3>(&[0.0; 3], &spd_3x3(), &SigmaPointScaling::merwe()).unwrap();
        let s: f64 = sp.weights_mean.iter().sum();
        assert!((s - 1.0).abs() < 1e-12, "mean weights sum to {s}");
    }

    #[test]
    fn scaled_round_trip_recovers_mean_and_cov() {
        // sigma_points_scaled + weighted_sample_statistics must recover
        // (μ, Σ) exactly for several valid tunings (linear/identity map).
        let mean = [1.5, -0.5, 2.0];
        let cov = spd_3x3();
        for scaling in [
            SigmaPointScaling::merwe(),
            SigmaPointScaling::new(1.0, 2.0, 0.0),
            SigmaPointScaling::new(0.5, 2.0, 3.0 - 3.0), // kappa = 3 - N
            SigmaPointScaling::new(0.1, 0.0, 1.0),
        ] {
            let sp = sigma_points_scaled::<3>(&mean, &cov, &scaling).unwrap();
            let (m, c) =
                weighted_sample_statistics::<3>(&sp.points, &sp.weights_mean, &sp.weights_cov)
                    .unwrap();
            for i in 0..3 {
                assert!((m[i] - mean[i]).abs() < 1e-10, "mean[{i}] for {scaling:?}");
                for j in 0..3 {
                    assert!(
                        (c[i][j] - cov[i][j]).abs() < 1e-9,
                        "cov[{i}][{j}] = {} != {} for {scaling:?}",
                        c[i][j],
                        cov[i][j]
                    );
                }
            }
        }
    }

    #[test]
    fn scaled_invalid_scaling_errors() {
        // kappa = -N makes N + lambda = alpha^2 (N + kappa) = 0.
        let err = sigma_points_scaled::<3>(
            &[0.0; 3],
            &spd_3x3(),
            &SigmaPointScaling::new(1e-3, 2.0, -3.0),
        )
        .unwrap_err();
        assert_eq!(err, SigmaPointsError::InvalidScaling);
    }

    #[test]
    fn scaled_not_positive_definite_errors() {
        let not_pd = [[1.0, 2.0, 0.0], [2.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        assert_eq!(
            sigma_points_scaled::<3>(&[0.0; 3], &not_pd, &SigmaPointScaling::merwe()),
            Err(SigmaPointsError::NotPositiveDefinite)
        );
    }

    #[test]
    fn scaled_non_finite_errors() {
        assert_eq!(
            sigma_points_scaled::<3>(
                &[0.0, f64::NAN, 0.0],
                &spd_3x3(),
                &SigmaPointScaling::merwe()
            ),
            Err(SigmaPointsError::NonFiniteInput)
        );
        assert_eq!(
            sigma_points_scaled::<3>(
                &[0.0; 3],
                &spd_3x3(),
                &SigmaPointScaling::new(f64::INFINITY, 2.0, 0.0)
            ),
            Err(SigmaPointsError::NonFiniteInput)
        );
    }

    #[test]
    fn weighted_sample_statistics_length_mismatch_returns_none() {
        let pts = vec![[0.0; 3], [1.0; 3]];
        assert!(weighted_sample_statistics::<3>(&pts, &[0.5], &[0.5, 0.5]).is_none());
        assert!(weighted_sample_statistics::<3>(&[], &[], &[]).is_none());
    }

    // ── split_gaussian ────────────────────────────────────────────

    #[test]
    fn split_gaussian_invalid_k() {
        let m = [0.0; 3];
        let c = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        let d = [1.0, 0.0, 0.0];
        assert_eq!(
            split_gaussian::<3>(&m, &c, &d, 0),
            Err(GaussianSplitError::InvalidK)
        );
    }

    #[test]
    fn split_gaussian_invalid_direction_zero() {
        let m = [0.0; 3];
        let c = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        let d = [0.0, 0.0, 0.0];
        assert_eq!(
            split_gaussian::<3>(&m, &c, &d, 3),
            Err(GaussianSplitError::InvalidDirection)
        );
    }

    #[test]
    fn split_gaussian_invalid_direction_nan() {
        let m = [0.0; 3];
        let c = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        let d = [f64::NAN, 0.0, 0.0];
        assert_eq!(
            split_gaussian::<3>(&m, &c, &d, 3),
            Err(GaussianSplitError::InvalidDirection)
        );
    }

    #[test]
    fn split_gaussian_non_finite_mean() {
        let m = [f64::INFINITY, 0.0, 0.0];
        let c = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        let d = [1.0, 0.0, 0.0];
        assert_eq!(
            split_gaussian::<3>(&m, &c, &d, 3),
            Err(GaussianSplitError::NonFiniteInput)
        );
    }

    #[test]
    fn split_gaussian_negative_variance() {
        // Indefinite covariance: e^T C e = -1 along x.
        let m = [0.0; 2];
        let c = [[-1.0, 0.0], [0.0, 1.0]];
        let d = [1.0, 0.0];
        assert_eq!(
            split_gaussian::<2>(&m, &c, &d, 3),
            Err(GaussianSplitError::NegativeVarianceAlongDirection)
        );
    }

    #[test]
    fn split_gaussian_k1_passthrough() {
        let m = [1.0, 2.0, 3.0];
        let c = [[2.0, 0.1, 0.0], [0.1, 1.5, 0.2], [0.0, 0.2, 1.0]];
        let d = [1.0, 0.0, 0.0];
        let comps = split_gaussian::<3>(&m, &c, &d, 1).unwrap();
        assert_eq!(comps.len(), 1);
        assert_eq!(comps[0].0, 1.0);
        assert_eq!(comps[0].1, m);
        assert_eq!(comps[0].2, c);
    }

    #[test]
    fn split_gaussian_k3_weights_sum_to_one() {
        let m = [0.0; 4];
        let c: [[f64; 4]; 4] =
            std::array::from_fn(|i| std::array::from_fn(|j| if i == j { 1.0 } else { 0.0 }));
        let d = [1.0, 0.0, 0.0, 0.0];
        let comps = split_gaussian::<4>(&m, &c, &d, 3).unwrap();
        let sum_w: f64 = comps.iter().map(|(w, _, _)| w).sum();
        assert!((sum_w - 1.0).abs() < 1e-15);
    }

    /// Mixture mean and covariance reconstruction:
    /// \\(\bar{\boldsymbol{\mu}} = \sum_k w_k \boldsymbol{\mu}_k\\),
    /// \\(\bar{\Sigma} = \sum_k w_k (\Sigma_k + (\boldsymbol{\mu}_k - \bar{\boldsymbol{\mu}})(\boldsymbol{\mu}_k - \bar{\boldsymbol{\mu}})^\top)\\).
    fn mixture_moments<const N: usize>(
        comps: &[(f64, [f64; N], [[f64; N]; N])],
    ) -> ([f64; N], [[f64; N]; N]) {
        let mut m = [0.0_f64; N];
        for (w, mu, _) in comps {
            for i in 0..N {
                m[i] += w * mu[i];
            }
        }
        let mut s = [[0.0_f64; N]; N];
        for (w, mu, cov) in comps {
            for i in 0..N {
                for j in 0..N {
                    let d_i = mu[i] - m[i];
                    let d_j = mu[j] - m[j];
                    s[i][j] += w * (cov[i][j] + d_i * d_j);
                }
            }
        }
        (m, s)
    }

    #[test]
    fn split_gaussian_k3_round_trip_3d() {
        let m = [1.0, -2.0, 0.5];
        let c = [[4.0, 0.5, 0.0], [0.5, 2.0, 0.3], [0.0, 0.3, 1.0]];
        let d = [1.0, 1.0, 0.0]; // non-axis-aligned direction
        let comps = split_gaussian::<3>(&m, &c, &d, 3).unwrap();
        let (m_back, c_back) = mixture_moments(&comps);
        for i in 0..3 {
            assert!((m_back[i] - m[i]).abs() < 1e-12);
            for j in 0..3 {
                assert!(
                    (c_back[i][j] - c[i][j]).abs() < 1e-12,
                    "({i},{j}): {} vs {}",
                    c_back[i][j],
                    c[i][j]
                );
            }
        }
    }

    #[test]
    fn split_gaussian_k5_round_trip_6d() {
        let m = [1.0, 2.0, 3.0, 0.1, 0.2, 0.3];
        let mut c = [[0.0_f64; 6]; 6];
        for i in 0..6 {
            c[i][i] = (i + 1) as f64;
        }
        c[0][1] = 0.2;
        c[1][0] = 0.2;
        let d = [0.0, 1.0, 0.0, 0.0, 0.0, 0.0];
        let comps = split_gaussian::<6>(&m, &c, &d, 5).unwrap();
        let (m_back, c_back) = mixture_moments(&comps);
        for i in 0..6 {
            assert!((m_back[i] - m[i]).abs() < 1e-12);
            for j in 0..6 {
                assert!((c_back[i][j] - c[i][j]).abs() < 1e-12);
            }
        }
    }

    #[test]
    fn split_gaussian_k2_round_trip() {
        let m = [0.0, 0.0];
        let c = [[1.0, 0.0], [0.0, 1.0]];
        let d = [1.0, 0.0];
        let comps = split_gaussian::<2>(&m, &c, &d, 2).unwrap();
        let (m_back, c_back) = mixture_moments(&comps);
        for i in 0..2 {
            assert!((m_back[i] - m[i]).abs() < 1e-12);
            for j in 0..2 {
                assert!((c_back[i][j] - c[i][j]).abs() < 1e-12);
            }
        }
    }

    #[test]
    fn split_gaussian_unit_direction_invariant() {
        // Result should be invariant to the magnitude of the direction vector.
        let m = [1.0, 2.0];
        let c = [[1.5, 0.2], [0.2, 0.8]];
        let d1 = [1.0, 0.0];
        let d2 = [42.0, 0.0]; // same direction, different magnitude
        let comps1 = split_gaussian::<2>(&m, &c, &d1, 3).unwrap();
        let comps2 = split_gaussian::<2>(&m, &c, &d2, 3).unwrap();
        for (a, b) in comps1.iter().zip(comps2.iter()) {
            assert!((a.0 - b.0).abs() < 1e-15);
            for i in 0..2 {
                assert!((a.1[i] - b.1[i]).abs() < 1e-14);
                for j in 0..2 {
                    assert!((a.2[i][j] - b.2[i][j]).abs() < 1e-14);
                }
            }
        }
    }

    // ── sigma_points ────────────────────────────────────────────

    #[test]
    fn sigma_points_count() {
        let m = [0.0_f64; 6];
        let c: [[f64; 6]; 6] =
            std::array::from_fn(|i| std::array::from_fn(|j| if i == j { 1.0 } else { 0.0 }));
        let pts = sigma_points::<6>(&m, &c).unwrap();
        assert_eq!(pts.len(), 13); // 2N+1
    }

    #[test]
    fn sigma_points_symmetric_pairs() {
        // For each i in 1..N, points 2i-1 and 2i should be symmetric about the mean.
        let m = [1.0, 2.0, -0.5];
        let c = [[2.0, 0.1, 0.0], [0.1, 1.0, 0.2], [0.0, 0.2, 0.5]];
        let pts = sigma_points::<3>(&m, &c).unwrap();
        assert_eq!(pts.len(), 7);
        assert_eq!(pts[0], m);
        for i in 0..3 {
            let plus = &pts[1 + 2 * i];
            let minus = &pts[2 + 2 * i];
            for k in 0..3 {
                let avg = 0.5 * (plus[k] + minus[k]);
                assert!(
                    (avg - m[k]).abs() < 1e-14,
                    "pair {i} not symmetric in dim {k}"
                );
            }
        }
    }

    #[test]
    fn sigma_points_round_trip_through_sample_statistics() {
        // 2N+1 sigma points scaled by √N round-trip to (μ, Σ) under sample_statistics.
        let m = [1.0, 2.0, 3.0];
        let c = [[2.0, 0.3, 0.1], [0.3, 1.5, -0.2], [0.1, -0.2, 0.8]];
        let pts = sigma_points::<3>(&m, &c).unwrap();
        let (m_back, c_back) = sample_statistics::<3>(&pts).unwrap();
        for i in 0..3 {
            assert!(
                (m_back[i] - m[i]).abs() < 1e-13,
                "mean[{i}] = {} vs {}",
                m_back[i],
                m[i]
            );
            for j in 0..3 {
                assert!(
                    (c_back[i][j] - c[i][j]).abs() < 1e-12,
                    "cov[{i}][{j}] = {} vs {}",
                    c_back[i][j],
                    c[i][j]
                );
            }
        }
    }

    #[test]
    fn sigma_points_round_trip_6d() {
        let m = [0.5, -1.0, 2.0, 0.1, 0.2, -0.3];
        // Diagonally dominant 6×6 PD matrix with some cross-coupling.
        let mut c = [[0.0_f64; 6]; 6];
        for i in 0..6 {
            c[i][i] = (i + 1) as f64;
        }
        c[0][1] = 0.2;
        c[1][0] = 0.2;
        c[2][3] = 0.1;
        c[3][2] = 0.1;
        let pts = sigma_points::<6>(&m, &c).unwrap();
        let (m_back, c_back) = sample_statistics::<6>(&pts).unwrap();
        for i in 0..6 {
            assert!((m_back[i] - m[i]).abs() < 1e-12);
            for j in 0..6 {
                assert!((c_back[i][j] - c[i][j]).abs() < 1e-11);
            }
        }
    }

    #[test]
    fn sigma_points_not_psd_returns_error() {
        let m = [0.0_f64; 2];
        let c = [[-1.0, 0.0], [0.0, 1.0]];
        assert_eq!(
            sigma_points::<2>(&m, &c),
            Err(SigmaPointsError::NotPositiveDefinite)
        );
    }

    /// Extreme dynamic range: 6D Keplerian uncertainty with position (km)
    /// and velocity (km/s) blocks differing by ~10 orders of magnitude.
    /// Sigma points + sample_statistics must still round-trip to (μ, Σ).
    #[test]
    fn sigma_points_round_trip_extreme_dynamic_range() {
        // Position block diag(1e10, 1e10, 1e10) km²; velocity block
        // diag(1, 1, 1) (km/s)². Cross-coupling held small to keep
        // round-trip dominated by the diagonal-scale spread.
        let m = [1.0e8, -2.0e8, 0.5e8, 1.0, -2.0, 0.5];
        let mut c = [[0.0_f64; 6]; 6];
        for i in 0..3 {
            c[i][i] = 1.0e10;
        }
        for i in 3..6 {
            c[i][i] = 1.0;
        }
        c[0][3] = 1.0e3;
        c[3][0] = 1.0e3;
        let pts = sigma_points::<6>(&m, &c).unwrap();
        let (m_back, c_back) = sample_statistics::<6>(&pts).unwrap();
        for i in 0..6 {
            let denom = m[i].abs().max(1.0);
            let rel = (m_back[i] - m[i]).abs() / denom;
            assert!(rel < 1e-10, "mean[{i}] rel err = {rel}");
            for j in 0..6 {
                let scale = c[i][j].abs().max(c[i][i].sqrt() * c[j][j].sqrt() * 1e-20);
                let rel_c = (c_back[i][j] - c[i][j]).abs() / scale;
                assert!(rel_c < 1e-10, "cov[{i}][{j}] rel err = {rel_c}");
            }
        }
    }

    #[test]
    fn sigma_points_non_finite_input() {
        let m = [f64::NAN, 0.0_f64];
        let c = [[1.0, 0.0], [0.0, 1.0]];
        assert_eq!(
            sigma_points::<2>(&m, &c),
            Err(SigmaPointsError::NonFiniteInput)
        );
    }

    // ── sample_statistics ───────────────────────────────────────

    #[test]
    fn sample_statistics_empty() {
        let samples: Vec<[f64; 3]> = vec![];
        assert!(sample_statistics::<3>(&samples).is_none());
    }

    #[test]
    fn sample_statistics_single_sample() {
        let samples = vec![[1.0, 2.0, 3.0]];
        let (m, c) = sample_statistics::<3>(&samples).unwrap();
        assert_eq!(m, [1.0, 2.0, 3.0]);
        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(c[i][j], 0.0);
            }
        }
    }

    #[test]
    fn sample_statistics_two_samples() {
        // Two-point set: mean is average, covariance = (1/(n-1)) Σ outer(d, d).
        let samples = vec![[0.0, 0.0], [2.0, 4.0]];
        let (m, c) = sample_statistics::<2>(&samples).unwrap();
        assert_eq!(m, [1.0, 2.0]);
        // d_1 = (-1, -2), d_2 = (1, 2). Sum of outers = 2 * [[1, 2], [2, 4]]. Divide by n-1 = 1.
        assert!((c[0][0] - 2.0).abs() < 1e-15);
        assert!((c[0][1] - 4.0).abs() < 1e-15);
        assert!((c[1][0] - 4.0).abs() < 1e-15);
        assert!((c[1][1] - 8.0).abs() < 1e-15);
    }

    #[test]
    fn sample_statistics_symmetric() {
        // Covariance must be exactly symmetric (not just within tolerance).
        let samples = vec![
            [1.0, 2.0, 3.0],
            [2.0, 1.0, 0.0],
            [-1.0, 3.0, 1.0],
            [0.5, 1.5, 2.5],
        ];
        let (_, c) = sample_statistics::<3>(&samples).unwrap();
        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(c[i][j], c[j][i]);
            }
        }
    }
}
