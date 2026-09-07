//! Regenerate `src/statistics/split_library.rs`.
//!
//! ```text
//! cargo run --release --example generate_split_library > /tmp/split.rs \
//!     && mv /tmp/split.rs src/statistics/split_library.rs \
//!     && cargo fmt
//! ```
//!
//! Write to a temporary and move it: redirecting straight onto the
//! module truncates it before cargo runs, and the crate then fails to
//! compile because the module it is regenerating no longer exists. The
//! `cargo fmt` belongs to the command too, since the table is emitted
//! one array element per line and rustfmt then collapses the short
//! entries, so only the formatted output matches what is committed.
//!
//! Prints the diagnostics that justify the table to stderr and the table
//! itself to stdout. `tests/split_library.rs` re-solves the same problem
//! from the same code and checks what is committed, so a table edited by
//! hand fails the suite.

#[path = "../tools/split_library_fit.rs"]
mod fit;

/// The shipped table solves under the unit-variance constraint, so the
/// entries need no dilation afterwards.
const CONSTRAINT: Constraint = Constraint::UnitVariance;

use fit::{
    Constraint, MAX_K, MIN_K, SHIPPED_RULE, SigmaRule, SplitFit, continued_seed, fit_best,
    generator_config, l2_distance, tail_ratio, uniform_seed,
};

/// Solve the whole range under one constraint at widths given by
/// `width`, continuing each `k` from the converged `k - 2`.
///
/// The continuation must come from `k - 2`, not `k - 1`: the
/// parameterisation differs by parity, since an odd split carries a
/// central component and an even one does not, so a cross-parity start
/// lands somewhere the solve has to climb out of and the fitter's
/// convergence guard refuses it.
fn solve_all(
    constraint: Constraint,
    width: &dyn Fn(usize) -> f64,
    label: &str,
    strict: bool,
) -> Vec<(usize, SplitFit)> {
    let mut out: Vec<(usize, SplitFit)> = Vec::new();
    let mut prev_odd: Option<SplitFit> = None;
    let mut prev_even: Option<SplitFit> = None;
    for k in MIN_K..=MAX_K {
        let sigma = width(k);
        let mut seeds = vec![uniform_seed(k, sigma, constraint)];
        if let Some(p) = if k % 2 == 1 { &prev_odd } else { &prev_even } {
            seeds.push(continued_seed(k, sigma, p, constraint));
        }
        let f = match fit_best(k, sigma, &seeds, &generator_config(), constraint) {
            Ok(f) => f,
            // A diagnostic sweep over a rule the crate does not ship may
            // legitimately fail; the passes the table is built from may
            // not, and a dropped k there would leave a short table.
            Err(e) if !strict => {
                eprintln!("  K = {k} under {label}: {e}");
                continue;
            }
            Err(e) => panic!("K = {k} under {label}: {e}"),
        };
        if k % 2 == 1 {
            prev_odd = Some(f.clone());
        } else {
            prev_even = Some(f.clone());
        }
        out.push((k, f));
    }
    out
}

/// The component widths a rule implies, one per `k`, from the first of
/// the two passes.
///
/// The rule alone is not usable as the shipped width: the L2 optimum at
/// a fixed width carries less variance than its parent (0.951 of it at
/// `k` = 3), so a mixture of that width cannot also satisfy the variance
/// constraint without moving its means outward, and the outward move is
/// what the correction accounts for. Dividing the rule's width by the
/// square root of the unconstrained optimum's variance gives the width
/// at which that optimum sits once it carries unit variance; the second
/// pass then finds the true optimum at that width rather than the
/// dilated point, which is not one.
///
/// Measured, solving at the width rather than dilating is worth between
/// 1.0 and 1.8 times the parent's mass beyond 5 sigma, in the library's
/// favour, at no run-time cost.
fn widths_for(rule: SigmaRule, strict: bool) -> Vec<f64> {
    solve_all(
        Constraint::WeightsOnly,
        &|k| rule.sigma(k),
        &format!("{} width pass", rule.label()),
        strict,
    )
    .iter()
    .map(|(_, f)| f.sigma / f.variance.sqrt())
    .collect()
}

/// The widths the shipped table uses.
fn shipped_widths() -> Vec<f64> {
    widths_for(SHIPPED_RULE, true)
}

