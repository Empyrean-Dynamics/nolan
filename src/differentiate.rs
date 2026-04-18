//! Generic `differentiate` convenience API.
//!
//! Hides Jet seeding and derivative extraction behind a small family of
//! functions. Pick the variant that matches the order of derivatives you
//! need and the shape of your function:
//!
//! | Variant | Signature | Returns |
//! |---------|-----------|---------|
//! | [`differentiate1`] | `f: R^N → R` | `(f, ∇f)` |
//! | [`differentiate2`] | `f: R^N → R` | `(f, ∇f, ∇²f)` |
//! | [`differentiate3`] | `f: R^N → R` | `(f, ∇f, ∇²f, ∇³f)` |
//! | [`differentiate1_vec`] | `f: R^N → R^M` | `(f, J)` |
//! | [`differentiate2_vec`] | `f: R^N → R^M` | `(f, J, H)` |
//! | [`differentiate3_vec`] | `f: R^N → R^M` | `(f, J, H, T)` |
//!
//! Gradients are returned as `[f64; N]` arrays. Hessians and third-order
//! tensors are returned as fully-populated dense arrays (all permutations
//! filled), so they can be consumed directly by linear-algebra code that
//! does not know about the symmetric lower-triangular storage.
//!
//! Because stable Rust does not yet support `generic_const_exprs`,
//! callers of the fully-generic [`differentiate2`] and [`differentiate3`]
//! must pass both `N` and `H` (and `T` for order 3). Specialized helpers
//! [`differentiate2_6`], [`differentiate2_9`], [`differentiate3_6`], and
//! [`differentiate3_9`] inline the hessian/tensor sizes for the common
//! 6-parameter and 9-parameter cases.

use crate::jets::{Jet1, Jet2, Jet3};
use crate::traits::{FirstOrder, SecondOrder, ThirdOrder};

/// Gradient \\( \nabla f \\): \\( [\partial f / \partial p_0, \ldots, \partial f / \partial p_{N-1}] \\).
pub type Gradient<const N: usize> = [f64; N];

/// Hessian \\( \nabla^2 f \\): dense \\( N \times N \\) matrix with
/// \\( H_{ij} = \partial^2 f / \partial p_i \partial p_j \\). Fully populated
/// (not just the lower triangle) so consumers can index symmetrically.
pub type Hessian<const N: usize> = [[f64; N]; N];

/// Third-order derivative tensor: dense \\( N \times N \times N \\) array with
/// \\( T_{ijk} = \partial^3 f / \partial p_i \partial p_j \partial p_k \\).
/// All 6 permutations are populated.
pub type ThirdOrderTensor<const N: usize> = [[[f64; N]; N]; N];

// ─── Scalar: f: R^N → R ──────────────────────────────────────────────────

/// Evaluate scalar-valued `f(x)` and return `(value, gradient)`.
///
/// # Example
/// ```
/// use nolan::differentiate::differentiate1;
/// // f(x, y) = x*y + x²
/// // ∂f/∂x = y + 2x, ∂f/∂y = x
/// let (value, grad) = differentiate1([1.5, 2.0], |[x, y]| x * y + x * x);
/// assert!((value - 5.25).abs() < 1e-12);
/// assert!((grad[0] - 5.0).abs() < 1e-12);
/// assert!((grad[1] - 1.5).abs() < 1e-12);
/// ```
pub fn differentiate1<const N: usize, F>(x: [f64; N], f: F) -> (f64, Gradient<N>)
where
    F: FnOnce([Jet1<N>; N]) -> Jet1<N>,
{
    let seeded = seed_jet1::<N>(x);
    let y = f(seeded);
    (y.value, y.extract_grad::<N>())
}

/// Evaluate scalar-valued `f(x)` and return `(value, gradient, hessian)`.
///
/// On stable Rust, `H` must equal `N*(N+1)/2`. Prefer [`differentiate2_6`]
/// or [`differentiate2_9`] for the common 6- and 9-parameter cases.
pub fn differentiate2<const N: usize, const H: usize, F>(
    x: [f64; N],
    f: F,
) -> (f64, Gradient<N>, Hessian<N>)
where
    F: FnOnce([Jet2<N, H>; N]) -> Jet2<N, H>,
{
    let seeded = seed_jet2::<N, H>(x);
    let y = f(seeded);
    (y.value, y.extract_grad::<N>(), y.extract_hess::<N>())
}

