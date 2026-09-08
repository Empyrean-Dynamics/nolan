//! The univariate splitting library, checked against the optimisation it
//! came from, against the paper that published it, and against the split
//! it replaced.
//!
//! The committed table in `src/statistics/split_library.rs` is emitted by
//! `examples/generate_split_library.rs`. Both that example and this suite
//! compile `tools/split_library_fit.rs`, so a table edited by hand — or
//! left behind by a change to the fit — fails here.

#[path = "../tools/split_library_fit.rs"]
mod fit;

use fit::{
    Constraint, FitError, MAX_K, MIN_K, SHIPPED_RULE, SigmaRule, SplitFit, continued_seed,
    cost_gradient, expand, fit_best, free_parameters, generator_config, l2_distance,
    mixture_variance, tail_ratio, uniform_seed,
};
use hyperjet::jets::Jet1;
use hyperjet::statistics::multivariate::GaussianSplitError;
use hyperjet::statistics::{
    MAX_SPLIT_COMPONENTS, MIN_SPLIT_COMPONENTS, normal_sf, split_gaussian, univariate_split,
};
use std::f64::consts::PI;

/// The split the crate carried before the library: equal weights, means
/// at uniformly spaced multiples of \\(\sigma\\), the shared width set to
/// preserve the total variance.
fn legacy_split(k: usize) -> (Vec<f64>, Vec<f64>, f64) {
    let offsets: Vec<f64> = (0..k).map(|i| i as f64 - (k - 1) as f64 / 2.0).collect();
    let s: f64 = offsets.iter().map(|c| c * c).sum();
    let d = ((k - 1) as f64 / s).sqrt();
    (
        vec![1.0 / k as f64; k],
        offsets.iter().map(|c| c * d).collect(),
        (1.0 - (k - 1) as f64 / k as f64).sqrt(),
    )
}

/// Solve the whole range under one constraint at the given widths,
/// continuing each `k` from the converged `k - 2`.
///
/// The continuation must come from `k - 2`, not `k - 1`: the
/// parameterisation differs by parity, since an odd split carries a
/// central component and an even one does not, so a cross-parity start
/// lands somewhere the solve has to climb out of. Measured, seeding
/// `k` = 15 from `k` = 14 stalls at a scaled gradient of 0.109 and the
/// fitter refuses it.
fn solve_all(constraint: Constraint, width: &dyn Fn(usize) -> f64) -> Vec<SplitFit> {
    let mut out = Vec::new();
    let mut prev_odd: Option<SplitFit> = None;
    let mut prev_even: Option<SplitFit> = None;
    for k in MIN_K..=MAX_K {
        let sigma = width(k);
        let mut seeds = vec![uniform_seed(k, sigma, constraint)];
        if let Some(p) = if k % 2 == 1 { &prev_odd } else { &prev_even } {
            seeds.push(continued_seed(k, sigma, p, constraint));
        }
        let f = fit_best(k, sigma, &seeds, &generator_config(), constraint)
            .unwrap_or_else(|e| panic!("k = {k}: {e}"));
        if k % 2 == 1 {
            prev_odd = Some(f.clone());
        } else {
            prev_even = Some(f.clone());
        }
        out.push(f);
    }
    out
}

/// Re-derive the shipped widths the way the generator does: the rule's
/// width divided by the square root of the unconstrained optimum's
/// variance, which is the width at which that optimum sits once it
/// carries unit variance.
fn shipped_widths() -> Vec<f64> {
    solve_all(Constraint::WeightsOnly, &|k| SHIPPED_RULE.sigma(k))
        .iter()
        .map(|f| f.sigma / f.variance.sqrt())
        .collect()
}

// ── The committed table is the solution of the stated problem ───────

/// Tolerances for comparing the committed table against a fresh solve.
/// See the test below for the measurement each is drawn from.
const WIDTH_TOLERANCE: f64 = 1e-6;
const WEIGHT_TOLERANCE: f64 = 5e-4;
const MEAN_TOLERANCE: f64 = 5e-3;
const L2_RELATIVE_TOLERANCE: f64 = 1e-4;
const L2_ABSOLUTE_FLOOR: f64 = 1e-15;

