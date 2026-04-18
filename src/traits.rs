use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait Differentiable:
    Copy
    + Clone
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
    + Add<f64, Output = Self>
    + Sub<f64, Output = Self>
    + Mul<f64, Output = Self>
    + Div<f64, Output = Self>
    + AddAssign
    + SubAssign
    + AddAssign<f64>
    + SubAssign<f64>
    + MulAssign<f64>
    + DivAssign<f64>
{
    /// Function value.
    fn value(&self) -> f64;
    /// Initialize a constant differentiable (derivatives are all zero).
    fn constant(value: f64) -> Self;
    /// Initialize a variable with unit gradient at `param_idx`.
    ///
    /// For scalar types (f64), returns the value and ignores the index.
    /// For Jet types, seeds the gradient vector with 1.0 at `param_idx`.
    fn variable(value: f64, param_idx: usize) -> Self;
}

pub trait FirstOrder: Differentiable {
    /// \\( \frac{\partial f}{\partial p_i} \\): derivative with respect to parameter \\( p_i \\) (parameter stored at index i).
    fn grad(&self, i: usize) -> f64;
    /// \\( \frac{\partial f}{\partial p_i} \\): Mutable derivative with respect to parameter \\( p_i \\) (parameter stored at index i).
    fn grad_mut(&mut self, i: usize) -> &mut f64;

    /// Extract the full gradient as an \\( M \\)-element array.
    ///
    /// Returns \\( \left[ \frac{\partial f}{\partial p_0}, \ldots, \frac{\partial f}{\partial p_{M-1}} \right] \\).
    fn extract_grad<const M: usize>(&self) -> [f64; M] {
        std::array::from_fn(|i| self.grad(i))
    }
}

pub trait SecondOrder: FirstOrder {
    /// \\( \frac{\partial^2 f}{\partial p_i \partial p_j} \\): Second derivative with respect to parameters \\( p_i \\) and \\( p_j \\) (parameters stored at indices i & j).
    fn hess(&self, i: usize, j: usize) -> f64;
    /// \\( \frac{\partial^2 f}{\partial p_i \partial p_j} \\): Mutable second derivative with respect to parameters \\( p_i \\) and \\( p_j \\) (parameters stored at indices i & j).
    fn hess_mut(&mut self, i: usize, j: usize) -> &mut f64;

    /// Extract the full Hessian as a symmetric \\( M \times M \\) array.
    ///
    /// Returns \\( H_{ij} = \frac{\partial^2 f}{\partial p_i \partial p_j} \\)
    /// with the lower triangle mirrored to the upper triangle.
    fn extract_hess<const M: usize>(&self) -> [[f64; M]; M] {
        let mut h = [[0.0; M]; M];
        #[allow(clippy::needless_range_loop)]
        for i in 0..M {
            for j in 0..=i {
                let v = self.hess(i, j);
                h[i][j] = v;
                h[j][i] = v;
            }
        }
        h
    }
}

pub trait HigherOrder: SecondOrder {
    // TODO: Add support for higher order derivatives
}

/// Third-order derivative access.
///
/// Provides access to \\( \partial^3 f / \partial p_i \partial p_j \partial p_k \\).
/// The stored tensor is fully symmetric under any permutation of
/// \\( (i, j, k) \\); the accessor canonicalizes indices internally.
pub trait ThirdOrder: SecondOrder {
    /// Third derivative with respect to parameters \\( p_i, p_j, p_k \\).
    /// Accepts any index order; the tensor is symmetric.
    fn tens(&self, i: usize, j: usize, k: usize) -> f64;
    /// Mutable third derivative. Accepts any index order.
    fn tens_mut(&mut self, i: usize, j: usize, k: usize) -> &mut f64;

    /// Extract the full third-order tensor as a symmetric \\( M \times M \times M \\) array.
    ///
    /// The returned tensor has all 6 permutations of every \\( (i, j, k) \\) populated.
    fn extract_tens<const M: usize>(&self) -> [[[f64; M]; M]; M] {
        let mut t = [[[0.0; M]; M]; M];
        for i in 0..M {
            for j in 0..=i {
                for k in 0..=j {
                    let v = self.tens(i, j, k);
                    t[i][j][k] = v;
                    t[i][k][j] = v;
                    t[j][i][k] = v;
                    t[j][k][i] = v;
                    t[k][i][j] = v;
                    t[k][j][i] = v;
                }
            }
        }
        t
    }
}

pub trait DifferentiableMath: Differentiable {
    // Trigonometry
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    fn asin(self) -> Self;
    fn acos(self) -> Self;
    fn atan(self) -> Self;
    fn atan2(y: Self, x: Self) -> Self;
    fn sin_cos(self) -> (Self, Self);
    fn sinh(self) -> Self;
    fn cosh(self) -> Self;
    fn tanh(self) -> Self;

    // Exponential and logarithmic functions
    fn exp(self) -> Self;
    fn log(self, base: f64) -> Self;
    fn log10(self) -> Self;
    fn ln(self) -> Self;

    // Power functions
    fn sqrt(self) -> Self;
    fn powf(self, n: f64) -> Self;
    fn powi(self, n: i32) -> Self;

    // Absolute value
    fn abs(self) -> Self;
}

/// Marker trait for types that support both automatic differentiation
/// (first-order gradient access) and mathematical functions.
///
/// This combines [`DifferentiableMath`] (computation) with [`FirstOrder`]
/// (derivative extraction) into a single bound for convenience.
pub trait AutoDiff: DifferentiableMath + FirstOrder {}
