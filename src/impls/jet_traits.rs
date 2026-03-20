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
    use crate::jets::hess_size;

    #[test]
    fn test_f64_variable_returns_value() {
        let x = <f64 as Differentiable>::variable(1.23, 0);
        assert_eq!(x, 1.23);
        let y = <f64 as Differentiable>::variable(4.56, 42);
        assert_eq!(y, 4.56);
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

    // ─── extract_grad ──────────────────────────────────────────

    #[test]
    fn test_extract_grad_identity() {
        // A variable seeded at index 2 should have unit gradient there.
        let x = Jet1::<6>::variable(3.0, 2);
        let g: [f64; 6] = x.extract_grad();
        assert_eq!(g, [0.0, 0.0, 1.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_extract_grad_constant() {
        let c = Jet1::<6>::constant(5.0);
        let g: [f64; 6] = c.extract_grad();
        assert_eq!(g, [0.0; 6]);
    }

    #[test]
    fn test_extract_grad_after_arithmetic() {
        // f = x * y where x = var(2.0, 0), y = var(3.0, 1)
        // df/dx = y.value = 3.0, df/dy = x.value = 2.0
        let x = Jet1::<2>::variable(2.0, 0);
        let y = Jet1::<2>::variable(3.0, 1);
        let f = x * y;
        let g: [f64; 2] = f.extract_grad();
        assert_eq!(g[0], 3.0); // df/dx = y
        assert_eq!(g[1], 2.0); // df/dy = x
    }

    #[test]
    fn test_extract_grad_chain_rule() {
        // f = sin(x) where x = var(0.5, 0)
        // df/dx = cos(0.5)
        let x = Jet1::<1>::variable(0.5, 0);
        let f = x.sin();
        let g: [f64; 1] = f.extract_grad();
        assert!((g[0] - 0.5_f64.cos()).abs() < 1e-15);
    }

    #[test]
    fn test_extract_grad_jet2() {
        // extract_grad should also work on Jet2 (via FirstOrder)
        let x = Jet2::<3, { hess_size(3) }>::variable(1.0, 1);
        let g: [f64; 3] = x.extract_grad();
        assert_eq!(g, [0.0, 1.0, 0.0]);
    }

    // ─── extract_hess ──────────────────────────────────────────

    #[test]
    fn test_extract_hess_constant() {
        let c = Jet2::<3, { hess_size(3) }>::constant(5.0);
        let h: [[f64; 3]; 3] = c.extract_hess();
        assert_eq!(h, [[0.0; 3]; 3]);
    }

    #[test]
    fn test_extract_hess_variable() {
        // A single variable has zero Hessian (it's linear).
        let x = Jet2::<3, { hess_size(3) }>::variable(2.0, 0);
        let h: [[f64; 3]; 3] = x.extract_hess();
        assert_eq!(h, [[0.0; 3]; 3]);
    }

    #[test]
    fn test_extract_hess_product() {
        // f = x * y where x = var(2.0, 0), y = var(3.0, 1)
        // d²f/dx² = 0, d²f/dy² = 0, d²f/dxdy = 1.0
        let x = Jet2::<2, { hess_size(2) }>::variable(2.0, 0);
        let y = Jet2::<2, { hess_size(2) }>::variable(3.0, 1);
        let f = x * y;
        let h: [[f64; 2]; 2] = f.extract_hess();
        assert_eq!(h[0][0], 0.0); // d²f/dx²
        assert_eq!(h[1][1], 0.0); // d²f/dy²
        assert_eq!(h[0][1], 1.0); // d²f/dxdy
        assert_eq!(h[1][0], 1.0); // symmetry
    }

    #[test]
    fn test_extract_hess_square() {
        // f = x^2 where x = var(3.0, 0)
        // d²f/dx² = 2.0
        let x = Jet2::<1, { hess_size(1) }>::variable(3.0, 0);
        let f = x * x;
        let h: [[f64; 1]; 1] = f.extract_hess();
        assert_eq!(h[0][0], 2.0);
    }

    #[test]
    fn test_extract_hess_symmetry() {
        // f = x * y * z — mixed partials should be symmetric
        let x = Jet2::<3, { hess_size(3) }>::variable(1.0, 0);
        let y = Jet2::<3, { hess_size(3) }>::variable(2.0, 1);
        let z = Jet2::<3, { hess_size(3) }>::variable(3.0, 2);
        let f = x * y * z;
        let h: [[f64; 3]; 3] = f.extract_hess();
        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(
                    h[i][j], h[j][i],
                    "Hessian not symmetric at [{i}][{j}]: {} != {}",
                    h[i][j], h[j][i]
                );
            }
        }
        // d²f/dxdy = z.value = 3.0
        assert_eq!(h[0][1], 3.0);
        // d²f/dxdz = y.value = 2.0
        assert_eq!(h[0][2], 2.0);
        // d²f/dydz = x.value = 1.0
        assert_eq!(h[1][2], 1.0);
    }

    #[test]
    fn test_extract_hess_sin() {
        // f = sin(x) where x = var(1.0, 0)
        // d²f/dx² = -sin(1.0)
        let x = Jet2::<1, { hess_size(1) }>::variable(1.0, 0);
        let f = x.sin();
        let h: [[f64; 1]; 1] = f.extract_hess();
        assert!((h[0][0] - (-1.0_f64.sin())).abs() < 1e-15);
    }
}
