//! Angle wrapping primitives.
//!
//! Wrap an angle into a canonical interval. Used everywhere astrodynamics
//! code subtracts angles (residuals, anomaly differences, RA/Dec
//! distances, etc.) — without wrapping, a subtraction across the 0/360°
//! or 0/2π boundary produces nonsense.
//!
//! Four conventions:
//!
//! | Function    | Input → Output range  | Units   |
//! |-------------|-----------------------|---------|
//! | `wrap_pi`   | any → (-π, π]         | radians |
//! | `wrap_2pi`  | any → [0, 2π)         | radians |
//! | `wrap_180`  | any → (-180°, 180°]   | degrees |
//! | `wrap_360`  | any → [0°, 360°)      | degrees |
//!
//! The half-open intervals are chosen so that `wrap_pi(π) == π` (not
//! `-π`) and `wrap_360(360°) == 0°` — matches the convention used by
//! JPL Horizons, MPC ADES, and most other astrodynamics references.
//!
//! # Examples
//!
//! ```
//! use nolan::angles::{wrap_pi, wrap_180};
//! use std::f64::consts::PI;
//!
//! // Differences across the wrap boundary come out correctly.
//! let ra_a_deg = 359.0;
//! let ra_b_deg = 1.0;
//! assert!((wrap_180(ra_a_deg - ra_b_deg) - (-2.0)).abs() < 1e-12);
//!
//! // π itself is preserved.
//! assert!((wrap_pi(PI) - PI).abs() < 1e-12);
//! ```

/// Wrap an angle (radians) into the half-open interval `(-π, π]`.
///
/// # Examples
///
/// ```
/// use nolan::angles::wrap_pi;
/// use std::f64::consts::PI;
///
/// assert!((wrap_pi(0.0) - 0.0).abs() < 1e-15);
/// assert!((wrap_pi(PI) - PI).abs() < 1e-15);
/// assert!((wrap_pi(-PI) - PI).abs() < 1e-15);  // -π wraps to +π
/// assert!((wrap_pi(3.0 * PI) - PI).abs() < 1e-15);
/// ```
#[inline]
pub fn wrap_pi(x: f64) -> f64 {
    use std::f64::consts::{PI, TAU};
    let mut y = x % TAU;
    if y > PI {
        y -= TAU;
    } else if y <= -PI {
        y += TAU;
    }
    y
}

/// Wrap an angle (radians) into the half-open interval `[0, 2π)`.
///
/// # Examples
///
/// ```
/// use nolan::angles::wrap_2pi;
/// use std::f64::consts::{PI, TAU};
///
/// assert!((wrap_2pi(0.0) - 0.0).abs() < 1e-15);
/// assert!((wrap_2pi(TAU) - 0.0).abs() < 1e-15);  // 2π wraps to 0
/// assert!((wrap_2pi(-PI) - PI).abs() < 1e-15);
/// assert!((wrap_2pi(3.0 * TAU + 0.5) - 0.5).abs() < 1e-15);
/// ```
#[inline]
pub fn wrap_2pi(x: f64) -> f64 {
    use std::f64::consts::TAU;
    let y = x % TAU;
    if y < 0.0 { y + TAU } else { y }
}

/// Wrap an angle (degrees) into the half-open interval `(-180°, 180°]`.
///
/// Suitable for differences of right ascension, declination, longitude
/// of ascending node, argument of periapsis, mean anomaly — anywhere
/// in degrees-space arithmetic the result should land in the
/// principal-value branch centered on zero.
///
/// # Examples
///
/// ```
/// use nolan::angles::wrap_180;
///
/// assert!((wrap_180(0.0) - 0.0).abs() < 1e-12);
/// assert!((wrap_180(180.0) - 180.0).abs() < 1e-12);    // exactly 180° preserved
/// assert!((wrap_180(-180.0) - 180.0).abs() < 1e-12);   // −180° wraps to +180°
/// assert!((wrap_180(359.0 - 1.0) - (-2.0)).abs() < 1e-12);  // typical RA wrap
/// assert!((wrap_180(720.0) - 0.0).abs() < 1e-12);      // multiple revolutions
/// ```
#[inline]
pub fn wrap_180(x: f64) -> f64 {
    let mut y = x % 360.0;
    if y > 180.0 {
        y -= 360.0;
    } else if y <= -180.0 {
        y += 360.0;
    }
    y
}