/// Evaluate scalar-valued `f(x)` and return
/// `(value, gradient, hessian, third-order-tensor)`.
///
/// On stable Rust, `H` must equal `N*(N+1)/2` and `T` must equal
/// `N*(N+1)*(N+2)/6`. Prefer [`differentiate3_6`] or [`differentiate3_9`]
/// for the common cases.
pub fn differentiate3<const N: usize, const H: usize, const T: usize, F>(
    x: [f64; N],
    f: F,
) -> (f64, Gradient<N>, Hessian<N>, ThirdOrderTensor<N>)
where
    F: FnOnce([Jet3<N, H, T>; N]) -> Jet3<N, H, T>,
{
    let seeded = seed_jet3::<N, H, T>(x);
    let y = f(seeded);
    (
        y.value,
        y.extract_grad::<N>(),
        y.extract_hess::<N>(),
        y.extract_tens::<N>(),
    )
}

// ─── Vector: f: R^N → R^M ────────────────────────────────────────────────

/// Evaluate vector-valued `f(x)` and return `(values, jacobian)`.
///
/// `jacobian[m][i]` = \\( \partial f_m / \partial p_i \\).
///
/// # Example
/// ```
/// use nolan::differentiate::differentiate1_vec;
/// // Spherical coordinates: f(x, y, z) = (r, phi)
/// let (values, jac) = differentiate1_vec([3.0, 4.0, 0.0], |[x, y, z]| {
///     let r = (x * x + y * y + z * z).sqrt();
///     let phi = y.atan2(x);
///     [r, phi]
/// });
/// assert!((values[0] - 5.0).abs() < 1e-12);      // r = 5
/// assert!((jac[0][0] - 3.0 / 5.0).abs() < 1e-12); // ∂r/∂x = x/r
/// ```
pub fn differentiate1_vec<const N: usize, const M: usize, F>(
    x: [f64; N],
    f: F,
) -> ([f64; M], [Gradient<N>; M])
where
    F: FnOnce([Jet1<N>; N]) -> [Jet1<N>; M],
{
    let seeded = seed_jet1::<N>(x);
    let ys = f(seeded);
    let values = std::array::from_fn(|m| ys[m].value);
    let jacobian = std::array::from_fn(|m| ys[m].extract_grad::<N>());
    (values, jacobian)
}

/// Evaluate vector-valued `f(x)` and return `(values, jacobian, hessians)`.
///
/// `hessians[m][i][j]` = \\( \partial^2 f_m / \partial p_i \partial p_j \\).
pub fn differentiate2_vec<const N: usize, const H: usize, const M: usize, F>(
    x: [f64; N],
    f: F,
) -> ([f64; M], [Gradient<N>; M], [Hessian<N>; M])
where
    F: FnOnce([Jet2<N, H>; N]) -> [Jet2<N, H>; M],
{
    let seeded = seed_jet2::<N, H>(x);
    let ys = f(seeded);
    let values = std::array::from_fn(|m| ys[m].value);
    let jacobian = std::array::from_fn(|m| ys[m].extract_grad::<N>());
    let hessians = std::array::from_fn(|m| ys[m].extract_hess::<N>());
    (values, jacobian, hessians)
}

/// Evaluate vector-valued `f(x)` and return
/// `(values, jacobian, hessians, third-order-tensors)`.
///
/// `tens[m][i][j][k]` = \\( \partial^3 f_m / \partial p_i \partial p_j \partial p_k \\).
pub fn differentiate3_vec<const N: usize, const H: usize, const T: usize, const M: usize, F>(
    x: [f64; N],
    f: F,
) -> (
    [f64; M],
    [Gradient<N>; M],
    [Hessian<N>; M],
    [ThirdOrderTensor<N>; M],
)
where
    F: FnOnce([Jet3<N, H, T>; N]) -> [Jet3<N, H, T>; M],
{
    let seeded = seed_jet3::<N, H, T>(x);
    let ys = f(seeded);
    let values = std::array::from_fn(|m| ys[m].value);
    let jacobian = std::array::from_fn(|m| ys[m].extract_grad::<N>());
    let hessians = std::array::from_fn(|m| ys[m].extract_hess::<N>());
    let tensors = std::array::from_fn(|m| ys[m].extract_tens::<N>());
    (values, jacobian, hessians, tensors)
}

// ─── Specialized N=6 / N=9 helpers ───────────────────────────────────────

/// [`differentiate2`] specialized for 6 parameters (H = 21).
pub fn differentiate2_6<F>(x: [f64; 6], f: F) -> (f64, Gradient<6>, Hessian<6>)
where
    F: FnOnce([Jet2<6, 21>; 6]) -> Jet2<6, 21>,
{
    differentiate2::<6, 21, _>(x, f)
}

