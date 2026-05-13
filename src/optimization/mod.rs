//! Nonlinear least-squares optimization.
//!
//! Provides a generic Gauss-Newton / Levenberg-Marquardt solver for
//! problems of the form: minimize \\(\sum_i r_i(\mathbf{x})^2\\)
//! where \\(\mathbf{x} \in \mathbb{R}^N\\).
//!
//! The caller implements [`NLLSProblem`] to provide residuals and
//! Jacobians; the solver handles the iteration, damping, convergence,
//! and covariance extraction.
//!
//! # Example
//!
//! ```
//! use hyperjet::optimization::*;
//!
//! // Solve: find x such that [x[0] - 3, x[1] - 7] = [0, 0]
//! let solution = solve_nlls(
//!     |x: &[f64; 2]| NLLSEvaluation {
//!         residuals: vec![x[0] - 3.0, x[1] - 7.0],
//!         jacobian: vec![[1.0, 0.0], [0.0, 1.0]],
//!         cost: (x[0] - 3.0).powi(2) + (x[1] - 7.0).powi(2),
//!     },
//!     [0.0; 2],
//!     &NLLSConfig::default(),
//!     None,
//! ).unwrap();
//! assert!((solution.x[0] - 3.0).abs() < 0.1);
//! assert!((solution.x[1] - 7.0).abs() < 0.1);
//! ```

mod nlls;

pub use nlls::{
    ConvergenceReason, NLLSConfig, NLLSError, NLLSEvaluation, NLLSEvaluation2, NLLSMethod,
    NLLSPrior, NLLSProblem, NLLSProblem2, NLLSSolution, solve, solve_nlls, solve2,
};
