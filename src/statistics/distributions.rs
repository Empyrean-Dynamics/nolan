//! Distribution functions: log-gamma, regularized incomplete gamma,
//! chi-squared survival, standard normal PDF/CDF/SF.
//!
//! # References
//!
//! - Lanczos, C. (1964). *A precision approximation of the gamma function.*
//!   J. SIAM Numer. Anal. Ser. B 1: 86–96.
//!   <https://doi.org/10.1137/0701008>
//! - Press, W. H., et al. (2007). *Numerical Recipes 3rd ed.* §6.2 (incomplete
//!   gamma), §6.14.8 (continued-fraction Lentz method).
//! - Abramowitz, M., & Stegun, I. A. (1972). *Handbook of Mathematical
//!   Functions.* Dover. 26.2.11 (the all-positive error-function series) and
//!   26.2.14 (the continued fraction for the tail).
//! - Olver, F. W. J., et al., eds. *NIST Digital Library of Mathematical
//!   Functions.* Release 1.2.4. §7.6.2 (series) and §7.9.3 (continued
//!   fraction). <https://dlmf.nist.gov/7.6> and <https://dlmf.nist.gov/7.9>
//! - Cody, W. J. (1969). *Rational Chebyshev approximation for the error
//!   function.* Math. Comp. 23(107): 631–637.
//!   <https://doi.org/10.1090/S0025-5718-1969-0247736-4> — the source of the
//!   split of \\(t^2\\) into an exactly representable head and a small
//!   remainder, which is what keeps the far tail relatively accurate.
//! - Kahan, W. (1965). *Further remarks on reducing truncation errors.*
//!   Comm. ACM 8(1): 40. <https://doi.org/10.1145/363707.363723> — the
//!   compensated summation used by the error-function series.

use std::f64::consts::PI;

/// Below this argument the Lanczos sum is reached through the recurrence
/// rather than directly.
///
/// \\(x - 1\\) is what the Lanczos form needs, and for
/// \\(x < 2^{-54}\\) that rounds to exactly \\(-1\\), which puts a zero
/// in the first denominator of the sum and sends the result to infinity
/// — at \\(x = 10^{-300}\\), where the true value is an ordinary
/// \\(690.78\\). The recurrence removes that, and it also removes the
/// accuracy loss well above it: the direct form is wrong by
/// \\(2.7\times10^{-10}\\) relative at \\(x = 10^{-8}\\).
///
/// The threshold is 0.5 and not lower on purpose. Every argument the
/// crate itself supplies is \\(k/2 \ge 0.5\\) — [`chi2_sf`] with
/// \\(k = 1\\) gives exactly 0.5 — so no value this crate computes
/// moves.
const LN_GAMMA_RECURRENCE_BELOW: f64 = 0.5;

/// Natural logarithm of the gamma function via the Lanczos approximation
/// (g = 7, n = 9 coefficients), with the recurrence
/// \\(\ln\Gamma(x) = \ln\Gamma(x+1) - \ln x\\) below
/// \\(x = 1/2\\).
///
/// Domain: \\(x \ge 0\\). Returns \\(+\infty\\) at 0 and at
/// \\(+\infty\\), and `f64::NAN` for a negative or NaN argument. For
/// negative arguments the reflection formula should be used externally.
///
/// # Accuracy
///
/// Relative error under \\(2\times10^{-14}\\) everywhere on the
/// domain, with one stated exception. The worst found on a dense sweep
/// is \\(1.07\times10^{-14}\\), at \\(x = 2.592\\). \\(\ln\Gamma\\) has zeros at \\(x = 1\\) and
/// \\(x = 2\\), and no form of this kind can be relatively accurate
/// beside a zero: the absolute error stays near \\(10^{-15}\\) while the
/// function passes through nothing. Measured worst relative errors are
/// \\(1.7\times10^{-12}\\) at \\(x = 1.00068\\) and
/// \\(1.0\times10^{-12}\\) at \\(x = 1.99768\\), and `ln_gamma(1.0)`
/// returns \\(-8.9\times10^{-16}\\) rather than zero. Away from those
/// two neighbourhoods the worst is \\(1.07\times10^{-14}\\) below
/// \\(x = 10\\) and \\(6.0\times10^{-16}\\) beyond it. Callers who need
/// the exponential of this — as [`upper_inc_gamma_reg`] does — care
/// about the ABSOLUTE error, which is \\(10^{-15}\\) throughout.
///
/// # Examples
///
/// ```
/// use hyperjet::statistics::ln_gamma;
/// use std::f64::consts::PI;
///
/// // Γ(1) = 1, so ln Γ(1) = 0.
/// assert!(ln_gamma(1.0).abs() < 1e-15);
/// // Γ(5) = 24, so ln Γ(5) = ln 24.
/// assert!((ln_gamma(5.0) - (24.0_f64).ln()).abs() < 1e-12);
/// // Γ(0.5) = √π.
/// assert!((ln_gamma(0.5) - PI.sqrt().ln()).abs() < 1e-12);
/// // Small arguments are ordinary numbers, not infinities.
/// assert!((ln_gamma(1e-300) - 690.775_527_898_213_7).abs() < 1e-12);
/// ```
pub fn ln_gamma(x: f64) -> f64 {
    if x.is_nan() || x < 0.0 {
        return f64::NAN;
    }
    if x == 0.0 || x.is_infinite() {
        return f64::INFINITY;
    }
    if x < LN_GAMMA_RECURRENCE_BELOW {
        // Gamma(x) = Gamma(x + 1) / x, so the sum is evaluated where its
        // argument is safely above the cliff and the division becomes a
        // subtraction of logarithms. One level of recursion only, since
        // x + 1 is at least 1.
        return ln_gamma(x + 1.0) - x.ln();
    }
    const COEFFS: [f64; 9] = [
        0.999_999_999_999_809_9,
        676.520_368_121_885_1,
        -1259.139_216_722_402_8,
        771.323_428_777_653_1,
        -176.615_029_162_140_6,
        12.507_343_278_686_905,
        -0.138_571_095_265_720_12,
        9.984_369_578_019_572e-6,
        1.505_632_735_149_311_6e-7,
    ];

    let x = x - 1.0;
    let mut sum = COEFFS[0];
    for (i, &c) in COEFFS[1..].iter().enumerate() {
        sum += c / (x + i as f64 + 1.0);
    }
    let t = x + 7.5;
    0.5 * (2.0 * PI).ln() + (x + 0.5) * t.ln() - t + sum.ln()
}

/// The smallest \\(a\\) this routine will answer for.
///
/// Below the seam it computes \\(Q = 1 - P\\), and for small \\(a\\)
/// the true \\(Q\\) is itself of order \\(a\\): at \\(a = 10^{-16}\\),
/// \\(x = 1/2\\) it is \\(7\times10^{-17}\\), so \\(P\\) rounds to
/// exactly one and the subtraction returns nothing at all. The
/// measured relative error of that arm runs \\(6\times10^{-11}\\) at
/// \\(a = 10^{-4}\\) and 282 at \\(a = 10^{-16}\\) — a plausible
/// number, silently wrong, which is the one outcome this crate does not
/// ship.
///
/// Getting it right needs \\(Q\\) formed directly rather than by
/// complement:
/// \\[
///   Q(a,x) = -\operatorname{expm1}(u) - e^{u} a T,
///   \qquad u = a\ln x - \ln\Gamma(1+a),
/// \\]
/// whose leading terms are both \\(O(a)\\) and neither subtracts from
/// one. It needs \\(\ln\Gamma(1+a)\\) to RELATIVE accuracy for small
/// \\(a\\), which is a different routine from [`ln_gamma`] — that one
/// passes through a zero at 1 and is accurate there only in absolute
/// terms. Nothing in this crate reaches below the threshold, so the
/// arithmetic is refused rather than approximated: [`chi2_sf`] supplies
/// \\(a = k/2\\), whose smallest value is exactly this threshold at
/// \\(k = 1\\).
///
/// The refusal is a NaN and not a `Result`, which is a deliberate
/// choice and a defensible one only while the refused regimes stay
/// unreachable: this module's scalar surface is uniformly `f64` in and
/// `f64` out, `f64::NAN` is already how every function in it reports an
/// argument it will not answer for, and it is what the C library does
/// for the same reason. The cost is that a NaN propagates silently
/// until something checks it, so a caller's only real defence is to
/// test its own argument against this constant first — which is why it
/// is exported. If a caller ever does need \\(a < 1/2\\), that
/// argument is void and the signature should change rather than the
/// threshold move.
pub const UPPER_INC_GAMMA_MIN_A: f64 = 0.5;