/// [`differentiate2`] specialized for 9 parameters (H = 45).
pub fn differentiate2_9<F>(x: [f64; 9], f: F) -> (f64, Gradient<9>, Hessian<9>)
where
    F: FnOnce([Jet2<9, 45>; 9]) -> Jet2<9, 45>,
{
    differentiate2::<9, 45, _>(x, f)
}

/// [`differentiate3`] specialized for 6 parameters (H = 21, T = 56).
pub fn differentiate3_6<F>(x: [f64; 6], f: F) -> (f64, Gradient<6>, Hessian<6>, ThirdOrderTensor<6>)
where
    F: FnOnce([Jet3<6, 21, 56>; 6]) -> Jet3<6, 21, 56>,
{
    differentiate3::<6, 21, 56, _>(x, f)
}

/// [`differentiate3`] specialized for 9 parameters (H = 45, T = 165).
pub fn differentiate3_9<F>(x: [f64; 9], f: F) -> (f64, Gradient<9>, Hessian<9>, ThirdOrderTensor<9>)
where
    F: FnOnce([Jet3<9, 45, 165>; 9]) -> Jet3<9, 45, 165>,
{
    differentiate3::<9, 45, 165, _>(x, f)
}

// ─── Runtime-dispatched differentiate ────────────────────────────────────

use crate::traits::AutoDiff;

/// Selects which order of derivatives to compute at runtime.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Order {
    First,
    Second,
    Third,
}

/// Result of [`differentiate_dyn`] / [`differentiate_dyn_6`] / [`differentiate_dyn_9`].
///
/// Use the accessors ([`Self::values`], [`Self::jacobian`], [`Self::hessians`],
/// [`Self::tensors`]) for uniform consumption, or match on the variant when the
/// higher-order fields are needed unconditionally.
///
/// The Hessian and third-order-tensor fields are `Box`ed so the enum stays
/// small regardless of which variant is returned; this keeps the `First`
/// branch fast for cases where runtime dispatch is frequent.
#[derive(Clone, Debug)]
pub enum Derivatives<const N: usize, const M: usize> {
    First {
        values: [f64; M],
        jacobian: [Gradient<N>; M],
    },
    Second {
        values: [f64; M],
        jacobian: [Gradient<N>; M],
        hessians: Box<[Hessian<N>; M]>,
    },
    Third {
        values: [f64; M],
        jacobian: [Gradient<N>; M],
        hessians: Box<[Hessian<N>; M]>,
        tensors: Box<[ThirdOrderTensor<N>; M]>,
    },
}

impl<const N: usize, const M: usize> Derivatives<N, M> {
    pub fn order(&self) -> Order {
        match self {
            Derivatives::First { .. } => Order::First,
            Derivatives::Second { .. } => Order::Second,
            Derivatives::Third { .. } => Order::Third,
        }
    }

    pub fn values(&self) -> &[f64; M] {
        match self {
            Derivatives::First { values, .. }
            | Derivatives::Second { values, .. }
            | Derivatives::Third { values, .. } => values,
        }
    }

    pub fn jacobian(&self) -> &[Gradient<N>; M] {
        match self {
            Derivatives::First { jacobian, .. }
            | Derivatives::Second { jacobian, .. }
            | Derivatives::Third { jacobian, .. } => jacobian,
        }
    }

    /// Returns the Hessians if order is Second or Third, else None.
    pub fn hessians(&self) -> Option<&[Hessian<N>; M]> {
        match self {
            Derivatives::Second { hessians, .. } | Derivatives::Third { hessians, .. } => {
                Some(hessians.as_ref())
            }
            Derivatives::First { .. } => None,
        }
    }

    /// Returns the third-order tensors if order is Third, else None.
    pub fn tensors(&self) -> Option<&[ThirdOrderTensor<N>; M]> {
        match self {
            Derivatives::Third { tensors, .. } => Some(tensors.as_ref()),
            _ => None,
        }
    }
}

