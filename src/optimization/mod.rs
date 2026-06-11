//! Nonlinear least-squares optimization.
//!
//! Provides a gain-ratio trust-region Levenberg-Marquardt solver for
//! problems of the form: minimize \\(\sum_i r_i(\mathbf{x})^2\\)
//! where \\(\mathbf{x} \in \mathbb{R}^N\\).
//!
//! The caller implements one of the [`lm`] problem traits (or hands a
//! closure to [`lm::solve_nlls`]) to provide residuals and Jacobians;
//! the solver handles damping, step acceptance, convergence, and
//! covariance extraction. See the [`lm`] module documentation for the
//! algorithm and the full API.
//!
//! # Example
//!
//! ```
//! use hyperjet::optimization::lm::{LMConfig, NLLSEvaluation, solve_nlls};
//!
//! // Solve: find x such that [x[0] - 3, x[1] - 7] = [0, 0]
//! let solution = solve_nlls(
//!     |x: &[f64; 2]| {
//!         let residuals = vec![x[0] - 3.0, x[1] - 7.0];
//!         let cost = residuals.iter().map(|r| r * r).sum();
//!         Ok::<_, std::convert::Infallible>(NLLSEvaluation {
//!             residuals,
//!             jacobian: vec![[1.0, 0.0], [0.0, 1.0]],
//!             cost,
//!         })
//!     },
//!     |x: &[f64; 2]| {
//!         Ok::<_, std::convert::Infallible>(
//!             (x[0] - 3.0).powi(2) + (x[1] - 7.0).powi(2),
//!         )
//!     },
//!     [0.0; 2],
//!     &LMConfig::default(),
//!     None,
//! )
//! .unwrap();
//! assert!(solution.converged);
//! assert!((solution.x[0] - 3.0).abs() < 1e-8);
//! assert!((solution.x[1] - 7.0).abs() < 1e-8);
//! ```

pub mod lm;