/// Re-run the optimisation and confront the committed table with it.
///
/// # Why the tolerances are what they are
///
/// Nothing here is reproducible to `f64`, and the tolerances say so.
/// Both the committed entries AND the width they sit at are outputs of
/// an iterative solve, so what two runs agree on is bounded by where the
/// solver stops, not by the arithmetic. The width in particular is
/// derived, not closed form: it is the rule's width divided by the
/// square root of the FIRST pass's variance, so it inherits that solve's
/// convergence precision, and a different libm moves it.
///
/// Measured on one machine over eleven variants of the whole two-stage
/// pipeline — the uniform start scaled by up to 10% either way, and the
/// driver's stopping tolerances and initial damping each moved by a
/// factor of ten in both directions — the worst deviation from the
/// committed values was:
///
/// | quantity | worst deviation | tolerance here | margin |
/// |---|---|---|---|
/// | width | 1.1e-8 | 1e-6 | 90x |
/// | weights | 5.2e-6 | 5e-4 | 100x |
/// | means | 8.1e-5 | 5e-3 | 60x |
///
/// The spread grows with `k` because the minimum flattens: at `k` = 15
/// the distance itself is 2.9e-10, so the curvature that pins the
/// parameters is correspondingly small and a solve can stop a little
/// further away. All three worst cases are at the top of the range.
///
/// The margins are wide against the measurement and still narrow against
/// anything that would matter. The dilated table this one replaced
/// differed by 3.9e-2 in the means at `k` = 3, and the uniform split
/// before that by 0.13, so real drift is caught with orders to spare.
///
/// The sharp assertion is the last one, which does not depend on where
/// in the basin a solve stopped: the committed table's own \(L^2\)
/// distance is no worse than a fresh solve's. That comparison needs
/// room too. Measured, the committed point is up to 6.9e-7 worse in
/// relative terms than the best any variant found, so the bound is 1e-4,
/// and an absolute floor of 1e-15 is added underneath it because the
/// distance is assembled as a difference against a leading term of
/// \(1/(2\sqrt{\pi})\) and so cannot be resolved below about 1e-16
/// however small it gets.
#[test]
fn the_committed_table_re_solves_to_the_same_minimum() {
    let widths = shipped_widths();
    let solved = solve_all(Constraint::UnitVariance, &|k| widths[k - MIN_K]);

    for (k, solved) in (MIN_K..=MAX_K).zip(&solved) {
        let table = univariate_split(k).expect("tabulated");

        assert_eq!(table.weights.len(), k, "k = {k}: weight count");
        assert_eq!(table.means.len(), k, "k = {k}: mean count");

        for i in 0..k {
            assert!(
                (table.weights[i] - solved.weights[i]).abs() < WEIGHT_TOLERANCE,
                "k = {k}, weight {i}: table {} against re-solved {}, outside \
                 {WEIGHT_TOLERANCE:e} (measured solver spread 5.2e-6)",
                table.weights[i],
                solved.weights[i]
            );
            assert!(
                (table.means[i] - solved.means[i]).abs() < MEAN_TOLERANCE,
                "k = {k}, mean {i}: table {} against re-solved {}, outside \
                 {MEAN_TOLERANCE:e} (measured solver spread 8.1e-5)",
                table.means[i],
                solved.means[i]
            );
        }
        assert!(
            (table.sigma - solved.sigma).abs() < WIDTH_TOLERANCE,
            "k = {k}: width {} against re-solved {}, outside {WIDTH_TOLERANCE:e}. \
             The width is the first pass's output, not a closed form, so it is \
             reproducible only to that solve's convergence precision (measured \
             spread 1.1e-8 across starting points and stopping tolerances)",
            table.sigma,
            solved.sigma
        );

        let table_l2 = l2_distance(table.weights, table.means, table.sigma);
        assert!(
            table_l2 <= solved.l2 * (1.0 + L2_RELATIVE_TOLERANCE) + L2_ABSOLUTE_FLOOR,
            "k = {k}: the committed table is a worse minimum than a fresh solve, \
             {table_l2:e} against {:e}, by more than {L2_RELATIVE_TOLERANCE:e} \
             relative (measured 6.9e-7) plus {L2_ABSOLUTE_FLOOR:e} absolute (the \
             rounding scale of the distance)",
            solved.l2
        );
    }
}

