//! Multivariate Gaussian primitives: mixture splitting, sigma-point
//! ensembles, sample moments.
//!
//! All routines are generic over the state dimension `N`. They are pure
//! mathematical primitives with no domain coupling.
//!
//! # References
//!
//! - Vittaldev, V., & Russell, R. P. (2016). *Multidirectional Gaussian
//!   mixture models for nonlinear uncertainty propagation.* Computer
//!   Modeling in Engineering & Sciences 111(1): 83–117.
//!   [doi:10.3970/cmes.2016.111.083](https://doi.org/10.3970/cmes.2016.111.083)
//!   (the univariate splitting library [`split_gaussian`] draws from)
//! - Vittaldev, V., & Russell, R. P. (2016). *Space object collision
//!   probability using multidirectional Gaussian mixture models.*
//!   Journal of Guidance, Control, and Dynamics 39(9): 2163–2169.
//!   [doi:10.2514/1.G001610](https://doi.org/10.2514/1.G001610)
//! - DeMars, K. J., Bishop, R. H., & Jah, M. K. (2013). *Entropy-based
//!   approach for uncertainty propagation of nonlinear dynamical
//!   systems.* Journal of Guidance, Control, and Dynamics 36(4):
//!   1047–1057.
//!   [doi:10.2514/1.58987](https://doi.org/10.2514/1.58987)
//! - Julier, S. J., & Uhlmann, J. K. (1997). *A new extension of the
//!   Kalman filter to nonlinear systems.* SPIE 3068: 182–193. (canonical
//!   \\(2N+1\\) sigma-point construction used by [`sigma_points`])

use crate::linalg::generic::{mat_cholesky, mat_quadratic_form, mat_vec_mul, vec_norm};
use crate::statistics::split_library::{MAX_SPLIT_COMPONENTS, univariate_split};

// ─── Gaussian mixture splitting ─────────────────────────────────────

/// Error returned by [`split_gaussian`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GaussianSplitError {
    /// `k == 0`. A split must produce at least one component.
    InvalidK,
    /// `k` exceeds [`MAX_SPLIT_COMPONENTS`]: the splitting library has
    /// no entry that wide.
    KAboveLibrary { k: usize, max: usize },
    /// Direction vector is the zero vector or contains NaN/Inf.
    InvalidDirection,
    /// Mean or covariance contains NaN/Inf.
    NonFiniteInput,
    /// Covariance is not positive-semidefinite along the requested
    /// direction: \\(\mathbf{e}^\top \Sigma \, \mathbf{e} < 0\\).
    NegativeVarianceAlongDirection,
    /// Variance along the requested direction is positive but so small
    /// (subnormal) that the deflation scale \\(1/\sigma_v^2\\) overflows.
    DegenerateVarianceAlongDirection,
}

impl std::fmt::Display for GaussianSplitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidK => write!(f, "k must be at least 1"),
            Self::KAboveLibrary { k, max } => {
                write!(
                    f,
                    "k = {k} exceeds the splitting library's {max} components"
                )
            }
            Self::InvalidDirection => write!(f, "direction must be non-zero and finite"),
            Self::NonFiniteInput => write!(f, "mean or covariance contains NaN/Inf"),
            Self::NegativeVarianceAlongDirection => {
                write!(f, "covariance is not PSD along the split direction")
            }
            Self::DegenerateVarianceAlongDirection => {
                write!(
                    f,
                    "variance along the split direction is too small to deflate without overflow"
                )
            }
        }
    }
}

impl std::error::Error for GaussianSplitError {}