/// Regularized **upper** incomplete gamma function `Q(a, x) = Γ(a, x) / Γ(a) = 1 - P(a, x)`.
///
/// Domain: `a ≥ 0.5` ([`UPPER_INC_GAMMA_MIN_A`]) and `x ≥ 0`. Returns
/// `f64::NAN` outside it, including for an `a` that is positive but
/// below the threshold — see that constant for why the small-`a` regime
/// is refused rather than answered.
///
/// Also returns `f64::NAN` if either half fails to converge within its
/// iteration ceiling. That ceiling follows `a`, and no argument tested
/// up to `a = 1e6` comes within a factor of two of it, but a truncated
/// sum returned as a probability would be a wrong answer wearing the
/// clothes of a right one.
///
/// Implementation: series for `x < a + 1` (faster convergence in that
/// regime), Lentz continued fraction for `x ≥ a + 1`. Standard
/// NR §6.2 split.
///
/// # Accuracy
///
/// Relative error under \\(10^{-12}\\) for \\(a \le 100\\); the worst
/// measured across \\(a \in [0.5, 100]\\) and
/// \\(x \in [10^{-4}, 800]\\) is \\(1.8\times10^{-13}\\), at
/// \\(a = 100\\).
///
/// Beyond that there is no flat bound to give, and the mechanism below
/// is the contract instead: the error is at most
/// \\(\varepsilon \max(x,\, a \ln x)\\), which a caller can evaluate
/// for its own arguments. Measured at \\(x = a\\), which is
/// \\(\chi^2\\) at a reduced statistic of one:
///
/// | \\(a\\) | relative error | the bound above |
/// |---|---|---|
/// | \\(10^{5}\\) | \\(1.4\times10^{-10}\\) | \\(2.6\times10^{-10}\\) |
/// | \\(10^{6}\\) | \\(6.8\times10^{-10}\\) | \\(3.1\times10^{-9}\\) |
/// | \\(10^{8}\\) | \\(2.6\times10^{-7}\\) | \\(4.1\times10^{-7}\\) |
/// | \\(10^{10}\\) | \\(2.3\times10^{-5}\\) | \\(5.1\times10^{-5}\\) |
/// | \\(10^{12}\\) | \\(2.0\times10^{-3}\\) | \\(6.1\times10^{-3}\\) |
/// | \\(10^{14}\\) | \\(1.8\times10^{-1}\\) | \\(7.2\times10^{-1}\\) |
///
/// So the result carries fewer than six significant figures above
/// roughly \\(a = 10^{8}\\) and none at all by \\(10^{14}\\), where it
/// returns 0.59 for a true 0.5. It is NOT refused there — it is still a
/// probability, merely a wrong one — and this is the reason the bound
/// is published as a formula rather than a number. Past
/// \\(a \approx 3.7\times10^{15}\\) the value leaves the unit interval
/// altogether and the exit check turns it into a NaN.
///
/// The error is set by the largest intermediate in the exponent
/// \\(-x + a\ln x - \ln\Gamma(a)\\), not by the exponent itself. At
/// \\(a = 100,\ x = 198\\) the three terms are \\(-198\\),
/// \\(+529.2\\) and \\(-359.1\\) and their sum is \\(-27.9\\), so three
/// roundings of a quantity near 529 give the \\(1.7\times10^{-13}\\)
/// measured there. It therefore degrades as
/// \\(\varepsilon \max(x,\, a\ln x)\\) and not as anything about the
/// answer. This is the same mechanism [`normal_pdf`] avoids by splitting
/// its exponent; the same treatment would work here and has not been
/// applied, because no caller in this crate reaches an \\(a\\) where it
/// matters.
///
/// The step across the internal seam at \\(x = a + 1\\), measured over
/// one unit in the last place, grows with \\(a\\) for the same reason
/// and at the same rate: under \\(10^{-14}\\) for \\(a \le 10\\),
/// \\(2.3\times10^{-14}\\) at \\(a = 50\\), \\(2.3\times10^{-13}\\)
/// at \\(a = 100\\) and \\(7.1\times10^{-12}\\) at \\(a = 5000\\). It
/// is not a discontinuity of the split: the two representations agree
/// to within what either of them can resolve there. Every \\(a\\) the
/// crate itself supplies is at most 7.5, where the step is under
/// \\(5\times10^{-15}\\).
///
/// # Examples
///
/// ```
/// use hyperjet::statistics::upper_inc_gamma_reg;
///
/// // Q(a, 0) = 1 for any a > 0.
/// assert!((upper_inc_gamma_reg(2.0, 0.0) - 1.0).abs() < 1e-12);
/// // Q(1, x) = e^-x for a = 1 (exponential SF).
/// assert!((upper_inc_gamma_reg(1.0, 2.5) - (-2.5_f64).exp()).abs() < 1e-12);
/// ```
pub fn upper_inc_gamma_reg(a: f64, x: f64) -> f64 {
    if a < UPPER_INC_GAMMA_MIN_A || x < 0.0 || x.is_nan() || a.is_nan() {
        return f64::NAN;
    }
    if x == 0.0 {
        return 1.0;
    }
    let q = if x < a + 1.0 {
        1.0 - lower_gamma_series(a, x)
    } else {
        upper_gamma_cf(a, x)
    };
    // Q is a probability. Anything else is arithmetic that has come
    // apart — at a = 1e15 the series returned -18.23 — and a number
    // outside the unit interval must not be handed back as one. This
    // also carries through the NaN either half returns on exhaustion.
    //
    // It is a backstop and not a guarantee: a value that is merely
    // WRONG stays inside the interval and is returned. See the accuracy
    // section for what the result is worth at large `a`.
    if !(0.0..=1.0).contains(&q) {
        return f64::NAN;
    }
    q
}

/// Shared convergence parameters for the incomplete-gamma series and
/// continued-fraction halves. They MUST stay identical between the two:
/// [`upper_inc_gamma_reg`] switches representation at `x = a + 1`, and
/// differing tolerances would make `Q(a, x)` discontinuous across that
/// seam. That applies to the ceiling below as much as to this
/// tolerance, which is why one function serves both.
const GAMMA_EPS: f64 = 1e-15;

/// How many terms or levels either half may take before it gives up.
///
/// A fixed 200 was not enough and the shortfall was silent. The series
/// needs about \\(9\sqrt{a}\\) terms at \\(x = a\\) — which is
/// \\(\chi^2\\) at a reduced statistic of one, the commonest query a
/// fit makes — so 200 first binds at \\(a = 576.5\\) and by
/// \\(a = 50000\\) the truncated answer was 21% wrong. Measured terms
/// needed against this rule:
///
/// | \\(a\\) | series | fraction | this rule |
/// |---|---|---|---|
/// | 100 | 89 | 39 | 320 |
/// | 5000 | 484 | 152 | 1049 |
/// | 50000 | 1101 | 328 | 2884 |
/// | \\(10^6\\) | 1724 | 894 | 12200 |
///
/// The margin is never within 40% of binding: it is 1.65 at
/// \\(a = 10^{6}\\), its tightest point on that sweep, and 1.74 at
/// \\(a = 10^{9}\\). The mechanism gives the same answer — the series
/// needs about \\(8.31\sqrt a\\) terms against the \\(12\sqrt a\\)
/// this allows — so the ratio tends to 1.44 rather than closing.
/// Exhaustion is still an error rather than a truncation, because a
/// rule fitted to a sweep is not a proof.
fn gamma_max_iter(a: f64) -> usize {
    200 + (12.0 * a.sqrt()).ceil() as usize
}

/// The prefactor \\(e^{-x + a \ln x - \ln\Gamma(a)}\\) common to both
/// incomplete-gamma halves.
#[inline]
fn gamma_prefactor(a: f64, x: f64) -> f64 {
    (-x + a * x.ln() - ln_gamma(a)).exp()
}

/// Regularized **lower** incomplete gamma `P(a, x) = γ(a, x) / Γ(a)` via
/// series expansion. Internal — exposed only through
/// [`upper_inc_gamma_reg`].
fn lower_gamma_series(a: f64, x: f64) -> f64 {
    let mut sum = 1.0 / a;
    let mut term = 1.0 / a;
    let mut converged = false;
    for n in 1..gamma_max_iter(a) {
        term *= x / (a + n as f64);
        sum += term;
        if term.abs() < GAMMA_EPS * sum.abs() {
            converged = true;
            break;
        }
    }
    if !converged {
        return f64::NAN;
    }
    sum * gamma_prefactor(a, x)
}

/// Regularized **upper** incomplete gamma `Q(a, x)` via Lentz's
/// continued-fraction method. Internal — exposed only through
/// [`upper_inc_gamma_reg`].
fn upper_gamma_cf(a: f64, x: f64) -> f64 {
    const TINY: f64 = 1e-30;

    let mut f = TINY;
    let mut c = TINY;
    let mut d = 0.0_f64;
    let mut converged = false;

    for n in 0..gamma_max_iter(a) {
        let an = if n == 0 {
            1.0
        } else {
            -(n as f64) * (n as f64 - a)
        };
        let bn = x - a + 1.0 + 2.0 * n as f64;

        d = bn + an * d;
        if d.abs() < TINY {
            d = TINY;
        }
        c = bn + an / c;
        if c.abs() < TINY {
            c = TINY;
        }
        d = 1.0 / d;
        let delta = c * d;
        f *= delta;
        if (delta - 1.0).abs() < GAMMA_EPS {
            converged = true;
            break;
        }
    }
    if !converged {
        return f64::NAN;
    }

    f * gamma_prefactor(a, x)
}

/// Chi-squared survival function: `P(X² ≥ x) = 1 - F_χ²(x; k) = Q(k/2, x/2)`.
///
/// Used for hypothesis testing and p-value reporting on χ² statistics.
///
/// Returns:
/// - `f64::NAN` if `x` is NaN or `k == 0`
/// - `1.0` if `x ≤ 0` (the survival function is 1 to the left of the
///   support)
/// - `0.0` at `x = +∞`, the limit of the survival function, matching
///   [`normal_sf`] at its own infinity rather than returning the NaN
///   that `-∞ + ∞` in the exponent would otherwise produce
/// - `upper_inc_gamma_reg(k/2, x/2)` otherwise
///
/// # Accuracy
///
/// Relative error under \\(5\times10^{-13}\\) across \\(k \le 15\\)
/// and \\(x\\) from \\(10^{-8}\\) to 2000, measured against
/// arbitrary-precision references; the worst found on a dense sweep of
/// that range is \\(1.1\times10^{-13}\\), near \\(k = 12,\ x = 1167\\).
/// The grid the test suite carries is coarser and finds
/// \\(5.3\times10^{-14}\\) at the nearest point it holds.
///
/// Outside that range it inherits the mechanism described at
/// [`upper_inc_gamma_reg`] and degrades with the arguments rather than
/// with the answer: \\(7.7\times10^{-11}\\) at
/// \\(k = x = 10^{5}\\). No single constant covers the whole domain,
/// so both are stated.
///
/// # Examples
///
/// ```
/// use hyperjet::statistics::chi2_sf;
///
/// // χ²(1) at x=0 has SF=1 (all probability to the right of 0).
/// assert!((chi2_sf(0.0, 1) - 1.0).abs() < 1e-12);
/// // χ²(1) at x=large has SF≈0.
/// assert!(chi2_sf(50.0, 1) < 1e-10);
/// // For k=2, χ² is exponential with mean 2: SF(x) = exp(-x/2).
/// assert!((chi2_sf(4.0, 2) - (-2.0_f64).exp()).abs() < 1e-12);
/// ```
pub fn chi2_sf(x: f64, k: usize) -> f64 {
    if x.is_nan() || k == 0 {
        return f64::NAN;
    }
    if x <= 0.0 {
        return 1.0;
    }
    if x.is_infinite() {
        // The limit, and not what the arithmetic below would give: the
        // prefactor forms `-x + a ln x`, which at an infinite `x` is
        // `-inf + inf` and therefore NaN.
        return 0.0;
    }
    let a = k as f64 / 2.0;
    let z = x / 2.0;
    upper_inc_gamma_reg(a, z)
}