/// Report, for one sigma rule, the mixture that rule WOULD ship.
///
/// Both passes, not just the first. Reporting the unconstrained
/// intermediate here would describe a mixture the crate never emits, and
/// it reads low on exactly the quantity the rule is chosen for: measured,
/// the intermediate understates the shipped mixture's tail by 8 to 45
/// percent across the range.
fn report(rule: SigmaRule) {
    eprintln!(
        "\n=== {} (both passes; what this rule would ship) ===",
        rule.label()
    );
    eprintln!("  K   sigma      L2          variance    |grad|    tail 3s     4s          5s");
    let widths = widths_for(rule, false);
    let expected = MAX_K - MIN_K + 1;
    if widths.len() != expected {
        eprintln!(
            "  (width pass solved {} of {expected}; rows below are those that reached a minimum)",
            widths.len()
        );
        return;
    }
    for (k, f) in solve_all(
        Constraint::UnitVariance,
        &|k| widths[k - MIN_K],
        rule.label(),
        false,
    ) {
        eprintln!(
            "{k:3}  {:.7}  {:.4e}  {:.8}  {:.1e}  {:.4e}  {:.4e}  {:.4e}",
            f.sigma,
            f.l2,
            f.variance,
            f.gradient_norm,
            tail_ratio(&f.weights, &f.means, f.sigma, 3.0),
            tail_ratio(&f.weights, &f.means, f.sigma, 4.0),
            tail_ratio(&f.weights, &f.means, f.sigma, 5.0),
        );
    }
}

/// The split the crate shipped before the library: equal weights, means
/// at uniformly spaced multiples of \\(\sigma\\), component variance
/// \\(\sigma^2/K\\) scaled to preserve the total.
fn legacy_split(k: usize) -> (Vec<f64>, Vec<f64>, f64) {
    let offsets: Vec<f64> = (0..k).map(|i| i as f64 - (k - 1) as f64 / 2.0).collect();
    let s: f64 = offsets.iter().map(|c| c * c).sum();
    let d = ((k - 1) as f64 / s).sqrt();
    let weights = vec![1.0 / k as f64; k];
    let means: Vec<f64> = offsets.iter().map(|c| c * d).collect();
    let sigma = (1.0 - (k - 1) as f64 / k as f64).sqrt();
    (weights, means, sigma)
}

fn main() {
    for rule in [SigmaRule::Narrow, SigmaRule::Medium, SigmaRule::Wide] {
        report(rule);
    }

    eprintln!("\n=== the split this table replaces ===");
    eprintln!("  K   sigma      L2          tail 3s     4s          5s");
    for k in MIN_K..=MAX_K {
        let (w, m, s) = legacy_split(k);
        eprintln!(
            "{k:3}  {s:.7}  {:.4e}  {:.4e}  {:.4e}  {:.4e}",
            l2_distance(&w, &m, s),
            tail_ratio(&w, &m, s, 3.0),
            tail_ratio(&w, &m, s, 4.0),
            tail_ratio(&w, &m, s, 5.0),
        );
    }

    // Where does double precision stop determining the fit? Continue the
    // shipped rule past the tabulated range until the L2 distance nears
    // the rounding error of the terms it is assembled from.
    eprintln!(
        "\n=== floor probe, {} beyond the tabulated range ===",
        SHIPPED_RULE.label()
    );
    eprintln!("  K   sigma      L2          variance    maxmu   minw       |grad|");
    let mut prev_odd: Option<SplitFit> = None;
    let mut prev_even: Option<SplitFit> = None;
    for k in MIN_K..=25 {
        let mut seeds = vec![uniform_seed(k, SHIPPED_RULE.sigma(k), CONSTRAINT)];
        if let Some(p) = if k % 2 == 1 { &prev_odd } else { &prev_even } {
            seeds.push(continued_seed(k, SHIPPED_RULE.sigma(k), p, CONSTRAINT));
        }
        match fit_best(
            k,
            SHIPPED_RULE.sigma(k),
            &seeds,
            &generator_config(),
            CONSTRAINT,
        ) {
            Ok(f) => {
                if k > MAX_K {
                    eprintln!(
                        "{k:3}  {:.7}  {:.4e}  {:.8}  {:6.3}  {:.2e}  {:.1e}",
                        f.sigma,
                        f.l2,
                        f.variance,
                        f.means.last().unwrap(),
                        f.weights.iter().cloned().fold(f64::INFINITY, f64::min),
                        f.gradient_norm,
                    );
                }
                if k % 2 == 1 {
                    prev_odd = Some(f);
                } else {
                    prev_even = Some(f);
                }
            }
            Err(e) => eprintln!("{k:3}  FAILED: {e}"),
        }
    }

    let widths = shipped_widths();
    let fits = solve_all(
        Constraint::UnitVariance,
        &|k| widths[k - MIN_K],
        "the shipped unit-variance pass",
        true,
    );
    eprintln!("\n=== emitting {} ===", SHIPPED_RULE.label());

    println!("{}", HEADER);
    println!("/// Univariate split entries for `k` = {MIN_K} through {MAX_K}, indexed by");
    println!("/// `k - {MIN_K}`.");
    println!(
        "static LIBRARY: [UnivariateSplit; {}] = [",
        MAX_K - MIN_K + 1
    );
    for (k, f) in &fits {
        let u = f;
        println!("    // K = {k}");
        println!("    UnivariateSplit {{");
        println!("        weights: &[");
        for w in &u.weights {
            println!("            {:e},", w);
        }
        println!("        ],");
        println!("        means: &[");
        for m in &u.means {
            println!("            {:e},", m);
        }
        println!("        ],");
        println!("        sigma: {:e},", u.sigma);
        println!("    }},");
    }
    println!("];");
    println!("{}", FOOTER);
}

