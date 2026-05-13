//! 1D grid generation and linear interpolation primitives.
//!
//! - [`linspace`] — evenly-spaced linear grid on \\([a, b]\\).
//! - [`logspace`] — evenly-spaced logarithmic grid on \\([a, b]\\)
//!   (both endpoints strictly positive).
//! - [`linear_clamped`] — linear interpolation of a tabulated function
//!   \\(y(x)\\), clamped to the table endpoints outside the sampled
//!   domain.
//!
//! Semantics match NumPy `numpy.linspace` / `numpy.logspace` (endpoint
//! inclusive). Single-point grids return a one-element vector containing
//! `min` (matching NumPy's default `endpoint=True` behaviour at `n=1`).

use std::ops::{Add, Mul};

/// Evenly-spaced linear grid of `n` points on the inclusive interval
/// \\([\text{min}, \text{max}]\\).
///
/// Endpoints are exact: `linspace(a, b, n)[0] == a` and
/// `linspace(a, b, n)[n-1] == b` for `n >= 2`.
///
/// - `n == 0` returns an empty vector.
/// - `n == 1` returns `[min]`.
///
/// # Examples
///
/// ```
/// use nolan::grids::linspace;
///
/// assert_eq!(linspace(0.0, 1.0, 3), vec![0.0, 0.5, 1.0]);
/// assert_eq!(linspace(0.0, 1.0, 1), vec![0.0]);
/// assert!(linspace(0.0, 1.0, 0).is_empty());
/// ```
pub fn linspace(min: f64, max: f64, n: usize) -> Vec<f64> {
    if n == 0 {
        return Vec::new();
    }
    if n == 1 {
        return vec![min];
    }
    let mut grid = Vec::with_capacity(n);
    let denom = (n - 1) as f64;
    for i in 0..n {
        if i == 0 {
            grid.push(min);
        } else if i == n - 1 {
            grid.push(max);
        } else {
            let frac = i as f64 / denom;
            grid.push(min + frac * (max - min));
        }
    }
    grid
}

/// Logarithmically-spaced grid of `n` points on the inclusive interval
/// \\([\text{min}, \text{max}]\\), with `min > 0` and `max > 0`.
///
/// The returned values themselves are in linear units (\\(10^{\log x}\\)
/// style); the spacing is uniform in `ln(x)`. Returns NaN-filled output
/// if either endpoint is non-positive (the `ln` of which is `-inf` /
/// `NaN`).
///
/// - `n == 0` returns an empty vector.
/// - `n == 1` returns `[min]`.
///
/// # Examples
///
/// ```
/// use nolan::grids::logspace;
///
/// let g = logspace(1.0, 100.0, 3);
/// assert!((g[0] - 1.0).abs() < 1e-12);
/// assert!((g[1] - 10.0).abs() < 1e-12);
/// assert!((g[2] - 100.0).abs() < 1e-12);
/// ```
pub fn logspace(min: f64, max: f64, n: usize) -> Vec<f64> {
    if n == 0 {
        return Vec::new();
    }
    if n == 1 {
        return vec![min];
    }
    let log_min = min.ln();
    let log_max = max.ln();
    let denom = (n - 1) as f64;
    let mut grid = Vec::with_capacity(n);
    for i in 0..n {
        if i == 0 {
            grid.push(min);
        } else if i == n - 1 {
            grid.push(max);
        } else {
            let frac = i as f64 / denom;
            grid.push((log_min + frac * (log_max - log_min)).exp());
        }
    }
    grid
}