/// Above this magnitude the density splits its exponent; below it, the
/// direct form is already accurate to about one unit in the last place,
/// because the rounding of \\(x^2/2\\) costs a relative
/// \\(\varepsilon x^2 / 2 \le 2.2\times10^{-16}\\) there.
const PDF_SPLIT_ABOVE: f64 = 2.0;

/// Above this magnitude \\(\varphi(x)\\) and \\(Q(x)\\) are both smaller
/// than half the smallest positive subnormal, so the nearest double to
/// either is zero.
///
/// The last argument whose upper tail is still a representable nonzero
/// double is \\(t = 38.4854\ldots\\), where \\(Q = 4.94\times10^{-324}\\),
/// the smallest positive subnormal itself; by \\(t = 39\\) the true value
/// is \\(5.4\times10^{-333}\\), nine orders below anything a double can
/// hold. The cutoff is here so that the exponent split cannot square an
/// argument into an infinity at absurd inputs — it is NOT a clamp. The
/// arithmetic itself, not this constant, is what finally underflows: the
/// last argument at which [`normal_sf`] returns a nonzero is
/// \\(38.48531\\) against a true edge of \\(38.48541\\), and
/// [`normal_pdf`] \\(38.56996\\) against \\(38.58016\\). In those two
/// slivers the true value is the smallest subnormal and the result is
/// zero. Note that the two edges differ from each other and that neither
/// is this constant; 39 sits clear of both.
const TAIL_UNDERFLOWS_ABOVE: f64 = 39.0;

/// Standard normal probability density function
/// \\(\varphi(x) = (2\pi)^{-1/2} e^{-x^2/2}\\).
///
/// # Accuracy
///
/// Relative error below \\(10^{-15}\\) at every \\(x\\) where the density
/// is a normal double; the worst measured over half a million arguments
/// against arbitrary-precision references is \\(4.9\times10^{-16}\\), at
/// \\(x = 33.95\\). The naive \\(e^{-x^2/2}\\) does not manage
/// that in the tail: rounding \\(x^2/2\\) to a double misplaces the
/// exponent by \\(\varepsilon x^2/2\\), which the exponential turns into
/// the same relative error in the result — \\(1.8\times10^{-14}\\) at
/// \\(x = 37\\). So for \\(\lvert x\rvert > 2\\) the exponent is split as
/// \\[
///   \frac{x^2}{2} = \frac{h^2}{2} + \frac{(x-h)(x+h)}{2},
///   \qquad h = \frac{\lfloor 64 x\rfloor}{64},
/// \\]
/// in which \\(h^2/2\\) is exactly representable and the remainder never
/// exceeds \\(0.61\\), so neither piece carries an error that grows with
/// \\(x\\) (Cody 1969). For \\(\lvert x\rvert \le 2\\) the direct form is
/// already good to about one unit in the last place and is used unchanged.
///
/// The granularity of \\(h\\) is what sets the remaining error. \\(x - h\\)
/// is exact but \\(x + h\\) is not, so the product carries a half-ulp of
/// \\(2x\\) scaled by \\(x - h \le 1/64\\): at sixteenths the bound is
/// \\(1.1\times10^{-15}\\) and the measured worst is
/// \\(7.2\times10^{-16}\\), at sixty-fourths they are
/// \\(2.7\times10^{-16}\\) and the figure quoted above. Finer than that
/// buys nothing: the floor is the two exponentials and the two roundings
/// around them.
///
/// # Examples
///
/// ```
/// use hyperjet::statistics::normal_pdf;
/// use std::f64::consts::PI;
///
/// // φ(0) = 1/√(2π).
/// assert!((normal_pdf(0.0) - 1.0 / (2.0 * PI).sqrt()).abs() < 1e-15);
/// // Symmetric: φ(x) == φ(-x).
/// assert_eq!(normal_pdf(1.5), normal_pdf(-1.5));
/// // Accurate deep in the tail, where the naive exponent is not:
/// // φ(30) = 1.4736e-196 to sixteen figures.
/// assert!((normal_pdf(30.0) / 1.473_646_134_878_547_6e-196 - 1.0).abs() < 1e-15);
/// ```
#[inline]
pub fn normal_pdf(x: f64) -> f64 {
    let ax = x.abs();
    if ax <= PDF_SPLIT_ABOVE {
        return (-0.5 * x * x).exp() / (2.0 * PI).sqrt();
    }
    if ax.is_nan() {
        return f64::NAN;
    }
    if ax >= TAIL_UNDERFLOWS_ABOVE {
        return 0.0;
    }
    // `head` keeps six fractional bits, so `head * head` and half of it
    // are both exact for every argument the branch above lets through:
    // `head` is `k / 64` with `k < 2496`, and `k * k` is under 2^53. A
    // test walks every reachable `k` and checks that exactness rather
    // than trusting the arithmetic here.
    let head = (ax * 64.0).trunc() / 64.0;
    let rest = (ax - head) * (ax + head);
    (-0.5 * head * head).exp() * (-0.5 * rest).exp() / (2.0 * PI).sqrt()
}

/// Where the error-function series hands over to the continued fraction.
///
/// The series computes \\(Q\\) as \\(1/2\\) minus a quantity that reaches
/// \\(0.46\\) here, so the subtraction magnifies the relative error of
/// that quantity by \\(0.46/Q(1.75) = 11.4\\). That magnification is the
/// whole of the function's worst case, so this constant is the only dial
/// that moves it, and the trade is explicit: handing over at \\(2\\)
/// magnifies by \\(21\\), here by \\(11.4\\), at \\(1.5\\) by
/// \\(6.5\\) and at \\(1\\) by \\(2.2\\). Against that, the fraction's
/// depth grows as \\(t^{-2}\\): 150 levels here, 196 at \\(1.5\\), 271
/// at \\(1.25\\), 407 at \\(1\\). Moving to \\(1.5\\) would buy roughly
/// a third off the worst case for roughly a third more time in the
/// band beyond it.
const SERIES_SWITCH: f64 = 1.75;

/// The series stops when a term falls below this fraction of the sum;
/// it takes 22 terms at [`SERIES_SWITCH`] and fewer below.
const SERIES_REL_EPS: f64 = 1e-18;

/// A ceiling on the series length, never reached in the branch's domain.
const SERIES_MAX_TERMS: usize = 100;

/// \\(\sum_{n\ge 0} t^{2n} / (1\cdot 3\cdots(2n+1))\\), by compensated
/// summation (Kahan 1965).
///
/// Every term is positive, so the sum itself cannot cancel. Compensation
/// is worth its handful of operations only because the caller subtracts
/// the result from \\(1/2\\) and multiplies its error by up to
/// \\(11.4\\) at the handover. It buys under a factor of two there and
/// costs four operations a term, which is worth it only because that
/// handover is where the whole function's error lives.
///
/// A note on how that is known, since it is easy to get wrong: the
/// figures come from arbitrary-precision references, not from comparing
/// this branch against a deep evaluation of the continued fraction. Two
/// double-precision routines that share a density share an error floor,
/// their errors partly cancel, and the comparison flatters both.
fn erf_series_sum(t: f64) -> f64 {
    let t_sq = t * t;
    let mut term = 1.0_f64;
    let mut sum = 1.0_f64;
    let mut correction = 0.0_f64;
    let mut converged = false;
    for n in 0..SERIES_MAX_TERMS {
        term *= t_sq / (2 * n + 3) as f64;
        let adjusted = term - correction;
        let raised = sum + adjusted;
        correction = (raised - sum) - adjusted;
        sum = raised;
        if term < SERIES_REL_EPS * sum {
            converged = true;
            break;
        }
    }
    // Returning a silently truncated sum is the failure mode this guard
    // exists to refuse. It takes 22 terms at the handover and fewer
    // below, so the ceiling is unreachable in the branch's domain.
    debug_assert!(
        converged,
        "the error-function series ran out of terms at t^2 = {t_sq}"
    );
    sum
}

/// Truncation depth for [`upper_tail_cf`].
///
/// The depth that reaches a relative \\(3\times10^{-16}\\) was measured
/// against correctly rounded references at every \\(t\\) from
/// [`SERIES_SWITCH`] outward: 119 at the switch, 93 at \\(t = 2\\), 47 at
/// \\(t = 3\\), 19 at \\(t = 6\\), 5 at \\(t = 37\\). This rule stays
/// above that requirement everywhere, by at least 24% — its tightest
/// point is \\(t = 1.8\\), where it spends 143 terms on a requirement of
/// 115. The margin is not taken on trust: a test runs the fraction at
/// this depth and at four times it across the whole domain and requires
/// the two to agree.
#[inline]
fn cf_depth(t: f64) -> usize {
    // At t = 0 the expression is infinite and the saturating cast turns
    // it into `usize::MAX`, which would leave `upper_tail_cf` spinning.
    // The public entry points cannot reach that, but an in-crate caller
    // could, so it is named rather than left to be discovered.
    debug_assert!(
        t >= SERIES_SWITCH,
        "cf_depth is only calibrated from the handover outward, got {t}"
    );
    12 + (360.0 / (t * t) + 35.0 / t).ceil() as usize
}

