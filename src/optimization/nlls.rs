//! Nonlinear least-squares solver (Gauss-Newton / Levenberg-Marquardt).

use crate::linalg::generic::{mat_inv, mat_quadratic_form, mat_solve, mat_symmetrize};

// ── Types ───────────────────────────────────────────────────────────

/// Result of evaluating the residual function at a point.
///
/// The caller pre-weights residuals and Jacobian by the observation
/// weight Cholesky factor: \\(\mathbf{r}_w = L\mathbf{r}\\),
/// \\(J_w = LJ\\) where \\(W = LL^T\\). The solver then accumulates
/// \\(J^T J\\) and \\(J^T r\\) without an explicit weight matrix.
#[derive(Clone, Debug)]
pub struct NLLSEvaluation<const N: usize> {
    /// Pre-weighted residual values.
    pub residuals: Vec<f64>,
    /// Pre-weighted Jacobian rows: \\(\partial r_i / \partial x_j\\).
    /// Must have the same length as `residuals`.
    pub jacobian: Vec<[f64; N]>,
    /// Total cost \\(\sum r_i^2\\). Used for Levenberg-Marquardt adaptation.
    pub cost: f64,
}

/// Second-order evaluation (extends first-order with Hessians).
///
/// The exact Hessian of the cost function is:
/// \\[
///   H = J^T J + \sum_i r_i \, H_i
/// \\]
/// where \\(H_i = \partial^2 r_i / \partial x_j \partial x_k\\).
/// The second term corrects the Gauss-Newton approximation.
#[derive(Clone, Debug)]
pub struct NLLSEvaluation2<const N: usize> {
    /// First-order data (residuals, Jacobian, cost).
    pub first_order: NLLSEvaluation<N>,
    /// Hessian of each residual: \\(\partial^2 r_i / \partial x_j \partial x_k\\).
    /// Must have the same length as `first_order.residuals`.
    pub hessians: Vec<[[f64; N]; N]>,
}

/// The problem to solve. Implement this trait to provide residuals
/// and Jacobians for a specific application (orbit determination,
/// maneuver targeting, etc.).
pub trait NLLSProblem<const N: usize> {
    /// Evaluate residuals and Jacobian at the given parameter values.
    fn evaluate(&mut self, x: &[f64; N]) -> NLLSEvaluation<N>;

    /// Apply problem-specific bounds to a proposed step \\(\Delta x\\)
    /// at the current iterate \\(\mathbf{x}\\).
    ///
    /// The solver calls this after computing the Gauss-Newton /
    /// Levenberg-Marquardt update and *before* applying it to
    /// \\(\mathbf{x}\\), giving the problem a chance to clamp step
    /// magnitudes when the linear model overshoots the region of
    /// validity (highly nonlinear residuals, Jacobians that collapse to
    /// zero far from the minimum, physically meaningful bounds on state
    /// components, etc.).
    ///
    /// The default implementation is a no-op — existing consumers are
    /// unaffected. Convergence checks run on the clamped step, so
    /// `constrain_step` interacts naturally with `step_tolerance`: a
    /// clamped step below the tolerance still declares convergence.
    ///
    /// LM's \\(\lambda\\) damping already limits step size, but only in
    /// proportion to \\(J^T J\\). When \\(J\\) itself is poorly scaled
    /// or near-zero (common in orbit determination with rough initial
    /// seeds), \\(\lambda\\) adaptation cannot recover. `constrain_step`
    /// provides an orthogonal, problem-driven bound.
    ///
    /// # Example: fractional bounds on position and velocity sub-steps
    ///
    /// For orbit determination with state \\([\mathbf{r}, \mathbf{v}]\\),
    /// bound each sub-step to at most half its current sub-magnitude:
    ///
    /// ```
    /// use hyperjet::optimization::{NLLSProblem, NLLSEvaluation};
    ///
    /// struct OdProblem;
    /// impl NLLSProblem<6> for OdProblem {
    ///     fn evaluate(&mut self, _x: &[f64; 6]) -> NLLSEvaluation<6> {
    ///         # NLLSEvaluation { residuals: vec![0.0], jacobian: vec![[0.0; 6]], cost: 0.0 }
    ///     }
    ///
    ///     fn constrain_step(&mut self, x: &[f64; 6], delta: &mut [f64; 6]) {
    ///         let r_mag = (x[0] * x[0] + x[1] * x[1] + x[2] * x[2]).sqrt();
    ///         let v_mag = (x[3] * x[3] + x[4] * x[4] + x[5] * x[5]).sqrt();
    ///         let dr = (delta[0] * delta[0]
    ///                 + delta[1] * delta[1]
    ///                 + delta[2] * delta[2]).sqrt();
    ///         let dv = (delta[3] * delta[3]
    ///                 + delta[4] * delta[4]
    ///                 + delta[5] * delta[5]).sqrt();
    ///         let max_dr = 0.5 * r_mag.max(1e-5);  // floor ~1500 km at 1 AU
    ///         let max_dv = 0.5 * v_mag.max(1e-6);
    ///         if dr > max_dr {
    ///             let s = max_dr / dr;
    ///             for k in 0..3 { delta[k] *= s; }
    ///         }
    ///         if dv > max_dv {
    ///             let s = max_dv / dv;
    ///             for k in 3..6 { delta[k] *= s; }
    ///         }
    ///     }
    /// }
    /// ```
    fn constrain_step(&mut self, _x: &[f64; N], _delta: &mut [f64; N]) {}
}

