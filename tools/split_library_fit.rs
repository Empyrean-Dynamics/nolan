//! Fitter for the univariate Gaussian splitting library.
//!
//! Not part of the published crate: `Cargo.toml`'s `include` list omits
//! this directory. It is compiled twice from source, by
//! `examples/generate_split_library.rs` (which emits the table) and by
//! `tests/split_library.rs` (which re-solves and checks the committed
//! table), so the emitted numbers and the checked numbers come from one
//! body of code.
//!
//! # What is solved
//!
//! A standard normal \\(p(x) = \mathcal{N}(x; 0, 1)\\) is approximated by a
//! homoscedastic Gaussian mixture
//!
//! \\[
//!   q(x) = \sum_{i=1}^{K} \alpha_i \, \mathcal{N}(x; \mu_i, \sigma^2)
//! \\]
//!
//! whose weights and means are symmetric about the origin, by minimising
//! the \\(L^2\\) distance
//!
//! \\[
//!   J = \int \left(p(x) - q(x)\right)^2 \mathrm{d}x
//!     = \frac{1}{2\sqrt{\pi}}
//!     + \sum_{i}\sum_{j} \frac{\alpha_i \alpha_j}{2\sigma\sqrt{\pi}}
//!       \exp\!\left(\frac{-(\mu_i - \mu_j)^2}{4\sigma^2}\right)
//!     - 2\sum_{i} \frac{\alpha_i}{\sqrt{2\pi(\sigma^2 + 1)}}
//!       \exp\!\left(\frac{-\mu_i^2}{2(\sigma^2 + 1)}\right)
//! \\]
//!
//! with \\(\sigma\\) fixed in advance by a rule in \\(K\\), under one of
//! two constraint sets ([`Constraint`]):
//!
//! - [`Constraint::WeightsOnly`] imposes \\(\sum_i \alpha_i = 1\\) and
//!   nothing more. This is Vittaldev & Russell 2016 (CMES 111(1):
//!   83–117) eq. 4 with their equality constraints, and it is the
//!   problem whose published seven-component table the suite reproduces.
//!   Symmetry and the weight sum reduce the \\(3K\\) raw parameters to
//!   \\(K - 1\\) free ones.
//! - [`Constraint::UnitVariance`] adds
//!   \\(\sigma^2 + \sum_i \alpha_i \mu_i^2 = 1\\), leaving \\(K - 2\\)
//!   free parameters. **This is what the shipped table solves**, because
//!   a split must reproduce the covariance it was handed, and a mixture
//!   that sheds a fixed fraction of the variance at every level
//!   compounds that loss under recursion. The paper's mixtures carry
//!   0.951 of the parent's variance at \\(K = 3\\).
//!
//! The closed form of the integral is why \\(L^2\\) is the metric rather
//! than a divergence needing quadrature.
//!
//! The shipped table takes two passes. A mixture cannot carry unit
//! variance at the rule's width without moving its means outward, so the
//! first pass solves the unconstrained problem at the rule's width to
//! measure that move; the shipped width is the rule's divided by the
//! square root of that solution's variance. The second pass then solves
//! the constrained problem at the shipped width. Dilating the first
//! pass's answer instead would be cheaper and is what an earlier draft
//! did, but a dilated optimum is not an optimum: at its own width it is
//! 38.5% worse in \\(L^2\\) at \\(K = 3\\) and 154% worse at
//! \\(K = 7\\).
//!
//! # How it is solved
//!
//! The reduced problem is a least-squares problem in function space —
//! \\(J = \lVert p - q \rVert^2_{L^2}\\) — so it carries an exact
//! Gauss-Newton system whose normal matrix
//!
//! \\[
//!   A_{ab} = \left\langle \frac{\partial q}{\partial \theta_a},
//!                         \frac{\partial q}{\partial \theta_b} \right\rangle,
//!   \qquad
//!   g_a = \left\langle p - q, \frac{\partial q}{\partial \theta_a} \right\rangle
//! \\]
//!
//! is positive semi-definite by construction and available in closed
//! form: every entry is an overlap integral of two Gaussians or of their
//! derivatives with respect to their means. Under
//! [`Constraint::UnitVariance`] those derivatives also carry a chain rule
//! through the innermost mean, which the variance constraint eliminates
//! (see [`expand`] and `inner_mean_gradient`). Everything is handed to
//! the crate's own Levenberg-Marquardt driver through
//! [`hyperjet::optimization::lm::solve_system`].
//!
//! The closed forms are checked against [`hyperjet::jets::Jet1`]
//! derivatives of the cost in `tests/split_library.rs`, at deliberately
//! non-stationary points: at a solution both gradients vanish whatever
//! the derivation says, so a check made only there would pass on a wrong
//! chain rule.
//!
//! Inequality constraints — means increasing away from the origin,
//! weights decreasing away from it — are enforced by refusing the trial
//! point, which is the driver's documented mechanism for an infeasible
//! evaluation.

#![allow(dead_code)]

use hyperjet::optimization::lm::{
    CostProblem, LMConfig, LMError, SystemEvaluation, SystemProblem, solve_system,
};
use std::f64::consts::PI;

/// The smallest component count the library tabulates.
pub const MIN_K: usize = 2;

/// The largest component count the library tabulates.
pub const MAX_K: usize = 15;

