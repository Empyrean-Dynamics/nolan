//! # nolan (`hyperjet`) — forward-mode automatic differentiation with const-generic jets
//!
//! Forward-mode automatic differentiation built around const-generic,
//! stack-allocated hyperdual numbers (*jets*): [`Jet1<N>`] for gradients,
//! [`Jet2<N, H>`] for full Hessians, and [`Jet3<N, H, T>`] for third-order
//! tensors. No `Vec`, no `Box`, no heap. The same function body type-checks
//! against `f64` (no derivatives), [`Jet1`] (gradients), [`Jet2`] (Hessians),
//! or [`Jet3`] (third-order tensors).
//!
//! ```
//! use hyperjet::jets::Jet1;
//!
//! let x = Jet1::<2>::variable(1.5, 0);
//! let y = Jet1::<2>::variable(2.0, 1);
//! let f = x * x + y;
//! assert_eq!(f.value, 4.25);
//! assert_eq!(f.grad, [3.0, 1.0]); // df/dx = 2x = 3, df/dy = 1
//! ```
//!
//! ## What's in the box
//!
//! - [`jets`] — `Jet1<N>` / `Jet2<N, H>` / `Jet3<N, H, T>` types and operator
//!   impls. Stack-allocated, const-generic over parameter count.
//! - [`traits`] — `Differentiable`, `FirstOrder`, `SecondOrder`, `ThirdOrder`,
//!   `DifferentiableMath`, `AutoDiff` — write functions once, run with any
//!   jet order.
//! - [`differentiate`] — convenience wrappers (`differentiate1`,
//!   `differentiate2_6`, `differentiate3_9`, runtime-dispatched
//!   `differentiate_dyn_*`) that hide the jet seeding + extraction
//!   boilerplate.
//! - [`linalg`] — stack-allocated generic matrix operations (`mat_solve`,
//!   `mat_inv`, `mat_cholesky`, eigendecomposition, condition number,
//!   covariance regularization) that compose with any jet order.
//! - [`statistics`] — multivariate Gaussian primitives (sigma points,
//!   Gaussian-mixture splitting, sample statistics) and scalar distributions
//!   (`ln_gamma`, `chi2_sf`, etc.).
//! - [`grids`] — `linspace`, `logspace`, `linear_clamped`.
//! - [`angles`] — `wrap_pi`, `wrap_2pi`, `wrap_180`, `wrap_360`.
//!
//! Built for astrodynamics (orbit determination, covariance propagation,
//! close-approach sensitivity analysis), but the core types are
//! domain-agnostic and useful anywhere exact stack-allocated forward-mode
//! derivatives matter: optimization, robotics inverse kinematics, physics
//! simulation, ML gradient checking, sensitivity analysis.
//!
//! [`Jet1`]: jets::Jet1
//! [`Jet2`]: jets::Jet2
//! [`Jet3`]: jets::Jet3
//! [`Jet1<N>`]: jets::Jet1
//! [`Jet2<N, H>`]: jets::Jet2
//! [`Jet3<N, H, T>`]: jets::Jet3

// Validate that every ```rust block in README.md compiles as a doctest.
// Blocks that reference undefined identifiers should be marked with
// `no_run` or `ignore`; blocks that are intended to be runnable are
// exercised by `cargo test --doc`.
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

pub mod angles;
pub mod differentiate;
pub mod grids;
pub mod impls;
pub mod jets;
pub mod linalg;
pub mod optimization;
pub mod statistics;
pub mod traits;

/// Returns the full version string, e.g. "1.0.0+a3f7b2c" or "1.0.1-dev+f82c1a0-dirty".
///
/// Source builds inside the repo get the git-described version from `build.rs`.
/// Builds from the published crates.io tarball — which intentionally does not
/// ship `build.rs` — fall back to the static `CARGO_PKG_VERSION` so that
/// `hyperjet::version()` reports a clean release identifier (e.g. `"1.7.1"`)
/// to downstream callers.
pub fn version() -> &'static str {
    option_env!("GIT_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"))
}