/// Second-order problem (provides Hessians for full Newton correction).
pub trait NLLSProblem2<const N: usize>: NLLSProblem<N> {
    /// Evaluate residuals, Jacobian, and Hessians at the given parameter values.
    fn evaluate2(&mut self, x: &[f64; N]) -> NLLSEvaluation2<N>;
}

/// Optional Bayesian prior on the parameters.
///
/// Augments the normal equations: \\(N \leftarrow N + P_0^{-1}\\),
/// \\(d \leftarrow d - P_0^{-1}(\mathbf{x} - \mathbf{x}_0)\\).
#[derive(Clone, Debug)]
pub struct NLLSPrior<const N: usize> {
    /// Prior mean.
    pub mean: [f64; N],
    /// Inverse of prior covariance matrix.
    pub covariance_inv: [[f64; N]; N],
}

/// Solver configuration.
#[derive(Clone, Debug)]
pub struct NLLSConfig {
    /// Maximum number of iterations.
    pub max_iterations: usize,
    /// Convergence threshold: \\(\Delta x^T N \Delta x < \text{tol}\\).
    pub step_tolerance: f64,
    /// Iteration method.
    pub method: NLLSMethod,
}

impl Default for NLLSConfig {
    fn default() -> Self {
        Self {
            max_iterations: 20,
            step_tolerance: 0.1,
            method: NLLSMethod::LevenbergMarquardt {
                lambda_initial: 1.0,
            },
        }
    }
}

/// Iteration method.
#[derive(Clone, Debug)]
pub enum NLLSMethod {
    /// Pure Gauss-Newton (no damping). Fast but may diverge.
    GaussNewton,
    /// Levenberg-Marquardt with adaptive damping.
    LevenbergMarquardt {
        /// Initial damping parameter.
        lambda_initial: f64,
    },
}

/// Solver output.
#[derive(Clone, Debug)]
pub struct NLLSSolution<const N: usize> {
    /// Converged parameter values.
    pub x: [f64; N],
    /// Formal covariance matrix \\((J^T J)^{-1}\\) at the solution.
    pub covariance: [[f64; N]; N],
    /// Final cost \\(\sum r_i^2\\).
    pub cost: f64,
    /// Number of iterations performed.
    pub iterations: usize,
    /// Whether the solver converged.
    pub converged: bool,
    /// Reason the solver stopped.
    pub reason: ConvergenceReason,
}

/// Reason the solver stopped iterating.
#[derive(Clone, Debug, PartialEq)]
pub enum ConvergenceReason {
    /// Step norm fell below tolerance.
    StepTolerance,
    /// Reached maximum iterations.
    MaxIterations,
    /// Normal matrix was singular.
    SingularMatrix,
}

/// Solver error.
#[derive(Clone, Debug)]
pub enum NLLSError {
    /// Normal matrix is singular — problem may be underdetermined.
    SingularMatrix { iteration: usize },
    /// No residuals returned by the evaluation function.
    EmptyResiduals,
    /// Jacobian and residual lengths don't match.
    DimensionMismatch { residuals: usize, jacobian: usize },
}