/// The scaled gradient norm at or below which a stopped solve is taken
/// to have found a minimum.
///
/// Measured over the two passes the shipped table is built from, the
/// accepted points land between \\(3.9\times10^{-14}\\) and
/// \\(3.8\times10^{-10}\\), so this bound clears the worst of them by
/// 1.4 orders of magnitude. The constrained pass alone, which produces
/// the entries, runs from \\(9.4\times10^{-14}\\) to
/// \\(6.3\times10^{-11}\\) and clears it by 2.2.
///
/// It is the only acceptance test the fitter applies to a solve that
/// returned `Ok`; see [`fit_sized`] for why the driver's own convergence
/// verdict is not usable on this objective.
pub const GRADIENT_TOLERANCE: f64 = 1e-8;

/// The rule the shipped table is built under.
///
/// [`SigmaRule::Narrow`] is closest to what the crate carried before the
/// library existed and reproduces the parent's tail worst of the three;
/// [`SigmaRule::Wide`] reproduces it best but drives the \(L^2\)
/// distance to \(10^{-13}\) by \(K = 12\), where the objective is
/// assembled as a difference against a leading term of
/// \(1/(2\sqrt{\pi})\) and stops determining the fit — measured, an
/// unbounded solve parks a component at \(7.1\sigma\) at \(K = 14\).
/// [`SigmaRule::Medium`] keeps every entry determined across the whole
/// range and still reproduces the parent's tail three orders better than
/// the narrow rule.
pub const SHIPPED_RULE: SigmaRule = SigmaRule::Medium;

/// Component standard deviation as a function of the component count.
///
/// Vittaldev & Russell fix \\(\sigma\\) in advance rather than optimising
/// it: with \\(\sigma\\) free the \\(L^2\\) problem has the trivial
/// solution \\(\alpha_0 = 1\\), \\(\sigma = 1\\), every other weight zero
/// — one component equal to the parent, at \\(L^2 = 0\\) and no split at
/// all. The optimum runs toward no deflation, not toward narrow
/// components: as \\(\sigma \to 0\\) the component approaches a Dirac
/// delta and the distance diverges. Their three published rules are
/// \\(\sigma^2 = (1/K)^e\\) for \\(e \in \\{1, 3/4, 1/2\\}\\).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SigmaRule {
    /// \\(\sigma^2 = 1/K\\). Vittaldev & Russell rule 1; the narrowest
    /// components, tabulated by them to \\(K = 39\\).
    Narrow,
    /// \\(\sigma^2 = (1/K)^{3/4}\\). Vittaldev & Russell rule 2.
    Medium,
    /// \\(\sigma^2 = (1/K)^{1/2}\\). Vittaldev & Russell rule 3; the
    /// widest components, tabulated by them to \\(K = 15\\).
    Wide,
}

impl SigmaRule {
    /// The exponent \\(e\\) in \\(\sigma^2 = (1/K)^e\\).
    pub fn exponent(self) -> f64 {
        match self {
            Self::Narrow => 1.0,
            Self::Medium => 0.75,
            Self::Wide => 0.5,
        }
    }

    /// Component standard deviation for `k` components.
    pub fn sigma(self, k: usize) -> f64 {
        (1.0 / k as f64).powf(0.5 * self.exponent())
    }

    /// Short label used in generated output.
    pub fn label(self) -> &'static str {
        match self {
            Self::Narrow => "sigma^2 = 1/K",
            Self::Medium => "sigma^2 = (1/K)^(3/4)",
            Self::Wide => "sigma^2 = (1/K)^(1/2)",
        }
    }
}

/// A converged univariate split of the standard normal.
#[derive(Clone, Debug)]
pub struct SplitFit {
    /// Component weights, ordered by increasing mean.
    pub weights: Vec<f64>,
    /// Component means, increasing, symmetric about zero.
    pub means: Vec<f64>,
    /// Common component standard deviation.
    pub sigma: f64,
    /// \\(L^2\\) distance from the standard normal at the solution.
    pub l2: f64,
    /// Mixture variance \\(\sigma^2 + \sum_i \alpha_i \mu_i^2\\).
    pub variance: f64,
    /// Outer iterations the driver spent.
    pub iterations: usize,
    /// Whether a convergence criterion fired.
    pub converged: bool,
    /// Why the driver stopped.
    pub reason: String,
    /// Scaled gradient norm at the returned point.
    pub gradient_norm: f64,
}

impl SplitFit {
    /// The same mixture dilated to unit variance: means and
    /// \\(\sigma\\) divided by \\(\sqrt{\mathrm{variance}}\\), weights
    /// untouched.
    ///
    /// The \\(L^2\\)-optimal mixture at a fixed \\(\sigma\\) does not
    /// reproduce the parent's variance — the constraint set of
    /// Vittaldev & Russell eq. 5 does not ask it to — and a split that
    /// loses a fixed fraction of the variance at every level compounds
    /// that loss under recursion. The dilation is the unique similarity
    /// transform that restores the second moment while preserving the
    /// mixture's shape, its symmetry and its homoscedasticity.
    pub fn to_unit_variance(&self) -> SplitFit {
        let s = self.variance.sqrt();
        let means: Vec<f64> = self.means.iter().map(|m| m / s).collect();
        let sigma = self.sigma / s;
        let l2 = l2_distance(&self.weights, &means, sigma);
        let variance = mixture_variance(&self.weights, &means, sigma);
        SplitFit {
            weights: self.weights.clone(),
            means,
            sigma,
            l2,
            variance,
            iterations: self.iterations,
            converged: self.converged,
            reason: self.reason.clone(),
            gradient_norm: self.gradient_norm,
        }
    }
}