/// The fitter refuses a solve that stopped short, and says so.
///
/// Without this the guard is decorative. The driver never fires one of
/// its own convergence criteria on this objective — every entry in the
/// table stops on `DampingExhausted` — so the fitter accepts a stop by
/// the scaled gradient instead, and a stalled solve has to be refused
/// rather than written into the table.
///
/// Two starts are poisoned here. The first is the real failure mode: a
/// continuation across parities, which lands `k` = 15 somewhere the
/// solve cannot climb out of. The second is a budget of one iteration
/// from the uniform start, which cannot reach a minimum from anywhere.
#[test]
fn a_solve_that_stops_short_is_refused_by_name() {
    let widths = shipped_widths();
    let sigma15 = widths[15 - MIN_K];

    // A converged k = 14, used to seed k = 15 across the parity change.
    let evens = solve_all(Constraint::UnitVariance, &|k| widths[k - MIN_K]);
    let k14 = &evens[14 - MIN_K];
    let cross_parity = continued_seed(15, sigma15, k14, Constraint::UnitVariance);
    let stalled = fit_best(
        15,
        sigma15,
        &[cross_parity],
        &generator_config(),
        Constraint::UnitVariance,
    );
    match stalled {
        // Either refusal is honest here: the start may be infeasible
        // outright, or it may run and stall. What must not happen is a
        // silent acceptance.
        Err(FitError::NotConverged(_)) | Err(FitError::Infeasible(_)) => {}
        Err(other) => panic!("expected a refusal, got {other}"),
        Ok(f) => panic!(
            "a cross-parity continuation at k = 15 was accepted with scaled gradient {:e}; \
             if the fit now reaches a minimum from there, this test needs a new poison",
            f.gradient_norm
        ),
    }

    // A budget too small to reach any minimum.
    let mut starved = generator_config();
    starved.max_iterations = 1;
    starved.max_inner_trials = 1;
    let sigma7 = widths[7 - MIN_K];
    let cut_short = fit_best(
        7,
        sigma7,
        &[uniform_seed(7, sigma7, Constraint::UnitVariance)],
        &starved,
        Constraint::UnitVariance,
    );
    // This one must fire the gradient guard specifically, naming the
    // quantity it judged: a solve that ran and stopped short, rather
    // than one that could not start.
    match cut_short {
        Err(FitError::NotConverged(why)) => assert!(
            why.contains("scaled gradient"),
            "the refusal does not name the gradient: {why}"
        ),
        other => panic!("a one-iteration budget was not refused by the gradient guard: {other:?}"),
    }
}

/// Nothing in the table came from the wrong rule: every entry's width is
/// the shipped rule's, dilated by the mixture's own variance deficit.
#[test]
fn every_entry_carries_the_shipped_sigma_rule() {
    for k in MIN_K..=MAX_K {
        let table = univariate_split(k).expect("tabulated");
        let raw = SHIPPED_RULE.sigma(k);
        // sigma_table = raw / sqrt(v) for some v just below one, so the
        // ratio sits above one and shrinks toward it as k rises.
        let ratio = table.sigma / raw;
        assert!(
            (1.0..1.06).contains(&ratio),
            "k = {k}: sigma {} is not the shipped rule's {raw} dilated",
            table.sigma
        );
    }
}

// ── Identities the table must satisfy ───────────────────────────────

#[test]
fn every_entry_matches_the_parent_moments() {
    for k in MIN_K..=MAX_K {
        let e = univariate_split(k).expect("tabulated");

        let sum: f64 = e.weights.iter().sum();
        assert!((sum - 1.0).abs() < 1e-14, "k = {k}: weights sum to {sum}");

        let mean: f64 = e.weights.iter().zip(e.means).map(|(w, m)| w * m).sum();
        assert!(mean.abs() < 1e-14, "k = {k}: mixture mean is {mean}");

        let var = mixture_variance(e.weights, e.means, e.sigma);
        assert!(
            (var - 1.0).abs() < 1e-13,
            "k = {k}: mixture variance is {var}"
        );
    }
}