/// Decompose an N-dimensional Gaussian \\(N(\boldsymbol{\mu}, \Sigma)\\) into
/// `k` sub-Gaussians along a given direction \\(\mathbf{e}\\),
/// preserving the mixture mean and total covariance.
///
/// Returns a `Vec` of `(weight, mean, covariance)` tuples. The weights
/// are unequal: they fall away from the centre, by three orders of
/// magnitude across the widest split.
///
/// # Algorithm
///
/// The split is the tabulated univariate library of
/// [`split_library`](super::split_library) — weights \\(w_i\\), means
/// \\(m_i\\) and a common width \\(s\\), all in units of the parent's
/// standard deviation — carried along \\(\mathbf{e}\\):
///
/// 1. Variance along **e**: \\(\sigma_v^2 = \mathbf{e}^\top \Sigma \\, \mathbf{e}\\),
///    and the deflation vector \\(\mathbf{w} = \Sigma \mathbf{e}\\).
/// 2. Shared sub-covariance: \\(\Sigma_k = \Sigma - \alpha \\, \mathbf{w}\mathbf{w}^\top\\)
///    with \\(\alpha = (1 - s^2)\\,\sigma_v^{-2}\\).
/// 3. Component means: \\(\boldsymbol{\mu}_i = \boldsymbol{\mu} + m_i \\, \mathbf{w} / \sigma_v\\).
/// 4. Weights: \\(w_i\\).
///
/// If \\(\sigma_v^2 = 0\\) the direction carries no spread: the routine
/// returns \\(K\\) copies of \\((\boldsymbol{\mu}, \Sigma)\\) carrying
/// the library's weights.
///
/// # Why the deflation follows \\(\Sigma\mathbf{e}\\)
///
/// Moment matching is exact: the second moment of the mean offsets is
/// \\[
/// \sum_i w_i m_i^2 \frac{\mathbf{w}\mathbf{w}^\top}{\sigma_v^2}
/// = (1 - s^2)\frac{\mathbf{w}\mathbf{w}^\top}{\sigma_v^2}
/// = \alpha\\,\mathbf{w}\mathbf{w}^\top,
/// \\]
/// using the library's own identity \\(s^2 + \sum_i w_i m_i^2 = 1\\).
/// That cancels the deflation, and the antisymmetric \\(m_i\\) leave the
/// mean unmoved — so the mixture reproduces
/// \\((\boldsymbol{\mu}, \Sigma)\\) for **any** direction, not merely
/// along \\(\mathbf{e}\\).
///
/// Every \\(\Sigma_k\\) is positive-semidefinite whenever \\(\Sigma\\) is.
/// By Cauchy–Schwarz in the \\(\Sigma\\)-inner product,
/// \\(\alpha (\mathbf{x}^\top \Sigma \mathbf{e})^2 \le
/// (1 - s^2)\\,\mathbf{x}^\top \Sigma \mathbf{x}\\), hence
/// \\[
/// \mathbf{x}^\top \Sigma_k \mathbf{x} \ge
/// s^2\\,\mathbf{x}^\top \Sigma \mathbf{x} \ge 0
/// \quad\text{for all } \mathbf{x},
/// \qquad \Sigma_k \succeq s^2 \Sigma .
/// \\]
/// Since \\(\Sigma - \Sigma_k = \alpha\\,\mathbf{w}\mathbf{w}^\top \succeq 0\\)
/// as well, every component is bracketed
/// \\(s^2\Sigma \preceq \Sigma_k \preceq \Sigma\\), and the
/// split-direction marginal is exact:
/// \\(\mathbf{e}^\top \Sigma_k \mathbf{e} = s^2 \sigma_v^2\\).
///
/// The library widens that bracket's floor. The uniform split this
/// replaced had \\(s^2 = 1/K\\); the library's \\(s^2\\) is larger at
/// every \\(K\\) — 0.461 against 0.333 at \\(K = 3\\), 0.131 against
/// 0.067 at \\(K = 15\\) — so components come away further from the
/// semi-definite boundary than they did before.
///
/// Deflating along \\(\mathbf{e}\\) itself does **not** have this property:
/// a rank-1 downdate \\(c\\,\mathbf{e}\mathbf{e}^\top\\) preserves
/// positive-semidefiniteness only for
/// \\(c \le 1/(\mathbf{e}^\top \Sigma^{-1} \mathbf{e})\\), the *conditional*
/// variance along \\(\mathbf{e}\\), which the *marginal* variance
/// \\(\sigma_v^2\\) can exceed by orders of magnitude on ill-conditioned
/// covariances. The two coincide exactly when \\(\mathbf{e}\\) is an
/// eigenvector of \\(\Sigma\\) (the Cauchy–Schwarz equality condition),
/// and the Kantorovich inequality bounds their ratio by
/// \\((\kappa+1)^2/(4\kappa)\\) in the condition number \\(\kappa\\) —
/// the "orders of magnitude" above is that bound at large \\(\kappa\\).
///
/// When \\(\mathbf{e}\\) *is* an eigenvector, \\(\Sigma\mathbf{e} = \lambda
/// \mathbf{e}\\) gives \\(\alpha\\,\mathbf{w}\mathbf{w}^\top =
/// (1 - s^2)\lambda\\,\mathbf{e}\mathbf{e}^\top\\) and
/// \\(\mathbf{w}/\sigma_v = \sigma_v \mathbf{e}\\): the construction reduces
/// algebraically to the marginal-variance deflation, so eigenvector-aligned
/// splits agree to floating-point round-off (a few ulp; the multiplication
/// order differs, so not bit-for-bit). The marginal spacing of the component means along
/// \\(\mathbf{e}\\) is likewise unchanged in general —
/// \\(\mathbf{e}^\top(\boldsymbol{\mu}_i - \boldsymbol{\mu}) = m_i \sigma_v\\)
/// — only their motion off \\(\mathbf{e}\\) differs.
///
/// # What the library buys
///
/// The library entry minimises the \\(L^2\\) distance between the
/// standard normal and the mixture, among symmetric mixtures of its
/// width carrying unit variance (Vittaldev & Russell 2016, with the
/// variance constraint this crate's covariance contract adds). It
/// replaces an equal-weight split with means at uniformly spaced
/// multiples of \\(\sigma_v\\), which matched the first two moments and
/// nothing else. The difference is in the tail, and it is large: writing
/// the mixture's mass beyond \\(\Delta\sigma\\) along the split
/// direction as a fraction of the parent's,
///
/// | \\(k\\) | split | \\(3\sigma\\) | \\(4\sigma\\) | \\(5\sigma\\) |
/// |---|---|---|---|---|
/// | 3 | uniform, equal weight | 0.066 | 1.1e-3 | 2.5e-6 |
/// | 3 | library | 0.46 | 0.079 | 4.4e-3 |
/// | 9 | uniform, equal weight | 1.6e-4 | 4.5e-11 | 4.8e-21 |
/// | 9 | library | 1.01 | 0.25 | 1.6e-3 |
/// | 15 | uniform, equal weight | 6.8e-7 | 4.3e-18 | 2.6e-35 |
/// | 15 | library | 1.00 | 0.91 | 0.033 |
///
/// Under the uniform split the tail got *worse* as \\(k\\) rose, because
/// every added component narrowed the shared width while the outermost
/// mean barely moved. Under the library it improves.
///
/// The deep tail is still not reproduced at any \\(k\\): a mixture of
/// Gaussians all narrower than the parent has a tail that decays faster
/// than the parent's, whatever the weights. The library moves the
/// crossover out, it does not remove it. A probability that depends on
/// mass beyond about \\(5\sigma\\) is not a question a Gaussian mixture
/// answers.
///
/// # Errors
///
/// - [`GaussianSplitError::InvalidK`] if `k == 0`.
/// - [`GaussianSplitError::KAboveLibrary`] if `k` exceeds
///   [`MAX_SPLIT_COMPONENTS`].
/// - [`GaussianSplitError::InvalidDirection`] if `direction` is zero or
///   contains non-finite components.
/// - [`GaussianSplitError::NonFiniteInput`] if `mean` or `cov` contains
///   non-finite components.
/// - [`GaussianSplitError::NegativeVarianceAlongDirection`] if
///   \\(\mathbf{e}^\top \Sigma \\, \mathbf{e} < 0\\).
/// - [`GaussianSplitError::DegenerateVarianceAlongDirection`] if
///   \\(\mathbf{e}^\top \Sigma \\, \mathbf{e}\\) is positive but subnormal,
///   so the deflation scale would overflow.
///
/// \\(\Sigma\\) itself is assumed positive-semidefinite; only the variance
/// along \\(\mathbf{e}\\) is checked. With an indefinite \\(\Sigma\\) the
/// component-PSD guarantee above does not hold.
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

    let entry = univariate_split(k).ok_or(GaussianSplitError::KAboveLibrary {
        k,
        max: MAX_SPLIT_COMPONENTS,
    })?;

    let sigma_v_sq = mat_quadratic_form(&e, cov);
    if sigma_v_sq < 0.0 {
        return Err(GaussianSplitError::NegativeVarianceAlongDirection);
    }
    if sigma_v_sq == 0.0 {
        // No spread along e: nothing to deflate and nothing to shift, and the
        // 1/sigma_v_sq scaling below is undefined. The components still carry
        // the library's weights, so the mixture stays normalized.
        return Ok(entry.weights.iter().map(|w| (*w, *mean, *cov)).collect());
    }
    let sigma_v = sigma_v_sq.sqrt();

    // Deflate along w = Σe, not along e: a rank-1 downdate c e eᵀ stays PSD
    // only for c <= 1/(eᵀΣ⁻¹e), which the marginal variance eᵀΣe exceeds
    // unless e is an eigenvector of Σ.
    let w = mat_vec_mul(cov, &e);
    let alpha = (1.0 - entry.sigma * entry.sigma) / sigma_v_sq;
    if !alpha.is_finite() {
        // sigma_v_sq is subnormal: the deflation scale overflows, and the
        // components would carry -inf/NaN entries.
        return Err(GaussianSplitError::DegenerateVarianceAlongDirection);
    }
    let mut sub_cov = *cov;
    for i in 0..N {
        for j in 0..N {
            sub_cov[i][j] -= alpha * w[i] * w[j];
        }
    }

    let mut components = Vec::with_capacity(k);
    for (weight, m) in entry.weights.iter().zip(entry.means) {
        let shift = m / sigma_v;
        let mut mean_k = *mean;
        for i in 0..N {
            mean_k[i] += shift * w[i];
        }
        components.push((*weight, mean_k, sub_cov));
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

    Ok(symmetric_cholesky_points(mean, &l, scale))
}