/// A function \\( f: \mathbb{R}^N \to \mathbb{R}^M \\) written generically
/// over any [`AutoDiff`] type.
///
/// Implementors write the function body once and it works with `Jet1`, `Jet2`,
/// or `Jet3` — the runtime dispatcher picks which one to seed based on the
/// requested [`Order`]. Typical impls are zero-sized structs:
///
/// ```
/// use nolan::differentiate::AutoDiffFn;
/// use nolan::traits::AutoDiff;
///
/// struct GravityAccel { mu: f64 }
/// impl AutoDiffFn<6, 3> for GravityAccel {
///     fn eval<T: AutoDiff>(&self, xs: [T; 6]) -> [T; 3] {
///         let [x, y, z, _vx, _vy, _vz] = xs;
///         let r2 = x * x + y * y + z * z;
///         let r = r2.sqrt();
///         let r3_inv = r.powi(-3) * self.mu;
///         [x * r3_inv, y * r3_inv, z * r3_inv]
///     }
/// }
/// ```
pub trait AutoDiffFn<const N: usize, const M: usize> {
    fn eval<T: AutoDiff>(&self, xs: [T; N]) -> [T; M];
}

/// Runtime-dispatched differentiate. Picks Jet1/Jet2/Jet3 based on `order`.
///
/// Prefer [`differentiate_dyn_6`] or [`differentiate_dyn_9`] for the common
/// state sizes; they inline the hessian and tensor widths so callers only
/// need to pass the output arity `M`.
pub fn differentiate_dyn<
    const N: usize,
    const H: usize,
    const T: usize,
    const M: usize,
    F: AutoDiffFn<N, M>,
>(
    order: Order,
    x: [f64; N],
    f: &F,
) -> Derivatives<N, M> {
    match order {
        Order::First => {
            let seeded = seed_jet1::<N>(x);
            let ys = f.eval(seeded);
            let values = std::array::from_fn(|m| ys[m].value);
            let jacobian = std::array::from_fn(|m| ys[m].extract_grad::<N>());
            Derivatives::First { values, jacobian }
        }
        Order::Second => {
            let seeded = seed_jet2::<N, H>(x);
            let ys = f.eval(seeded);
            let values = std::array::from_fn(|m| ys[m].value);
            let jacobian = std::array::from_fn(|m| ys[m].extract_grad::<N>());
            let hessians = Box::new(std::array::from_fn(|m| ys[m].extract_hess::<N>()));
            Derivatives::Second {
                values,
                jacobian,
                hessians,
            }
        }
        Order::Third => {
            let seeded = seed_jet3::<N, H, T>(x);
            let ys = f.eval(seeded);
            let values = std::array::from_fn(|m| ys[m].value);
            let jacobian = std::array::from_fn(|m| ys[m].extract_grad::<N>());
            let hessians = Box::new(std::array::from_fn(|m| ys[m].extract_hess::<N>()));
            let tensors = Box::new(std::array::from_fn(|m| ys[m].extract_tens::<N>()));
            Derivatives::Third {
                values,
                jacobian,
                hessians,
                tensors,
            }
        }
    }
}

/// [`differentiate_dyn`] specialized for 6 parameters (H = 21, T = 56).
pub fn differentiate_dyn_6<const M: usize, F: AutoDiffFn<6, M>>(
    order: Order,
    x: [f64; 6],
    f: &F,
) -> Derivatives<6, M> {
    differentiate_dyn::<6, 21, 56, M, F>(order, x, f)
}

/// [`differentiate_dyn`] specialized for 9 parameters (H = 45, T = 165).
pub fn differentiate_dyn_9<const M: usize, F: AutoDiffFn<9, M>>(
    order: Order,
    x: [f64; 9],
    f: &F,
) -> Derivatives<9, M> {
    differentiate_dyn::<9, 45, 165, M, F>(order, x, f)
}

// ─── Internal seeding helpers ────────────────────────────────────────────

#[inline]
fn seed_jet1<const N: usize>(x: [f64; N]) -> [Jet1<N>; N] {
    std::array::from_fn(|i| Jet1::<N>::variable(x[i], i))
}

#[inline]
fn seed_jet2<const N: usize, const H: usize>(x: [f64; N]) -> [Jet2<N, H>; N] {
    std::array::from_fn(|i| Jet2::<N, H>::variable(x[i], i))
}

#[inline]
fn seed_jet3<const N: usize, const H: usize, const T: usize>(x: [f64; N]) -> [Jet3<N, H, T>; N] {
    std::array::from_fn(|i| Jet3::<N, H, T>::variable(x[i], i))
}

// ─── Tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
#[allow(clippy::needless_range_loop)]
mod tests {
    use super::*;
    use crate::jets::{hess_size, tens_size};