/// \\(Q(t) = 1 - \Phi(t)\\) by the continued fraction of Abramowitz &
/// Stegun 26.2.14 (DLMF 7.9.3), truncated at `depth`:
/// \\[
///   Q(t) = \varphi(t)\;
///   \cfrac{1}{t + \cfrac{1}{t + \cfrac{2}{t + \cfrac{3}{t + \ddots}}}} .
/// \\]
///
/// Evaluated from the far end inward, which needs one division per level
/// rather than the two a forward recurrence would need, and which is the
/// stable direction for a fraction whose partial numerators grow.
///
/// The result is a product of two quantities each accurate to a few units
/// in the last place — the density and the fraction — so no subtraction
/// of nearly equal numbers happens anywhere on this branch, and the
/// relative accuracy holds however small \\(Q\\) becomes.
fn upper_tail_cf(t: f64, depth: usize) -> f64 {
    let mut fraction = 0.0_f64;
    for level in (1..=depth).rev() {
        fraction = level as f64 / (t + fraction);
    }
    normal_pdf(t) / (t + fraction)
}

/// \\(Q(t) = 1 - \Phi(t)\\) for \\(t \ge 0\\), to a relative accuracy of
/// \\(10^{-14}\\) at every \\(t\\) whose tail is a normal double.
///
/// The worst measured against arbitrary-precision references is
/// \\(6.3\times10^{-15}\\), at \\(t = 1.7466\\) — inside the series, just
/// under the handover, where the magnification below is largest. Beyond
/// the handover it stays under \\(10^{-15}\\), the largest seen across
/// several million arguments being \\(6.1\times10^{-16}\\). The bound therefore carries
/// a factor of \\(1.6\\), not more, and that is where the whole
/// function's error lives: the error budget of the series is about four
/// roundings in \\(u\\) times a magnification of \\(11.4\\), which is
/// \\(5\times10^{-15}\\) before the exponential's own contribution.
///
/// Only the SMALL side is ever formed. \\(Q\\) here is at most \\(1/2\\)
/// and falls to \\(10^{-324}\\), and both public entry points arrange to
/// call this with the argument whose tail is the small one, so a caller
/// never pays for a cancellation this routine could have avoided.
fn upper_tail(t: f64) -> f64 {
    debug_assert!(t >= 0.0 || t.is_nan(), "upper_tail takes t >= 0, got {t}");
    if t >= TAIL_UNDERFLOWS_ABOVE {
        return 0.0;
    }
    if t < SERIES_SWITCH {
        // A&S 26.2.11 / DLMF 7.6.2, rearranged so the density carries the
        // exponential: erf(t/√2) = 2 t φ(t) Σ. Every term is positive, so
        // the series cannot cancel. What is left is 1/2 − u with
        // u ≤ 0.46, exact by Sterbenz wherever u ≥ 1/4, which is
        // t ≳ 0.6745 — the only part of the branch where the
        // magnification u/Q is big enough to care. Below that the result
        // exceeds 1/4 and the subtraction costs a half-ulp of an O(1)
        // number.
        return 0.5 - t * normal_pdf(t) * erf_series_sum(t);
    }
    upper_tail_cf(t, cf_depth(t))
}

/// \\(\Phi(t) - 1/2\\) for \\(t \ge 0\\), the mass between the origin and
/// \\(t\\), to full relative accuracy however small \\(t\\) is.
///
/// Inside the handover this is the series' own product, before the
/// subtraction from \\(1/2\\) that [`upper_tail`] performs — every term
/// positive, nothing cancelling, so a bracket of \\(10^{-3}\\) about the
/// origin comes out as accurately as one of \\(3\sigma\\). Beyond the
/// handover the tail is at most 0.041 and subtracting it from \\(1/2\\)
/// costs a half-ulp of an \\(O(1)\\) number.
fn central_mass(t: f64) -> f64 {
    debug_assert!(t >= 0.0 || t.is_nan(), "central_mass takes t >= 0, got {t}");
    if t < SERIES_SWITCH {
        t * normal_pdf(t) * erf_series_sum(t)
    } else if t >= TAIL_UNDERFLOWS_ABOVE {
        0.5
    } else {
        0.5 - upper_tail_cf(t, cf_depth(t))
    }
}

/// Standard normal cumulative distribution function
/// \\(\Phi(x) = P(Z \le x)\\).
///
/// # Accuracy
///
/// Relative error below \\(10^{-14}\\) at every \\(x\\) for which
/// \\(\Phi(x)\\) is a normal double — the measured worst is
/// \\(6.3\times10^{-15}\\), at \\(x = \pm 1.7466\\) — which runs from
/// \\(x = -38.49\\),
/// where \\(\Phi\\) is the smallest positive subnormal, all the way up.
/// There is no cutoff, in either direction: \\(\Phi(-20)\\) is
/// \\(2.75\times10^{-89}\\) and \\(\Phi(-37)\\) is
/// \\(5.73\times10^{-300}\\), and both are returned. Below \\(-38.49\\)
/// the result is zero because the true value is smaller than any double,
/// not because it was rounded off.
///
/// \\(\Phi(0)\\) is exactly \\(1/2\\) and \\(\Phi(-x)\\) is exactly
/// [`normal_sf`]`(x)`.
///
/// # Monotonicity
///
/// Non-decreasing at the resolution of its own accuracy, and beyond
/// \\(1.75\sigma\\) rather better than that: over consecutive doubles
/// the continued-fraction branch is monotone at every point tested,
/// because the true increment there is about \\(t^2 2^{-52}\\) relative
/// against a jitter of a unit or two in the last place. Inside
/// \\(1.75\sigma\\) it is not. There the series forms
/// \\(1/2 - u\\), whose noise floor exceeds the true increment between
/// neighbouring doubles, so a walk of 20 000 consecutive doubles from
/// \\(-1.75\\) steps backward about 5 600 times, by at most
/// \\(5.5\times10^{-15}\\) relative. That count is measured on one
/// platform and shifts by a step or two on another, since it turns on
/// how the library's `exp` rounds; the behaviour it describes does
/// not. Anything that bisects on this function — an inverse CDF, say
/// — must tolerate steps of that size.
///
/// # Cost
///
/// About 21 ns inside \\(1.75\sigma\\), 97 ns from there to
/// \\(8\sigma\\) where the continued fraction runs deepest, and 25 ns
/// beyond that, against 6 ns for a rational approximation of the kind
/// this replaces. Accuracy in the tail is what the difference buys.
///
/// # Choosing between this and [`normal_sf`]
///
/// \\(\Phi(x)\\) for large positive \\(x\\) is a number just under one,
/// and doubles run out of room to distinguish it from one at
/// \\(x = 8.3\\) — not a defect, simply what \\(1 - 10^{-17}\\) rounds
/// to. Anyone who wants the vanishing quantity there wants
/// [`normal_sf`], which returns it to the same relative accuracy as this
/// function returns the lower tail. Forming `1.0 - normal_cdf(x)` throws
/// that away and is never the right thing to write.
///
/// # Method
///
/// Below \\(1.75\sigma\\) from the origin, the all-positive error-function
/// series of Abramowitz & Stegun 26.2.11; beyond it, the continued
/// fraction of 26.2.14, at a depth that follows \\(t\\). The Gaussian
/// factor comes from [`normal_pdf`], whose split exponent is what makes
/// the deep tail relatively accurate rather than merely small.
///
/// # Examples
///
/// ```
/// use hyperjet::statistics::normal_cdf;
///
/// assert_eq!(normal_cdf(0.0), 0.5);
/// // Symmetric: Φ(x) + Φ(-x) = 1.
/// for &x in &[0.5_f64, 1.0, 2.0, 3.0] {
///     assert!((normal_cdf(x) + normal_cdf(-x) - 1.0).abs() <= f64::EPSILON);
/// }
/// // 1σ contains 68.27%: Φ(1) - Φ(-1) = 0.682689492137086.
/// assert!((normal_cdf(1.0) - normal_cdf(-1.0) - 0.682_689_492_137_085_9).abs() < 1e-15);
/// // The far tail is a number, not a zero: Φ(-10) = 7.6199e-24.
/// assert!((normal_cdf(-10.0) / 7.619_853_024_160_525e-24 - 1.0).abs() < 1e-14);
/// ```
pub fn normal_cdf(x: f64) -> f64 {
    if x.is_nan() {
        return f64::NAN;
    }
    if x <= 0.0 {
        upper_tail(-x)
    } else {
        1.0 - upper_tail(x)
    }
}

/// Standard normal survival function \\(Q(x) = P(Z > x) = 1 - \Phi(x)\\).
///
/// # Accuracy
///
/// Relative error below \\(10^{-14}\\) at every \\(x\\) for which
/// \\(Q(x)\\) is a normal double, which is every \\(x\\) up to
/// \\(38.49\\); the measured worst is \\(6.3\times10^{-15}\\), at
/// \\(x = 1.7466\\). \\(Q(6) = 9.87\times10^{-10}\\) and
/// \\(Q(30) = 4.91\times10^{-198}\\) are returned to that accuracy, not
/// as zeros and not as the difference of two numbers near one.
///
/// This exists so that no caller ever writes `1.0 - normal_cdf(x)`. That
/// expression is exact arithmetic on an inexact premise: \\(\Phi(x)\\) is
/// stored as a double near one, whose spacing is \\(2.2\times10^{-16}\\),
/// so subtracting it from one leaves a tail with no significant figures
/// at all beyond \\(x \approx 8\\) and none whatsoever beyond
/// \\(x \approx 8.3\\). This function forms the small side directly and
/// never the large one.
///
/// \\(Q(0)\\) is exactly \\(1/2\\) and \\(Q(x)\\) is exactly
/// [`normal_cdf`]`(-x)`, so it inherits that function's monotonicity
/// exactly: non-increasing bit for bit beyond \\(1.75\sigma\\), and
/// inside it non-increasing only to within its own accuracy.
///
/// # Examples
///
/// ```
/// use hyperjet::statistics::{normal_cdf, normal_sf};
///
/// assert_eq!(normal_sf(0.0), 0.5);
/// // The exact reflection of the CDF.
/// assert_eq!(normal_sf(3.5), normal_cdf(-3.5));
/// // Q(6) = 9.86587645037698e-10 — a 6σ tail, to full relative accuracy.
/// assert!((normal_sf(6.0) / 9.865_876_450_376_98e-10 - 1.0).abs() < 1e-14);
/// // Where `1.0 - normal_cdf(x)` has nothing left, this still does.
/// assert_eq!(1.0 - normal_cdf(9.0), 0.0);
/// assert!(normal_sf(9.0) > 0.0);
/// ```
pub fn normal_sf(x: f64) -> f64 {
    if x.is_nan() {
        return f64::NAN;
    }
    if x >= 0.0 {
        upper_tail(x)
    } else {
        1.0 - upper_tail(-x)
    }
}

