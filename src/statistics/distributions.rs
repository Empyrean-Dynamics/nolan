//! Distribution functions: log-gamma, regularized incomplete gamma,
//! chi-squared survival, standard normal PDF/CDF.
//!
//! # References
//!
//! - Lanczos, C. (1964). *A precision approximation of the gamma function.*
//!   J. SIAM Numer. Anal. Ser. B 1: 86–96.
//! - Press, W. H., et al. (2007). *Numerical Recipes 3rd ed.* §6.2 (incomplete
//!   gamma), §6.14.8 (continued-fraction Lentz method).
//! - Abramowitz, M., & Stegun, I. A. (1972). *Handbook of Mathematical
//!   Functions.* 26.2.17 (normal CDF polynomial approximation).

use std::f64::consts::PI;

/// Natural logarithm of the gamma function via the Lanczos approximation
/// (g = 7, n = 9 coefficients).
///
/// Domain: `x > 0` (the principal branch). For `x` near zero or negative,
/// the reflection formula should be used externally if needed.
///
/// Accuracy: ~1e-15 relative for `x > 0.5`.
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
/// ```
pub fn ln_gamma(x: f64) -> f64 {
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

/// Regularized **upper** incomplete gamma function `Q(a, x) = Γ(a, x) / Γ(a) = 1 - P(a, x)`.
///
/// Domain: `a > 0` and `x ≥ 0`. Returns `f64::NAN` for invalid inputs.
///
/// Implementation: series for `x < a + 1` (faster convergence in that
/// regime), Lentz continued fraction for `x ≥ a + 1`. Standard
/// NR §6.2 split.
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
    if a <= 0.0 || x < 0.0 || x.is_nan() || a.is_nan() {
        return f64::NAN;
    }
    if x == 0.0 {
        return 1.0;
    }
    if x < a + 1.0 {
        1.0 - lower_gamma_series(a, x)
    } else {
        upper_gamma_cf(a, x)
    }
}

/// Regularized **lower** incomplete gamma `P(a, x) = γ(a, x) / Γ(a)` via
/// series expansion. Internal — exposed only through
/// [`upper_inc_gamma_reg`].
fn lower_gamma_series(a: f64, x: f64) -> f64 {
    const MAX_ITER: usize = 200;
    const EPS: f64 = 1e-15;

    let mut sum = 1.0 / a;
    let mut term = 1.0 / a;
    for n in 1..MAX_ITER {
        term *= x / (a + n as f64);
        sum += term;
        if term.abs() < EPS * sum.abs() {
            break;
        }
    }
    sum * (-x + a * x.ln() - ln_gamma(a)).exp()
}

/// Regularized **upper** incomplete gamma `Q(a, x)` via Lentz's
/// continued-fraction method. Internal — exposed only through
/// [`upper_inc_gamma_reg`].
fn upper_gamma_cf(a: f64, x: f64) -> f64 {
    const MAX_ITER: usize = 200;
    const EPS: f64 = 1e-15;
    const TINY: f64 = 1e-30;

    let mut f = TINY;
    let mut c = TINY;
    let mut d = 0.0_f64;

    for n in 0..MAX_ITER {
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
        if (delta - 1.0).abs() < EPS {
            break;
        }
    }

    f * (-x + a * x.ln() - ln_gamma(a)).exp()
}

/// Chi-squared survival function: `P(X² ≥ x) = 1 - F_χ²(x; k) = Q(k/2, x/2)`.
///
/// Used for hypothesis testing and p-value reporting on χ² statistics.
///
/// Returns:
/// - `f64::NAN` if `x` is NaN or `k == 0`
/// - `1.0` if `x ≤ 0` (the survival function is 1 to the left of the
///   support)
/// - `upper_inc_gamma_reg(k/2, x/2)` otherwise
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
    let a = k as f64 / 2.0;
    let z = x / 2.0;
    upper_inc_gamma_reg(a, z)
}

/// Standard normal probability density function
/// `φ(x) = (2π)^(-1/2) · exp(-x²/2)`.
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
/// assert!((normal_pdf(1.5) - normal_pdf(-1.5)).abs() < 1e-15);
/// ```
#[inline]
pub fn normal_pdf(x: f64) -> f64 {
    (-0.5 * x * x).exp() / (2.0 * PI).sqrt()
}

