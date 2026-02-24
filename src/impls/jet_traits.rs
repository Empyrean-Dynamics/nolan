use crate::jets::{Jet, hess_index};
use crate::traits::{AutoDiff, Differentiable, DifferentiableMath, FirstOrder, SecondOrder};

impl<const O: usize, const N: usize> Differentiable for Jet<O, N> {
    fn value(&self) -> f64 {
        self.value
    }

    fn constant(value: f64) -> Self {
        Self::constant(value)
    }
}

impl<const N: usize> FirstOrder for Jet<1, N> {
    fn grad(&self, i: usize) -> f64 {
        self.grad[i]
    }

    fn grad_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.grad[i]
    }
}

impl<const N: usize> FirstOrder for Jet<2, N> {
    fn grad(&self, i: usize) -> f64 {
        self.grad[i]
    }

    fn grad_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.grad[i]
    }
}

impl<const N: usize> SecondOrder for Jet<2, N> {
    fn hess(&self, i: usize, j: usize) -> f64 {
        self.hess[hess_index(i, j).unwrap()]
    }

    fn hess_mut(&mut self, i: usize, j: usize) -> &mut f64 {
        &mut self.hess[hess_index(i, j).unwrap()]
    }
}

impl<const O: usize, const N: usize> DifferentiableMath for Jet<O, N> {
    fn sin(self) -> Self {
        self.sin()
    }
    fn cos(self) -> Self {
        self.cos()
    }
    fn tan(self) -> Self {
        self.tan()
    }
    fn asin(self) -> Self {
        self.asin()
    }
    fn acos(self) -> Self {
        self.acos()
    }
    fn atan(self) -> Self {
        self.atan()
    }
    fn atan2(y: Self, x: Self) -> Self {
        y.atan2(x)
    }
    fn sin_cos(self) -> (Self, Self) {
        self.sin_cos()
    }
    fn sinh(self) -> Self {
        self.sinh()
    }
    fn cosh(self) -> Self {
        self.cosh()
    }
    fn tanh(self) -> Self {
        self.tanh()
    }
    fn exp(self) -> Self {
        self.exp()
    }
    fn log(self, base: f64) -> Self {
        self.log(base)
    }
    fn log10(self) -> Self {
        self.log10()
    }
    fn ln(self) -> Self {
        self.ln()
    }
    fn sqrt(self) -> Self {
        self.sqrt()
    }
    fn powf(self, n: f64) -> Self {
        self.powf(n)
    }
    fn powi(self, n: i32) -> Self {
        self.powi(n)
    }
    fn abs(self) -> Self {
        self.abs()
    }
}

impl<const O: usize, const N: usize> AutoDiff for Jet<O, N> {}
