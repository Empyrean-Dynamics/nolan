//! Gain-ratio Levenberg-Marquardt driver.
//!
//! Minimizes \\( \Phi(\mathbf{x}) = \sum_i r_i(\mathbf{x})^2 \\)
//! (optionally plus a Gaussian prior penalty
//! \\( (\mathbf{x}-\mathbf{x}_0)^\top P_0^{-1} (\mathbf{x}-\mathbf{x}_0) \\))
//! with a trust-region-style accept/reject loop:
//!
//! 1. One full system/Jacobian evaluation per **outer** iteration, at
//!    accepted points only.
//! 2. An **inner** trial loop solves the damped system
//!    \\( (A + \mu D^2)\,\mathbf{h} = \mathbf{g} \\) and evaluates a
//!    cheap trial cost at \\( \mathbf{x} + \mathbf{h} \\). A trial that
//!    fails the gain-ratio acceptance test is **rejected**: the iterate
//!    stays put, \\( \mu \\) is raised, and the system is re-solved with
//!    the *same* Jacobian.
//! 3. The damping parameter follows Nielsen's rule, driven by the gain
//!    ratio \\( \rho = (\Phi - \Phi_t) / \mathrm{pred} \\) — the ratio
//!    of actual to model-predicted cost reduction.
//!
//! State (the iterate, the held system, problem-side commits) moves
//! **only on acceptance**, so the returned solution — cost, covariance,
//! and problem diagnostics — always corresponds to the returned
//! \\( \mathbf{x} \\), and the cost is monotone non-increasing across
//! accepted iterates.
//!
//! The solver is deterministic by construction: fixed-order scalar
//! `f64` arithmetic only (`+`, `-`, `*`, `/`, `sqrt`, comparisons), no
//! libm transcendentals, no randomness, no threading. Identical inputs
//! produce identical bits on every IEEE 754 platform.
//!
//! Two entry points share one driver core:
//!
//! - [`solve`] — residual-level problems ([`ResidualProblem`]): the
//!   driver assembles the normal equations from residual/Jacobian rows
//!   and owns the optional prior.
//! - [`solve_system`] — normal-equations-level problems
//!   ([`SystemProblem`]): the problem hands the driver an assembled
//!   \\( (\Phi, A, \mathbf{g}) \\) triple per evaluation (e.g. a
//!   Schur-reduced system); the problem owns its complete objective,
//!   priors included.
//!
//! # References
//!
//! - Madsen, K., Nielsen, H.B. & Tingleff, O. (2004), *Methods for
//!   Non-Linear Least Squares Problems*, 2nd ed., IMM/DTU — Algorithm
//!   3.16, eqs. 2.18, 2.21, 3.14-3.15.
//! - Nielsen, H.B. (1999), *Damping Parameter in Marquardt's Method*,
//!   IMM-REP-1999-05, DTU.
//! - Moré, J.J. (1978), "The Levenberg-Marquardt Algorithm:
//!   Implementation and Theory", *Numerical Analysis*, LNM 630 —
//!   running-max column-norm scaling, best-retained-point invariant.
//! - Moré, Garbow & Hillstrom (1980), *User Guide for MINPACK-1*,
//!   ANL-80-74 — one-Jacobian-per-outer-iteration structure,
//!   ftol/xtol/gtol termination, acceptance threshold 1e-4.
//! - Marquardt, D.W. (1963), SIAM J. Appl. Math. 11(2);
//!   Levenberg, K. (1944), Q. Appl. Math. 2.
//!
//! # Example
//!
//! ```
//! use hyperjet::optimization::lm::{solve_nlls, LMConfig, NLLSEvaluation};
//!
//! #[derive(Debug)]
//! struct NoError;
//! impl std::fmt::Display for NoError {
//!     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//!         write!(f, "no error")
//!     }
//! }
//! impl std::error::Error for NoError {}
//!
//! // Fit y = a*x + b to (0,1), (1,3), (2,5), (3,7)  =>  a = 2, b = 1.
//! let xs = [0.0_f64, 1.0, 2.0, 3.0];
//! let ys = [1.0_f64, 3.0, 5.0, 7.0];
//! let residual = |p: &[f64; 2], x: f64, y: f64| p[0] * x + p[1] - y;
//!
//! let solution = solve_nlls(
//!     |p: &[f64; 2]| {
//!         let residuals: Vec<f64> =
//!             xs.iter().zip(&ys).map(|(&x, &y)| residual(p, x, y)).collect();
//!         let jacobian: Vec<[f64; 2]> = xs.iter().map(|&x| [x, 1.0]).collect();
//!         let cost = residuals.iter().map(|r| r * r).sum();
//!         Ok::<_, NoError>(NLLSEvaluation { residuals, jacobian, cost })
//!     },
//!     |p: &[f64; 2]| {
//!         Ok::<_, NoError>(
//!             xs.iter()
//!                 .zip(&ys)
//!                 .map(|(&x, &y)| {
//!                     let r = residual(p, x, y);
//!                     r * r
//!                 })
//!                 .sum(),
//!         )
//!     },
//!     [0.0; 2],
//!     &LMConfig::default(),
//!     None,
//! )
//! .unwrap();
//!
//! assert!(solution.converged);
//! assert!((solution.x[0] - 2.0).abs() < 1e-8);
//! assert!((solution.x[1] - 1.0).abs() < 1e-8);
//! ```

// Fixed-order indexed loops are deliberate throughout this module:
// the accumulation ORDER is part of the bit-determinism contract, and
// the index-based form keeps that order explicit.
#![allow(clippy::needless_range_loop)]

use crate::linalg::generic::{mat_cholesky, mat_inv, mat_symmetrize};

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

// ── Tuning constants ────────────────────────────────────────────────

/// Reseed floor applied to \\(\mu\\) on rejection:
/// \\(\mu \leftarrow \max(\mu, \mu_{\text{seed}}) \cdot \nu\\).
/// Guarantees no value of \\(\mu\\) (including 0) is an absorbing
/// state under the multiplicative escalation.
const MU_SEED: f64 = 1e-12;

/// Relative floor on the Moré scaling diagonal:
/// \\(d_j \ge 10^{-12} \max_k d_k\\). Relative (not absolute) so the
/// floor is invariant under a global rescaling of the parameters —
/// the parameter vector mixes units spanning many orders of magnitude.
const D_FLOOR_REL: f64 = 1e-12;

// ── Configuration ───────────────────────────────────────────────────

/// Solver configuration.
///
/// Construct via [`Default`] and mutate fields; the struct is
/// `#[non_exhaustive]`, so struct-literal construction is reserved to
/// this crate and new knobs can be added without breaking consumers.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct LMConfig {
    /// Maximum **outer** iterations (each holds one system/Jacobian
    /// snapshot; full evaluations = 1 initial + accepted steps +
    /// rolled-back acceptances). Must be ≥ 1.
    pub max_iterations: usize,
    /// Trial budget per outer iteration (rejections re-use the same
    /// system). Must be ≥ 1.
    pub max_inner_trials: usize,
    /// Initial damping scale:
    /// \\(\mu_0 = \tau \cdot \max_j (A_{jj} / d_j^2)\\)
    /// (Nielsen eq. 3.14, GSL's scaled variant). Must be finite
    /// and > 0. 1e-3 is the general default; use ~1e-6 for warm
    /// restarts near a converged fit and ~1.0 for poor seeds —
    /// although the accept/reject loop makes the solver insensitive
    /// to this choice (see the τ-insensitivity test).
    pub tau: f64,
    /// Damping cap: exceeding it while rejecting terminates with
    /// [`TerminationReason::DampingExhausted`]. Must be finite and > 0.
    pub mu_max: f64,
    /// Acceptance threshold \\(\eta\\): accept iff
    /// \\(\Phi - \Phi_t > \eta \cdot \mathrm{pred}\\) (MINPACK's 1e-4).
    /// Must be finite and ≥ 0.
    pub min_relative_decrease: f64,
    /// Scaled-gradient tolerance (MINPACK cosine form): converge when
    /// \\(|g_j| \le \texttt{gtol} \cdot d_j \sqrt{\Phi}\\) for all
    /// \\(j\\). 0 disables. Must be finite and ≥ 0.
    pub gtol: f64,
    /// Quadratic-form step tolerance (Milani \\(\lVert\cdot\rVert_C\\)
    /// convention): converge when
    /// \\(\mathbf{h}^\top A \mathbf{h} \le \texttt{qtol}\\) under the
    /// undamped, prior-augmented \\(A\\) that produced the step.
    /// 0 disables (the default — opt in for differential-correction
    /// style consumers). Must be finite and ≥ 0.
    pub qtol: f64,
    /// Relative step tolerance (Nielsen eq. 3.15b): converge when
    /// \\(\lVert\mathbf{h}\rVert_D \le \texttt{xtol}\,(\lVert\mathbf{x}\rVert_D + \texttt{xtol})\\).
    /// Degenerate in delta-coordinate formulations where
    /// \\(\lVert\mathbf{x}\rVert \approx 0\\) — use `qtol` there.
    /// 0 disables. Must be finite and ≥ 0.
    pub xtol: f64,
    /// Relative cost-reduction tolerance (MINPACK `ftol` analogue):
    /// converge when both the actual and the predicted reduction of the
    /// last accepted step are ≤ `ftol`·Φ and the gain ratio of that
    /// step is ≤ 2. 0 disables. Must be finite and ≥ 0.
    pub ftol: f64,
    /// Consecutive invalid (`Err` or non-finite) trial costs before the
    /// solve fails with [`LMError::PersistentInvalidTrials`]. Must
    /// be ≥ 1.
    pub max_consecutive_invalid: usize,
    /// Geodesic acceleration (Transtrum & Sethna 2012; GSL
    /// `multifit_nlinear`): augment each trial step with the
    /// second-order correction \\(\mathbf{h} = \mathbf{v} +
    /// \tfrac{1}{2}\mathbf{a}\\), where \\((A + \mu D^2)\,
    /// \mathbf{a} = -J^\top \mathbf{r}''_{vv}\\) and
    /// \\(\mathbf{r}''_{vv}\\) is the problem-supplied directional
    /// second derivative of the residuals along the velocity step
    /// (see [`CostProblem::second_directional_derivative`]).
    /// Dramatically reduces iteration counts on curved-valley
    /// ("sloppy") cost surfaces. Default `false`; also inert while
    /// the problem returns `None` from the hook or on the
    /// normal-equations path (no Jacobian rows).
    pub geodesic_acceleration: bool,
    /// Acceleration acceptance guard (GSL `avmax`): the accelerated
    /// step is used only when \\(\lVert\mathbf{a}\rVert_D /
    /// \lVert\mathbf{v}\rVert_D \le \texttt{avmax}\\); beyond it
    /// the truncated expansion is untrustworthy and the trial is
    /// rejected through the normal μ escalation. Must be finite and
    /// > 0 (default 0.75, GSL's default).
    pub avmax: f64,
}

impl Default for LMConfig {
    fn default() -> Self {
        Self {
            max_iterations: 100,
            max_inner_trials: 30,
            tau: 1e-3,
            mu_max: 1e32,
            min_relative_decrease: 1e-4,
            gtol: 1e-8,
            qtol: 0.0,
            xtol: 1e-8,
            ftol: 1.49e-8,
            max_consecutive_invalid: 5,
            geodesic_acceleration: false,
            avmax: 0.75,
        }
    }
}

// ── Problem traits ──────────────────────────────────────────────────

/// Trial-cost and step-lifecycle interface shared by both problem
/// levels.
pub trait CostProblem<const N: usize> {
    /// Domain error type, propagated through [`LMError`] without
    /// flattening.
    type Error: std::error::Error + 'static;

    /// Objective value at a **trial** point.
    ///
    /// Contract: the same objective, over the same fixed observation
    /// set, as the full evaluation — and no commits to persistent
    /// state (stage into pending slots; commit in
    /// [`on_step_accepted`](Self::on_step_accepted)).
    ///
    /// `Err` or a non-finite value rejects the trial through the
    /// normal damping escalation (bounded by
    /// [`LMConfig::max_consecutive_invalid`]); it is not fatal.
    fn evaluate_cost(&mut self, x: &[f64; N]) -> Result<f64, Self::Error>;

    /// Optional problem-driven clamp on a proposed step.
    ///
    /// The driver computes the predicted reduction from the **actual**
    /// (possibly clamped) step, so the clamp stays visible to the gain
    /// ratio; a clamped step can never declare convergence by itself.
    fn constrain_step(&mut self, _x: &[f64; N], _delta: &mut [f64; N]) {}

    /// Commit hook: persistent state (caches, baselines, diagnostics)
    /// moves ONLY here. Called once for the validated initial
    /// evaluation at \\(\mathbf{x}_0\\) and then after each full
    /// evaluation at an accepted point has succeeded, so committed
    /// state always corresponds to a retained iterate — including on
    /// zero-acceptance exits. Also called after the re-assembly a
    /// [`refresh_model`](Self::refresh_model) declaration triggers,
    /// which lands on the CURRENT accepted point rather than a new one.
    fn on_step_accepted(&mut self, _x: &[f64; N]) {}

    /// Discard hook: drop pending (staged) state from a rejected trial
    /// evaluation. Diagnostics and counters only — nothing committed
    /// needs rolling back.
    fn on_step_rejected(&mut self, _x_trial: &[f64; N]) {}

    /// Model-refresh hook for problems that evaluate an **inexact**
    /// model — a surrogate anchored at a reference point, a cached
    /// linearization, a reduced-fidelity dynamics. Called once per
    /// outer iteration at the current accepted point, after that
    /// iteration's convergence tests and before its first trial.
    ///
    /// This is the ONLY point at which such a problem may change the
    /// model its evaluations describe. Within one iteration
    /// \\(\Phi(\mathbf{x})\\), \\(A\\), \\(\mathbf{g}\\) and every
    /// trial \\(\Phi(\mathbf{x}+\mathbf{h})\\) — including the ones
    /// spent escalating \\(\mu\\) — must come from ONE objective, or
    /// the gain ratio measures the difference between two different
    /// functions and \\(A\\) is not the Gauss-Newton Hessian of the
    /// cost being compared. Switching models *between* iterations is
    /// ordinary inexact-model refresh and is fully supported here.
    ///
    /// Return `true` to declare that the model just changed. The
    /// driver then re-assembles at `x` under the new model, commits
    /// the result via [`on_step_accepted`](Self::on_step_accepted),
    /// and only then enters the trial loop — so the system it damps
    /// and the trials it compares against are the same objective
    /// again. Note that the objective's *value* may jump across a
    /// refresh: cost monotonicity holds per model, not across one.
    ///
    /// Return `false` (the default) to keep the current system; no
    /// evaluation is spent.
    fn refresh_model(&mut self, _x: &[f64; N]) -> bool {
        false
    }

    /// Directional second derivative of the pre-weighted residuals
    /// along `v` at `x`:
    /// \\(\mathbf{r}''_{vv} = d^2\mathbf{r}_w(\mathbf{x} +
    /// t\,\mathbf{v})/dt^2\big|_{t=0}\\), in the SAME row order as
    /// the most recent full evaluation at `x`. Powers geodesic
    /// acceleration ([`LMConfig::geodesic_acceleration`]); a single
    /// one-parameter second-order jet evaluation along `v` suffices —
    /// the full Hessian is never needed.
    ///
    /// `None` (the default) disables acceleration for this step. Must
    /// not commit persistent state (same contract as
    /// [`evaluate_cost`](Self::evaluate_cost)).
    fn second_directional_derivative(&mut self, _x: &[f64; N], _v: &[f64; N]) -> Option<Vec<f64>> {
        None
    }
}

/// Residual-level problem: the driver assembles
/// \\(A = J_w^\top J_w\\), \\(\mathbf{g} = -J_w^\top \mathbf{r}_w\\)
/// from pre-weighted rows and owns the optional prior.
pub trait ResidualProblem<const N: usize>: CostProblem<N> {
    /// Full evaluation (residuals + Jacobian + cost).
    ///
    /// Called at \\(\mathbf{x}_0\\) and at provisionally accepted
    /// points; committed state moves only after it succeeds (a failing
    /// or cost-inconsistent evaluation rolls the acceptance back). The
    /// final COMMITTED evaluation — the one followed by
    /// [`on_step_accepted`](CostProblem::on_step_accepted) —
    /// corresponds to the returned \\(\mathbf{x}\\); a later
    /// `evaluate` call may return `Ok` at a trial point whose
    /// acceptance was then rolled back (`on_step_rejected` fires for
    /// it).
    fn evaluate(&mut self, x: &[f64; N]) -> Result<NLLSEvaluation<N>, Self::Error>;
}

/// Normal-equations-level evaluation: cost, normal matrix, and
/// right-hand side describing **one** objective.
#[derive(Clone, Debug)]
pub struct SystemEvaluation<const N: usize> {
    /// Objective value \\(\Phi\\). Must include problem-composed
    /// priors and, for reduced systems (Schur), be the **profiled**
    /// cost \\(\min_b F(\mathbf{x}, b)\\).
    pub cost: f64,
    /// \\(A\\): symmetric positive semi-definite; the Gauss-Newton
    /// Hessian (×½) of the same objective as `cost`.
    pub normal: [[f64; N]; N],
    /// \\(\mathbf{g}\\): the negated gradient (×½) of the same
    /// objective, i.e. the right-hand side of
    /// \\(A\,\mathbf{h} = \mathbf{g}\\).
    pub rhs: [f64; N],
}

/// Normal-equations-level problem (e.g. a Schur-reduced system).
///
/// Validated on receipt: all entries finite and
/// \\(A_{jj} \ge 0\\), else the solve fails on the
/// [`LMError::InvalidSystem`] axis — a buggy reduction must surface
/// as what it is, not get masked into `DampingExhausted` through a
/// NaN-poisoned scaling diagonal.
pub trait SystemProblem<const N: usize>: CostProblem<N> {
    /// Full evaluation at \\(\mathbf{x}_0\\) and at provisionally
    /// accepted points; same lifecycle contract as
    /// [`ResidualProblem::evaluate`].
    fn evaluate_system(&mut self, x: &[f64; N]) -> Result<SystemEvaluation<N>, Self::Error>;
}

// ── Output ──────────────────────────────────────────────────────────

/// Why the solver stopped.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum TerminationReason {
    /// Scaled gradient below `gtol` at an accepted point
    /// (\\(|g_j| \le \texttt{gtol}\, d_j \sqrt{\Phi}\\) for all j).
    GradientTolerance,
    /// The UNDAMPED Gauss-Newton step at the accepted point fell
    /// below `qtol`/`xtol`; \\(\mathbf{x}\\) unchanged. Measured on
    /// the undamped step deliberately: a μ-shrunken step says nothing
    /// about stationarity.
    StepTolerance,
    /// Actual and predicted reduction of the last accepted step both
    /// below `ftol`·Φ with a consistent (ρ ≤ 2) model.
    CostTolerance,
    /// Outer iteration budget exhausted (`converged = false`).
    MaxIterations,
    /// \\(\mu\\) exceeded `mu_max` while rejecting
    /// (`converged = false`).
    DampingExhausted {
        /// The damping value that exceeded the cap.
        mu: f64,
    },
    /// Inner trial budget exhausted without an acceptance
    /// (`converged = false`).
    InnerTrialsExhausted {
        /// Trials consumed in the exhausted iteration.
        trials: usize,
    },
}

/// Why the covariance could not be produced. The solution (`x`, cost,
/// diagnostics) is still returned — a rank-deficient but converged fit
/// keeps its state; it just has no finite σ in the unobservable
/// subspace.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum CovarianceFailure {
    /// The (prior-augmented) normal matrix at the returned point is
    /// singular to working precision.
    SingularNormalMatrix,
}