impl std::fmt::Display for NLLSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SingularMatrix { iteration } => {
                write!(f, "singular normal matrix at iteration {iteration}")
            }
            Self::EmptyResiduals => write!(f, "evaluation returned empty residuals"),
            Self::DimensionMismatch {
                residuals,
                jacobian,
            } => write!(
                f,
                "dimension mismatch: {residuals} residuals vs {jacobian} Jacobian rows"
            ),
        }
    }
}

impl std::error::Error for NLLSError {}

// ── First-order solver ──────────────────────────────────────────────

/// Solve a nonlinear least-squares problem using the trait interface.
///
/// The caller implements [`NLLSProblem`] to provide residuals and
/// Jacobians. The solver iterates Gauss-Newton or Levenberg-Marquardt
/// steps until convergence.
pub fn solve<const N: usize>(
    problem: &mut impl NLLSProblem<N>,
    x0: [f64; N],
    config: &NLLSConfig,
    prior: Option<&NLLSPrior<N>>,
) -> Result<NLLSSolution<N>, NLLSError> {
    let mut x = x0;
    let mut lambda = match &config.method {
        NLLSMethod::GaussNewton => 0.0,
        NLLSMethod::LevenbergMarquardt { lambda_initial } => *lambda_initial,
    };
    let use_lm = matches!(config.method, NLLSMethod::LevenbergMarquardt { .. });
    let mut last_cost = f64::INFINITY;
    let mut final_cost = 0.0;
    let mut converged = false;
    let mut reason = ConvergenceReason::MaxIterations;
    let mut iterations = 0;
    let mut undamped_normal = [[0.0_f64; N]; N];

    for iter in 0..config.max_iterations {
        iterations = iter + 1;

        // Evaluate residuals and Jacobian
        let eval = problem.evaluate(&x);
        validate_evaluation::<N>(&eval)?;
        final_cost = eval.cost;

        // Assemble normal equations: normal = J^T J, rhs = J^T r
        let (normal, rhs) = assemble_normal_equations(&eval);
        undamped_normal = mat_symmetrize(&normal);

        // Apply prior augmentation
        let (mut solve_normal, mut solve_rhs) = (undamped_normal, rhs);
        if let Some(p) = prior {
            augment_with_prior(&mut solve_normal, &mut solve_rhs, &x, p);
        }

        // LM damping
        if use_lm {
            for i in 0..N {
                solve_normal[i][i] += lambda * undamped_normal[i][i];
            }
        }

        // Solve for update
        let mut delta_x = match mat_solve(&solve_normal, &solve_rhs) {
            Some(dx) => dx,
            None => {
                reason = ConvergenceReason::SingularMatrix;
                converged = false;
                break;
            }
        };

        // Problem-driven step bounds (default no-op).
        problem.constrain_step(&x, &mut delta_x);

        // Convergence check (use undamped normal for scale)
        let step_norm = mat_quadratic_form(&delta_x, &undamped_normal);
        if step_norm < config.step_tolerance {
            // Apply final update before declaring convergence
            for i in 0..N {
                x[i] += delta_x[i];
            }
            converged = true;
            reason = ConvergenceReason::StepTolerance;
            break;
        }

        // LM cost adaptation
        if use_lm {
            if eval.cost < last_cost {
                lambda = (lambda / 3.0).max(1e-6);
            } else {
                lambda = (lambda * 3.0).min(1e4);
            }
            last_cost = eval.cost;
        }

        // Update parameters
        for i in 0..N {
            x[i] += delta_x[i];
        }
    }

    // Posterior covariance
    let mut cov_normal = undamped_normal;
    if let Some(p) = prior {
        for i in 0..N {
            for j in 0..N {
                cov_normal[i][j] += p.covariance_inv[i][j];
            }
        }
    }
    let covariance = mat_inv(&cov_normal).unwrap_or([[0.0; N]; N]);
    let covariance = mat_symmetrize(&covariance);

    Ok(NLLSSolution {
        x,
        covariance,
        cost: final_cost,
        iterations,
        converged,
        reason,
    })
}