/// Why a fit could not be produced.
#[derive(Clone, Debug, PartialEq)]
pub enum FitError {
    /// `k` is outside `MIN_K..=MAX_K`.
    UnsupportedK(usize),
    /// The seed had the wrong length for `k`.
    SeedLength { expected: usize, got: usize },
    /// The driver stopped without meeting a convergence criterion.
    NotConverged(String),
    /// The converged point violates a constraint of the formulation.
    Infeasible(String),
}

impl std::fmt::Display for FitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedK(k) => write!(f, "k = {k} is outside {MIN_K}..=25"),
            Self::SeedLength { expected, got } => {
                write!(f, "seed has {got} entries, expected {expected}")
            }
            Self::NotConverged(why) => write!(f, "the solve did not converge: {why}"),
            Self::Infeasible(why) => write!(f, "the converged point is infeasible: {why}"),
        }
    }
}

impl std::error::Error for FitError {}

/// Trial rejection carried through the driver's invalid-trial path.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Infeasible;

impl std::fmt::Display for Infeasible {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "trial point violates an inequality constraint")
    }
}

impl std::error::Error for Infeasible {}

// ── Closed-form mixture quantities ──────────────────────────────────

/// \\(\mathcal{N}(u; 0, v)\\).
fn gaussian(u: f64, v: f64) -> f64 {
    (-0.5 * u * u / v).exp() / (2.0 * PI * v).sqrt()
}

/// \\(L^2\\) distance between the standard normal and the mixture.
pub fn l2_distance(weights: &[f64], means: &[f64], sigma: f64) -> f64 {
    let v = 2.0 * sigma * sigma;
    let w = 1.0 + sigma * sigma;
    let mut j = 1.0 / (2.0 * PI.sqrt());
    for (ai, mi) in weights.iter().zip(means) {
        j -= 2.0 * ai * gaussian(*mi, w);
        for (aj, mj) in weights.iter().zip(means) {
            j += ai * aj * gaussian(mi - mj, v);
        }
    }
    j
}

/// Mixture variance \\(\sigma^2 + \sum_i \alpha_i \mu_i^2\\) (the mean is
/// zero by symmetry).
pub fn mixture_variance(weights: &[f64], means: &[f64], sigma: f64) -> f64 {
    sigma * sigma
        + weights
            .iter()
            .zip(means)
            .map(|(a, m)| a * m * m)
            .sum::<f64>()
}

/// Upper-tail mass of the mixture beyond `delta`, divided by the same
/// tail of the standard normal.
pub fn tail_ratio(weights: &[f64], means: &[f64], sigma: f64, delta: f64) -> f64 {
    let mix: f64 = weights
        .iter()
        .zip(means)
        .map(|(a, m)| a * normal_upper_tail((delta - m) / sigma))
        .sum();
    mix / normal_upper_tail(delta)
}

/// \\(Q(z) = 1 - \Phi(z)\\) to full double precision, by the continued
/// fraction of Abramowitz & Stegun 26.2.14 in the tail and the series of
/// 26.2.11 near the origin. The crate's `normal_cdf` carries an absolute
/// error of \\(7.5\times 10^{-8}\\), which says nothing at all about a
/// tail of \\(10^{-12}\\).
pub fn normal_upper_tail(z: f64) -> f64 {
    if z < 0.0 {
        return 1.0 - normal_upper_tail(-z);
    }
    let phi = (-0.5 * z * z).exp() / (2.0 * PI).sqrt();
    if z < 2.0 {
        // Series: Phi(z) - 1/2 = phi(z) * sum_{n>=0} z^(2n+1) / (1*3*...*(2n+1)).
        let mut term = z;
        let mut sum = z;
        let mut n = 1.0;
        while term.abs() > 1e-20 * sum.abs() {
            term *= z * z / (2.0 * n + 1.0);
            sum += term;
            n += 1.0;
        }
        return 0.5 - phi * sum;
    }
    // Continued fraction Q(z) = phi(z) / (z + 1/(z + 2/(z + 3/(z + ...)))).
    let mut f = 0.0_f64;
    for n in (1..=400_usize).rev() {
        f = n as f64 / (z + f);
    }
    phi / (z + f)
}

// ── Parameterisation ────────────────────────────────────────────────

/// Which equality constraints the solve imposes beyond symmetry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Constraint {
    /// \\(\sum_i \alpha_i = 1\\) only. Vittaldev & Russell eq. 5, and the
    /// problem whose published table the suite reproduces.
    WeightsOnly,
    /// \\(\sum_i \alpha_i = 1\\) together with
    /// \\(\sigma^2 + \sum_i \alpha_i \mu_i^2 = 1\\). The shipped table,
    /// because a split must reproduce the covariance it was handed.
    UnitVariance,
}

/// Number of free parameters: \\(K - 1\\) under
/// [`Constraint::WeightsOnly`], one fewer under
/// [`Constraint::UnitVariance`].
pub const fn free_parameters(k: usize, constraint: Constraint) -> usize {
    match constraint {
        Constraint::WeightsOnly => k - 1,
        Constraint::UnitVariance => k - 2,
    }
}

/// Full-array index of the `i`-th strictly positive mean, `i` counted
/// from 1 at the centre outward.
const fn up(k: usize, i: usize) -> usize {
    k / 2 + i - 1 + k % 2
}

/// Full-array index of its mirror below the origin.
const fn low(k: usize, i: usize) -> usize {
    k - 1 - up(k, i)
}

/// Number of free weights in the parameterisation.
const fn free_weights(k: usize) -> usize {
    if k % 2 == 1 { k / 2 } else { k / 2 - 1 }
}