/// Linear interpolation of a tabulated function \\(y(x)\\) at a query
/// point `xq`, clamped to the endpoint values when `xq` falls outside
/// the sampled domain `[xs[0], xs[xs.len()-1]]`.
///
/// `xs` must be sorted in ascending order and `xs.len() == ys.len()`.
/// Both invariants are checked in debug builds via `debug_assert!`.
///
/// # Algorithm
///
/// - If `xs.len() == 1`, returns `ys[0]`.
/// - If `xq <= xs[0]`, returns `ys[0]`.
/// - If `xq >= xs[xs.len()-1]`, returns `ys[xs.len()-1]`.
/// - Otherwise: find the bracketing pair `xs[i] <= xq <= xs[i+1]` (via
///   binary search) and return
///   \\(y_i (1 - \alpha) + y_{i+1} \alpha\\), where
///   \\(\alpha = (x_q - x_i) / (x_{i+1} - x_i)\\). If two adjacent
///   `xs` values are bit-identical (`dx == 0`), returns `ys[i]`.
///
/// # Panics
///
/// Panics if `xs.is_empty()`. (Internal precondition — caller must
/// guard against empty tables.)
///
/// # Examples
///
/// ```
/// use nolan::grids::linear_clamped;
///
/// let xs = [0.0, 1.0, 2.0];
/// let ys = [10.0, 20.0, 25.0];
/// assert_eq!(linear_clamped(&xs, &ys, 0.5), 15.0);
/// assert_eq!(linear_clamped(&xs, &ys, -1.0), 10.0); // clamped left
/// assert_eq!(linear_clamped(&xs, &ys, 3.0), 25.0);  // clamped right
/// ```
pub fn linear_clamped<T>(xs: &[f64], ys: &[T], xq: f64) -> T
where
    T: Copy + Add<Output = T> + Mul<f64, Output = T>,
{
    assert!(!xs.is_empty(), "linear_clamped: xs must be non-empty");
    debug_assert_eq!(
        xs.len(),
        ys.len(),
        "linear_clamped: xs and ys must have the same length"
    );
    debug_assert!(
        xs.windows(2).all(|w| w[0] <= w[1]),
        "linear_clamped: xs must be sorted ascending"
    );

    let n = xs.len();
    if n == 1 || xq <= xs[0] {
        return ys[0];
    }
    if xq >= xs[n - 1] {
        return ys[n - 1];
    }

    // Binary search for the bracketing pair: xs[i] <= xq < xs[i+1].
    // partition_point returns the first index where xs[k] > xq; subtract 1.
    let upper = xs.partition_point(|&x| x <= xq);
    let i = upper - 1;
    let i = i.min(n - 2); // guarantee i+1 < n

    let dx = xs[i + 1] - xs[i];
    if dx == 0.0 {
        return ys[i];
    }
    let alpha = (xq - xs[i]) / dx;
    ys[i] * (1.0 - alpha) + ys[i + 1] * alpha
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── linspace ────────────────────────────────────────────────

    #[test]
    fn linspace_empty() {
        assert!(linspace(0.0, 1.0, 0).is_empty());
    }

    #[test]
    fn linspace_single() {
        assert_eq!(linspace(2.5, 7.0, 1), vec![2.5]);
    }

    #[test]
    fn linspace_two_points() {
        assert_eq!(linspace(-3.0, 5.0, 2), vec![-3.0, 5.0]);
    }

    #[test]
    fn linspace_uniform_spacing() {
        let g = linspace(0.0, 10.0, 11);
        assert_eq!(g.len(), 11);
        for (i, v) in g.iter().enumerate() {
            assert!((v - i as f64).abs() < 1e-12);
        }
    }

    #[test]
    fn linspace_endpoints_exact() {
        // Endpoints must hit min/max exactly, not via roundoff-accumulated arithmetic.
        let g = linspace(-1.234, 5.678, 1000);
        assert_eq!(g[0], -1.234);
        assert_eq!(g[999], 5.678);
    }

    #[test]
    fn linspace_negative_to_positive() {
        let g = linspace(-1.0, 1.0, 5);
        assert_eq!(g, vec![-1.0, -0.5, 0.0, 0.5, 1.0]);
    }

    // ── logspace ────────────────────────────────────────────────

    #[test]
    fn logspace_empty() {
        assert!(logspace(1.0, 100.0, 0).is_empty());
    }

    #[test]
    fn logspace_single() {
        assert_eq!(logspace(1.5, 100.0, 1), vec![1.5]);
    }

    #[test]
    fn logspace_decade_grid() {
        let g = logspace(1.0, 1000.0, 4);
        let expected = [1.0, 10.0, 100.0, 1000.0];
        for (a, b) in g.iter().zip(expected.iter()) {
            assert!((a - b).abs() < 1e-10, "{a} vs {b}");
        }
    }

    #[test]
    fn logspace_endpoints_exact() {
        let g = logspace(0.01, 1000.0, 256);
        assert_eq!(g[0], 0.01);
        assert_eq!(g[255], 1000.0);
    }

    #[test]
    fn logspace_uniform_in_log() {
        // Successive ratios must be constant in log-spaced grid.
        let g = logspace(0.5, 50.0, 8);
        let ratio = g[1] / g[0];
        for i in 1..g.len() - 1 {
            let r = g[i + 1] / g[i];
            assert!(
                (r - ratio).abs() < 1e-10,
                "ratio drift at i={i}: {r} vs {ratio}"
            );
        }
    }

    // ── linear_clamped ──────────────────────────────────────────

    #[test]
    #[should_panic(expected = "non-empty")]
    fn linear_clamped_empty_panics() {
        let xs: &[f64] = &[];
        let ys: &[f64] = &[];
        let _ = linear_clamped(xs, ys, 0.0);
    }

    #[test]
    fn linear_clamped_single() {
        assert_eq!(linear_clamped(&[2.0], &[42.0], 5.0), 42.0);
        assert_eq!(linear_clamped(&[2.0], &[42.0], -5.0), 42.0);
    }

    #[test]
    fn linear_clamped_clamps_left() {
        let xs = [0.0, 1.0, 2.0];
        let ys = [10.0, 20.0, 30.0];
        assert_eq!(linear_clamped(&xs, &ys, -1.0), 10.0);
        assert_eq!(linear_clamped(&xs, &ys, 0.0), 10.0);
    }

    #[test]
    fn linear_clamped_clamps_right() {
        let xs = [0.0, 1.0, 2.0];
        let ys = [10.0, 20.0, 30.0];
        assert_eq!(linear_clamped(&xs, &ys, 2.0), 30.0);
        assert_eq!(linear_clamped(&xs, &ys, 5.0), 30.0);
    }

    #[test]
    fn linear_clamped_midpoint() {
        let xs = [0.0, 1.0];
        let ys = [10.0, 20.0];
        assert_eq!(linear_clamped(&xs, &ys, 0.5), 15.0);
    }

    #[test]
    fn linear_clamped_three_point() {
        let xs = [0.0, 1.0, 3.0];
        let ys = [10.0, 20.0, 40.0];
        // Right segment: at xq=2.0, alpha = (2-1)/(3-1) = 0.5, y = 20·0.5 + 40·0.5 = 30
        assert_eq!(linear_clamped(&xs, &ys, 2.0), 30.0);
    }

    #[test]
    fn linear_clamped_at_knots() {
        let xs = [0.0, 1.0, 2.0, 3.0];
        let ys = [0.0, 1.0, 4.0, 9.0];
        for i in 0..xs.len() {
            assert_eq!(linear_clamped(&xs, &ys, xs[i]), ys[i]);
        }
    }

    #[test]
    fn linear_clamped_duplicate_x_picks_upper_segment() {
        // xs has a duplicate at x=1.0. `partition_point(|&x| x <= xq)` is the
        // first index where xs[k] > xq, so for xq = 1.0 it returns 3 (the
        // first index where xs > 1.0). The bracket is (i=2, i+1=3) on the
        // upper duplicate segment: dx = xs[3]-xs[2] = 1, alpha = 0, y = ys[2].
        let xs = [0.0, 1.0, 1.0, 2.0];
        let ys = [0.0, 10.0, 20.0, 30.0];
        assert_eq!(linear_clamped(&xs, &ys, 1.0), 20.0);
    }

    #[test]
    fn linear_clamped_collapsed_segment_interior_query() {
        // Query just past the duplicate: should interpolate cleanly on
        // the upper segment.
        let xs = [0.0, 1.0, 1.0, 2.0];
        let ys = [0.0, 10.0, 20.0, 30.0];
        // alpha = (1.5 - 1.0) / 1.0 = 0.5, y = 20·0.5 + 30·0.5 = 25.0
        assert_eq!(linear_clamped(&xs, &ys, 1.5), 25.0);
    }

    #[test]
    fn linear_clamped_negative_query() {
        let xs = [-2.0, -1.0, 0.0, 1.0];
        let ys = [-20.0, -10.0, 0.0, 10.0];
        assert_eq!(linear_clamped(&xs, &ys, -1.5), -15.0);
        assert_eq!(linear_clamped(&xs, &ys, 0.5), 5.0);
    }

    #[test]
    fn linear_clamped_user_type() {
        // T = Vec2 with manual Copy/Add/Mul<f64> impls.
        #[derive(Clone, Copy, PartialEq, Debug)]
        struct V2(f64, f64);
        impl std::ops::Add for V2 {
            type Output = V2;
            fn add(self, rhs: V2) -> V2 {
                V2(self.0 + rhs.0, self.1 + rhs.1)
            }
        }
        impl std::ops::Mul<f64> for V2 {
            type Output = V2;
            fn mul(self, s: f64) -> V2 {
                V2(self.0 * s, self.1 * s)
            }
        }

        let xs = [0.0, 1.0];
        let ys = [V2(10.0, 20.0), V2(20.0, 40.0)];
        let v = linear_clamped(&xs, &ys, 0.5);
        assert!((v.0 - 15.0).abs() < 1e-15);
        assert!((v.1 - 30.0).abs() < 1e-15);
    }

    #[test]
    fn linear_clamped_dense_table_monotone() {
        // Dense table: result must be monotone increasing in xq.
        let xs: Vec<f64> = (0..100).map(|i| i as f64 * 0.1).collect();
        let ys: Vec<f64> = xs.iter().map(|x| x * x).collect();
        let qs: Vec<f64> = (0..1000).map(|i| i as f64 * 0.01).collect();
        let mut last = f64::NEG_INFINITY;
        for q in &qs {
            let v = linear_clamped(&xs, &ys, *q);
            assert!(v >= last, "non-monotone at q={q}: {last} -> {v}");
            last = v;
        }
    }
}
