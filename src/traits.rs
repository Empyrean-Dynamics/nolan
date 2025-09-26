use std::ops::{Add, Div, Mul, Neg, Sub};

pub trait Differentiable:
    Clone
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
{
    /// Function value.
    fn value(&self) -> f64;
    /// Initialize a constant differentiable (derivatives are all zero).
    fn constant(value: f64) -> Self;
    /// Maximum derivative order supported.
    fn order(&self) -> usize;
    /// Number of tracked parameters.
    fn n_params(&self) -> usize;
}

pub trait FirstOrder: Differentiable {
    /// `∂/∂p_i`: derivative with respect to parameter `p_i` (parameter stored at index i).
    fn grad(&self, i: usize) -> f64;
    /// `∂/∂p_i`: Mutable derivative with respect to parameter `p_i` (parameter stored at index i).
    fn grad_mut(&mut self, i: usize) -> &mut f64;
}

pub trait SecondOrder: FirstOrder {
    /// `∂²/(∂p_i ∂p_j)`: Second derivative with respect to parameters `p_i` and `p_j` (parameters stored at indices i & j).
    fn hess(&self, i: usize, j: usize) -> f64;
    /// `∂²/(∂p_i ∂p_j)`: Mutable second derivative with respect to parameters `p_i` and `p_j` (parameters stored at indices i & j).
    fn hess_mut(&mut self, i: usize, j: usize) -> &mut f64;
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
    fn asinh(self) -> Self;
    fn acosh(self) -> Self;
    fn atanh(self) -> Self;

    // Exponential and logarithmic functions
    fn exp(self) -> Self;
    fn log(self, base: Self) -> Self;
    fn log2(self) -> Self;
    fn log10(self) -> Self;
    fn ln(self) -> Self;

    // Power functions
    fn sqrt(self) -> Self;
    fn powf(self, n: Self) -> Self;
    fn powi(self, n: i32) -> Self;
}