/// Solver output.
#[derive(Clone, Debug)]
pub struct LMSolution<const N: usize> {
    /// The retained iterate (best visited: cost is monotone
    /// non-increasing across accepted steps *of one model* — a
    /// [`refresh_model`](CostProblem::refresh_model) declaration
    /// re-measures the objective, so `cost` may rise across a refresh).
    pub x: [f64; N],
    /// \\((A_{\text{final}})^{-1}\\) at the returned `x`
    /// (data + driver prior). Inversion failure is carried explicitly —
    /// never fabricated as zeros, never discards the solution.
    pub covariance: Result<[[f64; N]; N], CovarianceFailure>,
    /// Full objective \\(\Phi\\) at the returned `x`, from the final
    /// full evaluation (data + prior penalty).
    pub cost: f64,
    /// `cost` minus the driver prior penalty (equals `cost` on the
    /// system path, where the problem owns its priors).
    pub data_cost: f64,
    /// Quadratic form \\(\mathbf{h}^\top A\,\mathbf{h}\\) of the last
    /// **accepted** step under the undamped, prior-augmented \\(A\\)
    /// that produced it — which, after a
    /// [`refresh_model`](CostProblem::refresh_model), is not the
    /// \\(A\\) of the returned system. `None` only when no step was
    /// ever accepted; a refresh does NOT reset it, so `None` always
    /// means what it says.
    ///
    /// A DIAGNOSTIC, not a convergence metric: the accepted step is
    /// μ-damped and possibly clamped, so it is not the quantity any
    /// termination test is decided on and is not comparable to
    /// `qtol`/`xtol`/`ftol` in either direction. Quote
    /// [`final_gn_qnorm`](Self::final_gn_qnorm) when reporting against
    /// a tolerance.
    pub accepted_step_qnorm: Option<f64>,
    /// Quadratic form \\(\mathbf{h}_{\text{GN}}^\top
    /// A\,\mathbf{h}_{\text{GN}}\\) of the **undamped** Gauss-Newton
    /// step at the returned `x` — the quantity the
    /// [`qtol`](LMConfig::qtol) step-convergence test is decided on,
    /// and therefore the only step norm comparable to that tolerance.
    /// `None` when the undamped system is singular at the returned
    /// point, where the test is skipped as well.
    pub final_gn_qnorm: Option<f64>,
    /// \\(\max_j |g_j| / d_j\\) at the returned `x` (diagnostic; the
    /// gradient *test* additionally normalizes by \\(\sqrt{\Phi}\\)).
    pub gradient_norm_scaled: f64,
    /// Outer iterations performed (each holds one accepted system
    /// snapshot; see [`LMConfig::max_iterations`] for the
    /// full-evaluation accounting).
    pub iterations: usize,
    /// Trial-cost evaluations performed.
    pub n_cost_evals: usize,
    /// Trials rejected by the gain-ratio test (finite costs) or by a
    /// failed damped-system factorization (μ raised, no cost
    /// evaluated).
    pub n_rejected_trials: usize,
    /// Trials rejected for `Err`/non-finite costs (including rolled-back
    /// acceptances).
    pub n_invalid_trials: usize,
    /// Trials whose step carried the geodesic-acceleration correction.
    pub n_accelerated_trials: usize,
    /// Final damping parameter.
    pub mu_final: f64,
    /// Whether a convergence criterion was met.
    pub converged: bool,
    /// The criterion met, or the budget that ran out.
    pub reason: TerminationReason,
}

// ── Errors ──────────────────────────────────────────────────────────

/// A configuration field violation.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum ConfigDefect {
    /// `max_iterations` must be ≥ 1.
    MaxIterationsZero,
    /// `max_inner_trials` must be ≥ 1.
    MaxInnerTrialsZero,
    /// `tau` must be finite and > 0.
    TauNotPositive {
        /// Offending value.
        value: f64,
    },
    /// `mu_max` must be finite and > 0.
    MuMaxNotPositive {
        /// Offending value.
        value: f64,
    },
    /// `tau` must not exceed `mu_max`: \\(\mu_0 \le \tau\\) by
    /// construction, so this guarantees the solve never starts above
    /// the damping cap.
    TauExceedsMuMax {
        /// Configured initial damping scale.
        tau: f64,
        /// Configured damping cap.
        mu_max: f64,
    },
    /// `min_relative_decrease` must be finite and ≥ 0.
    MinRelativeDecreaseNegative {
        /// Offending value.
        value: f64,
    },
    /// `gtol` must be finite and ≥ 0.
    GtolNegative {
        /// Offending value.
        value: f64,
    },
    /// `qtol` must be finite and ≥ 0.
    QtolNegative {
        /// Offending value.
        value: f64,
    },
    /// `xtol` must be finite and ≥ 0.
    XtolNegative {
        /// Offending value.
        value: f64,
    },
    /// `ftol` must be finite and ≥ 0.
    FtolNegative {
        /// Offending value.
        value: f64,
    },
    /// `max_consecutive_invalid` must be ≥ 1.
    MaxConsecutiveInvalidZero,
    /// `avmax` must be finite and > 0.
    AvmaxNotPositive {
        /// Offending value.
        value: f64,
    },
}

/// A prior violation.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum PriorDefect {
    /// Non-finite prior mean component.
    NonFiniteMean {
        /// Component index.
        index: usize,
    },
    /// Non-finite inverse-covariance entry.
    NonFiniteCovarianceInv {
        /// Row index.
        row: usize,
        /// Column index.
        col: usize,
    },
    /// Negative inverse-covariance diagonal (not a valid precision
    /// matrix).
    NegativeDiagonal {
        /// Diagonal index.
        index: usize,
        /// Offending value.
        value: f64,
    },
}

/// An invalid value in a full residual-level evaluation.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum EvaluationDefect {
    /// `cost` was NaN or infinite.
    Cost,
    /// `cost` was negative — impossible for a sum of squares (plus a
    /// PSD prior penalty). Left unvalidated, a negative cost would
    /// NaN-poison \\(\sqrt{\Phi}\\) in the gradient test and read as
    /// instant false convergence.
    NegativeCost {
        /// Offending value.
        value: f64,
    },
    /// A residual entry was NaN or infinite.
    Residual {
        /// Row index.
        index: usize,
    },
    /// A Jacobian entry was NaN or infinite.
    JacobianEntry {
        /// Row index.
        row: usize,
        /// Column index.
        col: usize,
    },
}

/// An invalid [`SystemEvaluation`].
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum SystemDefect {
    /// `cost` was NaN or infinite.
    NonFiniteCost,
    /// A normal-matrix entry was NaN or infinite.
    NonFiniteNormal {
        /// Row index.
        row: usize,
        /// Column index.
        col: usize,
    },
    /// A right-hand-side entry was NaN or infinite.
    NonFiniteRhs {
        /// Component index.
        index: usize,
    },
    /// `cost` was negative — impossible for a real (profiled) least-
    /// squares objective. A Schur consumer whose profiled cost rounds
    /// to a tiny negative through catastrophic cancellation must
    /// handle that deliberately on its side before returning; the
    /// driver never silently clamps.
    NegativeCost {
        /// Offending value.
        value: f64,
    },
    /// A negative normal-matrix diagonal entry — algebraically
    /// impossible for a real Gauss-Newton/Schur reduction; a definite
    /// bug in the problem.
    NegativeDiagonal {
        /// Diagonal index.
        index: usize,
        /// Offending value.
        value: f64,
    },
}

/// Solver error. Variants are split by failure axis; the problem's
/// domain error type `E` is carried through without flattening.
#[derive(Debug)]
#[non_exhaustive]
pub enum LMError<E> {
    /// A configuration field is invalid.
    InvalidConfig {
        /// Which field, and how.
        defect: ConfigDefect,
    },
    /// The supplied prior is invalid.
    InvalidPrior {
        /// Which part, and how.
        defect: PriorDefect,
    },
    /// A [`SystemEvaluation`] failed validation at the initial point
    /// (later structurally-invalid systems with a *negative diagonal*
    /// are also fatal; non-finite systems at provisionally accepted
    /// points roll back instead).
    InvalidSystem {
        /// Outer iteration (0 = initial evaluation).
        iteration: usize,
        /// What was invalid.
        defect: SystemDefect,
    },
    /// A full evaluation returned zero residual rows.
    EmptyResiduals {
        /// Outer iteration (0 = initial evaluation).
        iteration: usize,
    },
    /// Residual and Jacobian row counts disagree.
    DimensionMismatch {
        /// Outer iteration (0 = initial evaluation).
        iteration: usize,
        /// Residual rows returned.
        residuals: usize,
        /// Jacobian rows returned.
        jacobian: usize,
    },
    /// The scaling diagonal is identically zero (zero Jacobian, no
    /// prior): multiplicative damping can never de-singularize it, so
    /// escalating \\(\mu\\) is provably futile.
    ZeroDampingDiagonal {
        /// Outer iteration (0 = initial evaluation).
        iteration: usize,
    },
    /// The problem failed at \\(\mathbf{x}_0\\) — there is no valid
    /// reference point to retreat to.
    InitialEvaluationFailed {
        /// The domain error.
        source: E,
    },
    /// The re-assembly that a
    /// [`refresh_model`](CostProblem::refresh_model) declaration
    /// requires failed at the accepted point. There is no retreat: the
    /// problem has already moved to the new model, so the system the
    /// driver still holds describes an objective the problem no longer
    /// evaluates.
    ModelRefreshFailed {
        /// Outer iteration whose refresh failed.
        iteration: usize,
        /// The domain error.
        source: E,
    },
    /// Non-finite value in a full evaluation the driver cannot retreat
    /// from: the **initial** one, or the re-assembly a
    /// [`refresh_model`](CostProblem::refresh_model) declaration
    /// requires (there the problem has already switched models, so
    /// there is no earlier system left to fall back to). A non-finite
    /// TRIAL evaluation rolls the acceptance back instead of surfacing
    /// here.
    InvalidEvaluation {
        /// Outer iteration (0 = initial evaluation).
        iteration: usize,
        /// What was non-finite.
        defect: EvaluationDefect,
    },
    /// `max_consecutive_invalid` consecutive trial evaluations failed
    /// (`Err`, non-finite, or rolled-back acceptances). The best
    /// visited (= current accepted) state is carried for triage.
    PersistentInvalidTrials {
        /// Outer iteration of the last failure.
        iteration: usize,
        /// Consecutive failures observed.
        consecutive: usize,
        /// The last domain error, if the failures carried one.
        last_source: Option<E>,
        /// Best (= last accepted) iterate.
        best_x: Vec<f64>,
        /// Objective at `best_x`.
        best_cost: f64,
    },
}

impl<E: std::fmt::Display> std::fmt::Display for LMError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidConfig { defect } => write!(f, "invalid configuration: {defect:?}"),
            Self::InvalidPrior { defect } => write!(f, "invalid prior: {defect:?}"),
            Self::InvalidSystem { iteration, defect } => {
                write!(
                    f,
                    "invalid system evaluation at iteration {iteration}: {defect:?}"
                )
            }
            Self::EmptyResiduals { iteration } => {
                write!(
                    f,
                    "evaluation returned no residuals at iteration {iteration}"
                )
            }
            Self::DimensionMismatch {
                iteration,
                residuals,
                jacobian,
            } => write!(
                f,
                "dimension mismatch at iteration {iteration}: {residuals} residuals vs \
                 {jacobian} Jacobian rows"
            ),
            Self::ZeroDampingDiagonal { iteration } => write!(
                f,
                "scaling diagonal identically zero at iteration {iteration} (zero Jacobian, \
                 no prior): damping cannot regularize this system"
            ),
            Self::InitialEvaluationFailed { source } => {
                write!(f, "initial evaluation failed: {source}")
            }
            Self::ModelRefreshFailed { iteration, source } => write!(
                f,
                "re-assembly after a model refresh failed at iteration {iteration}: {source}"
            ),
            Self::InvalidEvaluation { iteration, defect } => {
                write!(f, "invalid evaluation at iteration {iteration}: {defect:?}")
            }
            Self::PersistentInvalidTrials {
                iteration,
                consecutive,
                best_cost,
                ..
            } => write!(
                f,
                "{consecutive} consecutive invalid trial evaluations at iteration {iteration} \
                 (best retained cost {best_cost})"
            ),
        }
    }
}

impl<E: std::error::Error + 'static> std::error::Error for LMError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InitialEvaluationFailed { source } => Some(source),
            Self::ModelRefreshFailed { source, .. } => Some(source),
            Self::PersistentInvalidTrials {
                last_source: Some(source),
                ..
            } => Some(source),
            _ => None,
        }
    }
}

// ── Internal: assembled system + sources ────────────────────────────

/// One full evaluation, reduced to the common driver currency.
struct AssembledSystem<const N: usize> {
    /// Full objective Φ (prior penalty included on the residual path).
    cost: f64,
    /// Objective minus the driver prior penalty.
    data_cost: f64,
    /// Symmetrized, prior-augmented normal matrix A.
    normal: [[f64; N]; N],
    /// Prior-augmented right-hand side g (so that A h = g).
    rhs: [f64; N],
    /// Pre-weighted Jacobian rows (residual path only) — needed for
    /// the geodesic-acceleration product \\(J^\top \mathbf{r}''_{vv}\\).
    /// `None` on the normal-equations path, which therefore cannot
    /// use acceleration.
    jacobian: Option<Vec<[f64; N]>>,
}

/// How a full evaluation failed.
enum AssembleFailure<E> {
    /// Problem-domain error: fatal at x0, rolls back elsewhere.
    Domain(E),
    /// Non-finite residual-path value: fatal at x0, rolls back
    /// elsewhere.
    NonFinite(EvaluationDefect),
    /// Non-finite system-path value: fatal at x0, rolls back elsewhere.
    SystemNonFinite(SystemDefect),
    /// Contract violation (empty/mismatched rows, negative diagonal):
    /// always fatal.
    Hard(HardDefect),
}

enum HardDefect {
    EmptyResiduals,
    DimensionMismatch { residuals: usize, jacobian: usize },
    NegativeDiagonal { index: usize, value: f64 },
}

impl HardDefect {
    fn into_error<E>(self, iteration: usize) -> LMError<E> {
        match self {
            Self::EmptyResiduals => LMError::EmptyResiduals { iteration },
            Self::DimensionMismatch {
                residuals,
                jacobian,
            } => LMError::DimensionMismatch {
                iteration,
                residuals,
                jacobian,
            },
            Self::NegativeDiagonal { index, value } => LMError::InvalidSystem {
                iteration,
                defect: SystemDefect::NegativeDiagonal { index, value },
            },
        }
    }
}

/// Adapter unifying the residual and system paths for the driver core.
trait SystemSource<const N: usize> {
    type Error: std::error::Error + 'static;

    fn assemble(
        &mut self,
        x: &[f64; N],
    ) -> Result<AssembledSystem<N>, AssembleFailure<Self::Error>>;
    fn trial_cost(&mut self, x: &[f64; N]) -> Result<f64, Self::Error>;
    fn constrain(&mut self, x: &[f64; N], delta: &mut [f64; N]);
    fn accepted(&mut self, x: &[f64; N]);
    fn rejected(&mut self, x_trial: &[f64; N]);
    fn refresh_model(&mut self, x: &[f64; N]) -> bool;
    fn second_directional_derivative(&mut self, x: &[f64; N], v: &[f64; N]) -> Option<Vec<f64>>;
}

/// Residual-level adapter: assembles normal equations, owns the prior
/// (an exactly-symmetrized copy — see [`solve`]).
struct ResidualSource<'a, P, const N: usize> {
    problem: &'a mut P,
    prior: Option<NLLSPrior<N>>,
}

impl<'a, P: ResidualProblem<N>, const N: usize> ResidualSource<'a, P, N> {
    /// Prior penalty \\((\mathbf{x}-\mathbf{m})^\top P_0^{-1} (\mathbf{x}-\mathbf{m})\\),
    /// fixed evaluation order.
    fn prior_penalty(&self, x: &[f64; N]) -> f64 {
        let Some(p) = &self.prior else { return 0.0 };
        let mut penalty = 0.0_f64;
        for i in 0..N {
            let mut row = 0.0_f64;
            for j in 0..N {
                row += p.covariance_inv[i][j] * (x[j] - p.mean[j]);
            }
            penalty += (x[i] - p.mean[i]) * row;
        }
        penalty
    }
}

impl<'a, P: ResidualProblem<N>, const N: usize> SystemSource<N> for ResidualSource<'a, P, N> {
    type Error = P::Error;

    fn assemble(
        &mut self,
        x: &[f64; N],
    ) -> Result<AssembledSystem<N>, AssembleFailure<Self::Error>> {
        let eval = self.problem.evaluate(x).map_err(AssembleFailure::Domain)?;

        if eval.residuals.is_empty() {
            return Err(AssembleFailure::Hard(HardDefect::EmptyResiduals));
        }
        if eval.residuals.len() != eval.jacobian.len() {
            return Err(AssembleFailure::Hard(HardDefect::DimensionMismatch {
                residuals: eval.residuals.len(),
                jacobian: eval.jacobian.len(),
            }));
        }
        if !eval.cost.is_finite() {
            return Err(AssembleFailure::NonFinite(EvaluationDefect::Cost));
        }
        if eval.cost < 0.0 {
            return Err(AssembleFailure::NonFinite(EvaluationDefect::NegativeCost {
                value: eval.cost,
            }));
        }
        for (i, r) in eval.residuals.iter().enumerate() {
            if !r.is_finite() {
                return Err(AssembleFailure::NonFinite(EvaluationDefect::Residual {
                    index: i,
                }));
            }
        }
        for (i, row) in eval.jacobian.iter().enumerate() {
            for (j, v) in row.iter().enumerate() {
                if !v.is_finite() {
                    return Err(AssembleFailure::NonFinite(
                        EvaluationDefect::JacobianEntry { row: i, col: j },
                    ));
                }
            }
        }

        // Normal equations: A = JᵀJ, g = -Jᵀr (fixed accumulation order).
        let mut normal = [[0.0_f64; N]; N];
        let mut rhs = [0.0_f64; N];
        for (r_i, j_i) in eval.residuals.iter().zip(eval.jacobian.iter()) {
            for j in 0..N {
                for k in 0..N {
                    normal[j][k] += j_i[j] * j_i[k];
                }
                rhs[j] -= j_i[j] * r_i;
            }
        }
        let mut normal = mat_symmetrize(&normal);

        // Prior augmentation: A += P0⁻¹, g -= P0⁻¹(x − m). The prior
        // was symmetrized on receipt, so `normal` stays exactly
        // symmetric and the solved system agrees with the quadratic
        // model used for the gain ratio.
        if let Some(p) = &self.prior {
            for i in 0..N {
                for j in 0..N {
                    normal[i][j] += p.covariance_inv[i][j];
                }
                let mut delta_weighted = 0.0_f64;
                for j in 0..N {
                    delta_weighted += p.covariance_inv[i][j] * (x[j] - p.mean[j]);
                }
                rhs[i] -= delta_weighted;
            }
        }

        let data_cost = eval.cost;
        let cost = data_cost + self.prior_penalty(x);
        if !cost.is_finite() {
            return Err(AssembleFailure::NonFinite(EvaluationDefect::Cost));
        }
        if cost < 0.0 {
            return Err(AssembleFailure::NonFinite(EvaluationDefect::NegativeCost {
                value: cost,
            }));
        }

        Ok(AssembledSystem {
            cost,
            data_cost,
            normal,
            rhs,
            jacobian: Some(eval.jacobian),
        })
    }

    fn trial_cost(&mut self, x: &[f64; N]) -> Result<f64, Self::Error> {
        let data = self.problem.evaluate_cost(x)?;
        // A non-finite data cost propagates as non-finite total: the
        // driver rejects it (never substitutes).
        Ok(data + self.prior_penalty(x))
    }

    fn constrain(&mut self, x: &[f64; N], delta: &mut [f64; N]) {
        self.problem.constrain_step(x, delta);
    }

    fn accepted(&mut self, x: &[f64; N]) {
        self.problem.on_step_accepted(x);
    }

    fn rejected(&mut self, x_trial: &[f64; N]) {
        self.problem.on_step_rejected(x_trial);
    }

    fn refresh_model(&mut self, x: &[f64; N]) -> bool {
        self.problem.refresh_model(x)
    }

    fn second_directional_derivative(&mut self, x: &[f64; N], v: &[f64; N]) -> Option<Vec<f64>> {
        self.problem.second_directional_derivative(x, v)
    }
}

/// Normal-equations-level adapter: the problem owns its objective
/// (priors included); the driver only validates.
struct DirectSource<'a, P, const N: usize> {
    problem: &'a mut P,
}

impl<'a, P: SystemProblem<N>, const N: usize> SystemSource<N> for DirectSource<'a, P, N> {
    type Error = P::Error;

    fn assemble(
        &mut self,
        x: &[f64; N],
    ) -> Result<AssembledSystem<N>, AssembleFailure<Self::Error>> {
        let sys = self
            .problem
            .evaluate_system(x)
            .map_err(AssembleFailure::Domain)?;

        if !sys.cost.is_finite() {
            return Err(AssembleFailure::SystemNonFinite(
                SystemDefect::NonFiniteCost,
            ));
        }
        if sys.cost < 0.0 {
            return Err(AssembleFailure::SystemNonFinite(
                SystemDefect::NegativeCost { value: sys.cost },
            ));
        }
        for i in 0..N {
            for j in 0..N {
                if !sys.normal[i][j].is_finite() {
                    return Err(AssembleFailure::SystemNonFinite(
                        SystemDefect::NonFiniteNormal { row: i, col: j },
                    ));
                }
            }
            if !sys.rhs[i].is_finite() {
                return Err(AssembleFailure::SystemNonFinite(
                    SystemDefect::NonFiniteRhs { index: i },
                ));
            }
        }
        for i in 0..N {
            if sys.normal[i][i] < 0.0 {
                return Err(AssembleFailure::Hard(HardDefect::NegativeDiagonal {
                    index: i,
                    value: sys.normal[i][i],
                }));
            }
        }

        Ok(AssembledSystem {
            cost: sys.cost,
            data_cost: sys.cost,
            normal: mat_symmetrize(&sys.normal),
            rhs: sys.rhs,
            jacobian: None,
        })
    }