#[test]
fn every_entry_is_symmetric_ordered_and_deflating() {
    for k in MIN_K..=MAX_K {
        let e = univariate_split(k).expect("tabulated");

        assert!(
            e.sigma > 0.0 && e.sigma < 1.0,
            "k = {k}: sigma {} does not deflate",
            e.sigma
        );

        for i in 0..k {
            let j = k - 1 - i;
            assert!(
                (e.means[i] + e.means[j]).abs() < 1e-14,
                "k = {k}: means {i} and {j} are not antisymmetric"
            );
            assert!(
                (e.weights[i] - e.weights[j]).abs() < 1e-14,
                "k = {k}: weights {i} and {j} are not symmetric"
            );
            assert!(e.weights[i] > 0.0, "k = {k}: weight {i} is not positive");
        }
        if k % 2 == 1 {
            assert!(
                e.means[k / 2].abs() < 1e-14,
                "k = {k}: the central mean is not at the origin"
            );
        }

        for i in 1..k {
            assert!(
                e.means[i] > e.means[i - 1],
                "k = {k}: means are not increasing at {i}"
            );
        }
        // Weights fall away from the centre.
        for i in k / 2 + 1..k {
            assert!(
                e.weights[i] < e.weights[i - 1],
                "k = {k}: weight {i} does not fall away from the centre"
            );
        }
    }
}

// ── Tail reproduction, against the split this replaced ──────────────

/// The measurement that motivated the library, tabulated for every `k`.
///
/// What is asserted is that the library beats the split it replaced at
/// every `k` and every distance, plus floors at a handful of `k` set an
/// order below the measured values so a re-fit does not fail on its last
/// digit. The measured table is printed; run with `--nocapture` to read
/// it.
#[test]
fn the_library_reproduces_the_parent_tail_far_better_than_the_split_it_replaced() {
    println!(
        "\n  k |            library 3s / 4s / 5s |             replaced 3s / 4s / 5s | gain at 5s"
    );
    let floors: [(usize, [f64; 3]); 5] = [
        (2, [0.45, 0.15, 3.0e-2]),
        (3, [0.40, 0.065, 3.5e-3]),
        (7, [0.80, 0.10, 6.5e-4]),
        (11, [0.95, 0.40, 3.4e-3]),
        (15, [0.95, 0.80, 2.6e-2]),
    ];

    for k in MIN_K..=MAX_K {
        let e = univariate_split(k).expect("tabulated");
        let (lw, lm, ls) = legacy_split(k);
        let lib: Vec<f64> = (3..=5)
            .map(|d| tail_ratio(e.weights, e.means, e.sigma, d as f64))
            .collect();
        let old: Vec<f64> = (3..=5)
            .map(|d| tail_ratio(&lw, &lm, ls, d as f64))
            .collect();
        println!(
            "{k:3} | {:.4e} {:.4e} {:.4e} | {:.4e} {:.4e} {:.4e} | {:.2e}",
            lib[0],
            lib[1],
            lib[2],
            old[0],
            old[1],
            old[2],
            lib[2] / old[2]
        );

        for d in 0..3 {
            assert!(
                lib[d] > old[d],
                "k = {k}: at {}s the library reproduces {} of the parent tail \
                 against the replaced split's {}",
                d + 3,
                lib[d],
                old[d]
            );
        }

        if let Some((_, floor)) = floors.iter().find(|(kk, _)| *kk == k) {
            for d in 0..3 {
                assert!(
                    lib[d] >= floor[d],
                    "k = {k}: at {}s the library reproduces {} of the parent tail, \
                     below the floor {}",
                    d + 3,
                    lib[d],
                    floor[d]
                );
            }
        }
    }
}

/// Split a univariate mixture again along the same direction, applying
/// `(weights, means, sigma)` to every component.
fn recurse(
    mixture: &[(f64, f64, f64)],
    weights: &[f64],
    means: &[f64],
    sigma: f64,
) -> Vec<(f64, f64, f64)> {
    let mut out = Vec::with_capacity(mixture.len() * weights.len());
    for (w, m, s) in mixture {
        for (wi, mi) in weights.iter().zip(means) {
            out.push((w * wi, m + mi * s, s * sigma));
        }
    }
    out
}