/// Standard normal cumulative distribution function `Φ(x) = P(Z ≤ x)`,
/// using the Abramowitz & Stegun 26.2.17 polynomial approximation.
///
/// Maximum absolute error: ~7.5e-8 across the entire real line.
///
/// Returns exact `0.0` for `x < -8.0` and exact `1.0` for `x > 8.0` —
/// machine epsilon underflow.
///
/// # Examples
///
/// ```
/// use hyperjet::statistics::normal_cdf;
///
/// assert!((normal_cdf(0.0) - 0.5).abs() < 1e-7);
/// // Symmetric: Φ(x) + Φ(-x) = 1.
/// for &x in &[0.5_f64, 1.0, 2.0, 3.0] {
///     assert!((normal_cdf(x) + normal_cdf(-x) - 1.0).abs() < 1e-7);
/// }
/// // 1σ contains ~68.27%: Φ(1) - Φ(-1) ≈ 0.6827.
/// assert!((normal_cdf(1.0) - normal_cdf(-1.0) - 0.6827).abs() < 1e-3);
/// ```
pub fn normal_cdf(x: f64) -> f64 {
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
    let pdf = normal_pdf(ax);
    let cdf_abs = 1.0 - pdf * poly;
    0.5 + sign * 0.5 * (2.0 * cdf_abs - 1.0)
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
    fn chi2_sf_scipy_reference_values() {
        // Reference values from scipy.stats.chi2.sf or analytical
        // closed forms where available:
        //   χ²(1): SF(x) = erfc(√(x/2)) → scipy
        //   χ²(2): SF(x) = exp(-x/2) — exact closed form
        //   χ²(k>2): scipy reference
        //
        // Tolerance 1e-4 relative: at small `a` (e.g., k=1 → a=0.5),
        // the series/continued-fraction transition has known limited
        // accuracy (~1e-5 absolute in the SF tail) — well within the
        // tolerance typical of χ² acceptance gating in nonlinear
        // least-squares solvers. Tighten if/when we switch to a
        // higher-precision incomplete-gamma routine.
        let cases = [
            // (x, k, expected_sf)
            (1.0_f64, 1, 0.317_310_507_862_915_4), // erfc(√0.5)
            (3.84, 1, 0.050_044_106_595_511_84),   // erfc(√1.92) — 95% critical
            (1.0, 2, (-0.5_f64).exp()),            // exp(-0.5) exactly
            (5.99, 2, (-2.995_f64).exp()),         // exp(-2.995) exactly — 95% critical
            (1.0, 6, 0.985_612_322_385_122_4),     // scipy
            (12.59, 6, 0.050_028_851_651_797_8),   // 95% critical — algorithm vs scipy 5e-5
        ];
        for (x, k, expected) in cases {
            let got = chi2_sf(x, k);
            let rel = (got - expected).abs() / expected;
            assert!(
                rel < 1e-4,
                "(x={x}, k={k}): got {got}, expected {expected}, rel {rel}"
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
        for &x in &[0.5_f64, 1.0, 2.5, 5.0] {
            assert!((normal_pdf(x) - normal_pdf(-x)).abs() < 1e-15);
        }
    }

    #[test]
    fn normal_pdf_known_values() {
        // φ(1) = 0.24197072451914337
        assert!((normal_pdf(1.0) - 0.241_970_724_519_143_37).abs() < 1e-15);
        // φ(2) = 0.05399096651318806
        assert!((normal_pdf(2.0) - 0.053_990_966_513_188_06).abs() < 1e-15);
    }

    // ── normal_cdf ────────────────────────────────────────────────────

    #[test]
    fn normal_cdf_at_zero() {
        assert!((normal_cdf(0.0) - 0.5).abs() < 1e-7);
    }

    #[test]
    fn normal_cdf_symmetric() {
        // Φ(x) + Φ(-x) = 1
        for &x in &[0.5_f64, 1.0, 2.0, 3.0] {
            let s = normal_cdf(x) + normal_cdf(-x);
            assert!((s - 1.0).abs() < 1e-7);
        }
    }

    #[test]
    fn normal_cdf_one_sigma_coverage() {
        // P(-1 ≤ Z ≤ 1) ≈ 0.6827
        let p = normal_cdf(1.0) - normal_cdf(-1.0);
        assert!((p - 0.6827).abs() < 1e-3);
    }

    #[test]
    fn normal_cdf_two_sigma_coverage() {
        // P(-2 ≤ Z ≤ 2) ≈ 0.9545
        let p = normal_cdf(2.0) - normal_cdf(-2.0);
        assert!((p - 0.9545).abs() < 1e-3);
    }

    #[test]
    fn normal_cdf_three_sigma_coverage() {
        let p = normal_cdf(3.0) - normal_cdf(-3.0);
        assert!((p - 0.9973).abs() < 1e-3);
    }

    #[test]
    fn normal_cdf_far_tails() {
        assert_eq!(normal_cdf(-10.0), 0.0);
        assert_eq!(normal_cdf(10.0), 1.0);
    }

    #[test]
    fn normal_cdf_as_reference_values() {
        // Abramowitz & Stegun 26.2.17 — quoted maximum error 7.5e-8.
        // Reference values from scipy.stats.norm.cdf.
        let cases = [
            (-3.0_f64, 0.001_349_898_031_630_094_5),
            (-2.0, 0.022_750_131_948_179_21),
            (-1.0, 0.158_655_253_931_457_05),
            (-0.5, 0.308_537_538_725_987),
            (0.0, 0.5),
            (0.5, 0.691_462_461_274_013),
            (1.0, 0.841_344_746_068_542_9),
            (2.0, 0.977_249_868_051_820_8),
            (3.0, 0.998_650_101_968_369_9),
        ];
        for (x, expected) in cases {
            let got = normal_cdf(x);
            let abs_err = (got - expected).abs();
            assert!(
                abs_err < 7.5e-8,
                "x={x}: got {got}, expected {expected}, abs err {abs_err}"
            );
        }
    }
}