// ── Second-order solver ─────────────────────────────────────────────

/// Solve with second-order (full Newton) corrections.
///
/// The Hessian of the cost function is corrected with:
/// \\(H = J^T J + \sum_i r_i H_i\\)
pub fn solve2<const N: usize>(
    problem: &mut impl NLLSProblem2<N>,
    x0: [f64; N],
    config: &NLLSConfig,
    prior: Option<&NLLSPrior<N>>,
) -> Result<NLLSSolution<N>, NLLSError> {
    let mut x = x0;
    let mut lambda = match &config.method {
        NLLSMethod::GaussNewton => 0.0,
        NLLSMethod::LevenbergMarquardt { lambda_initial } => *lambda_initial,
    };
    let use_lm = matches!(config.method, NLLSMethod::LevenbergMarquardt { .. });
    let mut last_cost = f64::INFINITY;
    let mut final_cost = 0.0;
    let mut converged = false;
    let mut reason = ConvergenceReason::MaxIterations;
    let mut iterations = 0;
    let mut undamped_normal = [[0.0_f64; N]; N];

    for iter in 0..config.max_iterations {
        iterations = iter + 1;

        let eval2 = problem.evaluate2(&x);
        let eval = &eval2.first_order;
        validate_evaluation::<N>(eval)?;
        final_cost = eval.cost;

        // Gauss-Newton normal equations
        let (gn_normal, rhs) = assemble_normal_equations(eval);

        // Second-order correction: add Σ r_i * H_i
        let mut normal = gn_normal;
        for (i, r_i) in eval.residuals.iter().enumerate() {
            if let Some(h_i) = eval2.hessians.get(i) {
                for j in 0..N {
                    for k in 0..N {
                        normal[j][k] += r_i * h_i[j][k];
                    }
                }
            }
        }

        undamped_normal = mat_symmetrize(&normal);

        let (mut solve_normal, mut solve_rhs) = (undamped_normal, rhs);
        if let Some(p) = prior {
            augment_with_prior(&mut solve_normal, &mut solve_rhs, &x, p);
        }

        if use_lm {
            for i in 0..N {
                solve_normal[i][i] += lambda * undamped_normal[i][i];
            }
        }

        let mut delta_x = match mat_solve(&solve_normal, &solve_rhs) {
            Some(dx) => dx,
            None => {
                reason = ConvergenceReason::SingularMatrix;
                converged = false;
                break;
            }
        };

        // Problem-driven step bounds (default no-op).
        problem.constrain_step(&x, &mut delta_x);

        let step_norm = mat_quadratic_form(&delta_x, &undamped_normal);
        if step_norm < config.step_tolerance {
            for i in 0..N {
                x[i] += delta_x[i];
            }
            converged = true;
            reason = ConvergenceReason::StepTolerance;
            break;
        }

        if use_lm {
            if eval.cost < last_cost {
                lambda = (lambda / 3.0).max(1e-6);
            } else {
                lambda = (lambda * 3.0).min(1e4);
            }
            last_cost = eval.cost;
        }

        for i in 0..N {
            x[i] += delta_x[i];
        }
    }

    let mut cov_normal = undamped_normal;
    if let Some(p) = prior {
        for i in 0..N {
            for j in 0..N {
                cov_normal[i][j] += p.covariance_inv[i][j];
            }
        }
    }
    let covariance = mat_inv(&cov_normal).unwrap_or([[0.0; N]; N]);
    let covariance = mat_symmetrize(&covariance);

    Ok(NLLSSolution {
        x,
        covariance,
        cost: final_cost,
        iterations,
        converged,
        reason,
    })
}

// ── Closure-based convenience ───────────────────────────────────────

/// Solve using a closure instead of the trait interface.
pub fn solve_nlls<const N: usize>(
    f: impl FnMut(&[f64; N]) -> NLLSEvaluation<N>,
    x0: [f64; N],
    config: &NLLSConfig,
    prior: Option<&NLLSPrior<N>>,
) -> Result<NLLSSolution<N>, NLLSError> {
    struct ClosureProblem<F>(F);
    impl<const N: usize, F: FnMut(&[f64; N]) -> NLLSEvaluation<N>> NLLSProblem<N>
        for ClosureProblem<F>
    {
        fn evaluate(&mut self, x: &[f64; N]) -> NLLSEvaluation<N> {
            (self.0)(x)
        }
    }
    solve(&mut ClosureProblem(f), x0, config, prior)
}