/// Wrap an angle (degrees) into the half-open interval `[0°, 360°)`.
///
/// # Examples
///
/// ```
/// use nolan::angles::wrap_360;
///
/// assert!((wrap_360(0.0) - 0.0).abs() < 1e-12);
/// assert!((wrap_360(360.0) - 0.0).abs() < 1e-12);   // exactly 360° wraps to 0°
/// assert!((wrap_360(-90.0) - 270.0).abs() < 1e-12);
/// assert!((wrap_360(1080.5) - 0.5).abs() < 1e-12);  // multiple revolutions
/// ```
#[inline]
pub fn wrap_360(x: f64) -> f64 {
    let y = x % 360.0;
    if y < 0.0 { y + 360.0 } else { y }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::{PI, TAU};

    // ── wrap_pi ───────────────────────────────────────────────

    #[test]
    fn wrap_pi_principal_branch() {
        assert!((wrap_pi(0.0) - 0.0).abs() < 1e-15);
        assert!((wrap_pi(0.5) - 0.5).abs() < 1e-15);
        assert!((wrap_pi(-0.5) - (-0.5)).abs() < 1e-15);
    }

    #[test]
    fn wrap_pi_boundary_at_plus_pi() {
        // +π is in the half-open interval (-π, π], so it stays put.
        assert!((wrap_pi(PI) - PI).abs() < 1e-15);
    }

    #[test]
    fn wrap_pi_boundary_at_minus_pi() {
        // -π is NOT in (-π, π], so it wraps to +π.
        assert!((wrap_pi(-PI) - PI).abs() < 1e-15);
    }

    #[test]
    fn wrap_pi_multiple_revolutions() {
        assert!((wrap_pi(3.0 * PI) - PI).abs() < 1e-12);
        assert!((wrap_pi(-3.0 * PI) - PI).abs() < 1e-12);
        assert!((wrap_pi(100.0 * TAU + 1.23) - 1.23).abs() < 1e-9);
    }

    #[test]
    fn wrap_pi_typical_residual() {
        // RA residual across 0/2π: model says 0.01 rad past zero,
        // observation says 0.01 rad before zero (i.e., 2π - 0.01).
        // Naive difference would be -2π + 0.02; wrap_pi pulls it back.
        let model = 0.01;
        let obs = TAU - 0.01;
        let raw_diff = model - obs;
        assert!((wrap_pi(raw_diff) - 0.02).abs() < 1e-12);
    }

    // ── wrap_2pi ──────────────────────────────────────────────

    #[test]
    fn wrap_2pi_principal_branch() {
        assert!((wrap_2pi(0.0) - 0.0).abs() < 1e-15);
        assert!((wrap_2pi(1.0) - 1.0).abs() < 1e-15);
        assert!((wrap_2pi(PI) - PI).abs() < 1e-15);
    }

    #[test]
    fn wrap_2pi_boundary_at_2pi() {
        // [0, 2π) — exactly 2π wraps to 0.
        assert!(wrap_2pi(TAU).abs() < 1e-15);
    }

    #[test]
    fn wrap_2pi_negative_inputs() {
        assert!((wrap_2pi(-PI) - PI).abs() < 1e-15);
        assert!((wrap_2pi(-0.1) - (TAU - 0.1)).abs() < 1e-15);
    }

    #[test]
    fn wrap_2pi_multiple_revolutions() {
        assert!((wrap_2pi(3.0 * TAU + 0.5) - 0.5).abs() < 1e-9);
        assert!((wrap_2pi(-3.0 * TAU + 0.5) - 0.5).abs() < 1e-9);
    }

    // ── wrap_180 ──────────────────────────────────────────────

    #[test]
    fn wrap_180_principal_branch() {
        assert!((wrap_180(0.0) - 0.0).abs() < 1e-12);
        assert!((wrap_180(90.0) - 90.0).abs() < 1e-12);
        assert!((wrap_180(-90.0) - (-90.0)).abs() < 1e-12);
    }

    #[test]
    fn wrap_180_boundary_at_plus_180() {
        assert!((wrap_180(180.0) - 180.0).abs() < 1e-12);
    }

    #[test]
    fn wrap_180_boundary_at_minus_180() {
        assert!((wrap_180(-180.0) - 180.0).abs() < 1e-12);
    }

    #[test]
    fn wrap_180_typical_ra_residual() {
        // RA = 359° vs RA = 1° — naive diff is 358°, wrapped to -2°.
        assert!((wrap_180(359.0 - 1.0) - (-2.0)).abs() < 1e-12);
        // Other direction.
        assert!((wrap_180(1.0 - 359.0) - 2.0).abs() < 1e-12);
    }

    #[test]
    fn wrap_180_multiple_revolutions() {
        assert!((wrap_180(720.0) - 0.0).abs() < 1e-12);
        assert!((wrap_180(720.0 + 30.0) - 30.0).abs() < 1e-12);
        assert!((wrap_180(-720.0 - 30.0) - (-30.0)).abs() < 1e-12);
    }

    // ── wrap_360 ──────────────────────────────────────────────

    #[test]
    fn wrap_360_principal_branch() {
        assert!((wrap_360(0.0) - 0.0).abs() < 1e-12);
        assert!((wrap_360(180.0) - 180.0).abs() < 1e-12);
    }

    #[test]
    fn wrap_360_boundary_at_360() {
        // [0°, 360°) — exactly 360° wraps to 0°.
        assert!(wrap_360(360.0).abs() < 1e-12);
    }

    #[test]
    fn wrap_360_negative_inputs() {
        assert!((wrap_360(-90.0) - 270.0).abs() < 1e-12);
        assert!((wrap_360(-180.0) - 180.0).abs() < 1e-12);
        assert!((wrap_360(-360.0) - 0.0).abs() < 1e-12);
    }

    #[test]
    fn wrap_360_multiple_revolutions() {
        assert!((wrap_360(1080.5) - 0.5).abs() < 1e-9);
        assert!((wrap_360(-1080.5) - 359.5).abs() < 1e-9);
    }

    // ── cross-function consistency ────────────────────────────

    #[test]
    fn wrap_pi_consistent_with_wrap_180() {
        // wrap_pi(x rad) == wrap_180(x in degrees) in radians.
        for &x_deg in &[-720.0_f64, -180.5, -30.0, 0.0, 45.0, 179.999, 359.0] {
            let x_rad = x_deg.to_radians();
            let wrapped_rad = wrap_pi(x_rad);
            let wrapped_deg_via_180 = wrap_180(x_deg);
            let diff = (wrapped_rad - wrapped_deg_via_180.to_radians()).abs();
            assert!(
                diff < 1e-12,
                "x_deg={x_deg}: rad path={wrapped_rad} vs deg path={wrapped_deg_via_180}"
            );
        }
    }

    #[test]
    fn wrap_2pi_consistent_with_wrap_360() {
        for &x_deg in &[-720.0_f64, -180.5, -30.0, 0.0, 45.0, 179.999, 359.0, 1080.5] {
            let x_rad = x_deg.to_radians();
            let wrapped_rad = wrap_2pi(x_rad);
            let wrapped_deg = wrap_360(x_deg);
            let diff = (wrapped_rad - wrapped_deg.to_radians()).abs();
            assert!(
                diff < 1e-12,
                "x_deg={x_deg}: rad path={wrapped_rad} vs deg path={wrapped_deg}"
            );
        }
    }
}
