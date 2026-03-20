use std::ops::{Add, Div, Mul, Neg, Sub};

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