fn mixture_tail_ratio(mixture: &[(f64, f64, f64)], delta: f64) -> f64 {
    let mix: f64 = mixture
        .iter()
        .map(|(w, m, s)| w * normal_sf((delta - m) / s))
        .sum();
    mix / normal_sf(delta)
}

/// Recursion along one direction made the replaced split worse, not
/// better; under the library it improves.
///
/// This is the measurement that motivated the change, and the reason it
/// could not be answered by raising the recursion depth. Each added level
/// of the uniform split narrows every component by another factor while
/// the outermost mean barely moves, so the mixture converges toward a
/// compact-support shape and its tail collapses.
#[test]
fn recursion_along_one_direction_collapses_the_replaced_splits_tail() {
    let e = univariate_split(3).expect("tabulated");
    let (lw, lm, ls) = legacy_split(3);

    let mut old = vec![(1.0_f64, 0.0_f64, 1.0_f64)];
    let mut lib = vec![(1.0_f64, 0.0_f64, 1.0_f64)];

    println!("\n  depth  n |         replaced 3s / 4s / 5s |          library 3s / 4s / 5s");
    let mut old_by_depth: Vec<[f64; 3]> = Vec::new();
    for depth in 1..=3 {
        old = recurse(&old, &lw, &lm, ls);
        lib = recurse(&lib, e.weights, e.means, e.sigma);
        let o: [f64; 3] = std::array::from_fn(|i| mixture_tail_ratio(&old, i as f64 + 3.0));
        let l: [f64; 3] = std::array::from_fn(|i| mixture_tail_ratio(&lib, i as f64 + 3.0));
        println!(
            "{depth:7} {:2} | {:.4e} {:.4e} {:.4e} | {:.4e} {:.4e} {:.4e}",
            old.len(),
            o[0],
            o[1],
            o[2],
            l[0],
            l[1],
            l[2]
        );
        for i in 0..3 {
            assert!(
                l[i] > o[i],
                "depth {depth}, {}s: library {} is not above the replaced split's {}",
                i + 3,
                l[i],
                o[i]
            );
        }
        old_by_depth.push(o);
    }

    // The replaced split's tail falls with every level rather than rising.
    for i in 0..3 {
        for d in 1..old_by_depth.len() {
            assert!(
                old_by_depth[d][i] < old_by_depth[d - 1][i],
                "{}s: the replaced split's tail did not fall from depth {d} to {}",
                i + 3,
                d + 1
            );
        }
    }

    // The published figures for the replaced split, to a factor of two:
    // 6.6e-2 / 1.1e-3 / 2.5e-6 at one level, 8.1e-4 / 6.4e-10 / 1.9e-19
    // at two.
    let expected: [[f64; 3]; 2] = [
        [6.571e-2, 1.071e-3, 2.478e-6],
        [8.120e-4, 6.402e-10, 1.905e-19],
    ];
    for (d, row) in expected.iter().enumerate() {
        for i in 0..3 {
            let got = old_by_depth[d][i];
            assert!(
                (got / row[i] - 1.0).abs() < 0.5,
                "depth {}, {}s: measured {got:e} against the published {:e}",
                d + 1,
                i + 3,
                row[i]
            );
        }
    }
}

/// The tail the library does NOT reproduce, stated as a test so it
/// cannot be forgotten.
///
/// Every component is narrower than the parent, so every mixture's tail
/// decays faster than the parent's and the ratio falls to zero far
/// enough out. This pins where that happens for the widest entry.
#[test]
fn the_deep_tail_is_beyond_any_entry_in_the_library() {
    let e = univariate_split(MAX_K).expect("tabulated");
    let far = tail_ratio(e.weights, e.means, e.sigma, 8.0);
    assert!(
        far < 1e-6,
        "the widest entry reproduces {far} of the parent's mass beyond 8 sigma; \
         if that is no longer near zero the claim in the module docs is stale"
    );
}