/// Expand the free parameters into full weight and mean arrays ordered
/// by increasing mean.
///
/// Both parameterisations eliminate the LARGEST weight through
/// \\(\sum_i \alpha_i = 1\\): the central one for odd \\(K\\), the
/// innermost pair for even \\(K\\). The weights fall by three or four
/// orders from the centre to the tail, so eliminating an outer one would
/// recover a number near \\(10^{-4}\\) as the difference of numbers near
/// \\(1/2\\) — measured to produce weights of \\(4\times10^{-16}\\), i.e.
/// pure rounding error, at \\(K = 10\\).
///
/// Under [`Constraint::UnitVariance`] the INNERMOST positive mean is
/// eliminated as well, through
/// \\(\sigma^2 + 2\sum_{i\ge 1} \alpha_i \mu_i^2 = 1\\):
/// \\[
///   \mu_1 = \sqrt{\frac{1 - \sigma^2 - 2\sum_{i\ge 2} \alpha_i \mu_i^2}
///                      {2\alpha_1}} .
/// \\]
/// The innermost rather than the outermost, because the constraint's
/// sensitivity to a mean is \\(4\alpha_i \mu_i\\) and the outermost
/// weight is the smallest in the table — \\(2.4\times10^{-4}\\) at
/// \\(K = 15\\) — so recovering the outermost mean would divide by it.
///
/// `None` when the variance constraint has no real solution here, which
/// the caller treats as an infeasible trial.
pub fn expand(
    k: usize,
    theta: &[f64],
    sigma: f64,
    constraint: Constraint,
) -> Option<(Vec<f64>, Vec<f64>)> {
    let n = k / 2;
    let (free_w, free_m) = theta.split_at(free_weights(k));

    // Half-arrays, 1-based over the positive side; index 0 unused.
    let mut half_a = vec![0.0_f64; n + 1];
    let mut half_m = vec![0.0_f64; n + 1];
    let mut centre_weight = 0.0;
    if k % 2 == 1 {
        half_a[1..=n].copy_from_slice(&free_w[..n]);
        centre_weight = 1.0 - 2.0 * free_w.iter().sum::<f64>();
    } else {
        half_a[2..=n].copy_from_slice(&free_w[..n - 1]);
        half_a[1] = 0.5 - free_w.iter().sum::<f64>();
    }

    match constraint {
        Constraint::WeightsOnly => {
            half_m[1..=n].copy_from_slice(&free_m[..n]);
        }
        Constraint::UnitVariance => {
            half_m[2..=n].copy_from_slice(&free_m[..n - 1]);
            let rest: f64 = (2..=n).map(|i| half_a[i] * half_m[i] * half_m[i]).sum();
            let num = 1.0 - sigma * sigma - 2.0 * rest;
            if half_a[1] <= 0.0 || num <= 0.0 || half_a[1].is_nan() || num.is_nan() {
                return None;
            }
            half_m[1] = (num / (2.0 * half_a[1])).sqrt();
        }
    }

    let mut weights = vec![0.0_f64; k];
    let mut means = vec![0.0_f64; k];
    if k % 2 == 1 {
        weights[k / 2] = centre_weight;
    }
    for i in 1..=n {
        weights[up(k, i)] = half_a[i];
        weights[low(k, i)] = half_a[i];
        means[up(k, i)] = half_m[i];
        means[low(k, i)] = -half_m[i];
    }
    Some((weights, means))
}

/// Derivative of the eliminated innermost mean with respect to each free
/// parameter, in `theta` order. Empty under
/// [`Constraint::WeightsOnly`], where no mean is eliminated.
///
/// With \\(\mu_1^2 = N / (2\alpha_1)\\) and
/// \\(N = 1 - \sigma^2 - 2\sum_{i\ge2}\alpha_i\mu_i^2\\):
/// \\[
///   \frac{\partial \mu_1}{\partial \alpha_1} = -\frac{\mu_1}{2\alpha_1},
///   \quad
///   \frac{\partial \mu_1}{\partial \alpha_j} = -\frac{\mu_j^2}{2\alpha_1\mu_1},
///   \quad
///   \frac{\partial \mu_1}{\partial \mu_j} = -\frac{\alpha_j\mu_j}{\alpha_1\mu_1}
///   \qquad (j \ge 2).
/// \\]
/// For even \\(K\\), \\(\alpha_1\\) is itself eliminated as
/// \\(1/2 - \sum_{j\ge2}\alpha_j\\), so a free \\(\alpha_j\\) also moves
/// \\(\mu_1\\) through \\(\alpha_1\\), adding
/// \\(+\mu_1/(2\alpha_1)\\).
fn inner_mean_gradient(
    k: usize,
    weights: &[f64],
    means: &[f64],
    constraint: Constraint,
) -> Vec<f64> {
    if constraint == Constraint::WeightsOnly {
        return Vec::new();
    }
    let n = k / 2;
    let a1 = weights[up(k, 1)];
    let m1 = means[up(k, 1)];
    let mut out = vec![0.0_f64; free_parameters(k, constraint)];
    let nw = free_weights(k);
    if k % 2 == 1 {
        // theta = (alpha_1..alpha_n, mu_2..mu_n)
        out[0] = -m1 / (2.0 * a1);
        for j in 2..=n {
            let mj = means[up(k, j)];
            let aj = weights[up(k, j)];
            out[j - 1] = -mj * mj / (2.0 * a1 * m1);
            out[nw + j - 2] = -aj * mj / (a1 * m1);
        }
    } else {
        // theta = (alpha_2..alpha_n, mu_2..mu_n)
        for j in 2..=n {
            let mj = means[up(k, j)];
            let aj = weights[up(k, j)];
            out[j - 2] = -mj * mj / (2.0 * a1 * m1) + m1 / (2.0 * a1);
            out[nw + j - 2] = -aj * mj / (a1 * m1);
        }
    }
    out
}