const HEADER: &str = r##"//! Univariate Gaussian splitting library.
//!
//! GENERATED FILE. Do not edit by hand. To regenerate, from the
//! repository (this generator is not part of the published crate):
//!
//! ```text
//! cargo run --release --example generate_split_library > /tmp/split.rs \
//!     && mv /tmp/split.rs src/statistics/split_library.rs \
//!     && cargo fmt
//! ```
//!
//! Both halves matter. Redirecting straight onto this file truncates it
//! before cargo runs, and the crate then fails to compile because the
//! module it is being asked to regenerate no longer exists, so the
//! output goes to a temporary first. And the `cargo fmt` is part of the
//! command rather than an afterthought: the generator emits one array
//! element per line and lets rustfmt decide what fits, so its raw output
//! is not the committed file and diffing the two without it shows
//! spurious changes. See `tools/split_library_fit.rs` for what is
//! solved.
//!
//! Each entry approximates the standard normal by a mixture of `k`
//! Gaussians of a common width, with weights and means symmetric about
//! the origin, chosen to minimise the \\(L^2\\) distance between the two
//! densities subject to the mixture carrying zero mean and unit
//! variance.
//!
//! The width is not optimised alongside them. With it free the problem
//! has the trivial solution \\(\alpha_0 = 1\\), \\(\sigma = 1\\),
//! every other weight zero: one component equal to the parent, at
//! \\(L^2 = 0\\) and no split at all. Narrow components are not the
//! escape — as \\(\sigma \to 0\\) the component approaches a Dirac
//! delta and the distance diverges, measuring 2.31 at
//! \\(\sigma = 0.1\\) and 281.6 at \\(\sigma = 0.001\\) — so the
//! optimum runs the other way, toward no deflation. Vittaldev & Russell
//! therefore fix \\(\sigma\\) by a rule in `k`, and this table uses
//! their second, \\(\sigma^2 = (1/k)^{3/4}\\), corrected as below.
//! Their first, \\(\sigma^2 = 1/k\\), deflates harder but reproduces
//! the parent's tail worse — by 1.5 orders of magnitude at
//! \\(5\sigma\\) for `k` = 3, 4.7 by `k` = 7 and 7.3 by `k` = 15,
//! measured on the mixture each rule would actually ship. Their
//! third, \\(\sigma^2 = (1/k)^{1/2}\\), reproduces the tail best but
//! drives the \\(L^2\\) distance to \\(10^{-13}\\) by `k` = 12,
//! where the objective — assembled as a difference against a leading
//! term of \\(1/(2\sqrt{\pi})\\) — stops determining the fit in
//! double precision.
//!
//! # The unit-variance constraint, and the width it implies
//!
//! Vittaldev & Russell constrain only that the weights sum to one, so
//! their mixtures carry slightly less variance than the parent — 0.951
//! of it at `k` = 3. A split that sheds a fixed fraction of the variance
//! at every level compounds that loss under recursion, and
//! [`split_gaussian`](super::multivariate::split_gaussian) must
//! reproduce the covariance it was handed, so the mixture here is
//! constrained to unit variance as well.
//!
//! That constraint also fixes the width, in two stages. A mixture cannot
//! carry unit variance at the rule's width without moving its means
//! outward, and the outward move is a dilation by
//! \\(1/\sqrt{v}\\) in the variance \\(v\\) of the unconstrained
//! optimum. So stage one solves the unconstrained problem at the rule's
//! width and reads off \\(v\\); the shipped width is the rule's divided
//! by \\(\sqrt{v}\\). Stage two then solves for the weights and means
//! that are optimal AT that width under the variance constraint.
//!
//! Stopping after the dilation would be the cheaper thing to do, and it
//! is what an earlier draft of this table did, but a dilated optimum is
//! not an optimum: measured at its own width it is 38.5% worse in
//! \\(L^2\\) at `k` = 3 and 154% worse at `k` = 7. Solving at the
//! width instead is worth between 1.0 and 1.8 times the parent's mass
//! beyond \\(5\sigma\\), in the library's favour, at no run-time cost.
//!
//! # References
//!
//! - Vittaldev, V., & Russell, R. P. (2016). *Multidirectional Gaussian
//!   mixture models for nonlinear uncertainty propagation.* Computer
//!   Modeling in Engineering & Sciences 111(1): 83–117.
//!   [doi:10.3970/cmes.2016.111.083](https://doi.org/10.3970/cmes.2016.111.083)
//!   (the optimisation, its constraints, and the \\(\sigma\\) rules)
//! - Vittaldev, V., & Russell, R. P. (2016). *Space object collision
//!   probability using multidirectional Gaussian mixture models.*
//!   Journal of Guidance, Control, and Dynamics 39(9): 2163–2169.
//!   [doi:10.2514/1.G001610](https://doi.org/10.2514/1.G001610)
//!   (the same library applied to a collision integral)
//! - DeMars, K. J., Bishop, R. H., & Jah, M. K. (2013). *Entropy-based
//!   approach for uncertainty propagation of nonlinear dynamical
//!   systems.* Journal of Guidance, Control, and Dynamics 36(4):
//!   1047–1057.
//!   [doi:10.2514/1.58987](https://doi.org/10.2514/1.58987)
//!   (the \\(L^2\\) cost in closed form, and libraries for \\(k \le 5\\))