/// \\(\Phi(\text{hi}) - \Phi(\text{lo})\\), the probability that a
/// standard normal lands in a bracket, formed so that the bracket is not
/// lost to cancellation.
///
/// Writing it as `normal_cdf(hi) - normal_cdf(lo)` is what this replaces.
/// When both ends sit on the same side of the origin, both CDF values
/// are near the same limit — near one above the origin, near zero below
/// — and the difference of two doubles near one has at best a few
/// figures left. On a far-miss geometry that error is then multiplied by
/// whatever the caller scales the bracket by, and a rounding of
/// \\(10^{-16}\\) on two \\(O(1)\\) values becomes the answer.
///
/// This differences the SMALL tails instead, in whichever of three
/// arrangements avoids the subtraction:
///
/// * both ends strictly above the origin — subtract upper tails;
/// * both strictly below it — subtract lower tails, which are upper
///   tails of the negated ends, in the reverse order;
/// * otherwise, which is any bracket containing the origin or touching
///   it — add the two masses between the origin and each end, both
///   positive, so nothing is subtracted at all.
///
/// # Accuracy
///
/// The result carries the relative accuracy of the quantities it
/// combines, magnified by their sum divided by the answer. A straddling
/// bracket adds two positive halves, so nothing is magnified at all and
/// a bracket of \\(\pm 10^{-3}\\) about the origin is as accurate as
/// one of \\(3\sigma\\). A bracket with both ends on one side
/// subtracts two tails, and there the magnification is real: it is 1
/// when the near end carries most of the tail, and grows as the bracket
/// narrows.
///
/// The case this does NOT fix should be stated plainly. A narrow
/// same-side bracket cancels, and how much it cancels follows its
/// width rather than its depth: the magnification is the larger of the
/// two quantities differenced divided by the bracket, which is roughly
/// one over the width in units of the local density. Measured:
///
/// | bracket | width | relative error |
/// |---|---|---|
/// | \\((6.001, 6)\\) | \\(10^{-3}\\) | \\(4.3\times10^{-14}\\) |
/// | \\((1.001, 1)\\) | \\(10^{-3}\\) | \\(2.1\times10^{-13}\\) |
/// | \\((1 + 10^{-9}, 1)\\) | \\(10^{-9}\\) | \\(9.6\times10^{-8}\\) |
///
/// The series band is no better than the tail here, which is the sign
/// that the loss is the geometry and not the method. Once the
/// magnification passes a hundred or so, neither this nor the naive
/// difference is reliably the closer of the two: at
/// \\((1.5 + 10^{-9},\, 1.5)\\) the naive form happens to land at
/// \\(9.7\times10^{-8}\\) and this one at \\(3.3\times10^{-7}\\). What
/// survives is the guarantee below, not an ordering. Recovering such a
/// bracket needs the integral over it rather than a difference of its
/// ends.
///
/// What is guaranteed is that no accuracy is lost which the bracket did
/// not already lose: the naive difference of CDF values loses figures
/// on EVERY same-side bracket however wide, and loses all of them past
/// \\(8\sigma\\).
///
/// `hi` and `lo` may be given in either order; the result is the signed
/// difference, and the arrangement is chosen from the ordered pair.
///
/// # Examples
///
/// ```
/// use hyperjet::statistics::{normal_cdf, normal_cdf_difference};
///
/// // 1σ contains 68.27%.
/// assert!((normal_cdf_difference(1.0, -1.0) - 0.682_689_492_137_085_9).abs() < 1e-15);
/// // A bracket four sigma deep in one tail keeps every figure.
/// let deep = normal_cdf_difference(-4.44, -5.95);
/// assert!((deep / 4.496_603_176_123_816e-6 - 1.0).abs() < 1e-14);
/// // Where the naive difference of CDF values has nothing left at all.
/// assert_eq!(normal_cdf(37.0) - normal_cdf(30.0), 0.0);
/// assert!((normal_cdf_difference(37.0, 30.0) / 4.906_713_927_148_187e-198 - 1.0).abs() < 1e-14);
/// ```
pub fn normal_cdf_difference(hi: f64, lo: f64) -> f64 {
    if hi < lo {
        return -normal_cdf_difference(lo, hi);
    }
    if hi.is_nan() || lo.is_nan() {
        return f64::NAN;
    }
    // Strictly, so that an endpoint AT the origin falls through to the
    // arm below. The tail there is exactly 1/2, and subtracting it from
    // the tail just beside it keeps nothing of a narrow bracket: at a
    // width of 1e-9 that arm is wrong by 3e-8 relative and the arm below
    // by 1.4e-17. `central_mass` takes either signed zero and returns it
    // unchanged, so the bracket costs nothing there.
    if lo > 0.0 {
        upper_tail(lo) - upper_tail(hi)
    } else if hi < 0.0 {
        upper_tail(-hi) - upper_tail(-lo)
    } else {
        // Not `1 - Q(hi) - Q(-lo)`: those two tails sum to nearly one
        // for a narrow bracket about the origin, and subtracting them
        // from one throws the bracket away. The two halves are formed
        // directly instead and added, both positive.
        central_mass(hi) + central_mass(-lo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── ln_gamma ─────────────────────────────────────────────────────

    #[test]
    fn ln_gamma_integer_values() {
        // ln Γ(n) = ln (n-1)! for integer n ≥ 1.
        let factorials = [1.0_f64, 1.0, 2.0, 6.0, 24.0, 120.0, 720.0, 5040.0];
        for (n, &f) in factorials.iter().enumerate() {
            let expected = f.ln();
            let got = ln_gamma(n as f64 + 1.0);
            assert!(
                (got - expected).abs() < 1e-12,
                "n={n}: got {got}, expected {expected}"
            );
        }
    }

    #[test]
    fn ln_gamma_half_integer() {
        // Γ(1/2) = √π
        assert!((ln_gamma(0.5) - PI.sqrt().ln()).abs() < 1e-12);
        // Γ(3/2) = (1/2)√π
        assert!((ln_gamma(1.5) - (0.5 * PI.sqrt()).ln()).abs() < 1e-12);
        // Γ(5/2) = (3/4)√π
        assert!((ln_gamma(2.5) - (0.75 * PI.sqrt()).ln()).abs() < 1e-12);
    }

    #[test]
    fn ln_gamma_large_argument() {
        // Stirling: ln Γ(x) ≈ (x - 1/2) ln x - x + (1/2) ln(2π) for large x.
        for &x in &[10.0_f64, 50.0, 100.0, 1000.0] {
            let stirling = (x - 0.5) * x.ln() - x + 0.5 * (2.0 * PI).ln();
            let got = ln_gamma(x);
            let rel = (got - stirling).abs() / stirling.abs();
            assert!(
                rel < 1e-3,
                "x={x}: got {got}, stirling {stirling}, rel {rel}"
            );
        }
    }

    // ── upper_inc_gamma_reg ───────────────────────────────────────────

    #[test]
    fn upper_inc_gamma_reg_boundary() {
        // Q(a, 0) = 1 for any a > 0
        for &a in &[0.5_f64, 1.0, 2.0, 5.0] {
            assert!((upper_inc_gamma_reg(a, 0.0) - 1.0).abs() < 1e-12);
        }
    }

    #[test]
    fn upper_inc_gamma_reg_a_equals_one() {
        // Q(1, x) = e^{-x} (exponential survival)
        for &x in &[0.1_f64, 1.0, 5.0, 10.0] {
            let expected = (-x).exp();
            let got = upper_inc_gamma_reg(1.0, x);
            assert!(
                (got - expected).abs() < 1e-12,
                "x={x}: got {got}, expected {expected}"
            );
        }
    }

    #[test]
    fn upper_inc_gamma_reg_large_x() {
        // Q(a, x) → 0 as x → ∞
        for &a in &[0.5_f64, 1.0, 2.0] {
            assert!(upper_inc_gamma_reg(a, 100.0) < 1e-30);
        }
    }

    #[test]
    fn upper_inc_gamma_reg_invalid_inputs() {
        assert!(upper_inc_gamma_reg(-1.0, 1.0).is_nan());
        assert!(upper_inc_gamma_reg(1.0, -1.0).is_nan());
        assert!(upper_inc_gamma_reg(f64::NAN, 1.0).is_nan());
        assert!(upper_inc_gamma_reg(1.0, f64::NAN).is_nan());
    }

    // ── chi2_sf ───────────────────────────────────────────────────────

    #[test]
    fn chi2_sf_at_zero() {
        for &k in &[1_usize, 2, 6] {
            assert!((chi2_sf(0.0, k) - 1.0).abs() < 1e-12);
        }
    }

    #[test]
    fn chi2_sf_k_equals_two_is_exponential() {
        // χ²(2) is Exp(1/2): SF(x) = exp(-x/2)
        for &x in &[1.0_f64, 3.0, 10.0] {
            let expected = (-x / 2.0).exp();
            let got = chi2_sf(x, 2);
            assert!(
                (got - expected).abs() < 1e-12,
                "x={x}: got {got}, expected {expected}"
            );
        }
    }

    #[test]
    fn chi2_sf_large_x_underflows_smoothly() {
        // For x >> k, SF should be vanishingly small but non-negative.
        for &k in &[1_usize, 6] {
            for &x in &[100.0_f64, 200.0] {
                let sf = chi2_sf(x, k);
                assert!(sf >= 0.0);
                assert!(sf < 1e-15);
            }
        }
    }

    /// The survival function at an infinite argument is its limit. The
    /// arithmetic underneath would not get there on its own: the
    /// prefactor forms `-x + a ln x`, which is `-inf + inf`.
    #[test]
    fn chi2_sf_at_infinity_is_zero_and_not_a_non_number() {
        for &k in &[1_usize, 2, 6, 15] {
            assert_eq!(chi2_sf(f64::INFINITY, k), 0.0, "k = {k}");
        }
        // The same question asked of the normal tail, for consistency.
        assert_eq!(normal_sf(f64::INFINITY), 0.0);
    }

    /// Small arguments used to return infinity: `x - 1.0` rounds to
    /// exactly -1 below 2^-54, which puts a zero in the first
    /// denominator of the Lanczos sum. The true values are ordinary
    /// doubles and are now returned.
    #[test]
    fn ln_gamma_is_finite_at_small_arguments() {
        // Correctly rounded, from tools/normal_reference_table.py.
        let cases = [
            (1e-300_f64, 690.775_527_898_213_7),
            (1e-100, 230.258_509_299_404_58),
            (5.551_115_123_125_783e-17, 37.429_947_750_237_05),
            (1e-16, 36.841_361_487_904_734),
            (1e-8, 18.420_680_738_180_21),
        ];
        for (x, expected) in cases {
            let got = ln_gamma(x);
            assert!(got.is_finite(), "ln Γ({x}) returned {got}");
            let rel = (got - expected).abs() / expected;
            assert!(
                rel < 1e-14,
                "ln Γ({x}) = {got}, expected {expected}, rel {rel}"
            );
        }
        // The recurrence changes nothing at or above its threshold, so
        // every value this crate computes for itself is untouched. The
        // smallest argument the crate supplies is k/2 = 0.5 at k = 1.
        assert!((ln_gamma(0.5) - PI.sqrt().ln()).abs() < 1e-15);
        assert!((ln_gamma(1.5) - (0.5 * PI.sqrt()).ln()).abs() < 1e-15);
    }

    #[test]
    fn ln_gamma_edges_are_named() {
        assert_eq!(ln_gamma(0.0), f64::INFINITY);
        assert_eq!(ln_gamma(f64::INFINITY), f64::INFINITY);
        assert!(ln_gamma(-1.0).is_nan());
        assert!(ln_gamma(-0.5).is_nan());
        assert!(ln_gamma(f64::NAN).is_nan());
    }

    /// The small-`a` regime is refused rather than answered, and the
    /// refusal is by name at a stated threshold.
    ///
    /// The routine forms `Q = 1 - P` below its seam, and for small `a`
    /// the true `Q` is itself of order `a`, so the subtraction returns
    /// nothing: at a = 1e-16 it was 282 times wrong, silently, inside a
    /// domain documented as `a > 0`.
    #[test]
    fn a_small_shape_parameter_is_refused_and_not_approximated() {
        for &a in &[1e-16_f64, 1e-12, 1e-8, 1e-4, 0.01, 0.1, 0.25, 0.4999] {
            for &x in &[1e-6_f64, 0.5, 1.0, 2.0] {
                let got = upper_inc_gamma_reg(a, x);
                assert!(got.is_nan(), "Q({a}, {x}) returned {got} below the domain");
            }
        }
        // The threshold itself answers, on both sides of its seam, and
        // it is exactly the smallest `a` that `chi2_sf` can supply.
        assert_eq!(UPPER_INC_GAMMA_MIN_A, 0.5);
        for &x in &[0.001_f64, 0.4, 1.4999, 1.50001, 20.0] {
            assert!(
                upper_inc_gamma_reg(UPPER_INC_GAMMA_MIN_A, x).is_finite(),
                "Q(0.5, {x}) should be answered"
            );
        }
        assert!(chi2_sf(1.0, 1).is_finite());
    }

    /// Which branch an argument takes is not left to arithmetic
    /// coincidence. An earlier version of this suite believed it was
    /// probing the series at a = 1e-16 while `a + 1` rounded to exactly
    /// 1.0 and sent it to the fraction instead, so the arm that was
    /// broken was never exercised.
    #[test]
    fn the_branches_are_reached_where_the_tests_believe_they_are() {
        for &a in &[0.5_f64, 1.0, 7.5, 100.0, 5000.0] {
            let seam = a + 1.0;
            let below = seam - seam * f64::EPSILON;
            assert!(
                below < seam,
                "a = {a}: the series probe is not below the seam"
            );
            let series_side = upper_inc_gamma_reg(a, below);
            let fraction_side = upper_inc_gamma_reg(a, seam);
            assert!(series_side.is_finite() && fraction_side.is_finite());
            // The step is bounded by the routine's own accuracy at that
            // argument, which is set by the largest term in the exponent
            // rather than by the answer — the mechanism the accuracy
            // section states. At a = 5000 that allows 3.8e-11 and the
            // measured step is 7.1e-12.
            let allowed = 1e-14 + 4.0 * f64::EPSILON * seam.max(a * seam.ln());
            let step = (series_side - fraction_side).abs() / fraction_side;
            assert!(
                step < allowed,
                "a = {a}: the branches step by {step:e} against an allowance of {allowed:e}"
            );
        }
    }

    /// The iteration ceiling was fixed at 200 and reaching it returned
    /// the truncated sum. It first binds at a = 576.5, and at a = 25000
    /// — a chi-squared with 50000 degrees of freedom at a reduced
    /// statistic of one — the truncated answer was 21% wrong. The
    /// ceiling now follows `a`, and exhausting it is an error.
    #[test]
    fn the_iteration_ceiling_follows_the_shape_parameter() {
        // Q(a, a) approaches 1/2 from below as a grows; a truncated sum
        // at these arguments overshoots it by whole percent.
        for &a in &[2500.0_f64, 5000.0, 25000.0, 50000.0] {
            let got = upper_inc_gamma_reg(a, a);
            assert!(got.is_finite(), "Q({a}, {a}) = {got}");
            assert!(
                (0.49..0.5).contains(&got),
                "Q({a}, {a}) = {got}, which is not just under one half"
            );
        }
        // The rule stays ahead of what each half needs, by the factor
        // of two its own documentation claims.
        for (a, series, fraction) in [
            (100.0_f64, 89_usize, 39_usize),
            (5000.0, 484, 152),
            (50000.0, 1101, 328),
            (1e6, 1724, 894),
        ] {
            let ceiling = gamma_max_iter(a);
            assert!(
                ceiling > 2 * series.max(fraction),
                "at a = {a} the ceiling {ceiling} is under twice the {} needed",
                series.max(fraction)
            );
        }
    }

    /// The top of the domain, where the contract is a formula rather
    /// than a number.
    ///
    /// Two different things happen up there and both are pinned. The
    /// result stays a probability and stops being the right one, and
    /// the published bound eps * max(x, a ln x) still covers how wrong
    /// it is — which is what makes that bound usable rather than
    /// decorative. Then the arithmetic leaves the unit interval and the
    /// exit check refuses.
    ///
    /// No reference table is needed to show the error, because
    /// `Q(a, a)` is below one half for every finite `a`: the median of
    /// a gamma distribution is under its mean, and `Q(a, a)` rises to
    /// one half only in the limit. A value above one half is therefore
    /// proof of arithmetic failure on its own terms.
    #[test]
    fn the_upper_edge_is_bounded_by_its_stated_mechanism_then_refused() {
        // Still correct in kind: a probability, and under one half.
        for &a in &[1e3_f64, 1e5, 1e6, 1e8] {
            let got = upper_inc_gamma_reg(a, a);
            assert!(
                (0.0..0.5).contains(&got),
                "Q({a:e}, {a:e}) = {got}, which is not a probability under one half"
            );
        }

        // Demonstrably wrong, with no reference: above one half. The
        // published bound covers the excess at each.
        for &a in &[1e10_f64, 1e12, 1e14] {
            let got = upper_inc_gamma_reg(a, a);
            assert!(
                (0.0..=1.0).contains(&got),
                "Q({a:e}, {a:e}) = {got} left the unit interval without being refused"
            );
            assert!(
                got > 0.5,
                "Q({a:e}, {a:e}) = {got} is no longer demonstrably wrong; re-measure the \
                 accuracy table if the routine improved"
            );
            let bound = f64::EPSILON * a.max(a * a.ln());
            let excess = (got - 0.5) / 0.5;
            assert!(
                excess <= bound,
                "Q({a:e}, {a:e}) = {got} exceeds one half by {excess:e}, over the \
                 published bound {bound:e}"
            );
        }

        // Six figures gone by 1e10 and all of them by 1e14.
        assert!((upper_inc_gamma_reg(1e10, 1e10) - 0.5) / 0.5 > 1e-6);
        assert!((upper_inc_gamma_reg(1e14, 1e14) - 0.5) / 0.5 > 0.1);

        // Past where it stops being a probability it is a NaN, rather
        // than the -18.23 the series produced.
        let broken = upper_inc_gamma_reg(1e15, 1e15);
        assert!(
            broken.is_nan(),
            "Q(1e15, 1e15) = {broken} should be refused"
        );

        // And the exit check cannot fire on anything the engine reaches.
        for &k in &[1_usize, 2, 6, 15] {
            for &x in &[0.5_f64, 1.0, 12.59, 100.0, 1000.0] {
                assert!(chi2_sf(x, k).is_finite(), "chi2_sf({x}, {k})");
            }
        }
    }

    /// A sweep of the documented domain, out past where a fixed ceiling
    /// used to bind. Every answer has to be a number in [0, 1]: a NaN
    /// here means a half gave up, which is now an error rather than a
    /// truncation, and either way must not happen on this range.
    #[test]
    fn no_argument_on_the_documented_domain_exhausts_a_ceiling() {
        let mut checked = 0_usize;
        let mut a = UPPER_INC_GAMMA_MIN_A;
        while a <= 60_000.0 {
            let mut x = 1e-4_f64;
            while x < 8.0 * a + 800.0 {
                let q = upper_inc_gamma_reg(a, x);
                assert!(
                    q.is_finite() && (0.0..=1.0).contains(&q),
                    "Q({a}, {x}) = {q}"
                );
                checked += 1;
                x *= 1.35;
            }
            for &x in &[
                a + 1.0 - f64::EPSILON * a,
                a + 1.0,
                a + 1.0 + f64::EPSILON * a,
            ] {
                assert!(upper_inc_gamma_reg(a, x).is_finite(), "seam at a = {a}");
            }
            a = if a < 100.0 { a + 0.5 } else { a * 1.6 };
        }
        assert!(
            checked > 5_000,
            "the sweep covered only {checked} arguments"
        );
    }

    #[test]
    fn chi2_sf_invalid_inputs() {
        assert!(chi2_sf(f64::NAN, 1).is_nan());
        assert!(chi2_sf(1.0, 0).is_nan());
    }

    // ── normal_pdf ────────────────────────────────────────────────────

    #[test]
    fn normal_pdf_at_zero() {
        let expected = 1.0 / (2.0 * PI).sqrt();
        assert!((normal_pdf(0.0) - expected).abs() < 1e-15);
    }

    #[test]
    fn normal_pdf_symmetric() {
        // Exactly, not approximately: the argument is squared before
        // anything else touches it, and both branches take `x.abs()`.
        for &x in &[0.5_f64, 1.0, 2.5, 5.0, 12.5, 30.0] {
            assert_eq!(normal_pdf(x), normal_pdf(-x));
        }
    }

    #[test]
    fn normal_pdf_known_values() {
        // φ(1) = 0.24197072451914337
        assert!((normal_pdf(1.0) - 0.241_970_724_519_143_37).abs() < 1e-15);
        // φ(2) = 0.05399096651318806
        assert!((normal_pdf(2.0) - 0.053_990_966_513_188_06).abs() < 1e-15);
    }

    /// The split of the exponent is meant to be invisible: the two forms
    /// must agree where they meet, or the density has a step in it.
    #[test]
    fn the_two_density_branches_agree_at_their_seam() {
        let below = normal_pdf(PDF_SPLIT_ABOVE);
        let above = normal_pdf(f64::from_bits(PDF_SPLIT_ABOVE.to_bits() + 1));
        // One ulp of argument moves φ by 2·φ·ulp(2)/2 ≈ 4e-16 relative,
        // so anything under a few times that is agreement.
        let step = (below - above).abs() / below;
        assert!(step < 4e-15, "the density steps by {step:e} at its seam");
    }

    #[test]
    fn normal_pdf_handles_non_finite_and_absurd_arguments() {
        assert!(normal_pdf(f64::NAN).is_nan());
        assert_eq!(normal_pdf(f64::INFINITY), 0.0);
        assert_eq!(normal_pdf(f64::NEG_INFINITY), 0.0);
        // Large finite arguments must not square themselves into an
        // infinity and come back NaN.
        for &x in &[40.0_f64, 1e100, 1e300, -1e300] {
            assert_eq!(normal_pdf(x), 0.0, "φ({x}) should underflow to zero");
        }
    }

    // ── normal_cdf and normal_sf ──────────────────────────────────────

    /// The Abramowitz & Stegun 26.2.17 rational approximation, which
    /// [`normal_cdf`] used before this crate carried a relatively
    /// accurate tail. Kept so the improvement is a measurement in the
    /// suite rather than a claim in a commit message.
    fn abramowitz_stegun_26_2_17(x: f64) -> f64 {
        if x < -8.0 {
            return 0.0;
        }
        if x > 8.0 {
            return 1.0;
        }
        let sign = if x >= 0.0 { 1.0 } else { -1.0 };
        let ax = x.abs();
        let t = 1.0 / (1.0 + 0.231_641_9 * ax);
        let t2 = t * t;
        let t3 = t2 * t;
        let t4 = t3 * t;
        let t5 = t4 * t;
        let poly = 0.319_381_530 * t - 0.356_563_782 * t2 + 1.781_477_937 * t3 - 1.821_255_978 * t4
            + 1.330_274_429 * t5;
        let cdf_abs = 1.0 - normal_pdf(ax) * poly;
        0.5 + sign * 0.5 * (2.0 * cdf_abs - 1.0)
    }

    /// What the replaced approximation was wrong by, and what the
    /// present one is right to, at the same points.
    ///
    /// The old form's error is ABSOLUTE — about 7.5e-8 everywhere, which
    /// is the whole of a 6σ tail and a hundred times the whole of an 8σ
    /// one. Reference values are correctly rounded, from
    /// `tools/normal_reference_table.py`.
    #[test]
    fn the_replaced_approximation_was_wrong_in_the_tail_by_these_amounts() {
        // (σ, Φ(-σ), the relative error the A&S form makes there)
        let cases = [
            (4.0_f64, 3.167_124_183_311_998e-5, 4e-4),
            (6.0, 9.865_876_450_376_98e-10, 3e-3),
            (8.0, 6.220_960_574_271_784e-16, 7e-2),
        ];
        for (sigma, reference, at_least) in cases {
            let old = abramowitz_stegun_26_2_17(-sigma);
            let old_rel = (old - reference).abs() / reference;
            assert!(
                old_rel > at_least,
                "at {sigma}σ the A&S form is only off by {old_rel:e}; \
                 this test exists because it is off by more"
            );
            let new_rel = (normal_cdf(-sigma) - reference).abs() / reference;
            assert!(
                new_rel < 1e-14,
                "at {sigma}σ the replacement is off by {new_rel:e}"
            );
            println!("{sigma}σ: A&S relative error {old_rel:e}, now {new_rel:e}");
        }
    }

    /// The clamp the old form carried was a jump discontinuity, not an
    /// underflow: it returned zero where the true value was 6e-16, a
    /// number a double holds with fifteen significant figures to spare.
    #[test]
    fn the_replaced_approximation_clamped_a_representable_tail_to_zero() {
        for &x in &[-8.01_f64, -10.0, -20.0, -37.0] {
            assert_eq!(
                abramowitz_stegun_26_2_17(x),
                0.0,
                "the A&S form is supposed to clamp at {x}"
            );
            assert!(
                normal_cdf(x) > 0.0,
                "Φ({x}) is representable and must not be clamped"
            );
        }
        // Measured across the clamp, as a fraction of the tail being
        // stepped over. A&S loses all of it; the true change across an
        // interval that narrow is nine orders smaller than the value.
        let tail = 6.220_960_574_271_784e-16;
        let jump = (abramowitz_stegun_26_2_17(-7.999_999_999)
            - abramowitz_stegun_26_2_17(-8.000_000_001))
            / tail;
        assert!(
            jump > 0.9,
            "the clamp should drop the whole tail, dropped {jump:e}"
        );
        let step = (normal_cdf(-7.999_999_999) - normal_cdf(-8.000_000_001)) / tail;
        assert!(step < 1e-7, "the replacement steps by {step:e} of the tail");
        println!("across the old clamp: A&S drops {jump:e} of the tail, this drops {step:e}");
    }

    #[test]
    fn the_cdf_and_the_survival_function_are_exact_at_the_origin() {
        assert_eq!(normal_cdf(0.0), 0.5);
        assert_eq!(normal_sf(0.0), 0.5);
        assert_eq!(normal_cdf(-0.0), 0.5);
        assert_eq!(normal_sf(-0.0), 0.5);
    }

    /// Q(x) and Φ(-x) are the same mathematical quantity, so they are
    /// the same bits — not merely close.
    #[test]
    fn the_survival_function_is_the_reflected_cdf_bit_for_bit() {
        let mut x = -38.5_f64;
        while x <= 38.5 {
            assert_eq!(
                normal_sf(x).to_bits(),
                normal_cdf(-x).to_bits(),
                "Q({x}) and Φ({}) disagree",
                -x
            );
            x += 0.031_25;
        }
    }

    #[test]
    fn the_cdf_and_its_reflection_sum_to_one() {
        for &x in &[0.125_f64, 0.5, 1.0, 1.749, 1.75, 2.0, 3.0, 5.0, 8.0, 20.0] {
            let sum = normal_cdf(x) + normal_cdf(-x);
            assert!(
                (sum - 1.0).abs() <= f64::EPSILON,
                "Φ({x}) + Φ(-{x}) = {sum}"
            );
        }
    }

    #[test]
    fn sigma_coverage_matches_the_textbook_figures() {
        // Correctly rounded P(-kσ ≤ Z ≤ kσ).
        let coverage = [
            (1.0_f64, 0.682_689_492_137_085_9),
            (2.0, 0.954_499_736_103_641_6),
            (3.0, 0.997_300_203_936_740_1),
        ];
        for (k, expected) in coverage {
            let p = normal_cdf(k) - normal_cdf(-k);
            assert!(
                (p - expected).abs() < 1e-15,
                "{k}σ coverage {p}, expected {expected}"
            );
        }
    }

    /// A distribution function that decreases anywhere is not one, at
    /// the scale anyone samples it at. This sweeps at 1e-3, where the
    /// true increment is around 1e-4 and swamps the rounding jitter; the
    /// behaviour between neighbouring doubles is a different question
    /// and is pinned separately below. The sweep crosses both the
    /// series/continued-fraction seam and the density's exponent seam.
    #[test]
    fn the_cdf_is_non_decreasing_across_the_whole_range() {
        let mut previous = 0.0_f64;
        for step in 0..80_000_u32 {
            let x = -40.0 + f64::from(step) * 0.001;
            let value = normal_cdf(x);
            assert!(
                value >= previous,
                "Φ dropped from {previous:e} to {value:e} at x = {x}"
            );
            previous = value;
        }
        assert_eq!(previous, 1.0);
    }

    /// Monotonicity between NEIGHBOURING doubles, which is a stronger
    /// question than the sampled sweep above answers and which has two
    /// different answers.
    ///
    /// Beyond the handover the continued fraction is monotone: 20 000
    /// consecutive doubles from each of several starting points, and not
    /// one step backward. The mechanism is that the true relative
    /// increment there is about t^2 * 2^-52 against a jitter of a unit
    /// or two in the last place of Q — a ratio of three at the handover,
    /// growing as t^2.
    ///
    /// That is a mechanism and not a proof. It rests on `exp` being
    /// monotone, which IEEE-754 does not require of a library function,
    /// so the assertion carries a small allowance rather than demanding
    /// a zero it cannot guarantee.
    ///
    /// Inside the handover the series is not monotone, and cannot be. It
    /// forms 1/2 − u, whose noise floor is larger than the true
    /// increment between adjacent doubles, so the result jitters. The
    /// counts are a property of how often the jitter wins rather than of
    /// any single rounding, so they are pinned within a tolerance: this
    /// platform gives 4 586 backward steps from −1.5 and the Linux legs
    /// of CI give 4 587.
    #[test]
    fn monotonicity_between_neighbouring_doubles_has_two_answers() {
        fn walk(start: f64) -> (usize, f64) {
            let mut x = start;
            let mut previous = normal_cdf(x);
            let mut backward = 0_usize;
            let mut worst = 0.0_f64;
            for _ in 0..20_000 {
                x = f64::from_bits(if x < 0.0 {
                    x.to_bits() - 1
                } else {
                    x.to_bits() + 1
                });
                let value = normal_cdf(x);
                if value < previous {
                    backward += 1;
                    worst = worst.max((previous - value) / previous);
                }
                previous = value;
            }
            (backward, worst)
        }

        // The continued-fraction branch, on both sides of the origin.
        // Zero on every platform measured; the allowance is a thousandth
        // of the walk, which a differently rounded `exp` could reach at
        // the handover and which a real regression would pass by three
        // orders of magnitude.
        const FRACTION_ALLOWANCE: usize = 20;
        for &start in &[
            -1.8_f64, -2.0, -3.0, -6.0, -20.0, -37.0, 1.75, 2.0, 3.0, 4.0, 6.0,
        ] {
            let (backward, _) = walk(start);
            println!("fraction branch from {start}: {backward} backward steps in 20000");
            assert!(
                backward <= FRACTION_ALLOWANCE,
                "the fraction branch stepped backward {backward} times from {start}, \
                 over an allowance of {FRACTION_ALLOWANCE}"
            );
        }

        // The series branch, where monotonicity does not hold. The
        // counts rise with the magnification, which is what sets the
        // noise floor. They are recorded on aarch64-apple-darwin and the
        // Linux legs differ by a single step at -1.5, so the tolerance
        // is two percent: a hundred times that spread, and still far
        // tighter than any real change, since fixing the series would
        // take these to zero and degrading it would multiply them.
        const SERIES_TOLERANCE: f64 = 0.02;
        let series = [
            (-1.75_f64, 5_611_usize),
            (-1.5, 4_586),
            (-1.0, 2_459),
            (-0.75, 1_019),
        ];
        for (start, recorded) in series {
            let (backward, worst) = walk(start);
            println!("{start}: {backward} backward steps in 20000, worst {worst:e}");
            let drift = (backward as f64 - recorded as f64).abs() / recorded as f64;
            assert!(
                drift <= SERIES_TOLERANCE,
                "backward steps from {start} moved from {recorded} to {backward}, \
                 a drift of {drift:.4} over the {SERIES_TOLERANCE} allowed"
            );
            // And the order of magnitude the documentation claims: the
            // series is non-monotone on a large minority of steps, not
            // on a handful and not on all of them.
            assert!(
                (1_000..10_000).contains(&backward),
                "backward steps from {start} came to {backward}, outside the \
                 documented order of magnitude"
            );
            assert!(
                worst < 1e-14,
                "a backward step of {worst:e} at {start} exceeds the accuracy bound"
            );
        }
    }

    /// The shipped truncation depth has to be deep enough across the
    /// domain, not only at the arguments the reference table samples.
    /// Run the fraction far deeper and require the same answer: if the
    /// rule were short anywhere on the sweep, the two would part company
    /// there.
    ///
    /// This establishes that the truncation has CONVERGED, which is not
    /// the same as establishing that the converged value is right; both
    /// sides are the same fraction. Accuracy comes from the
    /// arbitrary-precision references. The sweep is at 1.25e-3, which is
    /// dense, not exhaustive.
    #[test]
    fn the_continued_fraction_is_converged_at_the_shipped_depth() {
        let mut worst = (0.0_f64, 0.0_f64);
        let mut t = SERIES_SWITCH;
        while t < TAIL_UNDERFLOWS_ABOVE {
            let shipped = upper_tail_cf(t, cf_depth(t));
            let deeper = upper_tail_cf(t, 4 * cf_depth(t) + 40);
            let rel = (shipped - deeper).abs() / deeper;
            if rel > worst.0 {
                worst = (rel, t);
            }
            t += 0.001_25;
        }
        assert!(
            worst.0 < 1e-15,
            "the shipped depth is short by {:e} at t = {}",
            worst.0,
            worst.1
        );
        println!(
            "worst shipped-vs-deep disagreement {:e} at t = {}",
            worst.0, worst.1
        );
    }

    /// The two branches are separate constructions of the same quantity,
    /// so where they overlap they must agree — otherwise the CDF has a
    /// step at the handover.
    ///
    /// This establishes AGREEMENT and not accuracy. Both sides take
    /// their Gaussian factor from [`normal_pdf`], so they share an error
    /// floor and their errors partly cancel; the number this prints is
    /// smaller than either side's true error, and reading it as a bound
    /// is the fallacy `tests/normal_accuracy.rs` warns about. The
    /// accuracy claim rests on the arbitrary-precision references there
    /// and nowhere else.
    ///
    /// The sweep starts at 0.4 rather than 0 because that is where a
    /// 4000-level fraction stops being converged: it is good to 1.4e-16
    /// at 0.4 and only 4.6e-14 at 0.25. Nothing is lost, because below
    /// 0.4 the series magnification is under one half and there is no
    /// accuracy question to answer.
    #[test]
    fn the_two_branches_agree_where_they_overlap() {
        let mut worst = (0.0_f64, 0.0_f64);
        let mut t = 0.4_f64;
        while t < SERIES_SWITCH {
            let series = 0.5 - t * normal_pdf(t) * erf_series_sum(t);
            let fraction = upper_tail_cf(t, 4_000);
            let rel = (series - fraction).abs() / fraction;
            if rel > worst.0 {
                worst = (rel, t);
            }
            t += 0.000_5;
        }
        assert!(
            worst.0 < 1e-14,
            "the branches part company by {:e} at t = {}",
            worst.0,
            worst.1
        );
        println!(
            "worst branch disagreement {:e} at t = {} (a floor, not a bound)",
            worst.0, worst.1
        );
    }

    /// The exactness the exponent split rests on, walked rather than
    /// asserted in a comment: `head` is `k / 64` for an integer `k`, and
    /// `head * head` must be exactly `k * k / 4096` at every `k` the
    /// density can reach. If it were not, the split would be moving
    /// error around rather than removing it.
    #[test]
    fn every_reachable_split_head_squares_exactly() {
        let granularity = 64_u64;
        let highest = (TAIL_UNDERFLOWS_ABOVE as u64) * granularity;
        for k in 1..=highest {
            let head = k as f64 / granularity as f64;
            let exact = (k * k) as f64 / (granularity * granularity) as f64;
            assert_eq!(
                head * head,
                exact,
                "head = {head} squares inexactly at k = {k}"
            );
            assert_eq!(-0.5 * head * head, -(exact / 2.0));
        }
        // And the truncation really does land on that lattice.
        for &x in &[2.000_1_f64, 7.3, 19.999, 33.953_051_231_955_506, 38.9] {
            let head = (x * 64.0).trunc() / 64.0;
            assert_eq!(head * 64.0, (head * 64.0).trunc());
            assert!(x - head < 1.0 / 64.0);
        }
    }

    /// The series and the continued fraction have to meet, or the CDF
    /// has a step at the handover.
    #[test]
    fn the_series_and_the_continued_fraction_agree_at_their_seam() {
        let series_side =
            0.5 - SERIES_SWITCH * normal_pdf(SERIES_SWITCH) * erf_series_sum(SERIES_SWITCH);
        let fraction_side = upper_tail_cf(SERIES_SWITCH, cf_depth(SERIES_SWITCH));
        let rel = (series_side - fraction_side).abs() / fraction_side;
        assert!(rel < 1e-14, "the branches disagree by {rel:e} at the seam");
    }

    /// The tail is a number wherever a double can hold one. This is the
    /// property the replaced approximation did not have.
    #[test]
    fn the_tail_runs_to_the_edge_of_the_representable_range() {
        // Correctly rounded Q at each point.
        let deep = [
            (10.0_f64, 7.619_853_024_160_525e-24),
            (20.0, 2.753_624_118_606_233_7e-89),
            (30.0, 4.906_713_927_148_187e-198),
            (37.0, 5.725_571_222_524_577e-300),
            (38.0, 2.885_428_35e-316),
        ];
        for (x, expected) in deep {
            let got = normal_sf(x);
            let rel = (got - expected).abs() / expected;
            // 38σ is subnormal, where the spacing itself is 1.7e-8
            // relative; everything above it is a normal double.
            let bound = if x >= 37.5 { 1e-7 } else { 1e-14 };
            assert!(rel < bound, "Q({x}) = {got:e}, expected {expected:e}");
        }
        // Nonzero right up to the last representable tail, zero after.
        assert!(normal_sf(38.48) > 0.0);
        assert_eq!(normal_sf(38.6), 0.0);
        assert_eq!(normal_sf(f64::INFINITY), 0.0);
        assert_eq!(normal_cdf(f64::NEG_INFINITY), 0.0);
        assert_eq!(normal_cdf(f64::INFINITY), 1.0);
        assert_eq!(normal_sf(f64::NEG_INFINITY), 1.0);
    }

    #[test]
    fn non_numbers_propagate() {
        assert!(normal_cdf(f64::NAN).is_nan());
        assert!(normal_sf(f64::NAN).is_nan());
    }

    /// The whole point of publishing the survival function: past 8σ the
    /// complement of the CDF has no significant figures left, and past
    /// 8.3σ it has no figures at all.
    #[test]
    fn the_survival_function_outlives_the_complement_of_the_cdf() {
        assert_eq!(1.0 - normal_cdf(9.0), 0.0);
        assert!((normal_sf(9.0) / 1.128_588_405_953_840_5e-19 - 1.0).abs() < 1e-14);
    }
}