/// The symmetric \\(2N+1\\) point set \\(\\{\boldsymbol{\mu},\,
/// \boldsymbol{\mu} \pm s\,\mathbf{L}_{:,i}\\}\\) about the mean along
/// the Cholesky columns — shared by the unscaled ([`sigma_points`]) and
/// Merwe scaled ([`sigma_points_scaled`]) constructions, which differ
/// only in the spread \\(s\\).
#[allow(clippy::needless_range_loop)]
fn symmetric_cholesky_points<const N: usize>(
    mean: &[f64; N],
    l: &[[f64; N]; N],
    scale: f64,
) -> Vec<[f64; N]> {
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
    points
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

    let points = symmetric_cholesky_points(mean, &l, scale);

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
    use crate::linalg::generic::mat_mul;

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

    /// The three-component split is the library's, not the uniform
    /// equal-weight geometry it replaced: weights 0.211/0.577/0.211
    /// rather than three thirds, means at \\(\pm 1.128\sigma\\) rather
    /// than \\(\pm\sigma\\), and a shared width of \\(0.679\sigma\\)
    /// rather than \\(\sigma/\sqrt{3} = 0.577\sigma\\).
    ///
    /// The direction is an eigenvector of Σ, so w = Σe is parallel to e
    /// and the numbers below are the univariate library's own, read off
    /// the split without the Σe construction rotating them.
    #[test]
    fn split_gaussian_k3_carries_the_library_geometry() {
        let m = [0.0, 0.0];
        let c = [[4.0, 0.0], [0.0, 1.0]];
        let d = [1.0, 0.0]; // sigma_v = 2
        let comps = split_gaussian::<2>(&m, &c, &d, 3).unwrap();

        assert!(
            comps[1].0 - comps[0].0 > 0.2,
            "the central weight {} does not stand above the outer {}",
            comps[1].0,
            comps[0].0
        );
        assert!((comps[0].0 - comps[2].0).abs() < 1e-15);
        assert!((comps[0].0 + comps[1].0 + comps[2].0 - 1.0).abs() < 1e-15);

        // Means at +-1.128 sigma_v, i.e. +-2.257 here, not +-2.
        assert!(comps[1].1[0].abs() < 1e-15);
        assert!(
            (comps[2].1[0] - 2.0 * 1.128_427_913_490_078).abs() < 1e-12,
            "outer mean {} is not the library's",
            comps[2].1[0]
        );
        assert!((comps[0].1[0] + comps[2].1[0]).abs() < 1e-14);

        // Shared width 0.6792801 sigma_v along the split direction; the
        // orthogonal direction is untouched.
        let width = comps[0].2[0][0].sqrt() / 2.0;
        assert!(
            (width - 0.679_280_1).abs() < 1e-6,
            "component width {width} is not the library's"
        );
        assert!((comps[0].2[1][1] - 1.0).abs() < 1e-15);
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

    // ── split_gaussian: deflation along Σe ────────────────────────

    /// Householder reflection \\(I - 2\mathbf{v}\mathbf{v}^\top / \lVert\mathbf{v}\rVert^2\\).
    fn householder_6(v: [f64; 6]) -> [[f64; 6]; 6] {
        let norm_sq: f64 = v.iter().map(|x| x * x).sum();
        let mut h = [[0.0_f64; 6]; 6];
        for i in 0..6 {
            h[i][i] = 1.0;
            for j in 0..6 {
                h[i][j] -= 2.0 * v[i] * v[j] / norm_sq;
            }
        }
        h
    }

    /// A 6×6 covariance \\(Q \, \mathrm{diag}(\lambda) \, Q^\top\\) whose
    /// eigenvalues span eight decades (\\(10^4\\) down to \\(10^{-4}\\)),
    /// with \\(Q\\) a product of two Householder reflections so that no
    /// coordinate axis sits near an eigenvector: the largest component of
    /// \\(Q^\top \mathbf{e}_0\\) is ≈ 0.62. Along \\(\mathbf{e}_0\\) the
    /// marginal variance \\(\mathbf{e}_0^\top \Sigma \mathbf{e}_0\\) exceeds
    /// the conditional variance \\(1/(\mathbf{e}_0^\top \Sigma^{-1} \mathbf{e}_0)\\)
    /// by a factor ≈ \\(2.3 \times 10^5\\) — the regime where deflating by
    /// the marginal variance overshoots.
    fn ill_conditioned_6x6() -> [[f64; 6]; 6] {
        let q = mat_mul::<6, 6, 6>(
            &householder_6([1.0, 1.0, 1.0, 1.0, 1.0, 1.0]),
            &householder_6([1.0, -2.0, 3.0, -4.0, 5.0, -6.0]),
        );
        let lambda = [1.0e4, 3.0e3, 5.0e2, 1.0e1, 5.0e-2, 1.0e-4];
        let mut cov = [[0.0_f64; 6]; 6];
        for i in 0..6 {
            for j in 0..6 {
                let mut acc = 0.0;
                for m in 0..6 {
                    acc += q[i][m] * lambda[m] * q[j][m];
                }
                cov[i][j] = acc;
            }
        }
        // Symmetrize away the last ulp of asymmetry from the triple product.
        for i in 0..6 {
            for j in 0..i {
                let v = 0.5 * (cov[i][j] + cov[j][i]);
                cov[i][j] = v;
                cov[j][i] = v;
            }
        }
        cov
    }

    #[test]
    fn split_gaussian_components_psd_along_non_eigenvector() {
        // Deflating by the marginal variance σ_v² = eᵀΣe is only PSD-safe up
        // to the conditional variance 1/(eᵀΣ⁻¹e); the two agree only when e
        // is an eigenvector. Here they differ by five orders of magnitude, so
        // every component must still admit a Cholesky factor.
        let cov = ill_conditioned_6x6();
        let mean = [1.0, -2.0, 3.0, -0.5, 0.25, 4.0];
        let dir = [1.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        // Shifted-Cholesky PSD test: M ⪰ −tol·I iff M + tol·I admits a
        // Cholesky factor. λ_max ≈ 1e4, so tol = 1e-9 is ~1e-13 relative.
        let tol = 1.0e-9;
        for k in [2, 3, 5] {
            let comps = split_gaussian::<6>(&mean, &cov, &dir, k).unwrap();
            let s2 = {
                let e = univariate_split(k).unwrap();
                e.sigma * e.sigma
            };
            for (idx, (_, _, c)) in comps.iter().enumerate() {
                assert!(
                    mat_cholesky(c).is_some(),
                    "k = {k}, component {idx}: sub-covariance is not positive-definite"
                );
                // The theorem itself: s²Σ ⪯ Σ_k ⪯ Σ.
                let mut lower = *c;
                let mut upper = cov;
                for i in 0..6 {
                    for j in 0..6 {
                        lower[i][j] -= cov[i][j] * s2;
                        upper[i][j] -= c[i][j];
                    }
                    lower[i][i] += tol;
                    upper[i][i] += tol;
                }
                assert!(
                    mat_cholesky(&lower).is_some(),
                    "k = {k}, component {idx}: Σ_k − s²Σ is not PSD"
                );
                assert!(
                    mat_cholesky(&upper).is_some(),
                    "k = {k}, component {idx}: Σ − Σ_k is not PSD"
                );
                // The split-direction marginal is exact: eᵀΣ_k e = s²σ_v².
                let sv_k = mat_quadratic_form(&dir, c);
                let sv = mat_quadratic_form(&dir, &cov);
                assert!(
                    (sv_k - sv * s2).abs() <= 1e-12 * sv,
                    "k = {k}, component {idx}: eᵀΣ_k e = {sv_k} vs {}",
                    sv * s2
                );
            }
        }
    }

    #[test]
    fn split_gaussian_moment_match_ill_conditioned_non_eigenvector() {
        // Moment matching must stay exact on the same ill-conditioned Σ: the
        // spread of the component means cancels the deflation identically.
        let cov = ill_conditioned_6x6();
        let mean: [f64; 6] = [1.0, -2.0, 3.0, -0.5, 0.25, 4.0];
        let dir = [1.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let cov_scale = cov.iter().flatten().fold(0.0_f64, |a, x| a.max(x.abs()));
        let mean_scale = mean.iter().fold(0.0_f64, |a, x| a.max(x.abs()));
        for k in [2, 3, 5] {
            let comps = split_gaussian::<6>(&mean, &cov, &dir, k).unwrap();
            let (m_back, c_back) = mixture_moments(&comps);
            for i in 0..6 {
                let rel = (m_back[i] - mean[i]).abs() / mean_scale;
                assert!(rel < 1e-13, "k = {k}, mean[{i}] rel err = {rel}");
                for j in 0..6 {
                    let rel = (c_back[i][j] - cov[i][j]).abs() / cov_scale;
                    assert!(rel < 1e-13, "k = {k}, cov[{i}][{j}] rel err = {rel}");
                }
            }
        }
    }

    #[test]
    fn split_gaussian_eigenvector_direction_reduces_to_marginal_deflation() {
        // Σ is diagonal, so e₁ is an exact eigenvector with λ = 9: w = Σe = λe
        // and α wwᵀ = (1 − s²) λ eeᵀ, i.e. the marginal-variance deflation.
        // Expected values below are that analytic result; the routine reaches
        // them through a different multiplication order, so the tolerance is a
        // few ulp rather than bit equality (the values are O(10)).
        let mean = [1.0, -2.0, 0.5];
        let cov = [[4.0, 0.0, 0.0], [0.0, 9.0, 0.0], [0.0, 0.0, 16.0]];
        let dir = [0.0, 1.0, 0.0];
        let comps = split_gaussian::<3>(&mean, &cov, &dir, 3).unwrap();
        let entry = univariate_split(3).unwrap();
        // σ_v² = 9, so the (1,1) entry keeps 9s².
        let expected_cov = [
            [4.0, 0.0, 0.0],
            [0.0, 9.0 * entry.sigma * entry.sigma, 0.0],
            [0.0, 0.0, 16.0],
        ];
        // σ_v = 3, so the means step by 3·m_i along e₁.
        let expected_means: Vec<[f64; 3]> = entry
            .means
            .iter()
            .map(|m| [1.0, -2.0 + 3.0 * m, 0.5])
            .collect();
        assert_eq!(comps.len(), 3);
        for (idx, (w, m, c)) in comps.iter().enumerate() {
            assert!((w - entry.weights[idx]).abs() < 1e-15);
            for i in 0..3 {
                assert!(
                    (m[i] - expected_means[idx][i]).abs() < 1e-13,
                    "component {idx} mean[{i}] = {} vs {}",
                    m[i],
                    expected_means[idx][i]
                );
                for j in 0..3 {
                    assert!(
                        (c[i][j] - expected_cov[i][j]).abs() < 1e-13,
                        "component {idx} cov[{i}][{j}] = {} vs {}",
                        c[i][j],
                        expected_cov[i][j]
                    );
                }
            }
        }
    }

    #[test]
    fn split_gaussian_zero_variance_direction_returns_identical_components() {
        // Σ is singular along e₀ (σ_v² = 0): nothing to deflate, nothing to
        // shift, so the split degenerates to K copies of the input.
        let mean = [1.0, 2.0, 3.0];
        let cov = [[0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 2.0]];
        let dir = [1.0, 0.0, 0.0];
        for k in [2, 3, 5] {
            let comps = split_gaussian::<3>(&mean, &cov, &dir, k).unwrap();
            assert_eq!(comps.len(), k);
            let entry = univariate_split(k).unwrap();
            for (idx, (w, m, c)) in comps.iter().enumerate() {
                assert!((w - entry.weights[idx]).abs() < 1e-15);
                assert_eq!(*m, mean);
                assert_eq!(*c, cov);
            }
        }
    }

    #[test]
    fn split_gaussian_zero_variance_with_nonzero_sigma_e() {
        // eᵀΣe = 0 while Σe ≠ 0 — reachable only for a non-PSD Σ, and the one
        // case where the 1/σ_v² scaling would blow up. The result must stay
        // finite and match the σ_v² = 0 behavior above.
        let mean = [0.0, 0.0];
        let cov = [[0.0, 1.0], [1.0, 1.0]];
        let dir = [1.0, 0.0];
        let comps = split_gaussian::<2>(&mean, &cov, &dir, 3).unwrap();
        assert_eq!(comps.len(), 3);
        for (_, m, c) in &comps {
            assert_eq!(*m, mean);
            assert_eq!(*c, cov);
        }
    }

    #[test]
    fn split_gaussian_subnormal_variance_refuses() {
        // 0 < σ_v² < ~3.7e-309: the deflation scale 1/σ_v² overflows f64,
        // and without the guard the components carry -inf/NaN entries
        // returned as Ok — exactly the silent degradation the typed error
        // exists to prevent.
        let mean = [0.0, 0.0, 0.0];
        let cov = [[1.0e-310, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        let dir = [1.0, 0.0, 0.0];
        assert!(matches!(
            split_gaussian::<3>(&mean, &cov, &dir, 3),
            Err(GaussianSplitError::DegenerateVarianceAlongDirection)
        ));
    }

    #[test]
    fn split_gaussian_mean_offsets_follow_sigma_e() {
        // Component means step along w = Σe, and their projections onto e are
        // the library's m_i σ_v marginal spacing.
        let cov = ill_conditioned_6x6();
        let mean = [1.0, -2.0, 3.0, -0.5, 0.25, 4.0];
        let dir = [1.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        // Independent of the implementation's helpers: explicit w = Σe and
        // σ_v² = e·w, so this can fail on more than operation ordering.
        let mut w = [0.0_f64; 6];
        for i in 0..6 {
            for j in 0..6 {
                w[i] += cov[i][j] * dir[j];
            }
        }
        let sigma_v = (0..6).map(|i| dir[i] * w[i]).sum::<f64>().sqrt();
        for k in [2, 3, 5] {
            let comps = split_gaussian::<6>(&mean, &cov, &dir, k).unwrap();
            let entry = univariate_split(k).unwrap();
            for (idx, (_, m, _)) in comps.iter().enumerate() {
                for i in 0..6 {
                    let expected = entry.means[idx] * w[i] / sigma_v;
                    assert!(
                        (m[i] - mean[i] - expected).abs() <= 1e-12 * expected.abs().max(1.0),
                        "k = {k}, component {idx}: offset[{i}] = {} vs {expected}",
                        m[i] - mean[i]
                    );
                }
                let proj: f64 = (0..6).map(|i| dir[i] * (m[i] - mean[i])).sum();
                let expected = entry.means[idx] * sigma_v;
                assert!(
                    (proj - expected).abs() <= 1e-12 * sigma_v,
                    "k = {k}, component {idx}: eᵀ(μ_k − μ) = {proj} vs {expected}"
                );
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