/// Coefficients of \\(\partial q / \partial \theta_a\\) over the basis
/// \\(\\{g_j\\} \cup \\{h_j\\}\\), where \\(g_j(x) = \mathcal{N}(x;
/// \mu_j, \sigma^2)\\) and \\(h_j = \partial g_j / \partial \mu_j\\).
///
/// Returns `(cg, ch)` for each free parameter, in `theta` order.
fn basis_coefficients(
    k: usize,
    weights: &[f64],
    means: &[f64],
    constraint: Constraint,
) -> Vec<(Vec<f64>, Vec<f64>)> {
    let n = k / 2;
    let p = free_parameters(k, constraint);
    let nw = free_weights(k);
    let mut out: Vec<(Vec<f64>, Vec<f64>)> = (0..p).map(|_| (vec![0.0; k], vec![0.0; k])).collect();

    // Weight directions.
    if k % 2 == 1 {
        for j in 1..=n {
            let cg = &mut out[j - 1].0;
            cg[up(k, j)] += 1.0;
            cg[low(k, j)] += 1.0;
            cg[k / 2] -= 2.0;
        }
    } else {
        for j in 2..=n {
            let cg = &mut out[j - 2].0;
            cg[up(k, j)] += 1.0;
            cg[low(k, j)] += 1.0;
            cg[up(k, 1)] -= 1.0;
            cg[low(k, 1)] -= 1.0;
        }
    }

    // Mean directions. Under UnitVariance mu_1 is not free, so the free
    // means start at index 2.
    let first_free_mean = match constraint {
        Constraint::WeightsOnly => 1,
        Constraint::UnitVariance => 2,
    };
    for j in first_free_mean..=n {
        let a = weights[up(k, j)];
        let ch = &mut out[nw + j - first_free_mean].1;
        ch[up(k, j)] += a;
        ch[low(k, j)] -= a;
    }

    // Every free parameter also moves the eliminated innermost mean.
    let dm1 = inner_mean_gradient(k, weights, means, constraint);
    if !dm1.is_empty() {
        let a1 = weights[up(k, 1)];
        for (a, d) in dm1.iter().enumerate() {
            let ch = &mut out[a].1;
            ch[up(k, 1)] += a1 * d;
            ch[low(k, 1)] -= a1 * d;
        }
    }

    out
}

/// The four Gram blocks and the two projections onto the parent.
struct Overlaps {
    /// \\(\langle g_i, g_j \rangle\\).
    e: Vec<Vec<f64>>,
    /// \\(\langle g_i, h_j \rangle\\).
    d: Vec<Vec<f64>>,
    /// \\(\langle h_i, h_j \rangle\\).
    f: Vec<Vec<f64>>,
    /// \\(\langle p, g_j \rangle\\).
    b: Vec<f64>,
    /// \\(\langle p, h_j \rangle\\).
    bp: Vec<f64>,
}

fn overlaps(means: &[f64], sigma: f64) -> Overlaps {
    let k = means.len();
    let v = 2.0 * sigma * sigma;
    let w = 1.0 + sigma * sigma;
    let mut e = vec![vec![0.0; k]; k];
    let mut d = vec![vec![0.0; k]; k];
    let mut f = vec![vec![0.0; k]; k];
    for i in 0..k {
        for j in 0..k {
            let delta = means[i] - means[j];
            let eij = gaussian(delta, v);
            e[i][j] = eij;
            d[i][j] = delta / v * eij;
            f[i][j] = (1.0 / v - delta * delta / (v * v)) * eij;
        }
    }
    let b: Vec<f64> = means.iter().map(|m| gaussian(*m, w)).collect();
    let bp: Vec<f64> = means.iter().zip(&b).map(|(m, bj)| -m / w * bj).collect();
    Overlaps { e, d, f, b, bp }
}

/// The furthest a component mean may sit from the origin.
///
/// Not a modelling choice: beyond \\(8\sigma\\) the standard normal
/// density is \\(5\times10^{-15}\\) of its peak, so a component placed
/// there changes the \\(L^2\\) integrand by less than the rounding error
/// of the terms that make it up. The objective cannot determine such a
/// component, and an optimiser left free to move one wanders along a
/// direction the cost does not see — measured at \\(K = 15\\) under the
/// widest \\(\sigma\\) rule, where an unbounded solve parked a weight of
/// \\(5\times10^{-8}\\) at \\(13.4\sigma\\).
const MEAN_BOUND: f64 = 8.0;

/// Whether a trial point satisfies the formulation's inequality
/// constraints: positive weights, means strictly increasing away from
/// the origin, weights strictly decreasing away from it, and no
/// component beyond [`MEAN_BOUND`].
fn feasible(weights: &[f64], means: &[f64]) -> bool {
    if weights.iter().any(|a| *a <= 0.0 || a.is_nan()) {
        return false;
    }
    if means.iter().any(|m| !m.is_finite() || m.abs() > MEAN_BOUND) {
        return false;
    }
    for pair in means.windows(2) {
        if pair[1] <= pair[0] || pair[0].is_nan() {
            return false;
        }
    }
    // Weights fall monotonically from the centre outward. Compare each
    // adjacent pair on the upper half; symmetry carries the lower half.
    let start = means.len() / 2;
    for pair in weights[start..].windows(2) {
        if pair[1] >= pair[0] + 1e-15 || pair[0].is_nan() {
            return false;
        }
    }
    true
}

