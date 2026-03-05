use crate::jets::{Jet1, Jet2, hess_index};
use crate::traits::{AutoDiff, Differentiable, DifferentiableMath, FirstOrder, SecondOrder};

// ─── Differentiable ─────────────────────────────────────────────

impl<const N: usize> Differentiable for Jet1<N> {
    fn value(&self) -> f64 {
        self.value
    }

    fn constant(value: f64) -> Self {
        Self::constant(value)
    }
}

impl<const N: usize, const H: usize> Differentiable for Jet2<N, H> {
    fn value(&self) -> f64 {
        self.value
    }

    fn constant(value: f64) -> Self {
        Self::constant(value)
    }
}

// ─── FirstOrder ─────────────────────────────────────────────────

impl<const N: usize> FirstOrder for Jet1<N> {
    fn grad(&self, i: usize) -> f64 {
        self.grad[i]
    }

    fn grad_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.grad[i]
    }
}

impl<const N: usize, const H: usize> FirstOrder for Jet2<N, H> {
    fn grad(&self, i: usize) -> f64 {
        self.grad[i]
    }

    fn grad_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.grad[i]
    }
}

// ─── SecondOrder ────────────────────────────────────────────────

impl<const N: usize, const H: usize> SecondOrder for Jet2<N, H> {
    fn hess(&self, i: usize, j: usize) -> f64 {
        self.hess[hess_index(i, j).unwrap()]
    }

    fn hess_mut(&mut self, i: usize, j: usize) -> &mut f64 {
        &mut self.hess[hess_index(i, j).unwrap()]
    }
}

// ─── DifferentiableMath ─────────────────────────────────────────

impl<const N: usize> DifferentiableMath for Jet1<N> {
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

impl<const N: usize, const H: usize> DifferentiableMath for Jet2<N, H> {
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

// ─── AutoDiff ───────────────────────────────────────────────────

impl<const N: usize> AutoDiff for Jet1<N> {}
impl<const N: usize, const H: usize> AutoDiff for Jet2<N, H> {}