    fn trial_cost(&mut self, x: &[f64; N]) -> Result<f64, Self::Error> {
        self.problem.evaluate_cost(x)
    }

    fn constrain(&mut self, x: &[f64; N], delta: &mut [f64; N]) {
        self.problem.constrain_step(x, delta);
    }

    fn accepted(&mut self, x: &[f64; N]) {
        self.problem.on_step_accepted(x);
    }

    fn rejected(&mut self, x_trial: &[f64; N]) {
        self.problem.on_step_rejected(x_trial);
    }

    fn refresh_model(&mut self, x: &[f64; N]) -> bool {
        self.problem.refresh_model(x)
    }

    fn second_directional_derivative(&mut self, x: &[f64; N], v: &[f64; N]) -> Option<Vec<f64>> {
        self.problem.second_directional_derivative(x, v)
    }
}

// ── Public entry points ─────────────────────────────────────────────

/// Solve a residual-level nonlinear least-squares problem.
///
/// The optional `prior` is driver-owned: it augments the normal
/// equations **and** enters both compared costs, so the acceptance
/// test judges the same MAP objective the step minimizes.
pub fn solve<P: ResidualProblem<N>, const N: usize>(
    problem: &mut P,
    x0: [f64; N],
    config: &LMConfig,
    prior: Option<&NLLSPrior<N>>,
) -> Result<LMSolution<N>, LMError<P::Error>> {
    validate_config(config)?;
    let prior = match prior {
        Some(p) => {
            validate_prior(p)?;
            // Symmetrize the precision matrix once on receipt
            // (bitwise no-op for exactly symmetric input): the solved
            // system, the gain-ratio model, and the penalty must all
            // see the SAME quadratic form, and real priors derived
            // from an LU-based inversion are asymmetric at the ulp
            // level.
            Some(NLLSPrior {
                mean: p.mean,
                covariance_inv: mat_symmetrize(&p.covariance_inv),
            })
        }
        None => None,
    };
    let mut source = ResidualSource { problem, prior };
    solve_core(&mut source, x0, config)
}

/// Solve a normal-equations-level problem (e.g. Schur-reduced).
///
/// No prior argument: the problem owns its complete objective,
/// priors composed in.
pub fn solve_system<P: SystemProblem<N>, const N: usize>(
    problem: &mut P,
    x0: [f64; N],
    config: &LMConfig,
) -> Result<LMSolution<N>, LMError<P::Error>> {
    validate_config(config)?;
    let mut source = DirectSource { problem };
    solve_core(&mut source, x0, config)
}

/// Closure convenience for [`solve`]. Both closures are required —
/// a defaulted trial cost would be a hidden full-evaluation fallback.
pub fn solve_nlls<E, FEval, FCost, const N: usize>(
    eval: FEval,
    cost: FCost,
    x0: [f64; N],
    config: &LMConfig,
    prior: Option<&NLLSPrior<N>>,
) -> Result<LMSolution<N>, LMError<E>>
where
    E: std::error::Error + 'static,
    FEval: FnMut(&[f64; N]) -> Result<NLLSEvaluation<N>, E>,
    FCost: FnMut(&[f64; N]) -> Result<f64, E>,
{
    struct ClosureProblem<FEval, FCost> {
        eval: FEval,
        cost: FCost,
    }
    impl<E, FEval, FCost, const N: usize> CostProblem<N> for ClosureProblem<FEval, FCost>
    where
        E: std::error::Error + 'static,
        FEval: FnMut(&[f64; N]) -> Result<NLLSEvaluation<N>, E>,
        FCost: FnMut(&[f64; N]) -> Result<f64, E>,
    {
        type Error = E;
        fn evaluate_cost(&mut self, x: &[f64; N]) -> Result<f64, E> {
            (self.cost)(x)
        }
    }
    impl<E, FEval, FCost, const N: usize> ResidualProblem<N> for ClosureProblem<FEval, FCost>
    where
        E: std::error::Error + 'static,
        FEval: FnMut(&[f64; N]) -> Result<NLLSEvaluation<N>, E>,
        FCost: FnMut(&[f64; N]) -> Result<f64, E>,
    {
        fn evaluate(&mut self, x: &[f64; N]) -> Result<NLLSEvaluation<N>, E> {
            (self.eval)(x)
        }
    }
    let mut problem = ClosureProblem { eval, cost };
    solve(&mut problem, x0, config, prior)
}

// ── Validation ──────────────────────────────────────────────────────

fn validate_config<E>(config: &LMConfig) -> Result<(), LMError<E>> {
    let defect = if config.max_iterations == 0 {
        Some(ConfigDefect::MaxIterationsZero)
    } else if config.max_inner_trials == 0 {
        Some(ConfigDefect::MaxInnerTrialsZero)
    } else if !(config.tau.is_finite() && config.tau > 0.0) {
        Some(ConfigDefect::TauNotPositive { value: config.tau })
    } else if !(config.mu_max.is_finite() && config.mu_max > 0.0) {
        Some(ConfigDefect::MuMaxNotPositive {
            value: config.mu_max,
        })
    } else if config.tau > config.mu_max {
        // μ₀ ≤ τ by construction (iteration-0 scaling), so τ ≤ mu_max
        // guarantees the solve never STARTS above the damping cap.
        Some(ConfigDefect::TauExceedsMuMax {
            tau: config.tau,
            mu_max: config.mu_max,
        })
    } else if !(config.min_relative_decrease.is_finite() && config.min_relative_decrease >= 0.0) {
        Some(ConfigDefect::MinRelativeDecreaseNegative {
            value: config.min_relative_decrease,
        })
    } else if !(config.gtol.is_finite() && config.gtol >= 0.0) {
        Some(ConfigDefect::GtolNegative { value: config.gtol })
    } else if !(config.qtol.is_finite() && config.qtol >= 0.0) {
        Some(ConfigDefect::QtolNegative { value: config.qtol })
    } else if !(config.xtol.is_finite() && config.xtol >= 0.0) {
        Some(ConfigDefect::XtolNegative { value: config.xtol })
    } else if !(config.ftol.is_finite() && config.ftol >= 0.0) {
        Some(ConfigDefect::FtolNegative { value: config.ftol })
    } else if config.max_consecutive_invalid == 0 {
        Some(ConfigDefect::MaxConsecutiveInvalidZero)
    } else if !(config.avmax.is_finite() && config.avmax > 0.0) {
        Some(ConfigDefect::AvmaxNotPositive {
            value: config.avmax,
        })
    } else {
        None
    };
    match defect {
        Some(defect) => Err(LMError::InvalidConfig { defect }),
        None => Ok(()),
    }
}

fn validate_prior<E, const N: usize>(prior: &NLLSPrior<N>) -> Result<(), LMError<E>> {
    for (i, m) in prior.mean.iter().enumerate() {
        if !m.is_finite() {
            return Err(LMError::InvalidPrior {
                defect: PriorDefect::NonFiniteMean { index: i },
            });
        }
    }
    for i in 0..N {
        for j in 0..N {
            if !prior.covariance_inv[i][j].is_finite() {
                return Err(LMError::InvalidPrior {
                    defect: PriorDefect::NonFiniteCovarianceInv { row: i, col: j },
                });
            }
        }
        if prior.covariance_inv[i][i] < 0.0 {
            return Err(LMError::InvalidPrior {
                defect: PriorDefect::NegativeDiagonal {
                    index: i,
                    value: prior.covariance_inv[i][i],
                },
            });
        }
    }
    Ok(())
}

// ── Numerics helpers (fixed-order, deterministic) ───────────────────

/// Update the Moré scaling diagonal in place:
/// \\(d_j \leftarrow \max(d_j, \sqrt{A_{jj}})\\) (running max, never
/// decreasing). The stored scales stay HONEST — no floor is baked in,
/// so genuinely small scales (e.g. ~1e-14 non-gravitational
/// parameters next to O(1) positions) keep their true magnitudes in
/// every norm and test. The floor is applied only at division sites
/// via [`effective_scale`]. Returns the maximum diagonal, which is 0
/// iff the system is identically unscalable.
fn update_scaling<const N: usize>(d: &mut [f64; N], normal: &[[f64; N]; N]) -> f64 {
    let mut d_max = 0.0_f64;
    for j in 0..N {
        let col = normal[j][j].sqrt();
        if col > d[j] {
            d[j] = col;
        }
        if d[j] > d_max {
            d_max = d[j];
        }
    }
    d_max
}

/// Scale used where a division by \\(d_j\\) must be protected from an
/// exactly-zero column: honest \\(d_j\\) when positive, else the
/// relative floor \\(10^{-12}\max_k d_k\\). The floor exists ONLY to
/// keep μD² damping and the equilibration well-defined on zero
/// columns; it never overrides an honest nonzero scale.
#[inline]
fn effective_scale(d_j: f64, d_max: f64) -> f64 {
    if d_j > 0.0 { d_j } else { D_FLOOR_REL * d_max }
}

/// The damped solves run in the **equilibrated basis**
/// \\((B + \mu I)\,\mathbf{h}' = D^{-1}\mathbf{g}\\),
/// \\(B = D^{-1} A D^{-1}\\) — algebraically identical to
/// \\((A + \mu D^2)\,\mathbf{h} = \mathbf{g}\\), but with an O(1)
/// diagonal even when the parameters mix units spanning many orders
/// of magnitude.
///
/// Cholesky factor of the equilibrated damped system
/// \\(B + \mu I = D^{-1} A D^{-1} + \mu I\\). One factorization
/// serves both the velocity and (geodesic) acceleration solves.
fn damped_factor<const N: usize>(
    normal: &[[f64; N]; N],
    d: &[f64; N],
    d_max: f64,
    mu: f64,
) -> Option<[[f64; N]; N]> {
    let mut b = [[0.0_f64; N]; N];
    for i in 0..N {
        let di = effective_scale(d[i], d_max);
        for j in 0..N {
            b[i][j] = normal[i][j] / (di * effective_scale(d[j], d_max));
            // mat_cholesky has no NaN guard; a subnormal-scale 0/0
            // corner must surface here as an ordinary solve failure.
            if !b[i][j].is_finite() {
                return None;
            }
        }
        b[i][i] += mu;
    }
    mat_cholesky(&b)
}

/// Solve with a previously computed [`damped_factor`]:
/// \\((A + \mu D^2)\,\mathbf{h} = \mathbf{g}\\) through the
/// equilibrated triangular solves, mapping back via
/// \\(\mathbf{h} = D^{-1}\mathbf{h}'\\).
fn solve_with_factor<const N: usize>(
    l: &[[f64; N]; N],
    rhs: &[f64; N],
    d: &[f64; N],
    d_max: f64,
) -> Option<[f64; N]> {
    let mut scaled_rhs = [0.0_f64; N];
    for i in 0..N {
        scaled_rhs[i] = rhs[i] / effective_scale(d[i], d_max);
    }

    // Forward solve L y = D⁻¹g.
    let mut y = [0.0_f64; N];
    for i in 0..N {
        let mut sum = scaled_rhs[i];
        for k in 0..i {
            sum -= l[i][k] * y[k];
        }
        y[i] = sum / l[i][i];
    }
    // Back solve Lᵀ h' = y.
    let mut hp = [0.0_f64; N];
    for i in (0..N).rev() {
        let mut sum = y[i];
        for k in (i + 1)..N {
            sum -= l[k][i] * hp[k];
        }
        hp[i] = sum / l[i][i];
    }

    let mut h = [0.0_f64; N];
    for i in 0..N {
        h[i] = hp[i] / effective_scale(d[i], d_max);
        if !h[i].is_finite() {
            return None;
        }
    }
    Some(h)
}

/// Solve \\((A + \mu D^2)\,\mathbf{h} = \mathbf{g}\\) —
/// factorization plus triangular solves. Returns `None` when the
/// factorization fails or the solution is non-finite; the driver
/// treats that as a trial rejection (raise \\(\mu\\), retry) —
/// never as a fallback to another solver.
fn solve_damped<const N: usize>(
    normal: &[[f64; N]; N],
    rhs: &[f64; N],
    d: &[f64; N],
    d_max: f64,
    mu: f64,
) -> Option<[f64; N]> {
    let l = damped_factor(normal, d, d_max, mu)?;
    solve_with_factor(&l, rhs, d, d_max)
}

/// Predicted reduction of the objective for the **actual** (possibly
/// clamped) step, from the general quadratic model:
/// \\(\mathrm{pred} = 2\,\mathbf{h}^\top\mathbf{g} - \mathbf{h}^\top A\,\mathbf{h}\\)
/// (the factor 2 reflects the unhalved-cost convention
/// \\(\Phi = \sum r^2\\)). Valid for arbitrary steps; the exact-step
/// shortcut \\(\mathbf{h}^\top(\mu D^2 \mathbf{h} + \mathbf{g})\\) is
/// not used because clamps invalidate it.
fn predicted_reduction<const N: usize>(
    h: &[f64; N],
    normal: &[[f64; N]; N],
    rhs: &[f64; N],
) -> f64 {
    let mut hg = 0.0_f64;
    let mut hah = 0.0_f64;
    for i in 0..N {
        hg += h[i] * rhs[i];
        let mut row = 0.0_f64;
        for j in 0..N {
            row += normal[i][j] * h[j];
        }
        hah += h[i] * row;
    }
    2.0 * hg - hah
}

/// Quadratic form \\(\mathbf{h}^\top A\,\mathbf{h}\\) (fixed order).
fn quadratic_form<const N: usize>(h: &[f64; N], normal: &[[f64; N]; N]) -> f64 {
    let mut q = 0.0_f64;
    for i in 0..N {
        let mut row = 0.0_f64;
        for j in 0..N {
            row += normal[i][j] * h[j];
        }
        q += h[i] * row;
    }
    q
}

/// \\(\lVert \mathbf{v} \rVert_D = \sqrt{\sum_j (d_j v_j)^2}\\).
fn scaled_norm<const N: usize>(v: &[f64; N], d: &[f64; N]) -> f64 {
    let mut s = 0.0_f64;
    for j in 0..N {
        let t = d[j] * v[j];
        s += t * t;
    }
    s.sqrt()
}

/// Covariance at the returned point: \\(A^{-1}\\) inverted through the
/// equilibrated \\(B = D^{-1} A D^{-1}\\) for conditioning
/// (\\(A^{-1} = D^{-1} B^{-1} D^{-1}\\)). Failure is explicit — never
/// fabricated zeros.
fn covariance<const N: usize>(
    normal: &[[f64; N]; N],
    d: &[f64; N],
    d_max: f64,
) -> Result<[[f64; N]; N], CovarianceFailure> {
    let mut b = [[0.0_f64; N]; N];
    for i in 0..N {
        let di = effective_scale(d[i], d_max);
        for j in 0..N {
            b[i][j] = normal[i][j] / (di * effective_scale(d[j], d_max));
        }
    }
    let b_inv = mat_inv(&b).ok_or(CovarianceFailure::SingularNormalMatrix)?;
    let mut cov = [[0.0_f64; N]; N];
    for i in 0..N {
        let di = effective_scale(d[i], d_max);
        for j in 0..N {
            cov[i][j] = b_inv[i][j] / (di * effective_scale(d[j], d_max));
        }
    }
    let cov = mat_symmetrize(&cov);
    for i in 0..N {
        for j in 0..N {
            if !cov[i][j].is_finite() {
                return Err(CovarianceFailure::SingularNormalMatrix);
            }
        }
    }
    Ok(cov)
}

/// Nielsen rejection escalation: \\(\mu \leftarrow \max(\mu,
/// \mu_{\text{seed}}) \cdot \nu\\), \\(\nu \leftarrow 2\nu\\). Returns
/// the termination reason when the escalated \\(\mu\\) exceeds the
/// budget (the caller breaks the outer loop); `None` means retry the
/// inner loop at the new damping.
#[inline]
fn escalate_mu(mu: &mut f64, nu: &mut f64, mu_max: f64) -> Option<TerminationReason> {
    if *mu < MU_SEED {
        *mu = MU_SEED;
    }
    *mu *= *nu;
    *nu *= 2.0;
    if *mu > mu_max {
        Some(TerminationReason::DampingExhausted { mu: *mu })
    } else {
        None
    }
}

/// Construct the [`LMError::PersistentInvalidTrials`] terminal error at
/// the best (last accepted) iterate. Shared by the trial-cost and
/// rollback paths so the diagnostic payload cannot drift between them.
fn persistent_invalid_trials<E, const N: usize>(
    iteration: usize,
    consecutive: usize,
    last_source: Option<E>,
    x: &[f64; N],
    best_cost: f64,
) -> LMError<E> {
    let mut best_x = Vec::with_capacity(N);
    best_x.extend_from_slice(x);
    LMError::PersistentInvalidTrials {
        iteration,
        consecutive,
        last_source,
        best_x,
        best_cost,
    }
}

// ── Driver core ─────────────────────────────────────────────────────

/// Bookkeeping for the most recent accepted step, consumed by the
/// **cost** convergence test at the next outer iteration.
///
/// Everything here is a cross-model quantity — it only means anything
/// against the system it was measured on — which is why a
/// [`CostProblem::refresh_model`] declaration clears the whole record.
/// The step's own SIZE is not stored here for exactly that reason: it
/// stays true across a refresh, and is tracked separately so that
/// clearing this does not erase the fact that a step was accepted.
struct AcceptedStep {
    /// Whether `constrain_step` modified the step (a clamped step can
    /// never declare convergence).
    clamped: bool,
    /// Φ before the step (full-evaluation value).
    prev_cost: f64,
    /// Predicted reduction of the step.
    pred: f64,
    /// Actual reduction measured between full evaluations.
    actred: f64,
}

/// The convergence battery at an accepted point, returning the reason
/// that fired (all of which mean `converged = true`) or `None` to keep
/// iterating.
///
/// Factored out because it runs from TWO places — once per outer
/// iteration on the system the driver has been minimizing, and again
/// immediately after a [`CostProblem::refresh_model`] re-assembly,
/// which produces a different objective at the same point. Keeping one
/// implementation is what stops the two call sites from drifting into
/// different notions of "converged".
///
/// `last_accepted` gates the cost test only: pass `None` whenever the
/// last accepted step was measured against a DIFFERENT system than
/// `sys` (i.e. after a refresh), because `actred`/`pred` are then
/// cross-model quantities and comparing them is exactly what the
/// refresh exists to prevent. The gradient and step tests read only
/// `sys`, `d` and `x`, so they are always meaningful.
fn convergence_reason<const N: usize>(
    config: &LMConfig,
    x: &[f64; N],
    sys: &AssembledSystem<N>,
    d: &[f64; N],
    d_max: f64,
    last_accepted: Option<&AcceptedStep>,
) -> Option<TerminationReason> {
    // Gradient (MINPACK cosine form, multiplicative to avoid
    // division): |g_j| ≤ gtol · √A_jj · √Φ for all j, using the
    // CURRENT column norms (MINPACK semantics — the running-max d
    // would loosen the test on collapsed-sensitivity columns). At
    // an exact fit (Φ = 0) the gradient is exactly zero and the
    // test passes; a zero column forces g_j = 0, which also
    // passes. Cost is validated ≥ 0 at assembly, and the
    // is_finite guard makes a NaN threshold unreachable as a
    // matter of defense in depth — a NaN must never read as
    // "pass".
    if config.gtol > 0.0 {
        let sqrt_cost = sys.cost.sqrt();
        let mut pass = sqrt_cost.is_finite();
        for j in 0..N {
            if sys.rhs[j].abs() > config.gtol * sys.normal[j][j].sqrt() * sqrt_cost {
                pass = false;
                break;
            }
        }
        if pass {
            return Some(TerminationReason::GradientTolerance);
        }
    }

    // Step- and cost-based convergence are judged against the
    // UNDAMPED Gauss-Newton step at the accepted point. A step
    // that is tiny only because μ crushed it says nothing about
    // stationarity: on stiff valleys the rejections inflate μ by
    // orders of magnitude, and any μ-shrunken accepted step would
    // read as "converged" at an arbitrarily bad iterate (observed
    // on the 2020 CD3 capture-spanning fit at χ² ~ 1e12). The
    // undamped step is also what the legacy solver's step test
    // effectively measured (λ ≈ 1e-6 at convergence).
    //
    // A singular undamped system simply skips these tests — the
    // gradient and budget criteria still terminate.
    let h_gn = solve_damped(&sys.normal, &sys.rhs, d, d_max, 0.0)?;
    let q_gn = quadratic_form(&h_gn, &sys.normal);
    let q_pass = config.qtol > 0.0 && q_gn <= config.qtol;
    let x_pass = config.xtol > 0.0
        && scaled_norm(&h_gn, d) <= config.xtol * (scaled_norm(x, d) + config.xtol);
    if q_pass || x_pass {
        return Some(TerminationReason::StepTolerance);
    }

    // Cost tolerance (MINPACK info=1 analogue): the actual AND
    // predicted reduction of the last accepted (unclamped)
    // step both ≤ ftol·Φ_prev with a consistent model
    // (ρ ≤ 2) — AND the undamped step's own predicted gain is
    // below the same threshold, so a μ-starved step cannot
    // manufacture a plateau (for the exact GN step the
    // predicted reduction is \(\mathbf{h}^T \mathbf{g}\),
    // but the general form is used for uniformity).
    if let Some(acc) = last_accepted
        && !acc.clamped
        && config.ftol > 0.0
        && acc.prev_cost > 0.0
    {
        let threshold = config.ftol * acc.prev_cost;
        let ratio_ok = acc.pred > 0.0 && acc.actred <= 2.0 * acc.pred;
        let gn_exhausted =
            predicted_reduction(&h_gn, &sys.normal, &sys.rhs) <= config.ftol * sys.cost;
        if acc.actred.abs() <= threshold && acc.pred <= threshold && ratio_ok && gn_exhausted {
            return Some(TerminationReason::CostTolerance);
        }
    }

    None
}

