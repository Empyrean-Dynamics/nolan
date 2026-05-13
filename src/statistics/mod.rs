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
//!   `chi2_sf`, `normal_pdf`, `normal_cdf`.
//! - [`multivariate`] — `split_gaussian`, `sigma_points`,
//!   `sample_statistics` generic over the state dimension `N`.
//!
//! # Numerical notes
//!
//! - `ln_gamma`: Lanczos g=7 (max error ~1e-15 in the principal branch
//!   `x > 0.5`).
//! - `upper_inc_gamma_reg`: switches between series (for `x < a + 1`)
//!   and Lentz continued-fraction (for `x ≥ a + 1`) — standard pattern
//!   from Numerical Recipes §6.2.
//! - `normal_cdf`: Abramowitz & Stegun 26.2.17 polynomial
//!   approximation (max error < 7.5e-8).

pub mod distributions;
pub mod multivariate;

pub use distributions::{chi2_sf, ln_gamma, normal_cdf, normal_pdf, upper_inc_gamma_reg};
pub use multivariate::{
    GaussianSplitError, SigmaPointsError, sample_statistics, sigma_points, split_gaussian,
};