// ── Levenberg-Marquardt problem ─────────────────────────────────────

struct SplitProblem<const K: usize, const P: usize> {
    sigma: f64,
    constraint: Constraint,
}

impl<const K: usize, const P: usize> SplitProblem<K, P> {
    fn assemble(&self, x: &[f64; P]) -> Option<(f64, [[f64; P]; P], [f64; P])> {
        let (weights, means) = expand(K, x, self.sigma, self.constraint)?;
        if !feasible(&weights, &means) {
            return None;
        }
        let ov = overlaps(&means, self.sigma);
        let coeffs = basis_coefficients(K, &weights, &means, self.constraint);

        let mut cost = 1.0 / (2.0 * PI.sqrt());
        for i in 0..K {
            cost -= 2.0 * weights[i] * ov.b[i];
            for j in 0..K {
                cost += weights[i] * weights[j] * ov.e[i][j];
            }
        }

        let mut normal = [[0.0_f64; P]; P];
        for a in 0..P {
            let (cga, cha) = &coeffs[a];
            for b in a..P {
                let (cgb, chb) = &coeffs[b];
                let mut s = 0.0;
                for i in 0..K {
                    for j in 0..K {
                        s += cga[i] * ov.e[i][j] * cgb[j]
                            + cga[i] * ov.d[i][j] * chb[j]
                            + cha[i] * ov.d[j][i] * cgb[j]
                            + cha[i] * ov.f[i][j] * chb[j];
                    }
                }
                normal[a][b] = s;
                normal[b][a] = s;
            }
        }

        let mut rhs = [0.0_f64; P];
        for a in 0..P {
            let (cga, cha) = &coeffs[a];
            // <p, d_a q>
            let mut s = 0.0;
            for j in 0..K {
                s += cga[j] * ov.b[j] + cha[j] * ov.bp[j];
            }
            // -<q, d_a q>
            for (i, wi) in weights.iter().enumerate() {
                let mut t = 0.0;
                for j in 0..K {
                    t += cga[j] * ov.e[i][j] + cha[j] * ov.d[i][j];
                }
                s -= wi * t;
            }
            rhs[a] = s;
        }

        if !cost.is_finite()
            || normal.iter().flatten().any(|v| !v.is_finite())
            || rhs.iter().any(|v| !v.is_finite())
        {
            return None;
        }
        Some((cost, normal, rhs))
    }
}

impl<const K: usize, const P: usize> CostProblem<P> for SplitProblem<K, P> {
    type Error = Infeasible;

    fn evaluate_cost(&mut self, x: &[f64; P]) -> Result<f64, Self::Error> {
        let (weights, means) = expand(K, x, self.sigma, self.constraint).ok_or(Infeasible)?;
        if !feasible(&weights, &means) {
            return Err(Infeasible);
        }
        Ok(l2_distance(&weights, &means, self.sigma))
    }
}

impl<const K: usize, const P: usize> SystemProblem<P> for SplitProblem<K, P> {
    fn evaluate_system(&mut self, x: &[f64; P]) -> Result<SystemEvaluation<P>, Self::Error> {
        let (cost, normal, rhs) = self.assemble(x).ok_or(Infeasible)?;
        Ok(SystemEvaluation { cost, normal, rhs })
    }
}

fn fit_sized<const K: usize, const P: usize>(
    sigma: f64,
    seed: &[f64],
    config: &LMConfig,
    constraint: Constraint,
) -> Result<SplitFit, FitError> {
    if seed.len() != P {
        return Err(FitError::SeedLength {
            expected: P,
            got: seed.len(),
        });
    }
    let mut x0 = [0.0_f64; P];
    x0.copy_from_slice(seed);
    let mut problem = SplitProblem::<K, P> { sigma, constraint };
    let solution = solve_system(&mut problem, x0, config)
        .map_err(|e: LMError<Infeasible>| FitError::NotConverged(format!("{e}")))?;
    let reason = format!("{:?}", solution.reason);

    // The acceptance test is the gradient, not the driver's own verdict.
    //
    // Every entry in the shipped table stops on `DampingExhausted` at a
    // damping of 1e32 to 1e35, and none of them fires one of the
    // driver's convergence criteria. That is the correct stop for this
    // problem rather than a failure: the criteria are scaled by the
    // square root of the objective, and this objective is an absolute
    // L2 distance that runs down to 1e-10, so `gtol` compares the
    // gradient against a threshold that shrinks faster than the gradient
    // does. What is left when the damping runs away is an LM sitting at
    // the cost's floating-point floor with no improving step, which is
    // exactly a minimum.
    //
    // So the stop is accepted by name, on the quantity that actually
    // says a point is stationary. Measured across the two passes the
    // table is built from, the scaled gradient at the accepted point
    // runs from 3.9e-14 to 3.8e-10, inside the bound below by 1.4 orders
    // of magnitude. Accepting `DampingExhausted`
    // without looking at the gradient at all — which is what carrying an
    // unread `converged` flag amounts to — would let a future
    // regeneration that stalled early land silently in the table.
    let scaled_gradient = solution.gradient_norm_scaled;
    if scaled_gradient > GRADIENT_TOLERANCE || scaled_gradient.is_nan() {
        return Err(FitError::NotConverged(format!(
            "stopped at {reason} with scaled gradient {:e}, above the {GRADIENT_TOLERANCE:e} \
             bound, after {} iterations",
            scaled_gradient, solution.iterations
        )));
    }

    let (weights, means) = expand(K, &solution.x, sigma, constraint).ok_or_else(|| {
        FitError::Infeasible("the converged point has no real innermost mean".into())
    })?;
    if !feasible(&weights, &means) {
        return Err(FitError::Infeasible(format!(
            "weights {weights:?} means {means:?}"
        )));
    }
    Ok(SplitFit {
        l2: l2_distance(&weights, &means, sigma),
        variance: mixture_variance(&weights, &means, sigma),
        weights,
        means,
        sigma,
        iterations: solution.iterations,
        converged: solution.converged,
        reason,
        gradient_norm: solution.gradient_norm_scaled,
    })
}