fn solve_core<S: SystemSource<N>, const N: usize>(
    source: &mut S,
    x0: [f64; N],
    config: &LMConfig,
) -> Result<LMSolution<N>, LMError<S::Error>> {
    let mut x = x0;

    // Initial full evaluation: the only point where failure is fatal
    // (there is no accepted state to retreat to).
    let mut sys = match source.assemble(&x) {
        Ok(s) => s,
        Err(AssembleFailure::Domain(e)) => {
            return Err(LMError::InitialEvaluationFailed { source: e });
        }
        Err(AssembleFailure::NonFinite(defect)) => {
            return Err(LMError::InvalidEvaluation {
                iteration: 0,
                defect,
            });
        }
        Err(AssembleFailure::SystemNonFinite(defect)) => {
            return Err(LMError::InvalidSystem {
                iteration: 0,
                defect,
            });
        }
        Err(AssembleFailure::Hard(h)) => return Err(h.into_error(0)),
    };

    let mut d = [0.0_f64; N];
    let mut d_max = update_scaling(&mut d, &sys.normal);
    if d_max == 0.0 {
        return Err(LMError::ZeroDampingDiagonal { iteration: 0 });
    }

    // The validated initial evaluation is committed: problem-side
    // pending state (caches, diagnostics) corresponds to x0 even on
    // zero-acceptance exits, so the final-state contract holds on
    // every path.
    source.accepted(&x);

    // μ₀ = τ · max_j(A_jj / d_j²)  (≈ τ under iteration-0 scaling;
    // ≤ τ always, since d_j ≥ √A_jj — paired with the τ ≤ mu_max
    // config validation this guarantees μ₀ ≤ mu_max).
    let mut mu = {
        let mut m = 0.0_f64;
        for j in 0..N {
            let dj = effective_scale(d[j], d_max);
            let s = sys.normal[j][j] / (dj * dj);
            if s > m {
                m = s;
            }
        }
        config.tau * m
    };
    let mut nu = 2.0_f64;

    let mut last_accepted: Option<AcceptedStep> = None;
    // Survives a model refresh; `last_accepted` does not (see the
    // acceptance block).
    let mut last_accepted_qnorm: Option<f64> = None;
    let mut consecutive_invalid = 0usize;
    let mut last_invalid_source: Option<S::Error> = None;

    let mut iterations = 0usize;
    let mut n_cost_evals = 0usize;
    let mut n_rejected_trials = 0usize;
    let mut n_invalid_trials = 0usize;
    let mut n_accelerated_trials = 0usize;

    let mut converged = false;
    let mut reason = TerminationReason::MaxIterations;

    'outer: for iteration in 1..=config.max_iterations {
        iterations = iteration;

        // ── Convergence tests at the accepted point ──
        // Judged on the system the driver has been minimizing, with
        // the last accepted step's reductions available to the cost
        // test (they were measured against this same system).
        if let Some(r) = convergence_reason(config, &x, &sys, &d, d_max, last_accepted.as_ref()) {
            converged = true;
            reason = r;
            break 'outer;
        }

        // ── Model refresh (inexact-model problems) ──
        // A problem that evaluates a surrogate / cached linearization
        // may move its anchor HERE and nowhere else. The switch is
        // only sound together with a re-assembly: Φ(x), A, g and every
        // trial Φ(x + h) of the iteration below must describe ONE
        // objective, or ρ compares two different functions and A stops
        // being the Gauss-Newton Hessian of the compared cost. Placed
        // after the convergence tests deliberately — the tests judge
        // the objective the driver was actually minimizing, and a
        // solve that terminates here spends no refresh evaluation.
        if source.refresh_model(&x) {
            sys = match source.assemble(&x) {
                Ok(s) => s,
                // No retreat: the problem has already switched models,
                // so `sys` describes an objective it no longer
                // evaluates. Surface it rather than damping a stale
                // system.
                Err(AssembleFailure::Domain(e)) => {
                    return Err(LMError::ModelRefreshFailed {
                        iteration,
                        source: e,
                    });
                }
                Err(AssembleFailure::NonFinite(defect)) => {
                    return Err(LMError::InvalidEvaluation { iteration, defect });
                }
                Err(AssembleFailure::SystemNonFinite(defect)) => {
                    return Err(LMError::InvalidSystem { iteration, defect });
                }
                Err(AssembleFailure::Hard(h)) => return Err(h.into_error(iteration)),
            };
            d_max = update_scaling(&mut d, &sys.normal);
            if d_max == 0.0 {
                return Err(LMError::ZeroDampingDiagonal { iteration });
            }
            // The refreshed evaluation is the committed one at x.
            source.accepted(&x);
            // The last accepted step's reductions were measured on the
            // previous model; comparing them against this one is
            // exactly the cross-model comparison the refresh exists to
            // prevent, so the cost test restarts.
            last_accepted = None;

            // The battery above judged the model the problem just
            // replaced. The new one is a different objective at the
            // same point and may already be stationary there — a
            // surrogate refreshing onto the full model at a converged
            // iterate is precisely that case. Re-test BEFORE spending a
            // trial: otherwise the driver damps its way to
            // DampingExhausted / InnerTrialsExhausted and reports
            // failure at a point whose undamped Gauss-Newton step is
            // below `qtol`.
            //
            // Which tests re-run, and why: `last_accepted` was just
            // cleared, so the cost test is skipped BY CONSTRUCTION —
            // its actred/pred belong to the previous model and are not
            // comparable to this one. The step test (`qtol`/`xtol` on
            // the undamped Gauss-Newton step) and the MINPACK
            // gradient test read only the freshly assembled system, so
            // both are meaningful the instant the re-assembly lands.
            // The accepted DAMPED step is deliberately not a
            // convergence criterion anywhere — see the gain-ratio
            // design note; a μ-crushed step says nothing about
            // stationarity under either model.
            if let Some(r) = convergence_reason(config, &x, &sys, &d, d_max, None) {
                converged = true;
                reason = r;
                break 'outer;
            }
        }

        // ── Inner trial loop: same system, escalating μ ──
        let mut accepted_this_iteration = false;
        for _trial in 0..config.max_inner_trials {
            // One factorization serves the velocity and (geodesic)
            // acceleration solves at this μ.
            let velocity = damped_factor(&sys.normal, &d, d_max, mu)
                .and_then(|l| solve_with_factor(&l, &sys.rhs, &d, d_max).map(|h| (l, h)));
            let Some((factor, h_natural)) = velocity else {
                // Factorization failed or produced non-finite values:
                // reject (raise μ) and retry — never bail, never fall
                // back to a different solver.
                n_rejected_trials += 1;
                if let Some(r) = escalate_mu(&mut mu, &mut nu, config.mu_max) {
                    reason = r;
                    break 'outer;
                }
                continue;
            };

            let mut h = h_natural;

            // ── Geodesic acceleration (Transtrum & Sethna 2012) ──
            // h = v + a/2 with (A + μD²) a = −Jᵀ r''_vv, where r''_vv
            // is the problem-supplied directional second derivative of
            // the residuals along v. Residual path only (needs the
            // Jacobian rows); inert when the hook returns None.
            if config.geodesic_acceleration
                && let Some(jac) = sys.jacobian.as_ref()
                && let Some(w) = source.second_directional_derivative(&x, &h_natural)
            {
                if w.len() != jac.len() {
                    return Err(LMError::DimensionMismatch {
                        iteration,
                        residuals: w.len(),
                        jacobian: jac.len(),
                    });
                }
                let mut rhs_a = [0.0_f64; N];
                for (w_i, row) in w.iter().zip(jac.iter()) {
                    for j in 0..N {
                        rhs_a[j] -= row[j] * w_i;
                    }
                }
                if let Some(a) = solve_with_factor(&factor, &rhs_a, &d, d_max) {
                    let v_norm = scaled_norm(&h_natural, &d);
                    let a_norm = scaled_norm(&a, &d);
                    if a_norm <= config.avmax * v_norm {
                        for j in 0..N {
                            h[j] += 0.5 * a[j];
                        }
                        n_accelerated_trials += 1;
                    } else {
                        // GSL avmax guard: the truncated expansion is
                        // untrustworthy here — reject through the
                        // normal μ escalation (no cost evaluation
                        // spent).
                        n_rejected_trials += 1;
                        if let Some(r) = escalate_mu(&mut mu, &mut nu, config.mu_max) {
                            reason = r;
                            break 'outer;
                        }
                        continue;
                    }
                }
                // A failed acceleration solve simply proceeds with the
                // plain velocity step — v itself solved fine.
            }

            let h_unclamped = h;
            source.constrain(&x, &mut h);
            let clamped = h != h_unclamped;

            // Predicted reduction. For unclamped steps the model
            // decrease of the VELOCITY step is the right denominator —
            // identical to the actual step without acceleration, and
            // for accelerated steps the correction is an on-manifold
            // re-tracing the quadratic model cannot see (judging the
            // combined step against the quadratic model force-rejects
            // exactly the accelerated steps that work: on Rosenbrock
            // the combined step lands AT the minimum while the model
            // predicts an increase along it — Transtrum & Sethna 2012,
            // GSL lmaccel). A clamped step falls back to the general
            // quadratic model on the ACTUAL step, as before; pred ≤ 0
            // (or NaN) still forces rejection — no blind division
            // anywhere.
            let pred = if clamped {
                predicted_reduction(&h, &sys.normal, &sys.rhs)
            } else {
                predicted_reduction(&h_natural, &sys.normal, &sys.rhs)
            };

            let mut x_trial = [0.0_f64; N];
            for i in 0..N {
                x_trial[i] = x[i] + h[i];
            }

            n_cost_evals += 1;
            let trial_result = source.trial_cost(&x_trial);

            // Classify the trial. All comparisons are false on NaN, so
            // every Err / non-finite / model-inconsistent combination
            // lands in a rejection branch — no sentinel values.
            // NOTE: a finite trial cost does NOT reset the
            // consecutive-invalid counter yet — a provisional
            // acceptance can still be rolled back below and
            // re-classified as invalid. The counter resets only when
            // a trial is conclusively valid: a finite-cost gain-test
            // rejection, or a committed acceptance.
            let cost_trial = match trial_result {
                Ok(c) if c.is_finite() => Some(c),
                Ok(_) => {
                    n_invalid_trials += 1;
                    consecutive_invalid += 1;
                    None
                }
                Err(e) => {
                    n_invalid_trials += 1;
                    consecutive_invalid += 1;
                    last_invalid_source = Some(e);
                    None
                }
            };
            if consecutive_invalid >= config.max_consecutive_invalid {
                // Lifecycle symmetry: the trial that trips the limit
                // is still a non-committed trial — let the problem
                // discard its pending state before the error.
                source.rejected(&x_trial);
                return Err(persistent_invalid_trials(
                    iteration,
                    consecutive_invalid,
                    last_invalid_source,
                    &x,
                    sys.cost,
                ));
            }

            let accept = match cost_trial {
                Some(c) => pred > 0.0 && (sys.cost - c) > config.min_relative_decrease * pred,
                None => false,
            };

            if !accept {
                if cost_trial.is_some() {
                    n_rejected_trials += 1;
                    consecutive_invalid = 0;
                    last_invalid_source = None;
                }
                source.rejected(&x_trial);
                if let Some(r) = escalate_mu(&mut mu, &mut nu, config.mu_max) {
                    reason = r;
                    break 'outer;
                }
                continue;
            }
            let cost_trial = cost_trial.expect("accepted trial has a finite cost");

            // Provisional acceptance: run the full evaluation at the
            // trial point BEFORE committing. A failing, non-finite, or
            // cost-inconsistent (non-decreasing — the monotonicity
            // guard) evaluation rolls the acceptance back and is
            // treated as an invalid trial.
            let assembled = source.assemble(&x_trial);
            match assembled {
                Ok(new_sys) if new_sys.cost < sys.cost => {
                    // Commit. ρ from the trial cost that drove the
                    // decision; Nielsen's smooth μ update (cube via
                    // multiplication — no libm).
                    let rho = (sys.cost - cost_trial) / pred;
                    let t = 2.0 * rho - 1.0;
                    let shrink = 1.0 - t * t * t;
                    let factor = if shrink > 1.0 / 3.0 {
                        shrink
                    } else {
                        1.0 / 3.0
                    };
                    mu *= factor;
                    // Nielsen's factor reaches 2 for marginal accepts
                    // (ρ → 0⁺), so μ can GROW through acceptances —
                    // cap it like the reject path does.
                    if mu > config.mu_max {
                        mu = config.mu_max;
                    }
                    nu = 2.0;
                    consecutive_invalid = 0;
                    last_invalid_source = None;

                    source.accepted(&x_trial);

                    // Tracked outside `last_accepted` because that
                    // record is CLEARED by a model refresh (its
                    // actred/pred stop being comparable across one),
                    // while "how big was the last step actually taken"
                    // remains true. Clearing both would make a solve
                    // that accepted many steps and then refreshed
                    // report `None` — i.e. "no step was accepted",
                    // which callers read as "the start point was
                    // already stationary".
                    last_accepted_qnorm = Some(quadratic_form(&h, &sys.normal));
                    last_accepted = Some(AcceptedStep {
                        clamped,
                        prev_cost: sys.cost,
                        pred,
                        actred: sys.cost - new_sys.cost,
                    });

                    x = x_trial;
                    sys = new_sys;
                    accepted_this_iteration = true;
                }
                Err(AssembleFailure::Hard(hard)) => {
                    // The trial never committed; discard its pending
                    // state before surfacing the contract violation.
                    source.rejected(&x_trial);
                    return Err(hard.into_error(iteration));
                }
                other => {
                    // Roll back: x unchanged, sys unchanged, no commit.
                    let source_err = match other {
                        Err(AssembleFailure::Domain(e)) => Some(e),
                        _ => None,
                    };
                    n_invalid_trials += 1;
                    consecutive_invalid += 1;
                    if let Some(e) = source_err {
                        last_invalid_source = Some(e);
                    }
                    if consecutive_invalid >= config.max_consecutive_invalid {
                        source.rejected(&x_trial);
                        return Err(persistent_invalid_trials(
                            iteration,
                            consecutive_invalid,
                            last_invalid_source,
                            &x,
                            sys.cost,
                        ));
                    }
                    source.rejected(&x_trial);
                    if let Some(r) = escalate_mu(&mut mu, &mut nu, config.mu_max) {
                        reason = r;
                        break 'outer;
                    }
                    continue;
                }
            }

            if accepted_this_iteration {
                break;
            }
        }

        if !accepted_this_iteration {
            reason = TerminationReason::InnerTrialsExhausted {
                trials: config.max_inner_trials,
            };
            break 'outer;
        }

        // Running-max scaling update at the newly accepted system.
        d_max = update_scaling(&mut d, &sys.normal);
    }

    // Final state: `sys` is the full evaluation at the returned `x` on
    // every exit path (state moved only on committed acceptances).
    let gradient_norm_scaled = {
        let mut m = 0.0_f64;
        for j in 0..N {
            let s = sys.rhs[j].abs() / effective_scale(d[j], d_max);
            if s > m {
                m = s;
            }
        }
        m
    };

    // The quantity `qtol` is tested on, evaluated at the RETURNED
    // point: the undamped Gauss-Newton step's quadratic form. Reported
    // on every exit path so a non-converged solve can be described in
    // terms of the test it failed, instead of the μ-damped accepted
    // step (which is comparable to nothing).
    let final_gn_qnorm = solve_damped(&sys.normal, &sys.rhs, &d, d_max, 0.0)
        .map(|h| quadratic_form(&h, &sys.normal));

    Ok(LMSolution {
        x,
        covariance: covariance(&sys.normal, &d, d_max),
        cost: sys.cost,
        data_cost: sys.data_cost,
        accepted_step_qnorm: last_accepted_qnorm,
        final_gn_qnorm,
        gradient_norm_scaled,
        iterations,
        n_cost_evals,
        n_rejected_trials,
        n_invalid_trials,
        n_accelerated_trials,
        mu_final: mu,
        converged,
        reason,
    })
}

