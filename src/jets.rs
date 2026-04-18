/// Get the linear index of a Hessian matrix stored in lower triangular form.
#[inline(always)]
pub fn hess_index(i: usize, j: usize) -> Option<usize> {
    if i < j {
        hess_index(j, i)
    } else {
        Some(i * (i + 1) / 2 + j)
    }
}

/// Lower-triangular index assuming canonical ordering (i >= j).
#[inline(always)]
pub fn hess_idx(i: usize, j: usize) -> usize {
    i * (i + 1) / 2 + j
}

/// Calculate the size of the flattened hessian matrix given
/// n parameters.
#[inline(always)]
pub const fn hess_size(n: usize) -> usize {
    n * (n + 1) / 2
}

/// First-order jet: value + gradient. No hessian storage.
///
/// For \\( N = 6 \\): 56 bytes (1 `f64` value + 6 `f64` gradient).
#[derive(Clone, Copy)]
pub struct Jet1<const N: usize> {
    /// Function value \\( f \\).
    pub value: f64,
    /// First derivatives \\( \partial f / \partial p_i \\).
    pub grad: [f64; N],
}

/// Second-order jet: value + gradient + hessian.
///
/// `H` must equal [`hess_size(N)`] = \\( N (N+1) / 2 \\).
/// Use `Jet2::<N, { hess_size(N) }>::variable(...)` for correctly-sized instances.
///
/// For \\( N = 6, H = 21 \\): 224 bytes (1 + 6 + 21 `f64`s).
#[derive(Clone, Copy)]
pub struct Jet2<const N: usize, const H: usize> {
    /// Function value \\( f \\).
    pub value: f64,
    /// First derivatives \\( \partial f / \partial p_i \\).
    pub grad: [f64; N],
    /// Second derivatives \\( \partial^2 f / \partial p_i \partial p_j \\),
    /// stored in lower-triangular form.
    pub hess: [f64; H],
}

impl<const N: usize> Jet1<N> {
    /// Constant (all derivatives zero).
    pub fn constant(value: f64) -> Self {
        Self {
            value,
            grad: [0.0; N],
        }
    }

    /// Variable seeded at `param_idx` (unit gradient there).
    pub fn variable(value: f64, param_idx: usize) -> Self {
        let mut jet = Self::constant(value);
        jet.grad[param_idx] = 1.0;
        jet
    }
}

impl<const N: usize, const H: usize> Jet2<N, H> {
    /// Constant (all derivatives zero).
    pub fn constant(value: f64) -> Self {
        Self {
            value,
            grad: [0.0; N],
            hess: [0.0; H],
        }
    }

    /// Variable seeded at `param_idx` (unit gradient there).
    pub fn variable(value: f64, param_idx: usize) -> Self {
        let mut jet = Self::constant(value);
        jet.grad[param_idx] = 1.0;
        jet
    }
}

/// Get the linear index of a third-order tensor stored in lower-triangular form.
///
/// Canonical ordering: `i >= j >= k`. All permutations map to the same index.
#[inline(always)]
pub fn tens_index(i: usize, j: usize, k: usize) -> Option<usize> {
    let (mut a, mut b, mut c) = (i, j, k);
    if a < b {
        std::mem::swap(&mut a, &mut b);
    }
    if b < c {
        std::mem::swap(&mut b, &mut c);
    }
    if a < b {
        std::mem::swap(&mut a, &mut b);
    }
    Some(a * (a + 1) * (a + 2) / 6 + b * (b + 1) / 2 + c)
}

/// Third-order tensor index assuming canonical ordering (i >= j >= k).
#[inline(always)]
pub fn tens_idx(i: usize, j: usize, k: usize) -> usize {
    i * (i + 1) * (i + 2) / 6 + j * (j + 1) / 2 + k
}

/// Calculate the size of the flattened third-order tensor given
/// n parameters: \\( n(n+1)(n+2)/6 \\).
#[inline(always)]
pub const fn tens_size(n: usize) -> usize {
    n * (n + 1) * (n + 2) / 6
}

/// Third-order jet: value + gradient + hessian + third-order tensor.
///
/// `H` must equal [`hess_size(N)`] = \\( N (N+1) / 2 \\).
/// `T` must equal [`tens_size(N)`] = \\( N (N+1)(N+2) / 6 \\).
///
/// For \\( N = 6, H = 21, T = 56 \\): 672 bytes (1 + 6 + 21 + 56 `f64`s).
#[derive(Clone, Copy)]
pub struct Jet3<const N: usize, const H: usize, const T: usize> {
    /// Function value \\( f \\).
    pub value: f64,
    /// First derivatives \\( \partial f / \partial p_i \\).
    pub grad: [f64; N],
    /// Second derivatives \\( \partial^2 f / \partial p_i \partial p_j \\),
    /// stored in lower-triangular form.
    pub hess: [f64; H],
    /// Third derivatives \\( \partial^3 f / \partial p_i \partial p_j \partial p_k \\),
    /// stored in lower-triangular form (i >= j >= k).
    pub tens: [f64; T],
}