/// The cost gradient \\(\nabla_\theta J\\) from the fitter's own
/// closed forms, at an arbitrary point.
///
/// Exposed so a test can confront the hand-derived derivatives with the
/// crate's automatic differentiation somewhere the answer is not zero.
/// Comparing them only at a converged solution proves nothing: both
/// vanish there whatever the derivation says, so the check would pass on
/// a wrong chain rule through the eliminated innermost mean.
pub fn cost_gradient(
    k: usize,
    theta: &[f64],
    sigma: f64,
    constraint: Constraint,
) -> Option<Vec<f64>> {
    macro_rules! grad {
        ($($n:literal),+) => {
            match (constraint, k) {
                $((Constraint::WeightsOnly, $n) => {
                    let mut x = [0.0; { $n - 1 }];
                    x.copy_from_slice(theta);
                    SplitProblem::<$n, { $n - 1 }> { sigma, constraint }
                        .assemble(&x)
                        .map(|(_, _, rhs)| rhs.iter().map(|r| -2.0 * r).collect())
                },)+
                $((Constraint::UnitVariance, $n) => {
                    let mut x = [0.0; { $n - 2 }];
                    x.copy_from_slice(theta);
                    SplitProblem::<$n, { $n - 2 }> { sigma, constraint }
                        .assemble(&x)
                        .map(|(_, _, rhs)| rhs.iter().map(|r| -2.0 * r).collect())
                },)+
                _ => None,
            }
        };
    }
    grad!(3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15)
}

macro_rules! dispatch_k {
    ($k:expr, $sigma:expr, $seed:expr, $config:expr, $c:expr, $off:expr, $($n:literal),+) => {
        match $k {
            $($n => fit_sized::<$n, { $n - $off }>($sigma, $seed, $config, $c),)+
            other => Err(FitError::UnsupportedK(other)),
        }
    };
}

/// The two-component unit-variance split, in closed form.
///
/// Symmetry, \\(\sum_i \alpha_i = 1\\) and the variance constraint leave
/// nothing free: both weights are \\(1/2\\) and the means are
/// \\(\pm\sqrt{1 - \sigma^2}\\). There is no optimisation to run, and a
/// solve over zero parameters is not something to ask a driver for.
fn two_component_split(sigma: f64) -> Result<SplitFit, FitError> {
    if sigma <= 0.0 || sigma >= 1.0 || sigma.is_nan() {
        return Err(FitError::Infeasible(format!(
            "sigma {sigma} is not in (0, 1)"
        )));
    }
    let m = (1.0 - sigma * sigma).sqrt();
    let weights = vec![0.5, 0.5];
    let means = vec![-m, m];
    Ok(SplitFit {
        l2: l2_distance(&weights, &means, sigma),
        variance: mixture_variance(&weights, &means, sigma),
        weights,
        means,
        sigma,
        iterations: 0,
        converged: true,
        reason: "closed form; no free parameters".to_string(),
        gradient_norm: 0.0,
    })
}

/// Fit the `k`-component split at the given component standard
/// deviation, starting from `seed` in the free-parameter layout of
/// [`expand`].
pub fn fit(
    k: usize,
    sigma: f64,
    seed: &[f64],
    config: &LMConfig,
    constraint: Constraint,
) -> Result<SplitFit, FitError> {
    match constraint {
        Constraint::WeightsOnly => dispatch_k!(
            k, sigma, seed, config, constraint, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
            16, 17, 18, 19, 20, 21, 22, 23, 24, 25
        ),
        Constraint::UnitVariance if k == MIN_K => two_component_split(sigma),
        Constraint::UnitVariance => dispatch_k!(
            k, sigma, seed, config, constraint, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
            17, 18, 19, 20, 21, 22, 23, 24, 25
        ),
    }
}

/// Fit from every supplied seed and keep the lowest \\(L^2\\).
///
/// The cost has many local minima — Vittaldev & Russell say so, and a
/// continuation seed that reaches one is not evidence there is no better
/// one — so the generator solves from more than one start and takes the
/// best. Restarting the winner from its own solution is what makes the
/// result a fixed point rather than wherever the first pass stopped.
pub fn fit_best(
    k: usize,
    sigma: f64,
    seeds: &[Vec<f64>],
    config: &LMConfig,
    constraint: Constraint,
) -> Result<SplitFit, FitError> {
    let mut best: Option<SplitFit> = None;
    let mut last_err = FitError::UnsupportedK(k);
    for seed in seeds {
        let mut current = match fit(k, sigma, seed, config, constraint) {
            Ok(f) => f,
            Err(e) => {
                last_err = e;
                continue;
            }
        };
        // Restart from the solution until it stops moving, so the
        // recorded point is stationary under the driver itself.
        for _ in 0..8 {
            let restart_seed = free_from(k, &current, constraint);
            match fit(k, sigma, &restart_seed, config, constraint) {
                Ok(next) if next.l2 < current.l2 * (1.0 - 1e-12) => current = next,
                _ => break,
            }
        }
        if best.as_ref().is_none_or(|b| current.l2 < b.l2) {
            best = Some(current);
        }
    }
    best.ok_or(last_err)
}