// ── Tests ───────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    /// Test domain error.
    #[derive(Debug, PartialEq)]
    struct TestError(&'static str);
    impl std::fmt::Display for TestError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    impl std::error::Error for TestError {}

    /// A residual problem built from a residual-vector closure, with
    /// trial costs computed from the same closure (same objective,
    /// same bits) and full lifecycle logging.
    struct Tracked<F> {
        f: F,
        accepted: Vec<Vec<f64>>,
        rejected: Vec<Vec<f64>>,
        full_evals: Vec<f64>,
    }

    impl<F> Tracked<F> {
        fn new(f: F) -> Self {
            Self {
                f,
                accepted: Vec::new(),
                rejected: Vec::new(),
                full_evals: Vec::new(),
            }
        }
    }

    impl<F, const N: usize> CostProblem<N> for Tracked<F>
    where
        F: FnMut(&[f64; N]) -> (Vec<f64>, Vec<[f64; N]>),
    {
        type Error = TestError;
        fn evaluate_cost(&mut self, x: &[f64; N]) -> Result<f64, TestError> {
            let (residuals, _) = (self.f)(x);
            Ok(residuals.iter().map(|r| r * r).sum())
        }
        fn on_step_accepted(&mut self, x: &[f64; N]) {
            self.accepted.push(x.to_vec());
        }
        fn on_step_rejected(&mut self, x_trial: &[f64; N]) {
            self.rejected.push(x_trial.to_vec());
        }
    }

    impl<F, const N: usize> ResidualProblem<N> for Tracked<F>
    where
        F: FnMut(&[f64; N]) -> (Vec<f64>, Vec<[f64; N]>),
    {
        fn evaluate(&mut self, x: &[f64; N]) -> Result<NLLSEvaluation<N>, TestError> {
            let (residuals, jacobian) = (self.f)(x);
            let cost = residuals.iter().map(|r| r * r).sum();
            self.full_evals.push(cost);
            Ok(NLLSEvaluation {
                residuals,
                jacobian,
                cost,
            })
        }
    }

    fn config() -> LMConfig {
        LMConfig::default()
    }

    /// Residual/Jacobian pair returned by 1-D test problems.
    type Resid1 = (Vec<f64>, Vec<[f64; 1]>);

    /// Rational-arithmetic overshoot problem (libm-free, so traces are
    /// bit-identical cross-platform): r(d) = d + 4d/(1+d²), d = x − 5.
    /// Around |d| ≈ 1.5 the Jacobian dips and Gauss-Newton overshoots,
    /// so the accept/reject loop must field genuine rejections.
    fn overshoot_rational(x: &[f64; 1]) -> (Vec<f64>, Vec<[f64; 1]>) {
        let d = x[0] - 5.0;
        let q = 1.0 + d * d;
        let r = d + 4.0 * d / q;
        let j = 1.0 + 4.0 * (1.0 - d * d) / (q * q);
        (vec![r], vec![[j]])
    }

    // ── Convergence on reference problems ──

    #[test]
    fn test_linear_system() {
        let mut p = Tracked::new(|x: &[f64; 2]| {
            (vec![x[0] - 3.0, x[1] - 7.0], vec![[1.0, 0.0], [0.0, 1.0]])
        });
        let mut cfg = config();
        cfg.xtol = 1e-14;
        let sol = solve(&mut p, [0.0; 2], &cfg, None).unwrap();
        assert!(sol.converged, "{:?}", sol.reason);
        assert!((sol.x[0] - 3.0).abs() < 1e-10, "x0={}", sol.x[0]);
        assert!((sol.x[1] - 7.0).abs() < 1e-10, "x1={}", sol.x[1]);
        let cov = sol.covariance.unwrap();
        assert!((cov[0][0] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_overdetermined_linear() {
        let xs = [0.0, 1.0, 2.0, 3.0];
        let ys = [1.0, 3.0, 5.0, 7.0];
        let mut p = Tracked::new(move |p: &[f64; 2]| {
            let residuals: Vec<f64> = xs
                .iter()
                .zip(ys.iter())
                .map(|(&x, &y)| p[0] * x + p[1] - y)
                .collect();
            let jacobian: Vec<[f64; 2]> = xs.iter().map(|&x| [x, 1.0]).collect();
            (residuals, jacobian)
        });
        let sol = solve(&mut p, [0.0; 2], &config(), None).unwrap();
        assert!(sol.converged);
        assert!((sol.x[0] - 2.0).abs() < 1e-8, "a={}", sol.x[0]);
        assert!((sol.x[1] - 1.0).abs() < 1e-8, "b={}", sol.x[1]);
    }

    fn rosenbrock(x: &[f64; 2]) -> (Vec<f64>, Vec<[f64; 2]>) {
        (
            vec![10.0 * (x[1] - x[0] * x[0]), 1.0 - x[0]],
            vec![[-20.0 * x[0], 10.0], [-1.0, 0.0]],
        )
    }

    #[test]
    fn test_rosenbrock() {
        let mut p = Tracked::new(rosenbrock);
        let sol = solve(&mut p, [-1.0, 1.0], &config(), None).unwrap();
        assert!(sol.converged, "{:?}", sol.reason);
        assert!((sol.x[0] - 1.0).abs() < 1e-6, "x0={}", sol.x[0]);
        assert!((sol.x[1] - 1.0).abs() < 1e-6, "x1={}", sol.x[1]);
    }

    #[test]
    fn test_circle_fit() {
        let angles: [f64; 8] = [0.0, 0.7, 1.4, 2.1, 2.8, 3.5, 4.2, 4.9];
        let data: Vec<(f64, f64)> = angles
            .iter()
            .map(|&a| (2.0 + 5.0 * a.cos(), 3.0 + 5.0 * a.sin()))
            .collect();
        let mut p = Tracked::new(move |p: &[f64; 3]| {
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
            (residuals, jacobian)
        });
        let sol = solve(&mut p, [0.0, 0.0, 1.0], &config(), None).unwrap();
        assert!(sol.converged, "{:?}", sol.reason);
        assert!((sol.x[0] - 2.0).abs() < 1e-7, "cx={}", sol.x[0]);
        assert!((sol.x[1] - 3.0).abs() < 1e-7, "cy={}", sol.x[1]);
        assert!((sol.x[2] - 5.0).abs() < 1e-7, "r={}", sol.x[2]);
    }

    /// The motivating defect class (empyrean-ju91): far from the
    /// minimum of r = ln(1 + (x−5)²) the Jacobian collapses and pure
    /// Gauss-Newton overshoots by orders of magnitude. The removed
    /// always-accept solver provably diverged here without a
    /// problem-supplied step clamp; the accept/reject loop must
    /// converge with NO clamp.
    #[test]
    fn test_overshoot_converges_without_clamp() {
        let mut p = Tracked::new(|x: &[f64; 1]| {
            let d = x[0] - 5.0;
            let r = (1.0 + d * d).ln();
            let j = 2.0 * d / (1.0 + d * d);
            (vec![r], vec![[j]])
        });
        let mut cfg = config();
        cfg.max_iterations = 300;
        let sol = solve(&mut p, [100.0], &cfg, None).unwrap();
        assert!(sol.converged, "{:?}", sol.reason);
        assert!((sol.x[0] - 5.0).abs() < 1e-3, "x={}", sol.x[0]);
        assert!(
            sol.n_rejected_trials > 0,
            "overshoot should exercise rejections"
        );
    }

    // ── τ-insensitivity: the dead-zone regression ──

    /// A correct accept/reject solver is insensitive to the initial
    /// damping choice. This encodes the ju91/CD3 dead-zone symptom
    /// (convergence non-monotonic in λ_initial) as a permanent
    /// invariant.
    #[test]
    fn test_tau_insensitivity_sweep() {
        let taus = [1e-8, 1e-6, 1e-4, 1e-3, 1e-1, 1.0, 1e2];
        for &tau in &taus {
            let mut cfg = config();
            cfg.tau = tau;
            cfg.max_iterations = 500;

            let mut p = Tracked::new(rosenbrock);
            let sol = solve(&mut p, [-1.0, 1.0], &cfg, None).unwrap();
            assert!(sol.converged, "rosenbrock tau={tau}: {:?}", sol.reason);
            assert!(
                (sol.x[0] - 1.0).abs() < 1e-6 && (sol.x[1] - 1.0).abs() < 1e-6,
                "rosenbrock tau={tau}: x={:?}",
                sol.x
            );

            let mut p = Tracked::new(overshoot_rational);
            let sol = solve(&mut p, [6.5], &cfg, None).unwrap();
            assert!(sol.converged, "overshoot tau={tau}: {:?}", sol.reason);
            assert!(
                (sol.x[0] - 5.0).abs() < 1e-4,
                "overshoot tau={tau}: x={}",
                sol.x[0]
            );

            let mut p = Tracked::new(|x: &[f64; 1]| {
                let d = x[0] - 5.0;
                let r = (1.0 + d * d).ln();
                let j = 2.0 * d / (1.0 + d * d);
                (vec![r], vec![[j]])
            });
            let sol = solve(&mut p, [100.0], &cfg, None).unwrap();
            assert!(sol.converged, "log-overshoot tau={tau}: {:?}", sol.reason);
            assert!(
                (sol.x[0] - 5.0).abs() < 1e-3,
                "log-overshoot tau={tau}: x={}",
                sol.x[0]
            );
        }
    }

    // ── Prior handling ──

    #[test]
    fn test_prior_map_solution() {
        // r = x − 10 with prior N(0, 1): MAP at x = 5, covariance 1/2.
        let mut p = Tracked::new(|x: &[f64; 1]| (vec![x[0] - 10.0], vec![[1.0]]));
        let prior = NLLSPrior {
            mean: [0.0],
            covariance_inv: [[1.0]],
        };
        let sol = solve(&mut p, [0.0], &config(), Some(&prior)).unwrap();
        assert!(sol.converged, "{:?}", sol.reason);
        assert!((sol.x[0] - 5.0).abs() < 1e-8, "x={}", sol.x[0]);
        let cov = sol.covariance.unwrap();
        assert!((cov[0][0] - 0.5).abs() < 1e-10, "cov={}", cov[0][0]);
        // cost = data + prior penalty; data_cost = data only.
        assert!((sol.cost - 50.0).abs() < 1e-6, "cost={}", sol.cost);
        assert!(
            (sol.data_cost - 25.0).abs() < 1e-6,
            "data={}",
            sol.data_cost
        );
    }

    /// Defect-13 regression: with a weak data term and a tight prior,
    /// the solver must land on the MAP point, not the data minimum —
    /// the prior penalty enters both compared costs, so steps that
    /// trade a small data gain for a large prior penalty are rejected.
    #[test]
    fn test_prior_penalty_in_acceptance() {
        let mut p = Tracked::new(|x: &[f64; 1]| (vec![0.1 * (x[0] - 10.0)], vec![[0.1]]));
        let prior = NLLSPrior {
            mean: [0.0],
            covariance_inv: [[1.0]],
        };
        let mut cfg = config();
        cfg.xtol = 1e-13;
        // Start AT the data minimum: every step toward the MAP point
        // INCREASES the data cost, so a data-only acceptance objective
        // (the defect) would reject every trial and never leave
        // x = 10 — this start point is what makes the test
        // discriminating.
        let sol = solve(&mut p, [10.0], &cfg, Some(&prior)).unwrap();
        assert!(sol.converged, "{:?}", sol.reason);
        // MAP: minimize 0.01(x−10)² + x² → x = 0.1/1.01.
        let expected = 0.1 / 1.01;
        assert!(
            (sol.x[0] - expected).abs() < 1e-6,
            "x={} expected={expected}",
            sol.x[0]
        );
    }

    /// Uninformative data (zero Jacobian, finite residuals) with a
    /// prior: the posterior IS the prior. The solver must converge to
    /// the prior mean with the prior covariance — honestly, with
    /// finite cost (the old solver's INF-sentinel false convergence is
    /// structurally impossible here).
    #[test]
    fn test_prior_zero_jacobian_posterior_is_prior() {
        let mut p = Tracked::new(|_x: &[f64; 1]| (vec![5.0], vec![[0.0]]));
        let prior = NLLSPrior {
            mean: [2.0],
            covariance_inv: [[1.0]],
        };
        let sol = solve(&mut p, [0.0], &config(), Some(&prior)).unwrap();
        assert!(sol.converged, "{:?}", sol.reason);
        assert!((sol.x[0] - 2.0).abs() < 1e-8, "x={}", sol.x[0]);
        let cov = sol.covariance.unwrap();
        assert!((cov[0][0] - 1.0).abs() < 1e-8, "cov={}", cov[0][0]);
        assert!(sol.cost.is_finite());
        assert!((sol.data_cost - 25.0).abs() < 1e-10);
    }

    // ── Invalid inputs and error axes ──

    #[test]
    fn test_initial_domain_error() {
        struct Failing;
        impl CostProblem<1> for Failing {
            type Error = TestError;
            fn evaluate_cost(&mut self, _x: &[f64; 1]) -> Result<f64, TestError> {
                Err(TestError("cost"))
            }
        }
        impl ResidualProblem<1> for Failing {
            fn evaluate(&mut self, _x: &[f64; 1]) -> Result<NLLSEvaluation<1>, TestError> {
                Err(TestError("propagation failed"))
            }
        }
        let err = solve(&mut Failing, [0.0], &config(), None).unwrap_err();
        assert!(
            matches!(err, LMError::InitialEvaluationFailed { .. }),
            "{err:?}"
        );
    }

    #[test]
    fn test_nonfinite_initial_cost_is_error() {
        // An INF cost at x0 must be a loud error, not a sentinel the
        // solver silently converges on.
        struct InfCost;
        impl CostProblem<1> for InfCost {
            type Error = TestError;
            fn evaluate_cost(&mut self, _x: &[f64; 1]) -> Result<f64, TestError> {
                Ok(f64::INFINITY)
            }
        }
        impl ResidualProblem<1> for InfCost {
            fn evaluate(&mut self, _x: &[f64; 1]) -> Result<NLLSEvaluation<1>, TestError> {
                Ok(NLLSEvaluation {
                    residuals: vec![0.0],
                    jacobian: vec![[0.0]],
                    cost: f64::INFINITY,
                })
            }
        }
        let err = solve(&mut InfCost, [0.0], &config(), None).unwrap_err();
        assert!(
            matches!(
                err,
                LMError::InvalidEvaluation {
                    iteration: 0,
                    defect: EvaluationDefect::Cost
                }
            ),
            "{err:?}"
        );
    }

    #[test]
    fn test_empty_residuals_and_dimension_mismatch() {
        let mut p = Tracked::new(|_x: &[f64; 1]| (vec![], vec![]));
        let err = solve(&mut p, [0.0], &config(), None).unwrap_err();
        assert!(
            matches!(err, LMError::EmptyResiduals { iteration: 0 }),
            "{err:?}"
        );

        let mut p = Tracked::new(|_x: &[f64; 1]| (vec![1.0, 2.0], vec![[1.0]]));
        let err = solve(&mut p, [0.0], &config(), None).unwrap_err();
        assert!(
            matches!(
                err,
                LMError::DimensionMismatch {
                    iteration: 0,
                    residuals: 2,
                    jacobian: 1
                }
            ),
            "{err:?}"
        );
    }

    #[test]
    fn test_zero_damping_diagonal() {
        // Zero Jacobian, no prior: μ-escalation is provably futile —
        // surfaced up-front on its own axis.
        let mut p = Tracked::new(|_x: &[f64; 2]| (vec![1.0], vec![[0.0, 0.0]]));
        let err = solve(&mut p, [0.0; 2], &config(), None).unwrap_err();
        assert!(
            matches!(err, LMError::ZeroDampingDiagonal { iteration: 0 }),
            "{err:?}"
        );
    }

    #[test]
    fn test_invalid_config_each_field() {
        let check = |cfg: LMConfig, want: fn(&ConfigDefect) -> bool| {
            let mut p_local = Tracked::new(|x: &[f64; 1]| (vec![x[0]], vec![[1.0]]));
            let err = solve(&mut p_local, [1.0], &cfg, None).unwrap_err();
            match err {
                LMError::InvalidConfig { defect } => assert!(want(&defect), "{defect:?}"),
                other => panic!("expected InvalidConfig, got {other:?}"),
            }
        };
        let mut c = config();
        c.max_iterations = 0;
        check(c, |d| matches!(d, ConfigDefect::MaxIterationsZero));
        let mut c = config();
        c.max_inner_trials = 0;
        check(c, |d| matches!(d, ConfigDefect::MaxInnerTrialsZero));
        let mut c = config();
        c.tau = 0.0;
        check(c, |d| matches!(d, ConfigDefect::TauNotPositive { .. }));
        let mut c = config();
        c.tau = f64::NAN;
        check(c, |d| matches!(d, ConfigDefect::TauNotPositive { .. }));
        let mut c = config();
        c.mu_max = 0.0;
        check(c, |d| matches!(d, ConfigDefect::MuMaxNotPositive { .. }));
        let mut c = config();
        c.min_relative_decrease = -1.0;
        check(c, |d| {
            matches!(d, ConfigDefect::MinRelativeDecreaseNegative { .. })
        });
        let mut c = config();
        c.gtol = -1.0;
        check(c, |d| matches!(d, ConfigDefect::GtolNegative { .. }));
        let mut c = config();
        c.qtol = -1.0;
        check(c, |d| matches!(d, ConfigDefect::QtolNegative { .. }));
        let mut c = config();
        c.xtol = f64::INFINITY;
        check(c, |d| matches!(d, ConfigDefect::XtolNegative { .. }));
        let mut c = config();
        c.ftol = -1.0;
        check(c, |d| matches!(d, ConfigDefect::FtolNegative { .. }));
        let mut c = config();
        c.max_consecutive_invalid = 0;
        check(c, |d| matches!(d, ConfigDefect::MaxConsecutiveInvalidZero));
    }

    #[test]
    fn test_invalid_prior() {
        let mut p = Tracked::new(|x: &[f64; 1]| (vec![x[0]], vec![[1.0]]));
        let prior = NLLSPrior {
            mean: [f64::NAN],
            covariance_inv: [[1.0]],
        };
        let err = solve(&mut p, [0.0], &config(), Some(&prior)).unwrap_err();
        assert!(
            matches!(
                err,
                LMError::InvalidPrior {
                    defect: PriorDefect::NonFiniteMean { index: 0 }
                }
            ),
            "{err:?}"
        );

        let prior = NLLSPrior {
            mean: [0.0],
            covariance_inv: [[-1.0]],
        };
        let err = solve(&mut p, [0.0], &config(), Some(&prior)).unwrap_err();
        assert!(
            matches!(
                err,
                LMError::InvalidPrior {
                    defect: PriorDefect::NegativeDiagonal { index: 0, .. }
                }
            ),
            "{err:?}"
        );
    }

    // ── Trial-failure recovery and persistence ──

    /// A propagation blowup at an aggressive trial step is a
    /// recoverable rejection, not a fit-killer.
    #[test]
    fn test_inf_trial_cost_recovers() {
        struct Walled {
            inner: Tracked<fn(&[f64; 1]) -> Resid1>,
        }
        impl CostProblem<1> for Walled {
            type Error = TestError;
            fn evaluate_cost(&mut self, x: &[f64; 1]) -> Result<f64, TestError> {
                if x[0] > 50.0 {
                    return Ok(f64::INFINITY); // "propagation blew up"
                }
                self.inner.evaluate_cost(x)
            }
        }
        impl ResidualProblem<1> for Walled {
            fn evaluate(&mut self, x: &[f64; 1]) -> Result<NLLSEvaluation<1>, TestError> {
                self.inner.evaluate(x)
            }
        }
        // From x0 = −100 the log-overshoot GN step lands far past +50.
        fn log_overshoot(x: &[f64; 1]) -> (Vec<f64>, Vec<[f64; 1]>) {
            let d = x[0] - 5.0;
            let r = (1.0 + d * d).ln();
            let j = 2.0 * d / (1.0 + d * d);
            (vec![r], vec![[j]])
        }
        let mut p = Walled {
            inner: Tracked::new(log_overshoot as fn(&[f64; 1]) -> _),
        };
        let mut cfg = config();
        cfg.max_iterations = 300;
        // The far-side wall eats more than the default 5 consecutive
        // trials before damping shrinks the step under it.
        cfg.max_consecutive_invalid = 12;
        let sol = solve(&mut p, [-100.0], &cfg, None).unwrap();
        assert!(sol.converged, "{:?}", sol.reason);
        assert!((sol.x[0] - 5.0).abs() < 1e-3, "x={}", sol.x[0]);
        assert!(sol.n_invalid_trials > 0, "wall should have been hit");
    }

    #[test]
    fn test_persistent_invalid_trials() {
        struct AlwaysFailsCost;
        impl CostProblem<1> for AlwaysFailsCost {
            type Error = TestError;
            fn evaluate_cost(&mut self, _x: &[f64; 1]) -> Result<f64, TestError> {
                Err(TestError("always fails"))
            }
        }
        impl ResidualProblem<1> for AlwaysFailsCost {
            fn evaluate(&mut self, x: &[f64; 1]) -> Result<NLLSEvaluation<1>, TestError> {
                Ok(NLLSEvaluation {
                    residuals: vec![x[0] - 3.0],
                    jacobian: vec![[1.0]],
                    cost: (x[0] - 3.0) * (x[0] - 3.0),
                })
            }
        }
        let err = solve(&mut AlwaysFailsCost, [0.0], &config(), None).unwrap_err();
        match err {
            LMError::PersistentInvalidTrials {
                consecutive,
                last_source,
                best_x,
                best_cost,
                ..
            } => {
                assert_eq!(consecutive, 5);
                assert_eq!(last_source, Some(TestError("always fails")));
                assert_eq!(best_x, vec![0.0]);
                assert!((best_cost - 9.0).abs() < 1e-12);
            }
            other => panic!("expected PersistentInvalidTrials, got {other:?}"),
        }
    }

    // ── Termination reasons, one test each ──

    #[test]
    fn test_reason_gradient_tolerance() {
        // Linear: one exact step, then zero gradient at zero cost.
        let mut cfg = config();
        cfg.qtol = 0.0;
        cfg.xtol = 0.0;
        cfg.ftol = 0.0;
        let mut p = Tracked::new(|x: &[f64; 1]| (vec![x[0] - 3.0], vec![[1.0]]));
        let sol = solve(&mut p, [0.0], &cfg, None).unwrap();
        assert!(sol.converged);
        assert_eq!(sol.reason, TerminationReason::GradientTolerance);
        assert!((sol.x[0] - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_reason_step_tolerance_at_stationary_start() {
        // Start exactly at the minimum with the gradient test disabled:
        // the natural step is zero → StepTolerance with x unchanged.
        let mut cfg = config();
        cfg.gtol = 0.0;
        cfg.ftol = 0.0;
        let mut p = Tracked::new(|x: &[f64; 1]| (vec![x[0] - 3.0], vec![[1.0]]));
        let sol = solve(&mut p, [3.0], &cfg, None).unwrap();
        assert!(sol.converged);
        assert_eq!(sol.reason, TerminationReason::StepTolerance);
        assert_eq!(sol.x[0], 3.0);
        assert!(sol.accepted_step_qnorm.is_none());
    }

    #[test]
    fn test_reason_max_iterations() {
        let mut cfg = config();
        cfg.max_iterations = 2;
        let mut p = Tracked::new(rosenbrock);
        let sol = solve(&mut p, [-1.2, 1.0], &cfg, None).unwrap();
        assert!(!sol.converged);
        assert_eq!(sol.reason, TerminationReason::MaxIterations);
        assert_eq!(sol.iterations, 2);
    }

    #[test]
    fn test_reason_damping_exhausted_on_adversarial_cost() {
        // The trial cost claims every step makes things worse while the
        // gradient says otherwise: every trial is rejected, μ escalates
        // to the cap, and the solver reports it — explicitly, finitely.
        struct Adversarial;
        impl CostProblem<1> for Adversarial {
            type Error = TestError;
            fn evaluate_cost(&mut self, _x: &[f64; 1]) -> Result<f64, TestError> {
                Ok(10.0)
            }
        }
        impl ResidualProblem<1> for Adversarial {
            fn evaluate(&mut self, _x: &[f64; 1]) -> Result<NLLSEvaluation<1>, TestError> {
                Ok(NLLSEvaluation {
                    residuals: vec![1.0],
                    jacobian: vec![[1.0]],
                    cost: 1.0,
                })
            }
        }
        let mut cfg = config();
        cfg.gtol = 0.0;
        cfg.qtol = 0.0;
        cfg.xtol = 0.0;
        cfg.ftol = 0.0;
        let sol = solve(&mut Adversarial, [5.0], &cfg, None).unwrap();
        assert!(!sol.converged);
        assert!(
            matches!(sol.reason, TerminationReason::DampingExhausted { .. }),
            "{:?}",
            sol.reason
        );
        assert_eq!(sol.x[0], 5.0, "iterate must not move on rejections");
    }

    #[test]
    fn test_reason_inner_trials_exhausted() {
        struct Adversarial;
        impl CostProblem<1> for Adversarial {
            type Error = TestError;
            fn evaluate_cost(&mut self, _x: &[f64; 1]) -> Result<f64, TestError> {
                Ok(10.0)
            }
        }
        impl ResidualProblem<1> for Adversarial {
            fn evaluate(&mut self, _x: &[f64; 1]) -> Result<NLLSEvaluation<1>, TestError> {
                Ok(NLLSEvaluation {
                    residuals: vec![1.0],
                    jacobian: vec![[1.0]],
                    cost: 1.0,
                })
            }
        }
        let mut cfg = config();
        cfg.gtol = 0.0;
        cfg.qtol = 0.0;
        cfg.xtol = 0.0;
        cfg.ftol = 0.0;
        cfg.max_inner_trials = 3;
        cfg.mu_max = 1e300;
        let sol = solve(&mut Adversarial, [5.0], &cfg, None).unwrap();
        assert!(!sol.converged);
        assert_eq!(
            sol.reason,
            TerminationReason::InnerTrialsExhausted { trials: 3 }
        );
    }

    #[test]
    fn test_reason_cost_tolerance() {
        // A genuine cost plateau: an irreducible constant residual
        // dominates Φ near the minimum, so the relative reductions —
        // including the UNDAMPED step's own predicted gain — fall
        // below ftol·Φ while the gradient and step tests are disabled.
        let mut cfg = config();
        cfg.gtol = 0.0;
        cfg.qtol = 0.0;
        cfg.xtol = 0.0;
        cfg.max_iterations = 500;
        let mut p = Tracked::new(|x: &[f64; 1]| (vec![x[0] - 3.0, 10.0], vec![[1.0], [0.0]]));
        let sol = solve(&mut p, [0.0], &cfg, None).unwrap();
        assert!(sol.converged, "{:?}", sol.reason);
        assert_eq!(sol.reason, TerminationReason::CostTolerance);
        assert!((sol.x[0] - 3.0).abs() < 1e-2, "x={}", sol.x[0]);
    }

    // ── Clamps ──

    /// A clamp that shrinks every step below tolerance must NOT
    /// manufacture convergence (the old solver's documented false
    /// contract). With step reasons unreachable the solver runs its
    /// budget out while still making (slow) progress.
    #[test]
    fn test_clamped_step_cannot_declare_convergence() {
        struct Clamped {
            inner: Tracked<fn(&[f64; 1]) -> Resid1>,
        }
        impl CostProblem<1> for Clamped {
            type Error = TestError;
            fn evaluate_cost(&mut self, x: &[f64; 1]) -> Result<f64, TestError> {
                self.inner.evaluate_cost(x)
            }
            fn constrain_step(&mut self, _x: &[f64; 1], delta: &mut [f64; 1]) {
                let cap = 0.01;
                if delta[0].abs() > cap {
                    delta[0] = if delta[0] > 0.0 { cap } else { -cap };
                }
            }
        }
        impl ResidualProblem<1> for Clamped {
            fn evaluate(&mut self, x: &[f64; 1]) -> Result<NLLSEvaluation<1>, TestError> {
                self.inner.evaluate(x)
            }
        }
        fn linear(x: &[f64; 1]) -> (Vec<f64>, Vec<[f64; 1]>) {
            (vec![x[0] - 1000.0], vec![[1.0]])
        }
        let mut cfg = config();
        cfg.gtol = 0.0;
        cfg.ftol = 0.0;
        cfg.xtol = 0.0;
        cfg.qtol = 1.0; // generous: the 0.01-clamped step is far below
        cfg.max_iterations = 50;
        let mut p = Clamped {
            inner: Tracked::new(linear as fn(&[f64; 1]) -> _),
        };
        let sol = solve(&mut p, [0.0], &cfg, None).unwrap();
        assert!(
            !sol.converged,
            "clamp manufactured convergence: {:?}",
            sol.reason
        );
        assert_eq!(sol.reason, TerminationReason::MaxIterations);
        // ... while still making real progress (one clamped accepted
        // step per outer iteration).
        assert!((sol.x[0] - 0.5).abs() < 1e-9, "x={}", sol.x[0]);
    }

    // ── Lifecycle and state discipline ──

    #[test]
    fn test_lifecycle_hooks_and_monotone_cost() {
        let mut p = Tracked::new(overshoot_rational);
        let sol = solve(&mut p, [6.5], &config(), None).unwrap();
        assert!(sol.converged, "{:?}", sol.reason);
        assert!((sol.x[0] - 5.0).abs() < 1e-4);
        assert!(!p.rejected.is_empty(), "expected genuine rejections");
        assert!(!p.accepted.is_empty());
        // The final accepted hook saw the returned x.
        assert_eq!(p.accepted.last().unwrap()[0], sol.x[0]);
        // Full evaluations happened only at x0 + provisionally accepted
        // points; x0 itself is committed, so the counts match — and
        // every committed cost decreased monotonically.
        assert_eq!(p.full_evals.len(), p.accepted.len());
        assert_eq!(p.accepted[0][0], 6.5, "x0 is committed first");
        for w in p.full_evals.windows(2) {
            assert!(w[1] < w[0], "cost not monotone: {:?}", p.full_evals);
        }
        // No rejected trial point was ever committed.
        for r in &p.rejected {
            assert!(p.accepted.iter().all(|a| a != r));
        }
    }

    // ── Covariance ──

    #[test]
    fn test_covariance_known_linear() {
        let mut p = Tracked::new(|x: &[f64; 1]| (vec![x[0] - 3.0], vec![[1.0]]));
        let sol = solve(&mut p, [0.0], &config(), None).unwrap();
        let cov = sol.covariance.unwrap();
        assert!((cov[0][0] - 1.0).abs() < 1e-10);
    }

    /// Rank-deficient fit: converges in the observable subspace, keeps
    /// the solution, and reports the covariance failure explicitly —
    /// NEVER an all-zeros (σ = 0) matrix.
    #[test]
    fn test_covariance_singular_is_explicit_never_zeros() {
        let mut p = Tracked::new(|x: &[f64; 2]| (vec![x[0] - 3.0], vec![[1.0, 0.0]]));
        let sol = solve(&mut p, [0.0, 0.0], &config(), None).unwrap();
        assert!(sol.converged, "{:?}", sol.reason);
        assert!((sol.x[0] - 3.0).abs() < 1e-8, "x0={}", sol.x[0]);
        assert_eq!(sol.x[1], 0.0);
        assert_eq!(
            sol.covariance.unwrap_err(),
            CovarianceFailure::SingularNormalMatrix
        );
    }

    // ── Mixed scales (equilibrated solve) ──

    #[test]
    fn test_extreme_unit_mixing() {
        // Parameter scales differ by ~10 orders of magnitude with a
        // coupling row — the equilibrated Cholesky must handle what a
        // raw-basis factorization cannot.
        let a = 3.0_f64;
        let b = 7e9_f64;
        let mut p = Tracked::new(move |x: &[f64; 2]| {
            (
                vec![
                    x[0] - a,
                    1e-10 * (x[1] - b),
                    1e-5 * (x[0] - a) + 1e-15 * (x[1] - b),
                ],
                vec![[1.0, 0.0], [0.0, 1e-10], [1e-5, 1e-15]],
            )
        });
        let mut cfg = config();
        cfg.max_iterations = 200;
        let sol = solve(&mut p, [0.0, 0.0], &cfg, None).unwrap();
        assert!(sol.converged, "{:?}", sol.reason);
        assert!((sol.x[0] - a).abs() < 1e-6, "x0={}", sol.x[0]);
        assert!((sol.x[1] - b).abs() / b < 1e-6, "x1={}", sol.x[1]);
        assert!(sol.covariance.is_ok());
    }

    // ── System path ──

    /// Quadratic objective Φ = (x−a)ᵀQ(x−a) + c via the
    /// normal-equations-level trait: A = Q, g = Q(a−x).
    struct QuadSystem {
        q: [[f64; 2]; 2],
        a: [f64; 2],
        c: f64,
    }
    impl QuadSystem {
        fn cost(&self, x: &[f64; 2]) -> f64 {
            let d = [x[0] - self.a[0], x[1] - self.a[1]];
            let mut phi = self.c;
            for i in 0..2 {
                for j in 0..2 {
                    phi += d[i] * self.q[i][j] * d[j];
                }
            }
            phi
        }
    }
    impl CostProblem<2> for QuadSystem {
        type Error = TestError;
        fn evaluate_cost(&mut self, x: &[f64; 2]) -> Result<f64, TestError> {
            Ok(Self::cost(self, x))
        }
    }
    impl SystemProblem<2> for QuadSystem {
        fn evaluate_system(&mut self, x: &[f64; 2]) -> Result<SystemEvaluation<2>, TestError> {
            let mut rhs = [0.0; 2];
            for i in 0..2 {
                for j in 0..2 {
                    rhs[i] += self.q[i][j] * (self.a[j] - x[j]);
                }
            }
            Ok(SystemEvaluation {
                cost: Self::cost(self, x),
                normal: self.q,
                rhs,
            })
        }
    }

    #[test]
    fn test_solve_system_quadratic() {
        let mut p = QuadSystem {
            q: [[2.0, 0.0], [0.0, 8.0]],
            a: [1.0, 2.0],
            c: 3.0,
        };
        let sol = solve_system(&mut p, [10.0, -4.0], &config()).unwrap();
        assert!(sol.converged, "{:?}", sol.reason);
        assert!((sol.x[0] - 1.0).abs() < 1e-8, "x0={}", sol.x[0]);
        assert!((sol.x[1] - 2.0).abs() < 1e-8, "x1={}", sol.x[1]);
        assert!((sol.cost - 3.0).abs() < 1e-8);
        assert_eq!(sol.cost, sol.data_cost, "system path owns its objective");
        let cov = sol.covariance.unwrap();
        assert!((cov[0][0] - 0.5).abs() < 1e-10);
        assert!((cov[1][1] - 0.125).abs() < 1e-10);
    }

    #[test]
    fn test_invalid_system_negative_diagonal() {
        struct BadSystem;
        impl CostProblem<1> for BadSystem {
            type Error = TestError;
            fn evaluate_cost(&mut self, _x: &[f64; 1]) -> Result<f64, TestError> {
                Ok(1.0)
            }
        }
        impl SystemProblem<1> for BadSystem {
            fn evaluate_system(&mut self, _x: &[f64; 1]) -> Result<SystemEvaluation<1>, TestError> {
                Ok(SystemEvaluation {
                    cost: 1.0,
                    normal: [[-1.0]],
                    rhs: [0.0],
                })
            }
        }
        let err = solve_system(&mut BadSystem, [0.0], &config()).unwrap_err();
        assert!(
            matches!(
                err,
                LMError::InvalidSystem {
                    iteration: 0,
                    defect: SystemDefect::NegativeDiagonal { index: 0, .. }
                }
            ),
            "{err:?}"
        );
    }

    #[test]
    fn test_invalid_system_nonfinite() {
        struct NanSystem;
        impl CostProblem<1> for NanSystem {
            type Error = TestError;
            fn evaluate_cost(&mut self, _x: &[f64; 1]) -> Result<f64, TestError> {
                Ok(1.0)
            }
        }
        impl SystemProblem<1> for NanSystem {
            fn evaluate_system(&mut self, _x: &[f64; 1]) -> Result<SystemEvaluation<1>, TestError> {
                Ok(SystemEvaluation {
                    cost: 1.0,
                    normal: [[f64::NAN]],
                    rhs: [0.0],
                })
            }
        }
        let err = solve_system(&mut NanSystem, [0.0], &config()).unwrap_err();
        assert!(
            matches!(
                err,
                LMError::InvalidSystem {
                    iteration: 0,
                    defect: SystemDefect::NonFiniteNormal { row: 0, col: 0 }
                }
            ),
            "{err:?}"
        );
    }

    // ── Known-answer check on an exactly linear problem ──

    #[test]
    fn test_exact_solution_on_linear() {
        // Data lying exactly on \( y = 2x + 1 \): the least-squares
        // solution is \( p = [2, 1] \) with zero residual.
        let xs = [0.0, 1.0, 2.0, 3.0];
        let ys = [1.0, 3.0, 5.0, 7.0];

        let mut p = Tracked::new(move |p: &[f64; 2]| {
            let residuals: Vec<f64> = xs
                .iter()
                .zip(ys.iter())
                .map(|(&x, &y)| p[0] * x + p[1] - y)
                .collect();
            let jacobian: Vec<[f64; 2]> = xs.iter().map(|&x| [x, 1.0]).collect();
            (residuals, jacobian)
        });
        let mut cfg = config();
        cfg.xtol = 1e-14;
        let solution = solve(&mut p, [0.0; 2], &cfg, None).unwrap();

        assert!(solution.converged);
        assert!((solution.x[0] - 2.0).abs() < 1e-9);
        assert!((solution.x[1] - 1.0).abs() < 1e-9);
    }

    // ── Acceptance-test edge cases ──

    /// THE false-convergence regression that the 2020 CD3 capture-
    /// spanning fit exposed: on a stiff valley the rejections inflate
    /// μ by orders of magnitude, and a μ-crushed accepted step has a
    /// tiny quadratic form at an arbitrarily BAD iterate. Step-based
    /// convergence must therefore be judged on the UNDAMPED
    /// Gauss-Newton step — a μ-starved accepted step must never read
    /// as "converged".
    ///
    /// Adversarial cost: improvements exist only at micro-scale
    /// (|h| < 1e-7), so every natural-scale trial is rejected, μ
    /// inflates, and eventually micro-steps get accepted. The undamped
    /// GN step (and the gradient) remain large throughout — the solver
    /// must NOT declare convergence, no matter how generous qtol is.
    #[test]
    fn test_mu_starved_accepted_step_is_not_convergence() {
        // Positional cost: a microscopic basin at the origin inside a
        // wall — every macro trial is rejected (cost rises), only
        // |x| < 1e-7 micro-steps improve, while the residual/Jacobian
        // claim a huge gradient toward 1000 (so the undamped GN step
        // and the gradient stay enormous).
        fn stiff_cost(x: f64) -> f64 {
            if x.abs() < 1e-7 {
                1e6 - 1e-3 * (1e-7 - x.abs()) / 1e-7
            } else {
                1e6 + 1.0
            }
        }
        struct StiffValley;
        impl CostProblem<1> for StiffValley {
            type Error = TestError;
            fn evaluate_cost(&mut self, x: &[f64; 1]) -> Result<f64, TestError> {
                Ok(stiff_cost(x[0]))
            }
        }
        impl ResidualProblem<1> for StiffValley {
            fn evaluate(&mut self, x: &[f64; 1]) -> Result<NLLSEvaluation<1>, TestError> {
                Ok(NLLSEvaluation {
                    residuals: vec![x[0] - 1000.0],
                    jacobian: vec![[1.0]],
                    cost: stiff_cost(x[0]),
                })
            }
        }
        let mut cfg = config();
        cfg.gtol = 0.0;
        cfg.xtol = 0.0;
        cfg.ftol = 0.0;
        cfg.qtol = 1.0; // generous: any micro-step is far below this
        cfg.max_iterations = 30;
        let sol = solve(&mut StiffValley, [0.0], &cfg, None).unwrap();
        // Whatever the budget outcome, a step-based "converged" at the
        // garbage iterate is forbidden.
        assert_ne!(sol.reason, TerminationReason::StepTolerance, "{sol:?}");
        assert_ne!(sol.reason, TerminationReason::CostTolerance, "{sol:?}");
        assert!(!sol.converged, "{:?}", sol.reason);
        assert!(sol.x[0].abs() < 1.0, "stayed near start: {}", sol.x[0]);
    }

    /// Monotonicity guard: when the trial cost claims an improvement
    /// but the full evaluation at the provisionally accepted point
    /// shows a HIGHER cost (trial/full objective inconsistency — the
    /// problem-contract violation the design doc warns about), the
    /// acceptance must be rolled back, nothing committed, and the
    /// inconsistency surfaced as persistent invalid trials.
    #[test]
    fn test_rollback_on_inconsistent_full_evaluation() {
        struct Inconsistent {
            full_calls: usize,
            committed: Vec<f64>,
        }
        impl CostProblem<1> for Inconsistent {
            type Error = TestError;
            fn evaluate_cost(&mut self, x: &[f64; 1]) -> Result<f64, TestError> {
                // Honest, improving trial cost.
                Ok((x[0] - 3.0) * (x[0] - 3.0))
            }
            fn on_step_accepted(&mut self, x: &[f64; 1]) {
                self.committed.push(x[0]);
            }
        }
        impl ResidualProblem<1> for Inconsistent {
            fn evaluate(&mut self, x: &[f64; 1]) -> Result<NLLSEvaluation<1>, TestError> {
                self.full_calls += 1;
                if self.full_calls == 1 {
                    // Honest at x0.
                    return Ok(NLLSEvaluation {
                        residuals: vec![x[0] - 3.0],
                        jacobian: vec![[1.0]],
                        cost: (x[0] - 3.0) * (x[0] - 3.0),
                    });
                }
                // Full evaluation disagrees: claims things got WORSE.
                Ok(NLLSEvaluation {
                    residuals: vec![100.0],
                    jacobian: vec![[1.0]],
                    cost: 10000.0,
                })
            }
        }
        let mut p = Inconsistent {
            full_calls: 0,
            committed: Vec::new(),
        };
        let err = solve(&mut p, [0.0], &config(), None).unwrap_err();
        match err {
            LMError::PersistentInvalidTrials {
                best_x, best_cost, ..
            } => {
                assert_eq!(best_x, vec![0.0], "iterate must not move");
                assert!((best_cost - 9.0).abs() < 1e-12);
            }
            other => panic!("expected PersistentInvalidTrials, got {other:?}"),
        }
        assert_eq!(
            p.committed,
            vec![0.0],
            "only x0 may commit; rolled-back acceptances must not"
        );
    }

    /// pred ≤ 0 forces rejection even when the trial cost IMPROVES:
    /// accepting a step the local model calls bad would feed a
    /// negative ρ into Nielsen's update. A clamp that teleports the
    /// iterate to the mirror minimum (cost 0!) must still be rejected.
    #[test]
    fn test_pred_nonpositive_forces_rejection() {
        struct Teleport {
            inner: Tracked<fn(&[f64; 1]) -> Resid1>,
        }
        impl CostProblem<1> for Teleport {
            type Error = TestError;
            fn evaluate_cost(&mut self, x: &[f64; 1]) -> Result<f64, TestError> {
                self.inner.evaluate_cost(x)
            }
            fn constrain_step(&mut self, x: &[f64; 1], delta: &mut [f64; 1]) {
                // Adversarial clamp: from the x = 3 region, jump
                // straight to the mirror minimum at −2.
                delta[0] = -2.0 - x[0];
            }
        }
        impl ResidualProblem<1> for Teleport {
            fn evaluate(&mut self, x: &[f64; 1]) -> Result<NLLSEvaluation<1>, TestError> {
                self.inner.evaluate(x)
            }
        }
        // r = x² − 4: minima at ±2. From x0 = 3 the model's descent
        // direction is toward +2; the clamped step to −2 has pred < 0.
        fn bimodal(x: &[f64; 1]) -> Resid1 {
            (vec![x[0] * x[0] - 4.0], vec![[2.0 * x[0]]])
        }
        let mut cfg = config();
        cfg.gtol = 0.0;
        cfg.ftol = 0.0;
        cfg.xtol = 0.0;
        cfg.qtol = 0.0;
        cfg.max_iterations = 3;
        let mut p = Teleport {
            inner: Tracked::new(bimodal as fn(&[f64; 1]) -> _),
        };
        let sol = solve(&mut p, [3.0], &cfg, None).unwrap();
        // Every clamped trial lands at −2 with cost 0 — a genuine
        // improvement — yet pred < 0 must reject every one of them.
        assert_eq!(sol.x[0], 3.0, "model-inconsistent step was accepted");
        assert!(!sol.converged);
        assert!(sol.n_rejected_trials > 0);
        assert!(p.inner.accepted.is_empty());
    }

    /// NaN trial costs follow the same invalid-trial path as INF.
    #[test]
    fn test_nan_trial_cost_is_invalid() {
        struct NanCost;
        impl CostProblem<1> for NanCost {
            type Error = TestError;
            fn evaluate_cost(&mut self, _x: &[f64; 1]) -> Result<f64, TestError> {
                Ok(f64::NAN)
            }
        }
        impl ResidualProblem<1> for NanCost {
            fn evaluate(&mut self, x: &[f64; 1]) -> Result<NLLSEvaluation<1>, TestError> {
                Ok(NLLSEvaluation {
                    residuals: vec![x[0] - 3.0],
                    jacobian: vec![[1.0]],
                    cost: (x[0] - 3.0) * (x[0] - 3.0),
                })
            }
        }
        let err = solve(&mut NanCost, [0.0], &config(), None).unwrap_err();
        match err {
            LMError::PersistentInvalidTrials {
                consecutive,
                last_source,
                ..
            } => {
                assert_eq!(consecutive, 5);
                assert_eq!(last_source, None, "NaN carries no domain error");
            }
            other => panic!("expected PersistentInvalidTrials, got {other:?}"),
        }
    }

    // ── Negative-cost validation (NaN-poisoned gradient regression) ──

    /// A negative cost would NaN-poison √Φ in the gradient test and
    /// read as instant false convergence — it must be a loud error on
    /// the residual path, including at cancellation scale.
    #[test]
    fn test_negative_cost_residual_path_is_error() {
        for bad in [-1.0, -1e-30] {
            struct NegCost(f64);
            impl CostProblem<1> for NegCost {
                type Error = TestError;
                fn evaluate_cost(&mut self, _x: &[f64; 1]) -> Result<f64, TestError> {
                    Ok(self.0)
                }
            }
            impl ResidualProblem<1> for NegCost {
                fn evaluate(&mut self, x: &[f64; 1]) -> Result<NLLSEvaluation<1>, TestError> {
                    Ok(NLLSEvaluation {
                        residuals: vec![x[0] - 1000.0],
                        jacobian: vec![[1.0]],
                        cost: self.0,
                    })
                }
            }
            let err = solve(&mut NegCost(bad), [0.0], &config(), None).unwrap_err();
            assert!(
                matches!(
                    err,
                    LMError::InvalidEvaluation {
                        iteration: 0,
                        defect: EvaluationDefect::NegativeCost { .. }
                    }
                ),
                "cost={bad}: {err:?}"
            );
        }
    }

    /// Same regression on the system path — the realistic trigger is
    /// a Schur profiled cost rounding negative through cancellation.
    #[test]
    fn test_negative_cost_system_path_is_error() {
        for bad in [-5.0, -1e-30] {
            struct NegSystem(f64);
            impl CostProblem<1> for NegSystem {
                type Error = TestError;
                fn evaluate_cost(&mut self, _x: &[f64; 1]) -> Result<f64, TestError> {
                    Ok(self.0)
                }
            }
            impl SystemProblem<1> for NegSystem {
                fn evaluate_system(
                    &mut self,
                    _x: &[f64; 1],
                ) -> Result<SystemEvaluation<1>, TestError> {
                    Ok(SystemEvaluation {
                        cost: self.0,
                        normal: [[1.0]],
                        rhs: [1000.0],
                    })
                }
            }
            let err = solve_system(&mut NegSystem(bad), [0.0], &config()).unwrap_err();
            assert!(
                matches!(
                    err,
                    LMError::InvalidSystem {
                        iteration: 0,
                        defect: SystemDefect::NegativeCost { .. }
                    }
                ),
                "cost={bad}: {err:?}"
            );
        }
    }

    // ── Per-variant coverage for the remaining defect axes ──

    #[test]
    fn test_nonfinite_residual_and_jacobian_entries_are_errors() {
        // The cost must be finite so the per-entry checks (not the
        // cost check) are what fire.
        struct NanResidual;
        impl CostProblem<1> for NanResidual {
            type Error = TestError;
            fn evaluate_cost(&mut self, _x: &[f64; 1]) -> Result<f64, TestError> {
                Ok(1.0)
            }
        }
        impl ResidualProblem<1> for NanResidual {
            fn evaluate(&mut self, _x: &[f64; 1]) -> Result<NLLSEvaluation<1>, TestError> {
                Ok(NLLSEvaluation {
                    residuals: vec![f64::NAN],
                    jacobian: vec![[0.0]],
                    cost: 1.0,
                })
            }
        }
        let err = solve(&mut NanResidual, [0.0], &config(), None).unwrap_err();
        assert!(
            matches!(
                err,
                LMError::InvalidEvaluation {
                    iteration: 0,
                    defect: EvaluationDefect::Residual { index: 0 }
                }
            ),
            "{err:?}"
        );

        let mut p = Tracked::new(|_x: &[f64; 1]| (vec![1.0], vec![[f64::INFINITY]]));
        let err = solve(&mut p, [0.0], &config(), None).unwrap_err();
        assert!(
            matches!(
                err,
                LMError::InvalidEvaluation {
                    iteration: 0,
                    defect: EvaluationDefect::JacobianEntry { row: 0, col: 0 }
                }
            ),
            "{err:?}"
        );
    }

    #[test]
    fn test_system_nonfinite_cost_and_rhs_are_errors() {
        struct BadSys {
            cost: f64,
            rhs: f64,
        }
        impl CostProblem<1> for BadSys {
            type Error = TestError;
            fn evaluate_cost(&mut self, _x: &[f64; 1]) -> Result<f64, TestError> {
                Ok(1.0)
            }
        }
        impl SystemProblem<1> for BadSys {
            fn evaluate_system(&mut self, _x: &[f64; 1]) -> Result<SystemEvaluation<1>, TestError> {
                Ok(SystemEvaluation {
                    cost: self.cost,
                    normal: [[1.0]],
                    rhs: [self.rhs],
                })
            }
        }
        let err = solve_system(
            &mut BadSys {
                cost: f64::NAN,
                rhs: 0.0,
            },
            [0.0],
            &config(),
        )
        .unwrap_err();
        assert!(
            matches!(
                err,
                LMError::InvalidSystem {
                    iteration: 0,
                    defect: SystemDefect::NonFiniteCost
                }
            ),
            "{err:?}"
        );

        let err = solve_system(
            &mut BadSys {
                cost: 1.0,
                rhs: f64::NAN,
            },
            [0.0],
            &config(),
        )
        .unwrap_err();
        assert!(
            matches!(
                err,
                LMError::InvalidSystem {
                    iteration: 0,
                    defect: SystemDefect::NonFiniteRhs { index: 0 }
                }
            ),
            "{err:?}"
        );
    }

    #[test]
    fn test_invalid_prior_nonfinite_covariance_inv() {
        let mut p = Tracked::new(|x: &[f64; 1]| (vec![x[0]], vec![[1.0]]));
        let prior = NLLSPrior {
            mean: [0.0],
            covariance_inv: [[f64::NAN]],
        };
        let err = solve(&mut p, [0.0], &config(), Some(&prior)).unwrap_err();
        assert!(
            matches!(
                err,
                LMError::InvalidPrior {
                    defect: PriorDefect::NonFiniteCovarianceInv { row: 0, col: 0 }
                }
            ),
            "{err:?}"
        );
    }

    #[test]
    fn test_invalid_config_tau_exceeds_mu_max() {
        let mut cfg = config();
        cfg.tau = 1.0;
        cfg.mu_max = 0.5;
        let mut p = Tracked::new(|x: &[f64; 1]| (vec![x[0]], vec![[1.0]]));
        let err = solve(&mut p, [1.0], &cfg, None).unwrap_err();
        assert!(
            matches!(
                err,
                LMError::InvalidConfig {
                    defect: ConfigDefect::TauExceedsMuMax { .. }
                }
            ),
            "{err:?}"
        );
    }

    /// An asymmetric (at rounding level) prior precision matrix — the
    /// realistic LU-inversion artifact — is symmetrized on receipt,
    /// not rejected, and the solve proceeds to the exact MAP point of
    /// the symmetrized objective.
    #[test]
    fn test_asymmetric_prior_is_symmetrized_not_rejected() {
        let mut p = Tracked::new(|x: &[f64; 2]| {
            (vec![x[0] - 10.0, x[1] - 10.0], vec![[1.0, 0.0], [0.0, 1.0]])
        });
        // Off-diagonal asymmetric by 1e-16-scale, as an LU-derived
        // inverse would be.
        let prior = NLLSPrior {
            mean: [0.0, 0.0],
            covariance_inv: [[1.0, 0.1 + 1e-16], [0.1 - 1e-16, 1.0]],
        };
        let sol = solve(&mut p, [0.0, 0.0], &config(), Some(&prior)).unwrap();
        assert!(sol.converged, "{:?}", sol.reason);
        // MAP of (x−10)² + xᵀPx per component with symmetrized
        // P = [[1, 0.1], [0.1, 1]]: solve (I + P) x = (10, 10).
        // (2, 0.1; 0.1, 2) x = (10,10) → x = 10/2.1 each.
        let expected = 10.0 / 2.1;
        assert!((sol.x[0] - expected).abs() < 1e-6, "x0={}", sol.x[0]);
        assert!((sol.x[1] - expected).abs() < 1e-6, "x1={}", sol.x[1]);
    }

    // ── Rollback axes beyond cost inconsistency ──

    /// Domain error from the full evaluation at a provisionally
    /// accepted point: rolled back, surfaced as persistent invalid
    /// trials with the domain error retained.
    #[test]
    fn test_rollback_on_domain_error_at_accepted_point() {
        struct FailsAfterFirst {
            full_calls: usize,
        }
        impl CostProblem<1> for FailsAfterFirst {
            type Error = TestError;
            fn evaluate_cost(&mut self, x: &[f64; 1]) -> Result<f64, TestError> {
                Ok((x[0] - 3.0) * (x[0] - 3.0))
            }
        }
        impl ResidualProblem<1> for FailsAfterFirst {
            fn evaluate(&mut self, x: &[f64; 1]) -> Result<NLLSEvaluation<1>, TestError> {
                self.full_calls += 1;
                if self.full_calls > 1 {
                    return Err(TestError("propagation died at accepted point"));
                }
                Ok(NLLSEvaluation {
                    residuals: vec![x[0] - 3.0],
                    jacobian: vec![[1.0]],
                    cost: (x[0] - 3.0) * (x[0] - 3.0),
                })
            }
        }
        let err = solve(
            &mut FailsAfterFirst { full_calls: 0 },
            [0.0],
            &config(),
            None,
        )
        .unwrap_err();
        match err {
            LMError::PersistentInvalidTrials {
                last_source,
                best_x,
                ..
            } => {
                assert_eq!(
                    last_source,
                    Some(TestError("propagation died at accepted point"))
                );
                assert_eq!(best_x, vec![0.0]);
            }
            other => panic!("expected PersistentInvalidTrials, got {other:?}"),
        }
    }

    /// A mid-solve CONTRACT violation (dimension mismatch at an
    /// accepted point) is fatal on its own axis with the right
    /// iteration — not recycled into a rollback.
    #[test]
    fn test_hard_defect_mid_solve_is_fatal() {
        struct BreaksContract {
            full_calls: usize,
        }
        impl CostProblem<1> for BreaksContract {
            type Error = TestError;
            fn evaluate_cost(&mut self, x: &[f64; 1]) -> Result<f64, TestError> {
                Ok((x[0] - 3.0) * (x[0] - 3.0))
            }
        }
        impl ResidualProblem<1> for BreaksContract {
            fn evaluate(&mut self, x: &[f64; 1]) -> Result<NLLSEvaluation<1>, TestError> {
                self.full_calls += 1;
                if self.full_calls > 1 {
                    return Ok(NLLSEvaluation {
                        residuals: vec![1.0, 2.0],
                        jacobian: vec![[1.0]],
                        cost: 5.0,
                    });
                }
                Ok(NLLSEvaluation {
                    residuals: vec![x[0] - 3.0],
                    jacobian: vec![[1.0]],
                    cost: (x[0] - 3.0) * (x[0] - 3.0),
                })
            }
        }
        let err = solve(
            &mut BreaksContract { full_calls: 0 },
            [0.0],
            &config(),
            None,
        )
        .unwrap_err();
        assert!(
            matches!(
                err,
                LMError::DimensionMismatch {
                    iteration: 1,
                    residuals: 2,
                    jacobian: 1
                }
            ),
            "{err:?}"
        );
    }

    /// An overflowing predicted reduction (INF/NaN from an adversarial
    /// clamp) is a forced rejection — never a panic, never an accept.
    #[test]
    fn test_pred_overflow_forces_rejection() {
        struct HugeClamp {
            inner: Tracked<fn(&[f64; 1]) -> Resid1>,
        }
        impl CostProblem<1> for HugeClamp {
            type Error = TestError;
            fn evaluate_cost(&mut self, _x: &[f64; 1]) -> Result<f64, TestError> {
                // Adversarial liar: claims a perfect fit at the
                // teleported point, so ONLY the pred > 0 guard stands
                // between the solver and a model-inconsistent accept.
                Ok(0.0)
            }
            fn constrain_step(&mut self, _x: &[f64; 1], delta: &mut [f64; 1]) {
                delta[0] = 1e200;
            }
        }
        impl ResidualProblem<1> for HugeClamp {
            fn evaluate(&mut self, x: &[f64; 1]) -> Result<NLLSEvaluation<1>, TestError> {
                self.inner.evaluate(x)
            }
        }
        fn scaled(x: &[f64; 1]) -> Resid1 {
            (vec![1e150 * (x[0] - 1.0)], vec![[1e150]])
        }
        let mut cfg = config();
        cfg.gtol = 0.0;
        cfg.qtol = 0.0;
        cfg.xtol = 0.0;
        cfg.ftol = 0.0;
        cfg.max_iterations = 3;
        let mut p = HugeClamp {
            inner: Tracked::new(scaled as fn(&[f64; 1]) -> _),
        };
        let sol = solve(&mut p, [0.0], &cfg, None).unwrap();
        assert_eq!(sol.x[0], 0.0, "overflowed-pred step must not be accepted");
        assert!(!sol.converged);
    }

    /// Problem-side committed state is BIT-identical across rejected
    /// trials: stage in evaluate_cost, commit only in
    /// on_step_accepted, and verify the committed bits never moved on
    /// an always-rejecting run.
    #[test]
    fn test_committed_state_bit_identical_across_rejections() {
        struct Staged {
            committed: u64,
            pending: u64,
        }
        impl CostProblem<1> for Staged {
            type Error = TestError;
            fn evaluate_cost(&mut self, x: &[f64; 1]) -> Result<f64, TestError> {
                self.pending = x[0].to_bits();
                Ok(10.0) // adversarial: every trial is rejected
            }
            fn on_step_accepted(&mut self, _x: &[f64; 1]) {
                self.committed = self.pending;
            }
        }
        impl ResidualProblem<1> for Staged {
            fn evaluate(&mut self, x: &[f64; 1]) -> Result<NLLSEvaluation<1>, TestError> {
                self.pending = x[0].to_bits();
                Ok(NLLSEvaluation {
                    residuals: vec![1.0],
                    jacobian: vec![[1.0]],
                    cost: 1.0,
                })
            }
        }
        let mut cfg = config();
        cfg.gtol = 0.0;
        cfg.qtol = 0.0;
        cfg.xtol = 0.0;
        cfg.ftol = 0.0;
        let mut p = Staged {
            committed: u64::MAX,
            pending: u64::MAX,
        };
        let x0 = 5.0_f64;
        let sol = solve(&mut p, [x0], &cfg, None).unwrap();
        assert!(!sol.converged);
        assert_eq!(
            p.committed,
            x0.to_bits(),
            "committed state must still be the x0 commit, bit-for-bit"
        );
    }

    // ── Geodesic acceleration ──

    /// Rosenbrock with the EXACT directional second derivative
    /// supplied by the problem (what a one-parameter Jet2 evaluation
    /// provides in production): r₁ = 10(x₂ − x₁²) has
    /// ∂²r₁/∂x₁² = −20, so r''_vv = [−20 v₁², 0].
    struct RosenbrockGeo {
        hook_calls: std::cell::Cell<usize>,
    }
    impl CostProblem<2> for RosenbrockGeo {
        type Error = TestError;
        fn evaluate_cost(&mut self, x: &[f64; 2]) -> Result<f64, TestError> {
            let r1 = 10.0 * (x[1] - x[0] * x[0]);
            let r2 = 1.0 - x[0];
            Ok(r1 * r1 + r2 * r2)
        }
        fn second_directional_derivative(
            &mut self,
            _x: &[f64; 2],
            v: &[f64; 2],
        ) -> Option<Vec<f64>> {
            self.hook_calls.set(self.hook_calls.get() + 1);
            Some(vec![-20.0 * v[0] * v[0], 0.0])
        }
    }
    impl ResidualProblem<2> for RosenbrockGeo {
        fn evaluate(&mut self, x: &[f64; 2]) -> Result<NLLSEvaluation<2>, TestError> {
            let r1 = 10.0 * (x[1] - x[0] * x[0]);
            let r2 = 1.0 - x[0];
            Ok(NLLSEvaluation {
                residuals: vec![r1, r2],
                jacobian: vec![[-20.0 * x[0], 10.0], [-1.0, 0.0]],
                cost: r1 * r1 + r2 * r2,
            })
        }
    }

    /// Acceleration must reach the same minimum in fewer cost
    /// evaluations on the canonical curved-valley problem.
    #[test]
    fn test_geodesic_accelerates_rosenbrock() {
        let run = |geodesic: bool| {
            let mut cfg = config();
            cfg.geodesic_acceleration = geodesic;
            cfg.max_iterations = 500;
            let mut p = RosenbrockGeo {
                hook_calls: std::cell::Cell::new(0),
            };
            let sol = solve(&mut p, [-1.2, 1.0], &cfg, None).unwrap();
            (sol, p.hook_calls.get())
        };
        let (off, off_calls) = run(false);
        let (on, on_calls) = run(true);
        assert_eq!(off_calls, 0, "hook must not be called with the flag off");
        assert!(on_calls > 0, "hook must be exercised with the flag on");
        assert!(off.converged && on.converged);
        assert!((on.x[0] - 1.0).abs() < 1e-6 && (on.x[1] - 1.0).abs() < 1e-6);
        assert!((off.x[0] - 1.0).abs() < 1e-6 && (off.x[1] - 1.0).abs() < 1e-6);
        assert!(on.n_accelerated_trials > 0);
        assert!(
            on.n_cost_evals < off.n_cost_evals,
            "acceleration must reduce cost evaluations: on={} off={}",
            on.n_cost_evals,
            off.n_cost_evals
        );
    }

    /// An untrustworthy expansion (huge curvature) trips the avmax
    /// guard: the trial is rejected through μ escalation with no cost
    /// spent, no acceleration is ever applied, and the solve still
    /// finishes honestly.
    #[test]
    fn test_geodesic_avmax_guard_rejects_huge_curvature() {
        struct HugeCurvature;
        impl CostProblem<1> for HugeCurvature {
            type Error = TestError;
            fn evaluate_cost(&mut self, x: &[f64; 1]) -> Result<f64, TestError> {
                Ok((x[0] - 3.0) * (x[0] - 3.0))
            }
            fn second_directional_derivative(
                &mut self,
                _x: &[f64; 1],
                _v: &[f64; 1],
            ) -> Option<Vec<f64>> {
                Some(vec![1e12])
            }
        }
        impl ResidualProblem<1> for HugeCurvature {
            fn evaluate(&mut self, x: &[f64; 1]) -> Result<NLLSEvaluation<1>, TestError> {
                Ok(NLLSEvaluation {
                    residuals: vec![x[0] - 3.0],
                    jacobian: vec![[1.0]],
                    cost: (x[0] - 3.0) * (x[0] - 3.0),
                })
            }
        }
        let mut cfg = config();
        cfg.geodesic_acceleration = true;
        let sol = solve(&mut HugeCurvature, [0.0], &cfg, None).unwrap();
        assert_eq!(sol.n_accelerated_trials, 0);
        assert!(sol.n_rejected_trials > 0, "avmax violations must reject");
        // The guard never blocks honest termination.
        assert!(!sol.converged || (sol.x[0] - 3.0).abs() < 1e-6, "{sol:?}");
    }

    /// A hook returning the wrong number of rows is a contract
    /// violation — loud, on its own axis.
    #[test]
    fn test_geodesic_wrong_length_is_error() {
        struct WrongLen;
        impl CostProblem<1> for WrongLen {
            type Error = TestError;
            fn evaluate_cost(&mut self, x: &[f64; 1]) -> Result<f64, TestError> {
                Ok((x[0] - 3.0) * (x[0] - 3.0))
            }
            fn second_directional_derivative(
                &mut self,
                _x: &[f64; 1],
                _v: &[f64; 1],
            ) -> Option<Vec<f64>> {
                Some(vec![0.0, 0.0, 0.0])
            }
        }
        impl ResidualProblem<1> for WrongLen {
            fn evaluate(&mut self, x: &[f64; 1]) -> Result<NLLSEvaluation<1>, TestError> {
                Ok(NLLSEvaluation {
                    residuals: vec![x[0] - 3.0],
                    jacobian: vec![[1.0]],
                    cost: (x[0] - 3.0) * (x[0] - 3.0),
                })
            }
        }
        let mut cfg = config();
        cfg.geodesic_acceleration = true;
        let err = solve(&mut WrongLen, [0.0], &cfg, None).unwrap_err();
        assert!(matches!(err, LMError::DimensionMismatch { .. }), "{err:?}");
    }

    /// The normal-equations path carries no Jacobian rows, so the
    /// hook must never fire there even with the flag on.
    #[test]
    fn test_geodesic_inert_on_system_path() {
        struct CountingQuad {
            inner: QuadSystem,
            hook_calls: usize,
        }
        impl CostProblem<2> for CountingQuad {
            type Error = TestError;
            fn evaluate_cost(&mut self, x: &[f64; 2]) -> Result<f64, TestError> {
                self.inner.evaluate_cost(x)
            }
            fn second_directional_derivative(
                &mut self,
                _x: &[f64; 2],
                _v: &[f64; 2],
            ) -> Option<Vec<f64>> {
                self.hook_calls += 1;
                Some(vec![0.0])
            }
        }
        impl SystemProblem<2> for CountingQuad {
            fn evaluate_system(&mut self, x: &[f64; 2]) -> Result<SystemEvaluation<2>, TestError> {
                self.inner.evaluate_system(x)
            }
        }
        let mut cfg = config();
        cfg.geodesic_acceleration = true;
        let mut p = CountingQuad {
            inner: QuadSystem {
                q: [[2.0, 0.0], [0.0, 8.0]],
                a: [1.0, 2.0],
                c: 3.0,
            },
            hook_calls: 0,
        };
        let sol = solve_system(&mut p, [10.0, -4.0], &cfg).unwrap();
        assert!(sol.converged);
        assert_eq!(p.hook_calls, 0, "system path must never call the hook");
        assert_eq!(sol.n_accelerated_trials, 0);
    }

    // ── Determinism ──

    /// Golden-trace bit-determinism: the driver is fixed-order scalar
    /// f64 arithmetic (no libm), so on any IEEE 754 platform this
    /// rational-arithmetic problem must reproduce these exact bits.
    /// A failure here means the driver's arithmetic or its evaluation
    /// ORDER changed — treat as a breaking change, not a flake.
    #[test]
    fn test_golden_trace_bit_determinism() {
        let run = || {
            let mut p = Tracked::new(overshoot_rational);
            solve(&mut p, [6.5], &config(), None).unwrap()
        };
        let a = run();
        let b = run();
        assert_eq!(a.x[0].to_bits(), b.x[0].to_bits());
        assert_eq!(a.cost.to_bits(), b.cost.to_bits());
        assert_eq!(a.mu_final.to_bits(), b.mu_final.to_bits());
        assert_eq!(a.iterations, b.iterations);
        assert_eq!(a.n_cost_evals, b.n_cost_evals);
        assert_eq!(a.n_rejected_trials, b.n_rejected_trials);

        // Cross-platform pins (generated on macOS arm64; must hold on
        // every IEEE 754 platform — see module docs on determinism).
        assert_eq!(a.x[0].to_bits(), GOLDEN_X_BITS, "x = {:?}", a.x[0]);
        assert_eq!(a.cost.to_bits(), GOLDEN_COST_BITS, "cost = {:?}", a.cost);
        assert_eq!(
            a.mu_final.to_bits(),
            GOLDEN_MU_FINAL_BITS,
            "mu = {:?}",
            a.mu_final
        );
        assert_eq!(a.iterations, GOLDEN_ITERATIONS);
        assert_eq!(a.n_cost_evals, GOLDEN_COST_EVALS);
        assert_eq!(a.n_rejected_trials, GOLDEN_REJECTED);

        // The full committed-iterate SEQUENCE is part of the trace —
        // not just the endpoint — so an evaluation-order change
        // anywhere in the driver shows up here.
        let mut p = Tracked::new(overshoot_rational);
        let _ = solve(&mut p, [6.5], &config(), None).unwrap();
        let accepted_bits: Vec<u64> = p.accepted.iter().map(|v| v[0].to_bits()).collect();
        let rejected_bits: Vec<u64> = p.rejected.iter().map(|v| v[0].to_bits()).collect();
        assert_eq!(
            accepted_bits, GOLDEN_ACCEPTED_BITS,
            "accepted iterate sequence changed"
        );
        assert_eq!(
            rejected_bits, GOLDEN_REJECTED_BITS,
            "rejected trial sequence changed"
        );
    }

    // ── Model refresh (inexact-model problems) ──

    /// A problem whose objective is an **anchored surrogate**: the
    /// residual is measured against a reference the problem is allowed
    /// to move — but only inside `refresh_model`, never per call.
    /// Every callback is logged so the driver's refresh sequence can
    /// be asserted exactly.
    struct AnchoredSurrogate {
        /// The anchor the current objective is measured against.
        anchor: f64,
        /// Anchor to adopt at the next refresh (one-shot).
        next_anchor: Option<f64>,
        /// `Some` to make the re-assembly after the refresh fail.
        fail_after_refresh: bool,
        log: Vec<(&'static str, f64)>,
    }

    impl AnchoredSurrogate {
        fn new(anchor: f64) -> Self {
            Self {
                anchor,
                next_anchor: None,
                fail_after_refresh: false,
                log: Vec::new(),
            }
        }
        fn residual(&self, x: f64) -> f64 {
            x - self.anchor
        }
        fn steps(&self, kind: &str) -> Vec<f64> {
            self.log
                .iter()
                .filter(|(k, _)| *k == kind)
                .map(|(_, v)| *v)
                .collect()
        }
    }

    impl CostProblem<1> for AnchoredSurrogate {
        type Error = TestError;
        fn evaluate_cost(&mut self, x: &[f64; 1]) -> Result<f64, TestError> {
            self.log.push(("cost", x[0]));
            let r = self.residual(x[0]);
            Ok(r * r)
        }
        fn on_step_accepted(&mut self, x: &[f64; 1]) {
            self.log.push(("accepted", x[0]));
        }
        fn on_step_rejected(&mut self, x: &[f64; 1]) {
            self.log.push(("rejected", x[0]));
        }
        fn refresh_model(&mut self, x: &[f64; 1]) -> bool {
            self.log.push(("refresh", x[0]));
            match self.next_anchor.take() {
                Some(a) => {
                    self.anchor = a;
                    true
                }
                None => false,
            }
        }
    }

    impl ResidualProblem<1> for AnchoredSurrogate {
        fn evaluate(&mut self, x: &[f64; 1]) -> Result<NLLSEvaluation<1>, TestError> {
            self.log.push(("evaluate", x[0]));
            if self.fail_after_refresh && self.next_anchor.is_none() && self.anchor != 1.0 {
                return Err(TestError("refreshed model cannot be evaluated"));
            }
            let r = self.residual(x[0]);
            Ok(NLLSEvaluation {
                residuals: vec![r],
                jacobian: vec![[1.0]],
                cost: r * r,
            })
        }
    }

    /// A `refresh_model` declaration must re-assemble at the CURRENT
    /// accepted point and re-commit it BEFORE any trial of that
    /// iteration — otherwise the driver would damp a system built from
    /// the old model while comparing trials evaluated under the new
    /// one, which is exactly the cross-model gain ratio the hook
    /// exists to prevent.
    #[test]
    fn test_refresh_model_reassembles_and_recommits_before_any_trial() {
        let mut p = AnchoredSurrogate::new(1.0);
        p.next_anchor = Some(3.0);
        let sol = solve(&mut p, [0.0], &config(), None).unwrap();

        // Refresh happened once, at the accepted point x = 0.
        assert_eq!(p.steps("refresh")[0], 0.0);
        // ... and was immediately followed by a full evaluation and a
        // commit at that same point, before the first trial cost.
        let order: Vec<&'static str> = p.log.iter().map(|(k, _)| *k).collect();
        let first_refresh = order.iter().position(|k| *k == "refresh").unwrap();
        assert_eq!(
            &order[first_refresh..first_refresh + 3],
            &["refresh", "evaluate", "accepted"],
            "log: {:?}",
            p.log
        );
        assert_eq!(
            p.log[first_refresh + 1].1,
            0.0,
            "re-assembled at x, not x+h"
        );
        assert_eq!(p.log[first_refresh + 2].1, 0.0, "re-committed at x");

        // The solve minimized the REFRESHED objective, not the one it
        // started with.
        assert!(sol.converged, "{:?}", sol.reason);
        assert!((sol.x[0] - 3.0).abs() < 1e-6, "x = {}", sol.x[0]);
        assert!(sol.cost < 1e-12, "cost = {}", sol.cost);
    }

    /// The default hook is inert: a problem that never refreshes must
    /// spend no extra evaluation and must trace exactly as before.
    #[test]
    fn test_refresh_model_false_spends_no_evaluation() {
        let mut p = AnchoredSurrogate::new(3.0);
        let sol = solve(&mut p, [0.0], &config(), None).unwrap();
        assert!(sol.converged, "{:?}", sol.reason);
        // One evaluation at x0 plus one per accepted trial — no
        // re-assembly anywhere.
        let evaluates = p.steps("evaluate");
        let costs = p.steps("cost");
        assert_eq!(
            evaluates.len(),
            costs.len() + 1,
            "an extra full evaluation appeared without a refresh: {:?}",
            p.log
        );
        assert!(!p.steps("refresh").is_empty(), "hook was never called");
    }

    /// If the re-assembly a refresh requires fails, there is no
    /// retreat — the problem has already switched models, so the
    /// system the driver holds describes an objective the problem no
    /// longer evaluates. It must surface, not silently damp a stale
    /// system.
    #[test]
    fn test_refresh_model_reassembly_failure_surfaces() {
        let mut p = AnchoredSurrogate::new(1.0);
        p.next_anchor = Some(3.0);
        p.fail_after_refresh = true;
        let err = solve(&mut p, [0.0], &config(), None).unwrap_err();
        assert!(
            matches!(err, LMError::ModelRefreshFailed { iteration: 1, .. }),
            "{err:?}"
        );
    }

    /// Residual of the model `RefreshOntoStationaryModel` refreshes
    /// ONTO: constant and tiny, so its undamped Gauss-Newton step is
    /// 1e-9 and its quadratic form 1e-18 — stationary to any sane
    /// `qtol` — while no trial can lower its cost.
    const STATIONARY_RESIDUAL: f64 = 1e-9;

    /// A problem that refreshes onto a model which is ALREADY
    /// stationary at the point the refresh happens, but whose trial
    /// costs never improve — so the trial loop can only escalate μ
    /// until the damping cap. This is the shape of a surrogate handing
    /// back to the full model at a converged iterate: the handover
    /// lands on a point the NEW model also calls converged, and every
    /// step it can propose is below the noise.
    struct RefreshOntoStationaryModel {
        refreshed: bool,
    }

    impl RefreshOntoStationaryModel {
        /// Before the refresh, r = x − 1 (so x₀ = 0 is nowhere near
        /// stationary and the iteration-1 battery cannot fire). After
        /// it, the constant stationary residual.
        fn residual(&self, x: f64) -> f64 {
            if self.refreshed {
                STATIONARY_RESIDUAL
            } else {
                x - 1.0
            }
        }
    }

    impl CostProblem<1> for RefreshOntoStationaryModel {
        type Error = TestError;
        fn evaluate_cost(&mut self, x: &[f64; 1]) -> Result<f64, TestError> {
            let r = self.residual(x[0]);
            Ok(r * r)
        }
        fn refresh_model(&mut self, _x: &[f64; 1]) -> bool {
            if self.refreshed {
                false
            } else {
                self.refreshed = true;
                true
            }
        }
    }

    impl ResidualProblem<1> for RefreshOntoStationaryModel {
        fn evaluate(&mut self, x: &[f64; 1]) -> Result<NLLSEvaluation<1>, TestError> {
            let r = self.residual(x[0]);
            Ok(NLLSEvaluation {
                residuals: vec![r],
                jacobian: vec![[1.0]],
                cost: r * r,
            })
        }
    }

    /// The convergence battery must re-run against the model a refresh
    /// installed, at the point it was installed at, BEFORE the trial
    /// loop. Without it the driver walks straight into the trials,
    /// exhausts damping and reports FAILURE at a point where the
    /// undamped Gauss-Newton step — the quantity `qtol` is decided on
    /// — is twelve orders of magnitude below the tolerance.
    #[test]
    fn test_refresh_model_convergence_is_tested_on_the_new_model() {
        let mut cfg = config();
        cfg.qtol = 1e-12;
        let mut p = RefreshOntoStationaryModel { refreshed: false };
        let sol = solve(&mut p, [0.0], &cfg, None).unwrap();

        assert!(
            sol.converged,
            "declared failure at a point the refreshed model calls converged: {:?}",
            sol.reason
        );
        assert_eq!(sol.reason, TerminationReason::StepTolerance);
        // The refresh happened at x₀ and the driver never left it —
        // the new model was stationary there.
        assert_eq!(sol.x[0], 0.0);
        assert_eq!(sol.iterations, 1);
        // Not one trial was spent: the battery ran first.
        assert_eq!(sol.n_cost_evals, 0);
        // And the reported metric agrees with the reason given.
        let gn = sol.final_gn_qnorm.expect("well-posed after the refresh");
        assert!(gn <= cfg.qtol, "{gn:.3e} vs qtol {:.3e}", cfg.qtol);
    }

    /// The other direction: the post-refresh battery must not fire on
    /// a model that is NOT stationary at the handover point. The
    /// refresh here moves the anchor to 3 while the driver sits at 0,
    /// so the solve must go on to minimize the new objective.
    #[test]
    fn test_refresh_model_does_not_terminate_on_a_moving_model() {
        let mut cfg = config();
        cfg.qtol = 1e-12;
        let mut p = AnchoredSurrogate::new(1.0);
        p.next_anchor = Some(3.0);
        let sol = solve(&mut p, [0.0], &cfg, None).unwrap();
        assert!(sol.converged, "{:?}", sol.reason);
        assert!((sol.x[0] - 3.0).abs() < 1e-6, "x = {}", sol.x[0]);
        assert!(
            !p.steps("cost").is_empty(),
            "the post-refresh battery swallowed the trial loop"
        );
    }

    /// A problem that accepts steps for one iteration and only THEN
    /// refreshes, onto a model that is stationary where it lands.
    struct RefreshAfterAStep {
        refreshed: bool,
        iterations_seen: usize,
    }

    impl RefreshAfterAStep {
        fn residual(&self, x: f64) -> f64 {
            if self.refreshed {
                STATIONARY_RESIDUAL
            } else {
                x - 100.0
            }
        }
    }

    impl CostProblem<1> for RefreshAfterAStep {
        type Error = TestError;
        fn evaluate_cost(&mut self, x: &[f64; 1]) -> Result<f64, TestError> {
            let r = self.residual(x[0]);
            Ok(r * r)
        }
        fn refresh_model(&mut self, _x: &[f64; 1]) -> bool {
            self.iterations_seen += 1;
            if self.refreshed || self.iterations_seen < 2 {
                false
            } else {
                self.refreshed = true;
                true
            }
        }
    }

    impl ResidualProblem<1> for RefreshAfterAStep {
        fn evaluate(&mut self, x: &[f64; 1]) -> Result<NLLSEvaluation<1>, TestError> {
            let r = self.residual(x[0]);
            Ok(NLLSEvaluation {
                residuals: vec![r],
                jacobian: vec![[1.0]],
                cost: r * r,
            })
        }
    }

    /// A model refresh clears the ftol bookkeeping — it must not also
    /// erase the record that a step was ever accepted. The two are
    /// different claims: `actred`/`pred` stop being comparable across a
    /// refresh, but "the last step taken had this size" stays true.
    /// Reporting `None` here says "no step was accepted", which callers
    /// read as "the start point was already stationary" — scott's
    /// `ODResult::update_norm` reports a literal 0.0 on that reading.
    #[test]
    fn test_accepted_step_qnorm_survives_a_model_refresh() {
        let mut cfg = config();
        cfg.qtol = 1e-12;
        let mut p = RefreshAfterAStep {
            refreshed: false,
            iterations_seen: 0,
        };
        let sol = solve(&mut p, [0.0], &cfg, None).unwrap();

        assert!(sol.converged, "{:?}", sol.reason);
        assert!(p.refreshed, "the refresh never happened");
        assert!(
            sol.x[0] != 0.0,
            "the solve must have accepted a step before refreshing"
        );
        let q = sol.accepted_step_qnorm.expect(
            "a solve that accepted a step and then refreshed still accepted a step — \
             None here reads as 'the start point was already stationary'",
        );
        assert!(q > 0.0, "qnorm = {q}");
    }

    // ── Reported step norms ──

    /// `accepted_step_qnorm` and `final_gn_qnorm` measure different
    /// things and only the latter is comparable to `qtol`. On a
    /// μ-starved solve the accepted (damped) step is vanishingly
    /// small at a garbage iterate while the undamped Gauss-Newton step
    /// — the quantity the test is actually decided on — is enormous.
    /// Quoting the former against a tolerance is what makes a failure
    /// message read as "converged".
    #[test]
    fn test_final_gn_qnorm_is_the_convergence_metric_not_the_accepted_step() {
        fn stiff_cost(x: f64) -> f64 {
            if x.abs() < 1e-7 {
                1e6 - 1e-3 * (1e-7 - x.abs()) / 1e-7
            } else {
                1e6 + 1.0
            }
        }
        struct StiffValley;
        impl CostProblem<1> for StiffValley {
            type Error = TestError;
            fn evaluate_cost(&mut self, x: &[f64; 1]) -> Result<f64, TestError> {
                Ok(stiff_cost(x[0]))
            }
        }
        impl ResidualProblem<1> for StiffValley {
            fn evaluate(&mut self, x: &[f64; 1]) -> Result<NLLSEvaluation<1>, TestError> {
                Ok(NLLSEvaluation {
                    residuals: vec![x[0] - 1000.0],
                    jacobian: vec![[1.0]],
                    cost: stiff_cost(x[0]),
                })
            }
        }
        let mut cfg = config();
        cfg.gtol = 0.0;
        cfg.xtol = 0.0;
        cfg.ftol = 0.0;
        cfg.qtol = 1.0;
        cfg.max_iterations = 30;
        let sol = solve(&mut StiffValley, [0.0], &cfg, None).unwrap();
        assert!(!sol.converged, "{:?}", sol.reason);

        let gn = sol.final_gn_qnorm.expect("undamped system is well posed");
        assert!(
            gn > cfg.qtol,
            "the metric convergence is decided on must be ABOVE the tolerance on a \
             non-converged solve: {gn:.3e} vs qtol {:.3e}",
            cfg.qtol
        );
        if let Some(accepted) = sol.accepted_step_qnorm {
            assert!(
                accepted < cfg.qtol,
                "this regression needs a μ-crushed accepted step to be meaningful: {accepted:.3e}"
            );
            assert!(
                gn / accepted > 1e6,
                "the two norms should be orders of magnitude apart: {gn:.3e} vs {accepted:.3e}"
            );
        }
    }

    /// On a converged solve the reported metric is below the tolerance
    /// it was tested against — the other direction of the same
    /// contract.
    #[test]
    fn test_final_gn_qnorm_below_qtol_on_a_step_tolerance_exit() {
        let mut cfg = config();
        cfg.gtol = 0.0;
        cfg.ftol = 0.0;
        cfg.qtol = 1e-12;
        let mut p = Tracked::new(overshoot_rational);
        let sol = solve(&mut p, [6.5], &cfg, None).unwrap();
        assert!(sol.converged, "{:?}", sol.reason);
        let gn = sol.final_gn_qnorm.expect("well-posed at the solution");
        assert!(gn <= cfg.qtol, "{gn:.3e} vs qtol {:.3e}", cfg.qtol);
    }

    const GOLDEN_MU_FINAL_BITS: u64 = 4565320297239836560;
    const GOLDEN_ACCEPTED_BITS: [u64; 10] = [
        4619004367821864960, // x0 = 6.5 (the initial commit)
        4618792513637290406,
        4618291282969069438,
        4617770008120803146,
        4617507755895746747,
        4617364428888646038,
        4617321199543857098,
        4617315762137427473,
        4617315521566082796,
        4617315517979513644, // = GOLDEN_X_BITS (the returned x)
    ];
    const GOLDEN_REJECTED_BITS: [u64; 5] = [
        4594659349414467680,
        4594887588787221536,
        4596247515049879712,
        4602933352259637776,
        4614750054244392936,
    ];

    // x = 5.0000000159096025, cost ≈ 6.3e-15 — the overshoot_rational
    // problem from x0 = 6.5 under the default config, traced on macOS
    // arm64. Pure fixed-order f64 arithmetic end to end, so these bits
    // are the contract on every IEEE 754 platform.
    const GOLDEN_X_BITS: u64 = 4617315517979513644;
    const GOLDEN_COST_BITS: u64 = 4394527585940473101;
    const GOLDEN_ITERATIONS: usize = 10;
    const GOLDEN_COST_EVALS: usize = 14;
    const GOLDEN_REJECTED: usize = 5;
}
