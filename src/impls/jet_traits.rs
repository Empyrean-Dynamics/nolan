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

    fn variable(value: f64, param_idx: usize) -> Self {
        Self::variable(value, param_idx)
    }
}

impl<const N: usize, const H: usize> Differentiable for Jet2<N, H> {
    fn value(&self) -> f64 {
        self.value
    }

    fn constant(value: f64) -> Self {
        Self::constant(value)
    }

    fn variable(value: f64, param_idx: usize) -> Self {
        Self::variable(value, param_idx)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f64_variable_returns_value() {
        let x = <f64 as Differentiable>::variable(3.14, 0);
        assert_eq!(x, 3.14);
        let y = <f64 as Differentiable>::variable(2.71, 42);
        assert_eq!(y, 2.71);
    }

    #[test]
    fn test_jet1_9_variable_seeds_gradient() {
        let x = <Jet1<9> as Differentiable>::variable(1.5, 6);
        assert_eq!(x.value(), 1.5);
        // Gradient should be 1.0 at index 6, 0.0 elsewhere
        for i in 0..9 {
            let expected = if i == 6 { 1.0 } else { 0.0 };
            assert_eq!(
                x.grad(i),
                expected,
                "grad({i}) = {}, expected {expected}",
                x.grad(i)
            );
        }
    }

    #[test]
    fn test_jet1_6_variable_seeds_gradient() {
        for idx in 0..6 {
            let x = <Jet1<6> as Differentiable>::variable(2.0, idx);
            assert_eq!(x.value(), 2.0);
            for j in 0..6 {
                let expected = if j == idx { 1.0 } else { 0.0 };
                assert_eq!(x.grad(j), expected);
            }
        }
    }
}