    const TOL: f64 = 1e-12;

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() <= TOL * b.abs().max(1.0)
    }

    // ─── differentiate1 ───────────────────────────────────────────────

    #[test]
    fn differentiate1_matches_manual_seed() {
        // f(x, y) = sin(x * y). Compare to manual seeding.
        let a = 0.8_f64;
        let b = 1.1_f64;
        let (value, grad) = differentiate1([a, b], |[x, y]| (x * y).sin());

        let mx = Jet1::<2>::variable(a, 0);
        let my = Jet1::<2>::variable(b, 1);
        let manual = (mx * my).sin();
        assert!(close(value, manual.value));
        assert!(close(grad[0], manual.extract_grad::<2>()[0]));
        assert!(close(grad[1], manual.extract_grad::<2>()[1]));

        // And against closed form.
        let c = (a * b).cos();
        assert!(close(value, (a * b).sin()));
        assert!(close(grad[0], c * b));
        assert!(close(grad[1], c * a));
    }

    // ─── differentiate2 ───────────────────────────────────────────────

    #[test]
    fn differentiate2_matches_closed_form() {
        // f(x, y) = x² y
        // ∂/∂x = 2xy, ∂/∂y = x²
        // ∂²/∂x² = 2y, ∂²/∂x∂y = 2x, ∂²/∂y² = 0
        let a = 1.3_f64;
        let b = 0.7_f64;
        let (value, grad, hess) =
            differentiate2::<2, { hess_size(2) }, _>([a, b], |[x, y]| x * x * y);
        assert!(close(value, a * a * b));
        assert!(close(grad[0], 2.0 * a * b));
        assert!(close(grad[1], a * a));
        assert!(close(hess[0][0], 2.0 * b));
        assert!(close(hess[0][1], 2.0 * a));
        assert!(close(hess[1][0], 2.0 * a));
        assert!(close(hess[1][1], 0.0));
    }

    #[test]
    fn differentiate2_6_works() {
        // f(x0..x5) = x0 * x1 + x2 * x3 + x4 * x5
        let x = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let (value, grad, hess) = differentiate2_6(x, |[a, b, c, d, e, f]| a * b + c * d + e * f);
        assert!(close(value, 1.0 * 2.0 + 3.0 * 4.0 + 5.0 * 6.0));
        // grad = [x1, x0, x3, x2, x5, x4]
        assert!(close(grad[0], 2.0));
        assert!(close(grad[1], 1.0));
        assert!(close(grad[2], 4.0));
        assert!(close(grad[3], 3.0));
        assert!(close(grad[4], 6.0));
        assert!(close(grad[5], 5.0));
        // hess: only (0,1), (1,0), (2,3), (3,2), (4,5), (5,4) are 1; rest 0
        assert!(close(hess[0][1], 1.0));
        assert!(close(hess[1][0], 1.0));
        assert!(close(hess[2][3], 1.0));
        assert!(close(hess[3][2], 1.0));
        assert!(close(hess[4][5], 1.0));
        assert!(close(hess[5][4], 1.0));
        assert!(close(hess[0][0], 0.0));
        assert!(close(hess[2][5], 0.0));
    }

    // ─── differentiate3 ───────────────────────────────────────────────

    #[test]
    fn differentiate3_matches_closed_form() {
        // f(x) = x³. ∂³/∂x³ = 6.
        let (value, grad, hess, tens) =
            differentiate3::<1, { hess_size(1) }, { tens_size(1) }, _>([2.5], |[x]| x * x * x);
        assert!(close(value, 2.5f64.powi(3)));
        assert!(close(grad[0], 3.0 * 2.5 * 2.5));
        assert!(close(hess[0][0], 6.0 * 2.5));
        assert!(close(tens[0][0][0], 6.0));
    }

    #[test]
    fn differentiate3_6_gaussian() {
        // f(x0..x5) = exp(-(x0² + x1² + ... + x5²) / 2). A 6-D Gaussian.
        // At origin, all first/third derivatives are zero; hessian is -I.
        let x = [0.0; 6];
        let (value, grad, hess, tens) = differentiate3_6(x, |xs| {
            let mut r2 = Jet3::<6, 21, 56>::constant(0.0);
            for xi in xs {
                r2 += xi * xi;
            }
            (-r2 * 0.5).exp()
        });
        assert!(close(value, 1.0));
        for g in &grad {
            assert!(close(*g, 0.0));
        }
        for i in 0..6 {
            for j in 0..6 {
                let expected = if i == j { -1.0 } else { 0.0 };
                assert!(close(hess[i][j], expected));
            }
        }
        // Third derivatives of exp(-r²/2) at origin are zero (odd moments).
        for i in 0..6 {
            for j in 0..6 {
                for k in 0..6 {
                    assert!(close(tens[i][j][k], 0.0));
                }
            }
        }
    }

    // ─── differentiate1_vec ───────────────────────────────────────────

    #[test]
    fn differentiate1_vec_spherical() {
        // f(x, y, z) = (r, theta) where r = sqrt(x²+y²+z²), theta = atan2(z, sqrt(x²+y²))
        let x = 3.0_f64;
        let y = 4.0_f64;
        let z = 0.0_f64;
        let (values, jac) = differentiate1_vec::<3, 2, _>([x, y, z], |[x, y, z]| {
            let r = (x * x + y * y + z * z).sqrt();
            let rho = (x * x + y * y).sqrt();
            let theta = z.atan2(rho);
            [r, theta]
        });
        // r = 5, theta = 0 (z = 0)
        assert!(close(values[0], 5.0));
        assert!(close(values[1], 0.0));
        // ∂r/∂x = x/r = 3/5, ∂r/∂y = 4/5, ∂r/∂z = 0
        assert!(close(jac[0][0], 3.0 / 5.0));
        assert!(close(jac[0][1], 4.0 / 5.0));
        assert!(close(jac[0][2], 0.0));
    }

    // ─── differentiate2_vec ───────────────────────────────────────────

    #[test]
    fn differentiate2_vec_shape_matches_manual() {
        // f(x, y) = (x², xy) at (2, 3). Verify hessian shape and symmetry.
        let (values, jac, hess) =
            differentiate2_vec::<2, { hess_size(2) }, 2, _>([2.0, 3.0], |[x, y]| [x * x, x * y]);
        assert!(close(values[0], 4.0));
        assert!(close(values[1], 6.0));
        // Jacobian:
        //   ∂(x²)/∂x = 2x = 4, ∂(x²)/∂y = 0
        //   ∂(xy)/∂x = y = 3, ∂(xy)/∂y = x = 2
        assert!(close(jac[0][0], 4.0));
        assert!(close(jac[0][1], 0.0));
        assert!(close(jac[1][0], 3.0));
        assert!(close(jac[1][1], 2.0));
        // Hessians:
        //   f₀=x²: ∂²/∂x² = 2, all others 0
        //   f₁=xy: ∂²/∂x∂y = 1, diagonal 0
        assert!(close(hess[0][0][0], 2.0));
        assert!(close(hess[0][0][1], 0.0));
        assert!(close(hess[0][1][1], 0.0));
        assert!(close(hess[1][0][0], 0.0));
        assert!(close(hess[1][0][1], 1.0));
        assert!(close(hess[1][1][0], 1.0));
        assert!(close(hess[1][1][1], 0.0));
    }

    // ─── differentiate3_vec ───────────────────────────────────────────

    #[test]
    fn differentiate3_vec_symmetric_third_tensor() {
        // f(x, y) = (x³, y²·x). Verify third-order tensor entries.
        // f₀ = x³:  ∂³/∂x³ = 6, else 0
        // f₁ = y²x: ∂³/∂x∂y² = 2, ∂³/∂y²∂x = 2 (symmetry), else 0
        let (values, _jac, _hess, tens) =
            differentiate3_vec::<2, { hess_size(2) }, { tens_size(2) }, 2, _>(
                [1.5, 1.0],
                |[x, y]| [x * x * x, y * y * x],
            );
        assert!(close(values[0], 1.5f64.powi(3)));
        assert!(close(values[1], 1.0 * 1.5));
        assert!(close(tens[0][0][0][0], 6.0));
        assert!(close(tens[0][0][0][1], 0.0));
        // f₁: ∂³(y²x)/∂x∂y∂y = 2 — symmetric across all permutations
        assert!(close(tens[1][0][1][1], 2.0));
        assert!(close(tens[1][1][0][1], 2.0));
        assert!(close(tens[1][1][1][0], 2.0));
        assert!(close(tens[1][0][0][0], 0.0));
        assert!(close(tens[1][1][1][1], 0.0));
    }

    // ─── Parity: specialized vs generic ───────────────────────────────

    #[test]
    fn differentiate3_6_matches_generic() {
        // Use the same f twice — through generic and specialized helpers.
        // They must agree bit-for-bit.
        let x = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];

        let (v_g, g_g, h_g, t_g) =
            differentiate3::<6, { hess_size(6) }, { tens_size(6) }, _>(x, |[a, b, c, d, e, f]| {
                a * b * c + d * e * f
            });
        let (v_s, g_s, h_s, t_s) = differentiate3_6(x, |[a, b, c, d, e, f]| a * b * c + d * e * f);

        assert_eq!(v_g, v_s);
        for i in 0..6 {
            assert_eq!(g_g[i], g_s[i]);
        }
        for i in 0..6 {
            for j in 0..6 {
                assert_eq!(h_g[i][j], h_s[i][j]);
                for k in 0..6 {
                    assert_eq!(t_g[i][j][k], t_s[i][j][k]);
                }
            }
        }
    }

    #[test]
    fn differentiate2_9_matches_generic() {
        // f(x0..x8) = Σ i·xᵢ². Hessian is diagonal with 2i on the diagonal.
        let x = [0.5, 0.4, 0.3, 0.2, 0.1, 0.0, -0.1, -0.2, -0.3];

        let (v_g, g_g, h_g) = differentiate2::<9, { hess_size(9) }, _>(x, |xs| {
            let mut acc = Jet2::<9, { hess_size(9) }>::constant(0.0);
            for (i, xi) in xs.into_iter().enumerate() {
                acc += xi * xi * (i as f64);
            }
            acc
        });
        let (v_s, g_s, h_s) = differentiate2_9(x, |xs| {
            let mut acc = Jet2::<9, 45>::constant(0.0);
            for (i, xi) in xs.into_iter().enumerate() {
                acc += xi * xi * (i as f64);
            }
            acc
        });
        assert_eq!(v_g, v_s);
        for i in 0..9 {
            assert_eq!(g_g[i], g_s[i]);
            for j in 0..9 {
                assert_eq!(h_g[i][j], h_s[i][j]);
            }
        }
        // Spot-check a closed form: ∂²/∂x₃² = 2·3 = 6.
        assert!(close(h_s[3][3], 6.0));
        assert!(close(h_s[3][4], 0.0));
    }

    #[test]
    fn differentiate3_9_gives_nonzero_tens() {
        // f(x0..x8) = x0·x1·x2 + x3·x4·x5 + x6·x7·x8.
        // All pure trilinear terms: ∂³f/∂x_a ∂x_b ∂x_c = 1 for (a,b,c) in
        // {(0,1,2),(3,4,5),(6,7,8)} (and permutations), else 0.
        let x = [1.0; 9];
        let (value, _grad, _hess, tens) = differentiate3_9(x, |[a, b, c, d, e, f, g, h, i]| {
            a * b * c + d * e * f + g * h * i
        });
        assert!(close(value, 3.0));
        // Only these index triples (after canonicalizing) should be 1.
        assert!(close(tens[0][1][2], 1.0));
        assert!(close(tens[2][1][0], 1.0)); // symmetry
        assert!(close(tens[3][4][5], 1.0));
        assert!(close(tens[6][7][8], 1.0));
        // Something that should be zero:
        assert!(close(tens[0][0][0], 0.0));
        assert!(close(tens[0][3][6], 0.0));
    }

    // ─── Runtime dispatch ─────────────────────────────────────────────

    struct GravityAccel {
        mu: f64,
    }
    impl AutoDiffFn<6, 3> for GravityAccel {
        fn eval<T: AutoDiff>(&self, xs: [T; 6]) -> [T; 3] {
            let [x, y, z, _vx, _vy, _vz] = xs;
            let r2 = x * x + y * y + z * z;
            let r = r2.sqrt();
            let r3_inv = r.powi(-3) * self.mu;
            [x * r3_inv, y * r3_inv, z * r3_inv]
        }
    }

    #[test]
    fn differentiate_dyn_first_matches_flat_api() {
        let state = [1.0_f64, 0.5, 0.1, 0.0, 0.0, 0.0];
        let f = GravityAccel { mu: 1.327e11 };

        let d = differentiate_dyn_6(Order::First, state, &f);
        assert_eq!(d.order(), Order::First);
        assert!(d.hessians().is_none());
        assert!(d.tensors().is_none());

        let (flat_values, flat_jac) =
            differentiate1_vec::<6, 3, _>(state, |xs| f.eval::<Jet1<6>>(xs));

        for m in 0..3 {
            assert_eq!(d.values()[m], flat_values[m]);
            for i in 0..6 {
                assert_eq!(d.jacobian()[m][i], flat_jac[m][i]);
            }
        }
    }

    #[test]
    fn differentiate_dyn_second_matches_flat_api() {
        let state = [1.0_f64, 0.5, 0.1, 0.0, 0.0, 0.0];
        let f = GravityAccel { mu: 1.327e11 };

        let d = differentiate_dyn_6(Order::Second, state, &f);
        assert_eq!(d.order(), Order::Second);
        assert!(d.hessians().is_some());
        assert!(d.tensors().is_none());

        let (flat_values, flat_jac, flat_hess) =
            differentiate2_vec::<6, 21, 3, _>(state, |xs| f.eval::<Jet2<6, 21>>(xs));

        for m in 0..3 {
            assert_eq!(d.values()[m], flat_values[m]);
            for i in 0..6 {
                assert_eq!(d.jacobian()[m][i], flat_jac[m][i]);
                for j in 0..6 {
                    assert_eq!(d.hessians().unwrap()[m][i][j], flat_hess[m][i][j]);
                }
            }
        }
    }

    #[test]
    fn differentiate_dyn_third_matches_flat_api() {
        let state = [1.0_f64, 0.5, 0.1, 0.0, 0.0, 0.0];
        let f = GravityAccel { mu: 1.327e11 };

        let d = differentiate_dyn_6(Order::Third, state, &f);
        assert_eq!(d.order(), Order::Third);
        assert!(d.hessians().is_some());
        assert!(d.tensors().is_some());

        let (flat_values, flat_jac, flat_hess, flat_tens) =
            differentiate3_vec::<6, 21, 56, 3, _>(state, |xs| f.eval::<Jet3<6, 21, 56>>(xs));

        for m in 0..3 {
            assert_eq!(d.values()[m], flat_values[m]);
            for i in 0..6 {
                assert_eq!(d.jacobian()[m][i], flat_jac[m][i]);
                for j in 0..6 {
                    assert_eq!(d.hessians().unwrap()[m][i][j], flat_hess[m][i][j]);
                    for k in 0..6 {
                        assert_eq!(d.tensors().unwrap()[m][i][j][k], flat_tens[m][i][j][k]);
                    }
                }
            }
        }
    }

    #[test]
    fn differentiate_dyn_escalation_scenario() {
        // Illustrates the use case: decide order at runtime based on a
        // "nonlinearity diagnostic" (here we fake one with a threshold).
        // Default is Second; if the diagnostic trips, we escalate to Third.
        let state = [1.0_f64, 0.5, 0.1, 0.0, 0.0, 0.0];
        let f = GravityAccel { mu: 1.327e11 };

        let nonlinearity_proxy = 0.42_f64; // pretend we computed this upstream

        let order = if nonlinearity_proxy > 0.3 {
            Order::Third
        } else {
            Order::Second
        };

        let d = differentiate_dyn_6(order, state, &f);
        // We can consume uniformly — accessors work regardless of dispatch.
        assert_eq!(d.values().len(), 3);
        assert_eq!(d.jacobian().len(), 3);
        // And we know we got tensors because the diagnostic tripped.
        assert_eq!(d.order(), Order::Third);
        let tens = d.tensors().expect("third-order dispatch yields tensors");
        // Sanity: tensor is populated with real numbers.
        assert!(
            tens[0]
                .iter()
                .flatten()
                .any(|row| row.iter().any(|v| *v != 0.0))
        );
    }

    #[test]
    fn differentiate_dyn_9_works() {
        struct SumSquares;
        impl AutoDiffFn<9, 1> for SumSquares {
            fn eval<T: AutoDiff>(&self, xs: [T; 9]) -> [T; 1] {
                let mut acc = T::constant(0.0);
                for xi in xs {
                    acc += xi * xi;
                }
                [acc]
            }
        }
        let state = [1.0_f64; 9];
        let f = SumSquares;

        let d1 = differentiate_dyn_9(Order::First, state, &f);
        assert_eq!(d1.order(), Order::First);
        assert!(close(d1.values()[0], 9.0));
        for i in 0..9 {
            assert!(close(d1.jacobian()[0][i], 2.0));
        }

        let d2 = differentiate_dyn_9(Order::Second, state, &f);
        assert_eq!(d2.order(), Order::Second);
        assert!(close(d2.hessians().unwrap()[0][0][0], 2.0));
        assert!(close(d2.hessians().unwrap()[0][0][1], 0.0));

        // Third-order for a quadratic: all tens entries are zero.
        let d3 = differentiate_dyn_9(Order::Third, state, &f);
        for i in 0..9 {
            for j in 0..9 {
                for k in 0..9 {
                    assert!(close(d3.tensors().unwrap()[0][i][j][k], 0.0));
                }
            }
        }
    }
}
