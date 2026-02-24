use crate::traits::{Differentiable, DifferentiableMath};

impl Differentiable for f64 {
    fn value(&self) -> f64 {
        *self
    }
    fn constant(value: f64) -> Self {
        value
    }
}

impl DifferentiableMath for f64 {
    fn sin(self) -> Self {
        f64::sin(self)
    }
    fn cos(self) -> Self {
        f64::cos(self)
    }
    fn tan(self) -> Self {
        f64::tan(self)
    }
    fn asin(self) -> Self {
        f64::asin(self)
    }
    fn acos(self) -> Self {
        f64::acos(self)
    }
    fn atan(self) -> Self {
        f64::atan(self)
    }
    fn atan2(y: Self, x: Self) -> Self {
        f64::atan2(y, x)
    }
    fn sin_cos(self) -> (Self, Self) {
        f64::sin_cos(self)
    }
    fn sinh(self) -> Self {
        f64::sinh(self)
    }
    fn cosh(self) -> Self {
        f64::cosh(self)
    }
    fn tanh(self) -> Self {
        f64::tanh(self)
    }
    fn exp(self) -> Self {
        f64::exp(self)
    }
    fn log(self, base: f64) -> Self {
        f64::log(self, base)
    }
    fn log10(self) -> Self {
        f64::log10(self)
    }
    fn ln(self) -> Self {
        f64::ln(self)
    }
    fn sqrt(self) -> Self {
        f64::sqrt(self)
    }
    fn powf(self, n: f64) -> Self {
        f64::powf(self, n)
    }
    fn powi(self, n: i32) -> Self {
        f64::powi(self, n)
    }
    fn abs(self) -> Self {
        f64::abs(self)
    }
}