/// One entry of the univariate splitting library: a mixture of
/// equally-wide Gaussians approximating the standard normal.
///
/// The weights and means minimise the \\(L^2\\) distance to the
/// standard normal among symmetric mixtures of this width that carry
/// zero mean and unit variance. Because the variance is one, and not
/// merely close to it,
/// [`split_gaussian`](super::multivariate::split_gaussian) reproduces
/// the parent's covariance exactly rather than to within a deficit that
/// would compound under recursion.
#[derive(Clone, Copy, Debug)]
pub struct UnivariateSplit {
    /// Component weights, ordered by increasing mean. Positive, summing
    /// to one, and decreasing away from the centre.
    pub weights: &'static [f64],
    /// Component means in units of the parent standard deviation,
    /// increasing and antisymmetric about zero.
    pub means: &'static [f64],
    /// The standard deviation shared by every component, in units of the
    /// parent standard deviation. Always less than one: this is the
    /// factor by which a split deflates the spread along its direction.
    pub sigma: f64,
}
"##;

const FOOTER: &str = r##"
/// The smallest component count the library tabulates.
pub const MIN_SPLIT_COMPONENTS: usize = 2;

/// The largest component count the library tabulates.
///
/// Both a usefulness limit and, just above it, a numerical one. A caller
/// pays one propagation per component, so 15 already costs five times
/// the three-component split, and the return has flattened: across
/// `k` = 13 to 15 the mixture's mass beyond \\(4\sigma\\) climbs from
/// 0.71 to 0.91 of the parent's while its mass beyond \\(5\sigma\\)
/// climbs only from 0.012 to 0.033. No `k` under this rule reproduces
/// the deep tail, so raising `k` is not the lever that would.
///
/// Measured, the solve stays determined one step past the table, at
/// `k` = 16, and stops there: from `k` = 17 to 25 every start stalls at
/// a scaled gradient between \\(3\times10^{-2}\\) and
/// \\(8\times10^{-2}\\), which the fitter refuses rather than
/// tabulates. The unit-variance parameterisation eliminates the
/// innermost mean through a square root, and the further the weights
/// spread the worse that recovery is conditioned.
pub const MAX_SPLIT_COMPONENTS: usize = 15;

/// The library entry for `k` components, or `None` outside
/// [`MIN_SPLIT_COMPONENTS`]`..=`[`MAX_SPLIT_COMPONENTS`].
pub fn univariate_split(k: usize) -> Option<&'static UnivariateSplit> {
    if !(MIN_SPLIT_COMPONENTS..=MAX_SPLIT_COMPONENTS).contains(&k) {
        return None;
    }
    Some(&LIBRARY[k - MIN_SPLIT_COMPONENTS])
}
"##;