impl<const N: usize, const H: usize, const T: usize> Jet3<N, H, T> {
    /// Constant (all derivatives zero).
    pub fn constant(value: f64) -> Self {
        Self {
            value,
            grad: [0.0; N],
            hess: [0.0; H],
            tens: [0.0; T],
        }
    }

    /// Variable seeded at `param_idx` (unit gradient there).
    pub fn variable(value: f64, param_idx: usize) -> Self {
        let mut jet = Self::constant(value);
        jet.grad[param_idx] = 1.0;
        jet
    }
}

pub type Dual = Jet1<1>;
pub type HyperDual = Jet2<2, { hess_size(2) }>;
pub type HyperHyperDual = Jet3<2, { hess_size(2) }, { tens_size(2) }>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hess_index() {
        assert_eq!(hess_index(0, 0).unwrap(), 0);
        assert_eq!(hess_index(1, 0).unwrap(), 1);
        assert_eq!(hess_index(1, 1).unwrap(), 2);
        assert_eq!(hess_index(2, 0).unwrap(), 3);
        assert_eq!(hess_index(2, 1).unwrap(), 4);
        assert_eq!(hess_index(2, 2).unwrap(), 5);

        for i in 0..5 {
            for j in 0..5 {
                assert_eq!(hess_index(i, j), hess_index(j, i))
            }
        }
    }

    #[test]
    fn test_hess_size() {
        assert_eq!(hess_size(1), 1);
        assert_eq!(hess_size(2), 3);
        assert_eq!(hess_size(3), 6);
        assert_eq!(hess_size(4), 10);
    }

    #[test]
    fn test_tens_index() {
        // (0,0,0) -> 0
        assert_eq!(tens_index(0, 0, 0).unwrap(), 0);
        // (1,0,0) -> 1
        assert_eq!(tens_index(1, 0, 0).unwrap(), 1);
        // (1,1,0) -> 2
        assert_eq!(tens_index(1, 1, 0).unwrap(), 2);
        // (1,1,1) -> 3
        assert_eq!(tens_index(1, 1, 1).unwrap(), 3);
        // (2,0,0) -> 4
        assert_eq!(tens_index(2, 0, 0).unwrap(), 4);
        // (2,1,0) -> 5
        assert_eq!(tens_index(2, 1, 0).unwrap(), 5);
        // (2,1,1) -> 6
        assert_eq!(tens_index(2, 1, 1).unwrap(), 6);
        // (2,2,0) -> 7
        assert_eq!(tens_index(2, 2, 0).unwrap(), 7);
        // (2,2,1) -> 8
        assert_eq!(tens_index(2, 2, 1).unwrap(), 8);
        // (2,2,2) -> 9
        assert_eq!(tens_index(2, 2, 2).unwrap(), 9);

        // All permutations map to the same index
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    assert_eq!(tens_index(i, j, k), tens_index(i, k, j));
                    assert_eq!(tens_index(i, j, k), tens_index(j, i, k));
                    assert_eq!(tens_index(i, j, k), tens_index(j, k, i));
                    assert_eq!(tens_index(i, j, k), tens_index(k, i, j));
                    assert_eq!(tens_index(i, j, k), tens_index(k, j, i));
                }
            }
        }
    }

    #[test]
    fn test_tens_size() {
        assert_eq!(tens_size(1), 1);
        assert_eq!(tens_size(2), 4);
        assert_eq!(tens_size(3), 10);
        assert_eq!(tens_size(6), 56);
        assert_eq!(tens_size(9), 165);
    }

    // ─── Jet3 composition test ──────────────────────────────────────────────
    // End-to-end: multiple variables, nonlinear ops, addition of subexpressions,
    // full N=3 tensor (10 elements), plus permutation symmetry of tens(i,j,k).

    use crate::traits::ThirdOrder;

    const COMP_TOL: f64 = 1e-10;
    fn approx(actual: f64, expected: f64, what: &str) {
        let tol = COMP_TOL * expected.abs().max(1.0);
        assert!(
            (actual - expected).abs() <= tol,
            "{}: expected {}, got {} (|diff| = {:e})",
            what,
            expected,
            actual,
            (actual - expected).abs()
        );
    }

    #[test]
    fn jet3_composition_sin_xy_plus_exp_z_y2() {
        // f(x, y, z) = sin(x·y) + exp(z) · y²
        //
        // Closed form (with s = sin(xy), c = cos(xy), v = exp(z)):
        //   ∂/∂x = c·y
        //   ∂/∂y = c·x + 2vy
        //   ∂/∂z = v·y²
        //
        //   ∂²/∂x²    = -s·y²
        //   ∂²/∂x∂y  = c − s·xy
        //   ∂²/∂x∂z  = 0
        //   ∂²/∂y²    = -s·x² + 2v
        //   ∂²/∂y∂z  = 2v·y
        //   ∂²/∂z²    = v·y²
        //
        //   ∂³/∂x³      = -c·y³
        //   ∂³/∂y∂x²   = -c·x·y² - 2s·y
        //   ∂³/∂y²∂x   = -2s·x - c·x²·y
        //   ∂³/∂y³      = -c·x³
        //   ∂³/∂z∂x²   = 0
        //   ∂³/∂z∂y∂x  = 0
        //   ∂³/∂z∂y²   = 2v
        //   ∂³/∂z²∂x   = 0
        //   ∂³/∂z²∂y   = 2v·y
        //   ∂³/∂z³      = v·y²
        type J3 = Jet3<3, { hess_size(3) }, { tens_size(3) }>;
        let a: f64 = 0.9;
        let b: f64 = 1.3;
        let d: f64 = 0.4;
        let x = J3::variable(a, 0);
        let y = J3::variable(b, 1);
        let z = J3::variable(d, 2);
        let f = (x * y).sin() + z.exp() * (y * y);

        let s = (a * b).sin();
        let c = (a * b).cos();
        let v = d.exp();

        approx(f.value, s + v * b * b, "value");
        approx(f.grad[0], c * b, "∂/∂x");
        approx(f.grad[1], c * a + 2.0 * v * b, "∂/∂y");
        approx(f.grad[2], v * b * b, "∂/∂z");

        // hess_idx: (0,0)=0, (1,0)=1, (1,1)=2, (2,0)=3, (2,1)=4, (2,2)=5
        approx(f.hess[0], -s * b * b, "∂²/∂x²");
        approx(f.hess[1], c - s * a * b, "∂²/∂x∂y");
        approx(f.hess[2], -s * a * a + 2.0 * v, "∂²/∂y²");
        approx(f.hess[3], 0.0, "∂²/∂x∂z");
        approx(f.hess[4], 2.0 * v * b, "∂²/∂y∂z");
        approx(f.hess[5], v * b * b, "∂²/∂z²");

        // tens_idx canonical:
        // (0,0,0)=0, (1,0,0)=1, (1,1,0)=2, (1,1,1)=3,
        // (2,0,0)=4, (2,1,0)=5, (2,1,1)=6, (2,2,0)=7, (2,2,1)=8, (2,2,2)=9
        approx(f.tens[0], -c * b.powi(3), "∂³/∂x³");
        approx(f.tens[1], -c * a * b * b - 2.0 * s * b, "∂³/∂y∂x²");
        approx(f.tens[2], -2.0 * s * a - c * a * a * b, "∂³/∂y²∂x");
        approx(f.tens[3], -c * a.powi(3), "∂³/∂y³");
        approx(f.tens[4], 0.0, "∂³/∂z∂x²");
        approx(f.tens[5], 0.0, "∂³/∂z∂y∂x");
        approx(f.tens[6], 2.0 * v, "∂³/∂z∂y²");
        approx(f.tens[7], 0.0, "∂³/∂z²∂x");
        approx(f.tens[8], 2.0 * v * b, "∂³/∂z²∂y");
        approx(f.tens[9], v * b * b, "∂³/∂z³");
    }

    #[test]
    fn jet3_tens_accessor_is_fully_symmetric() {
        // For any composition, ThirdOrder::tens(i, j, k) must return the same value
        // for all 6 permutations of (i, j, k).
        type J3 = Jet3<3, { hess_size(3) }, { tens_size(3) }>;
        let x = J3::variable(0.9, 0);
        let y = J3::variable(1.3, 1);
        let z = J3::variable(0.4, 2);
        let f = (x * y).sin() + z.exp() * (y * y);

        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    let v_ijk = f.tens(i, j, k);
                    assert!(
                        (v_ijk - f.tens(i, k, j)).abs() < 1e-14,
                        "tens({i},{j},{k}) != tens({i},{k},{j})"
                    );
                    assert!(
                        (v_ijk - f.tens(j, i, k)).abs() < 1e-14,
                        "tens({i},{j},{k}) != tens({j},{i},{k})"
                    );
                    assert!(
                        (v_ijk - f.tens(j, k, i)).abs() < 1e-14,
                        "tens({i},{j},{k}) != tens({j},{k},{i})"
                    );
                    assert!(
                        (v_ijk - f.tens(k, i, j)).abs() < 1e-14,
                        "tens({i},{j},{k}) != tens({k},{i},{j})"
                    );
                    assert!(
                        (v_ijk - f.tens(k, j, i)).abs() < 1e-14,
                        "tens({i},{j},{k}) != tens({k},{j},{i})"
                    );
                }
            }
        }
    }
}