// ── Internal helpers ────────────────────────────────────────────────

fn validate_evaluation<const N: usize>(eval: &NLLSEvaluation<N>) -> Result<(), NLLSError> {
    if eval.residuals.is_empty() {
        return Err(NLLSError::EmptyResiduals);
    }
    if eval.residuals.len() != eval.jacobian.len() {
        return Err(NLLSError::DimensionMismatch {
            residuals: eval.residuals.len(),
            jacobian: eval.jacobian.len(),
        });
    }
    Ok(())
}

/// Assemble J^T J and J^T r from pre-weighted residuals and Jacobian.
fn assemble_normal_equations<const N: usize>(
    eval: &NLLSEvaluation<N>,
) -> ([[f64; N]; N], [f64; N]) {
    let mut normal = [[0.0_f64; N]; N];
    let mut rhs = [0.0_f64; N];

    for (r_i, j_i) in eval.residuals.iter().zip(eval.jacobian.iter()) {
        for j in 0..N {
            for k in 0..N {
                normal[j][k] += j_i[j] * j_i[k];
            }
            // Negative: Δx minimizes ||r + J·Δx||² → J^T J Δx = -J^T r
            rhs[j] -= j_i[j] * r_i;
        }
    }

    (normal, rhs)
}

/// Add Bayesian prior to normal equations.
fn augment_with_prior<const N: usize>(
    normal: &mut [[f64; N]; N],
    rhs: &mut [f64; N],
    x: &[f64; N],
    prior: &NLLSPrior<N>,
) {
    for i in 0..N {
        for j in 0..N {
            normal[i][j] += prior.covariance_inv[i][j];
        }
        // rhs -= P_0^{-1} * (x - x_0)
        let mut delta_weighted = 0.0;
        for j in 0..N {
            delta_weighted += prior.covariance_inv[i][j] * (x[j] - prior.mean[j]);
        }
        rhs[i] -= delta_weighted;
    }
}