// ── The fit itself ──────────────────────────────────────────────────

/// The fitter, run under Vittaldev & Russell's first \\(\sigma\\) rule at
/// \\(k = 7\\), reproduces the seven-component library printed as Table 1
/// of the CMES paper.
///
/// This is the only check in the suite against numbers the project did
/// not compute. It is what says the cost function, the constraint set and
/// the \\(\sigma\\) convention are the published ones and not a
/// plausible-looking variant.
#[test]
fn the_fitter_reproduces_the_published_seven_component_library() {
    // Vittaldev & Russell 2016, CMES 111(1), Table 1: sigma^2 = 1/N.
    let paper_weights = [
        0.028799777829539,
        0.109875486136781,
        0.222379075167735,
        0.277891321731891,
        0.222379075167735,
        0.109875486136781,
        0.028799777829539,
    ];
    let paper_means = [
        -2.107361692265483,
        -1.329872113204359,
        -0.648762460764688,
        0.0,
        0.648762460764688,
        1.329872113204359,
        2.107361692265483,
    ];

    let sigma = SigmaRule::Narrow.sigma(7);
    let f = fit_best(
        7,
        sigma,
        &[uniform_seed(7, sigma, Constraint::WeightsOnly)],
        &generator_config(),
        Constraint::WeightsOnly,
    )
    .expect("k = 7 under the narrow rule");

    for i in 0..7 {
        assert!(
            (f.weights[i] - paper_weights[i]).abs() < 1e-7,
            "weight {i}: {} against the paper's {}",
            f.weights[i],
            paper_weights[i]
        );
        assert!(
            (f.means[i] - paper_means[i]).abs() < 1e-6,
            "mean {i}: {} against the paper's {}",
            f.means[i],
            paper_means[i]
        );
    }
}

/// The \\(L^2\\) cost in closed form, evaluated in `Jet1` arithmetic over
/// the free parameters of [`Constraint::UnitVariance`].
///
/// This is an independent transcription of the objective, including the
/// elimination of the innermost mean through the variance constraint. It
/// shares no code with the fitter's derivative path, which is the point.
fn l2_cost_jet<const P: usize>(k: usize, theta: &[Jet1<P>], sigma: f64) -> Jet1<P> {
    let n = k / 2;
    let n_free_w = if k % 2 == 1 { n } else { n - 1 };
    let (free_w, free_m) = theta.split_at(n_free_w);

    // half_a[i] and half_m[i] for i = 1..=n; index 0 unused.
    let mut half_a = vec![Jet1::<P>::constant(0.0); n + 1];
    let mut half_m = vec![Jet1::<P>::constant(0.0); n + 1];
    let mut centre = Jet1::<P>::constant(0.0);
    if k % 2 == 1 {
        half_a[1..=n].copy_from_slice(&free_w[..n]);
        centre = Jet1::<P>::constant(1.0);
        for a in free_w {
            centre -= *a * 2.0;
        }
    } else {
        half_a[2..=n].copy_from_slice(&free_w[..n - 1]);
        let mut a1 = Jet1::<P>::constant(0.5);
        for a in free_w {
            a1 -= *a;
        }
        half_a[1] = a1;
    }
    half_m[2..=n].copy_from_slice(&free_m[..n - 1]);
    // mu_1 from the variance constraint.
    let mut rest = Jet1::<P>::constant(0.0);
    for i in 2..=n {
        rest += half_a[i] * half_m[i] * half_m[i];
    }
    let num = (rest * -2.0) + (1.0 - sigma * sigma);
    half_m[1] = (num / half_a[1] * 0.5).sqrt();

    // Assemble the full mixture.
    let mut weights = vec![Jet1::<P>::constant(0.0); k];
    let mut means = vec![Jet1::<P>::constant(0.0); k];
    if k % 2 == 1 {
        weights[k / 2] = centre;
    }
    for i in 1..=n {
        let hi = k / 2 + i - 1 + k % 2;
        let lo = k - 1 - hi;
        weights[hi] = half_a[i];
        weights[lo] = half_a[i];
        means[hi] = half_m[i];
        means[lo] = half_m[i] * -1.0;
    }

    let v = 2.0 * sigma * sigma;
    let w = 1.0 + sigma * sigma;
    let mut j = Jet1::<P>::constant(1.0 / (2.0 * PI.sqrt()));
    for i in 0..k {
        let z = means[i] * means[i] * (-0.5 / w);
        j -= weights[i] * 2.0 * (z.exp() / (2.0 * PI * w).sqrt());
        for l in 0..k {
            let d = means[i] - means[l];
            let z = d * d * (-0.5 / v);
            j += weights[i] * weights[l] * (z.exp() / (2.0 * PI * v).sqrt());
        }
    }
    j
}

