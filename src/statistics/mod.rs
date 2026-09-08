//! Statistical primitives.
//!
//! Standard normal distribution, regularized incomplete gamma /
//! chi-squared CDF/SF, Gaussian-mixture splitting, sigma-point
//! ensembles, sample-moment estimators. All routines are pure
//! mathematical primitives with no domain coupling — they work the
//! same whether the random variable is an orbital element, a residual,
//! or a photometric magnitude.
//!
//! # Layout
//!
//! - [`distributions`] — `ln_gamma`, regularized incomplete gamma,
//!   `chi2_sf`, `normal_pdf`, `normal_cdf`, `normal_sf`,
//!   `normal_cdf_difference`.
//! - [`multivariate`] — `split_gaussian`, `sigma_points`,
//!   `sample_statistics` generic over the state dimension `N`.
//! - [`split_library`] — the tabulated univariate Gaussian mixture
//!   `split_gaussian` splits with.
//!
//! # Numerical notes
//!
//! - `ln_gamma`: Lanczos g=7 (max error ~1e-15 in the principal branch
//!   `x > 0.5`).
//! - `upper_inc_gamma_reg`: switches between series (for `x < a + 1`)
//!   and Lentz continued-fraction (for `x ≥ a + 1`) — standard pattern
//!   from Numerical Recipes §6.2.
//! - `normal_cdf` / `normal_sf` / `normal_pdf`: the all-positive
//!   error-function series below 1.75σ and the Laplace continued
//!   fraction beyond it, over a density whose exponent is split so that
//!   its rounding does not grow with the argument. Relative error stays
//!   below 1e-14 at every argument whose result is a normal double,
//!   which reaches 38.49σ, and there is no cutoff anywhere short of
//!   that. Ask `normal_sf` for an upper tail; `1.0 - normal_cdf(x)` has
//!   no significant figures left beyond 8σ, and
//!   `normal_cdf_difference` for the probability of a bracket, which
//!   `normal_cdf(hi) - normal_cdf(lo)` loses when both ends sit on the
//!   same side of the origin.

pub mod distributions;
pub mod multivariate;
pub mod split_library;

pub use distributions::{
    UPPER_INC_GAMMA_MIN_A, chi2_sf, ln_gamma, normal_cdf, normal_cdf_difference, normal_pdf,
    normal_sf, upper_inc_gamma_reg,
};
pub use multivariate::{
    COVARIANCE_SYMMETRY_TOLERANCE, GaussianSplitError, ScaledSigmaPoints, SigmaPointScaling,
    SigmaPointsError, sample_statistics, sigma_points, sigma_points_scaled, split_gaussian,
    weighted_sample_statistics,
};
pub use split_library::{
    MAX_SPLIT_COMPONENTS, MIN_SPLIT_COMPONENTS, UnivariateSplit, univariate_split,
};