// ── Tests ───────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_system() {
        // Solve: r = [x0 - 3, x1 - 7] = 0
        // Jacobian = identity, should converge in 1 iteration
        let solution = solve_nlls(
            |x: &[f64; 2]| NLLSEvaluation {
                residuals: vec![x[0] - 3.0, x[1] - 7.0],
                jacobian: vec![[1.0, 0.0], [0.0, 1.0]],
                cost: (x[0] - 3.0).powi(2) + (x[1] - 7.0).powi(2),
            },
            [0.0; 2],
            &NLLSConfig {
                method: NLLSMethod::GaussNewton,
                ..Default::default()
            },
            None,
        )
        .unwrap();

        assert!(solution.converged);
        assert!(
            (solution.x[0] - 3.0).abs() < 1e-12,
            "x[0]={}",
            solution.x[0]
        );
        assert!(
            (solution.x[1] - 7.0).abs() < 1e-12,
            "x[1]={}",
            solution.x[1]
        );
        // Linear problem converges in 2 iterations: first computes the
        // correction, second verifies zero residuals.
        assert!(
            solution.iterations <= 2,
            "iterations={}",
            solution.iterations
        );
    }

    #[test]
    fn test_overdetermined_linear() {
        // 4 equations, 2 unknowns: y = a*x + b
        // Data: (0,1), (1,3), (2,5), (3,7) → a=2, b=1
        let xs = [0.0, 1.0, 2.0, 3.0];
        let ys = [1.0, 3.0, 5.0, 7.0];

        let solution = solve_nlls(
            |p: &[f64; 2]| {
                let (a, b) = (p[0], p[1]);
                let residuals: Vec<f64> = xs
                    .iter()
                    .zip(ys.iter())
                    .map(|(&x, &y)| a * x + b - y)
                    .collect();
                let jacobian: Vec<[f64; 2]> = xs.iter().map(|&x| [x, 1.0]).collect();
                let cost = residuals.iter().map(|r| r * r).sum();
                NLLSEvaluation {
                    residuals,
                    jacobian,
                    cost,
                }
            },
            [0.0; 2],
            &NLLSConfig {
                method: NLLSMethod::GaussNewton,
                ..Default::default()
            },
            None,
        )
        .unwrap();

        assert!(solution.converged);
        assert!((solution.x[0] - 2.0).abs() < 1e-10, "a={}", solution.x[0]);
        assert!((solution.x[1] - 1.0).abs() < 1e-10, "b={}", solution.x[1]);
    }

    #[test]
    fn test_rosenbrock() {
        // Rosenbrock: r1 = 10*(x1 - x0²), r2 = 1 - x0
        // Minimum at (1, 1)
        let solution = solve_nlls(
            |x: &[f64; 2]| {
                let r1 = 10.0 * (x[1] - x[0] * x[0]);
                let r2 = 1.0 - x[0];
                NLLSEvaluation {
                    residuals: vec![r1, r2],
                    jacobian: vec![[-20.0 * x[0], 10.0], [-1.0, 0.0]],
                    cost: r1 * r1 + r2 * r2,
                }
            },
            [-1.0, 1.0],
            &NLLSConfig {
                max_iterations: 50,
                step_tolerance: 1e-10,
                method: NLLSMethod::LevenbergMarquardt {
                    lambda_initial: 1.0,
                },
            },
            None,
        )
        .unwrap();

        assert!(solution.converged, "reason: {:?}", solution.reason);
        assert!((solution.x[0] - 1.0).abs() < 1e-6, "x[0]={}", solution.x[0]);
        assert!((solution.x[1] - 1.0).abs() < 1e-6, "x[1]={}", solution.x[1]);
    }

    #[test]
    fn test_circle_fit() {
        // Fit circle: (x-cx)² + (y-cy)² = r²
        // Parameters: [cx, cy, r]. 8 data points on a circle centered at (2, 3) radius 5.
        let angles: [f64; 8] = [0.0, 0.7, 1.4, 2.1, 2.8, 3.5, 4.2, 4.9];
        let data: Vec<(f64, f64)> = angles
            .iter()
            .map(|&a| (2.0 + 5.0 * a.cos(), 3.0 + 5.0 * a.sin()))
            .collect();

        let solution = solve_nlls(
            |p: &[f64; 3]| {
                let (cx, cy, r) = (p[0], p[1], p[2]);
                let mut residuals = Vec::new();
                let mut jacobian = Vec::new();
                for &(x, y) in &data {
                    let dx = x - cx;
                    let dy = y - cy;
                    let dist = (dx * dx + dy * dy).sqrt();
                    residuals.push(dist - r);
                    jacobian.push([-dx / dist, -dy / dist, -1.0]);
                }
                let cost = residuals.iter().map(|r| r * r).sum();
                NLLSEvaluation {
                    residuals,
                    jacobian,
                    cost,
                }
            },
            [0.0, 0.0, 1.0],
            &NLLSConfig {
                max_iterations: 30,
                step_tolerance: 1e-12,
                method: NLLSMethod::LevenbergMarquardt {
                    lambda_initial: 1.0,
                },
            },
            None,
        )
        .unwrap();

        assert!(solution.converged, "reason: {:?}", solution.reason);
        assert!((solution.x[0] - 2.0).abs() < 1e-8, "cx={}", solution.x[0]);
        assert!((solution.x[1] - 3.0).abs() < 1e-8, "cy={}", solution.x[1]);
        assert!((solution.x[2] - 5.0).abs() < 1e-8, "r={}", solution.x[2]);
    }

    #[test]
    fn test_prior_augmentation() {
        // Without prior: solve r = x - 10 → x = 10
        // With strong prior at x = 0: solution should be pulled toward 0
        let config = NLLSConfig {
            method: NLLSMethod::GaussNewton,
            ..Default::default()
        };

        let eval = |x: &[f64; 1]| NLLSEvaluation {
            residuals: vec![x[0] - 10.0],
            jacobian: vec![[1.0]],
            cost: (x[0] - 10.0).powi(2),
        };

        // Without prior
        let sol1 = solve_nlls(eval, [0.0; 1], &config, None).unwrap();
        assert!((sol1.x[0] - 10.0).abs() < 1e-10);

        // With strong prior at 0 (variance = 1, so weight = 1)
        let prior = NLLSPrior {
            mean: [0.0],
            covariance_inv: [[1.0]],
        };
        let sol2 = solve_nlls(eval, [0.0; 1], &config, Some(&prior)).unwrap();
        // Should be pulled toward 0: x = 10 * 1/(1+1) = 5
        assert!(
            (sol2.x[0] - 5.0).abs() < 1e-10,
            "x={} (expected 5.0)",
            sol2.x[0]
        );
    }

    #[test]
    fn test_singular_matrix() {
        // Zero Jacobian → singular normal matrix
        let result = solve_nlls(
            |_x: &[f64; 2]| NLLSEvaluation {
                residuals: vec![1.0],
                jacobian: vec![[0.0, 0.0]],
                cost: 1.0,
            },
            [0.0; 2],
            &NLLSConfig::default(),
            None,
        );

        let sol = result.unwrap();
        assert!(!sol.converged);
        assert_eq!(sol.reason, ConvergenceReason::SingularMatrix);
    }

    #[test]
    fn test_empty_residuals_error() {
        let result = solve_nlls(
            |_x: &[f64; 2]| NLLSEvaluation {
                residuals: vec![],
                jacobian: vec![],
                cost: 0.0,
            },
            [0.0; 2],
            &NLLSConfig::default(),
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_covariance_output() {
        // Linear problem with known covariance
        // r = [x - 3], J = [[1]], so N = J^T J = [[1]], cov = [[1]]
        let sol = solve_nlls(
            |x: &[f64; 1]| NLLSEvaluation {
                residuals: vec![x[0] - 3.0],
                jacobian: vec![[1.0]],
                cost: (x[0] - 3.0).powi(2),
            },
            [0.0],
            &NLLSConfig {
                method: NLLSMethod::GaussNewton,
                ..Default::default()
            },
            None,
        )
        .unwrap();

        assert!((sol.covariance[0][0] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_constrain_step_bounds_overshoot() {
        // Residual r(x) = ln(1 + (x - 5)²) with minimum at x = 5.
        //
        // Far from the minimum the residual grows only logarithmically
        // while the Jacobian J = 2(x-5)/(1 + (x-5)²) decays as 1/|x-5|,
        // so pure Gauss-Newton projects Δx = -r/J ≈ -(x-5) · ln(·)/2,
        // overshooting by orders of magnitude. LM's λ adaptation cannot
        // recover because damping is scaled by J² ≈ 1/|x-5|² — also
        // tiny — so each overshot step lands in an equally poor linear
        // regime and the cost grows across iterations.
        //
        // This is the pattern that motivates problem-driven step
        // bounds via `NLLSProblem::constrain_step` — common in any
        // nonlinear least-squares solve where the residual surface
        // has near-zero-Jacobian regions that LM's intrinsic λ
        // adaptation cannot escape.
        let eval = |x: &[f64; 1]| {
            let d = x[0] - 5.0;
            let r = (1.0 + d * d).ln();
            let j = 2.0 * d / (1.0 + d * d);
            NLLSEvaluation {
                residuals: vec![r],
                jacobian: vec![[j]],
                cost: r * r,
            }
        };
        let config = NLLSConfig {
            max_iterations: 20,
            step_tolerance: 1e-12,
            method: NLLSMethod::LevenbergMarquardt {
                lambda_initial: 0.0,
            },
        };

        // Unconstrained LM from x = 100 thrashes between ever-larger |x|;
        // cost is bounded below by the first-iteration value and grows
        // monotonically past it.
        let sol_free = solve_nlls(eval, [100.0], &config, None).unwrap();
        let initial_cost = {
            let d = 100.0_f64 - 5.0;
            let r = (1.0 + d * d).ln();
            r * r
        };
        assert!(
            sol_free.cost > initial_cost,
            "unconstrained LM did not overshoot: cost={} vs initial={}",
            sol_free.cost,
            initial_cost,
        );
        assert!(
            (sol_free.x[0] - 5.0).abs() > 100.0,
            "unconstrained LM unexpectedly stayed near minimum: x={}",
            sol_free.x[0],
        );

        // Clamp |Δx| to at most half of |x| — bounds step size by
        // iterate magnitude (rather than distance to the unknown
        // minimum, which is naturally unknown ahead of convergence).
        // Converges toward x = 5 in a handful of steps.
        struct Clamped;
        impl NLLSProblem<1> for Clamped {
            fn evaluate(&mut self, x: &[f64; 1]) -> NLLSEvaluation<1> {
                let d = x[0] - 5.0;
                let r = (1.0 + d * d).ln();
                let j = 2.0 * d / (1.0 + d * d);
                NLLSEvaluation {
                    residuals: vec![r],
                    jacobian: vec![[j]],
                    cost: r * r,
                }
            }
            fn constrain_step(&mut self, x: &[f64; 1], delta: &mut [f64; 1]) {
                let max_step = 0.5 * x[0].abs().max(0.1);
                if delta[0].abs() > max_step {
                    delta[0] *= max_step / delta[0].abs();
                }
            }
        }
        let sol_bound = solve(&mut Clamped, [100.0], &config, None).unwrap();
        assert!(
            sol_bound.converged,
            "bounded LM failed: x={}, cost={}, reason={:?}",
            sol_bound.x[0], sol_bound.cost, sol_bound.reason,
        );
        assert!(
            (sol_bound.x[0] - 5.0).abs() < 1e-3,
            "bounded LM did not reach minimum: x={}",
            sol_bound.x[0],
        );
    }

    #[test]
    fn test_constrain_step_default_is_noop() {
        // Default trait impl on NLLSProblem must leave existing behavior
        // unchanged. The closure-based ClosureProblem picks up the
        // default constrain_step, so this test exercises the same code
        // path as test_linear_system. Regression guard.
        let sol = solve_nlls(
            |x: &[f64; 2]| NLLSEvaluation {
                residuals: vec![x[0] - 3.0, x[1] - 7.0],
                jacobian: vec![[1.0, 0.0], [0.0, 1.0]],
                cost: (x[0] - 3.0).powi(2) + (x[1] - 7.0).powi(2),
            },
            [0.0; 2],
            &NLLSConfig {
                method: NLLSMethod::GaussNewton,
                ..Default::default()
            },
            None,
        )
        .unwrap();
        assert!(sol.converged);
        assert!((sol.x[0] - 3.0).abs() < 1e-10);
        assert!((sol.x[1] - 7.0).abs() < 1e-10);
    }

    #[test]
    fn test_lm_vs_gn() {
        // Both should converge on the same linear problem
        let eval = |x: &[f64; 2]| NLLSEvaluation {
            residuals: vec![x[0] - 1.0, x[1] - 2.0, x[0] + x[1] - 3.0],
            jacobian: vec![[1.0, 0.0], [0.0, 1.0], [1.0, 1.0]],
            cost: (x[0] - 1.0).powi(2) + (x[1] - 2.0).powi(2) + (x[0] + x[1] - 3.0).powi(2),
        };

        let sol_gn = solve_nlls(
            eval,
            [0.0; 2],
            &NLLSConfig {
                method: NLLSMethod::GaussNewton,
                ..Default::default()
            },
            None,
        )
        .unwrap();

        let sol_lm = solve_nlls(
            eval,
            [0.0; 2],
            &NLLSConfig {
                max_iterations: 50,
                step_tolerance: 1e-10,
                method: NLLSMethod::LevenbergMarquardt {
                    lambda_initial: 1.0,
                },
            },
            None,
        )
        .unwrap();

        assert!(sol_gn.converged, "GN: {:?}", sol_gn.reason);
        assert!(sol_lm.converged, "LM: {:?}", sol_lm.reason);
        // Both should find the same solution (tolerance relaxed for LM damping path)
        assert!(
            (sol_gn.x[0] - sol_lm.x[0]).abs() < 1e-4,
            "x[0]: GN={} LM={}",
            sol_gn.x[0],
            sol_lm.x[0]
        );
        assert!(
            (sol_gn.x[1] - sol_lm.x[1]).abs() < 1e-4,
            "x[1]: GN={} LM={}",
            sol_gn.x[1],
            sol_lm.x[1]
        );
    }
}