/// The fitter hands the driver a hand-derived closed-form gradient,
/// including a chain rule through the eliminated innermost mean. This
/// confronts it with the crate's own forward-mode derivative of the cost
/// AWAY from the solution.
///
/// Away from it on purpose. At the committed point the cost is
/// stationary, so both gradients vanish and the comparison passes on any
/// derivation, right or wrong — which is exactly the check an earlier
/// version of this test performed. Perturbing the parameters first makes
/// the two numbers large enough to disagree.
#[test]
fn the_closed_form_gradient_agrees_with_forward_mode_differentiation() {
    macro_rules! check {
        ($($k:literal),+) => {$({
            const K: usize = $k;
            const P: usize = K - 2;
            assert_eq!(P, free_parameters(K, Constraint::UnitVariance));

            let table = univariate_split(K).expect("tabulated");
            let sigma = table.sigma;
            let n = K / 2;
            let n_free_w = if K % 2 == 1 { n } else { n - 1 };

            // The committed point, in free-parameter coordinates.
            let mut base = [0.0_f64; P];
            let mut at = 0;
            let first = if K % 2 == 1 { 1 } else { 2 };
            for i in first..=n {
                base[at] = table.weights[K / 2 + i - 1 + K % 2];
                at += 1;
            }
            for i in 2..=n {
                base[at] = table.means[K / 2 + i - 1 + K % 2];
                at += 1;
            }
            assert_eq!(at, P);

            // Several deliberately non-stationary points.
            // Every parameter moves at every step, so the point is
            // non-stationary even when there is only one of them, and it
            // moves in a direction that stays feasible.
            //
            // The means alternate outward and inward rather than all
            // spreading. Spreading them all eats the variance budget the
            // eliminated innermost mean is recovered from, and the
            // headroom shrinks fast with k: measured, a uniform 1.8%
            // spread already leaves no real innermost mean at k = 14.
            // Alternating leaves the budget roughly where it was while
            // still moving every mean, and the free weights shrink
            // slightly, which returns headroom rather than spending it.
            for (_step, delta) in [(0usize, 0.01_f64), (1, 0.02), (2, 0.03)] {
                let mut theta = base;
                for (i, t) in theta.iter_mut().enumerate() {
                    *t *= if i < n_free_w {
                        1.0 - 0.3 * delta
                    } else {
                        let j = i - n_free_w;
                        let sign = if j % 2 == 0 { 1.0 } else { -1.0 };
                        1.0 + delta * sign * (1.0 + 0.1 * j as f64)
                    };
                }
                let analytic = match cost_gradient(K, &theta, sigma, Constraint::UnitVariance) {
                    Some(g) => g,
                    None => panic!(
                        "k = {K}, delta {delta}: the perturbed point is infeasible; theta = {theta:?}, \
                         expanded = {:?}",
                        expand(K, &theta, sigma, Constraint::UnitVariance)
                    ),
                };

                let jets: Vec<Jet1<P>> = (0..P)
                    .map(|i| Jet1::<P>::variable(theta[i], i))
                    .collect();
                let cost = l2_cost_jet::<P>(K, &jets, sigma);

                // The two transcriptions of the cost agree first.
                let (w, m) = expand(K, &theta, sigma, Constraint::UnitVariance)
                    .expect("feasible");
                let direct = l2_distance(&w, &m, sigma);
                // Absolute, not relative: the cost is assembled as a
                // difference against a leading term of 1/(2 sqrt(pi)),
                // so two transcriptions of it agree to the rounding
                // error of THAT, near 1e-16, however small the result.
                assert!(
                    (cost.value - direct).abs() <= 1e-15,
                    "k = {}: the jet cost {} disagrees with the closed form {}",
                    K, cost.value, direct
                );

                // Then their derivatives.
                let scale_g = cost.grad.iter().fold(0.0_f64, |a, g| a.max(g.abs()));
                assert!(
                    scale_g > 1e-9,
                    "k = {K}: the perturbed point is still stationary ({scale_g:e}); \
                     this test would prove nothing there"
                );
                for i in 0..P {
                    assert!(
                        (analytic[i] - cost.grad[i]).abs() <= 1e-8 * scale_g,
                        "k = {}, delta {}: d(L2)/d(theta_{}) analytic {:e} against jet {:e}",
                        K, delta, i, analytic[i], cost.grad[i]
                    );
                }
            }
        })+};
    }
    check!(3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
}

// ── The public surface ──────────────────────────────────────────────

#[test]
fn the_library_bounds_match_the_fitters() {
    assert_eq!(MIN_SPLIT_COMPONENTS, MIN_K);
    assert_eq!(MAX_SPLIT_COMPONENTS, MAX_K);
    assert!(univariate_split(MIN_SPLIT_COMPONENTS - 1).is_none());
    assert!(univariate_split(MAX_SPLIT_COMPONENTS + 1).is_none());
    for k in MIN_SPLIT_COMPONENTS..=MAX_SPLIT_COMPONENTS {
        assert!(univariate_split(k).is_some(), "k = {k}");
    }
}

#[test]
fn a_split_wider_than_the_library_is_refused_by_name() {
    let mean = [0.0; 3];
    let cov = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    let dir = [1.0, 0.0, 0.0];
    assert_eq!(
        split_gaussian::<3>(&mean, &cov, &dir, MAX_SPLIT_COMPONENTS + 1),
        Err(GaussianSplitError::KAboveLibrary {
            k: MAX_SPLIT_COMPONENTS + 1,
            max: MAX_SPLIT_COMPONENTS,
        })
    );
    assert!(split_gaussian::<3>(&mean, &cov, &dir, MAX_SPLIT_COMPONENTS).is_ok());
}

/// Every `k` the library carries reconstructs the parent's mean and
/// covariance from the split components, on a covariance with
/// off-diagonal structure and a direction that is not an eigenvector.
#[test]
fn every_k_round_trips_the_parent_moments_through_split_gaussian() {
    let mean = [1.0, -2.0, 0.5, 3.0];
    let cov = [
        [4.0, 0.5, 0.1, 0.0],
        [0.5, 2.0, 0.3, 0.2],
        [0.1, 0.3, 1.0, 0.4],
        [0.0, 0.2, 0.4, 3.0],
    ];
    let dir = [1.0, 1.0, -0.5, 0.25];

    for k in MIN_SPLIT_COMPONENTS..=MAX_SPLIT_COMPONENTS {
        let comps = split_gaussian::<4>(&mean, &cov, &dir, k).expect("split");
        assert_eq!(comps.len(), k);

        let sum_w: f64 = comps.iter().map(|(w, _, _)| w).sum();
        assert!((sum_w - 1.0).abs() < 1e-14, "k = {k}: weights sum {sum_w}");

        let mut m = [0.0_f64; 4];
        for (w, mu, _) in &comps {
            for i in 0..4 {
                m[i] += w * mu[i];
            }
        }
        let mut s = [[0.0_f64; 4]; 4];
        for (w, mu, c) in &comps {
            for i in 0..4 {
                for j in 0..4 {
                    s[i][j] += w * (c[i][j] + (mu[i] - m[i]) * (mu[j] - m[j]));
                }
            }
        }
        for i in 0..4 {
            assert!((m[i] - mean[i]).abs() < 1e-12, "k = {k}: mean {i}");
            for j in 0..4 {
                assert!(
                    (s[i][j] - cov[i][j]).abs() < 1e-12,
                    "k = {k}: covariance ({i},{j}): {} against {}",
                    s[i][j],
                    cov[i][j]
                );
            }
        }
    }
}