/// Scale a set of half-means so the mixture they describe has unit
/// variance at this `sigma`, leaving their ordering and ratios alone.
///
/// A seed built by any other recipe will generally miss the variance
/// constraint, and under [`Constraint::UnitVariance`] a miss is not a
/// poor start but an infeasible one: the eliminated innermost mean is a
/// square root that has no real value once the other components already
/// carry more than the whole variance. Measured, the uniform seed at
/// `k` = 6 overshoots by 2.6x and the solve cannot begin.
fn rescale_to_unit_variance(sigma: f64, half_a: &[f64], half_m: &mut [f64]) {
    let carried: f64 = half_a
        .iter()
        .zip(half_m.iter())
        .map(|(a, m)| a * m * m)
        .sum();
    if carried <= 0.0 {
        return;
    }
    let t = ((1.0 - sigma * sigma) / (2.0 * carried)).sqrt();
    for m in half_m.iter_mut() {
        *m *= t;
    }
}

/// Recover the free-parameter vector from a converged fit, so a solve
/// can be restarted at its own answer.
pub fn free_from(k: usize, f: &SplitFit, constraint: Constraint) -> Vec<f64> {
    let n = k / 2;
    let half_a: Vec<f64> = (1..=n).map(|i| f.weights[up(k, i)]).collect();
    let half_m: Vec<f64> = (1..=n).map(|i| f.means[up(k, i)]).collect();
    free_from_half(k, &half_a, &half_m, constraint)
}

/// Assemble a free-parameter vector from half-arrays given centre
/// outward: `half_a` holds \\(\alpha_1 \ldots \alpha_n\\) and `half_m`
/// holds \\(\mu_1 \ldots \mu_n\\), both including the entries the
/// parameterisation eliminates, which are dropped here.
fn free_from_half(k: usize, half_a: &[f64], half_m: &[f64], constraint: Constraint) -> Vec<f64> {
    // Odd K keeps every alpha_i (the centre is the eliminated one);
    // even K drops alpha_1.
    let mut seed: Vec<f64> = if k % 2 == 1 {
        half_a.to_vec()
    } else {
        half_a[1..].to_vec()
    };
    match constraint {
        Constraint::WeightsOnly => seed.extend_from_slice(half_m),
        // mu_1 comes from the variance constraint.
        Constraint::UnitVariance => seed.extend_from_slice(&half_m[1..]),
    }
    seed
}

/// A seed for `k` built by uniform mean spacing and equal weights — the
/// geometry the crate shipped before the library existed.
pub fn uniform_seed(k: usize, sigma: f64, constraint: Constraint) -> Vec<f64> {
    let spacing = 2.0 / (k as f64).sqrt();
    let n = k / 2;
    let mut half_m: Vec<f64> = if k % 2 == 1 {
        (1..=n).map(|i| i as f64 * spacing).collect()
    } else {
        (1..=n).map(|i| (i as f64 - 0.5) * spacing).collect()
    };
    let half_a = vec![1.0 / k as f64; n];
    if constraint == Constraint::UnitVariance {
        rescale_to_unit_variance(sigma, &half_a, &mut half_m);
    }
    free_from_half(k, &half_a, &half_m, constraint)
}

/// A seed for `k` continued from a converged solution at `k - 2`, which
/// is the continuation Vittaldev & Russell describe: spread the means
/// slightly, lower the weights, and add one element per tail.
pub fn continued_seed(
    k: usize,
    sigma: f64,
    previous: &SplitFit,
    constraint: Constraint,
) -> Vec<f64> {
    let pk = previous.weights.len();
    let pn = pk / 2;
    let prev_a: Vec<f64> = (1..=pn).map(|i| previous.weights[up(pk, i)]).collect();
    let prev_m: Vec<f64> = (1..=pn).map(|i| previous.means[up(pk, i)]).collect();

    let n = k / 2;
    let mut half_m = Vec::with_capacity(n);
    for i in 0..n {
        if i < prev_m.len() {
            half_m.push(prev_m[i] * 1.05);
        } else {
            let last = *half_m.last().unwrap_or(&0.8);
            let step = if half_m.len() >= 2 {
                last - half_m[half_m.len() - 2]
            } else {
                last.max(0.5)
            };
            half_m.push(last + step);
        }
    }
    let mut half_a = Vec::with_capacity(n);
    for i in 0..n {
        if i < prev_a.len() {
            half_a.push(prev_a[i] * 0.85);
        } else {
            half_a.push(half_a.last().copied().unwrap_or(0.01) * 0.25);
        }
    }

    if constraint == Constraint::UnitVariance {
        rescale_to_unit_variance(sigma, &half_a, &mut half_m);
    }
    free_from_half(k, &half_a, &half_m, constraint)
}

/// The driver settings the committed table was produced with.
pub fn generator_config() -> LMConfig {
    let mut config = LMConfig::default();
    config.max_iterations = 400;
    config.max_inner_trials = 60;
    config.tau = 1e-4;
    config.gtol = 1e-12;
    config.xtol = 1e-13;
    config.ftol = 1e-14;
    config.max_consecutive_invalid = 40;
    config
}
