use std::ops::{Add, Div, Mul, Sub};

use crate::jets::{HyperDual, Jet, MAX_HESS_STORAGE};

impl<const O: usize, const N: usize> Add for Jet<O, N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut result = Jet {
            value: self.value + rhs.value,
            grad: [0.0; N],
            hess: [0.0; MAX_HESS_STORAGE],
        };

        if O >= 1 {
            for i in 0..N {
                result.grad[i] = self.grad[i] + rhs.grad[i]
            }
        }

        // TODO: Implement Hessian addition
        result
    }
}

impl<const O: usize, const N: usize> Sub for Jet<O, N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut result = Jet {
            value: self.value - rhs.value,
            grad: [0.0; N],
            hess: [0.0; MAX_HESS_STORAGE],
        };

        if O >= 1 {
            for i in 0..N {
                result.grad[i] = self.grad[i] - rhs.grad[i]
            }
        }

        // TODO: Implement Hessian subtraction
        result
    }
}

impl<const O: usize, const N: usize> Mul for Jet<O, N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut result = Jet {
            value: self.value * rhs.value,
            grad: [0.0; N],
            hess: [0.0; MAX_HESS_STORAGE],
        };

        if O >= 1 {
            for i in 0..N {
                result.grad[i] = self.value * rhs.grad[i] + rhs.value * self.grad[i];
            }
        }

        // TODO: Implement Hessian multiplication
        result
    }
}

impl<const O: usize, const N: usize> Div for Jet<O, N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let mut result = Jet {
            value: self.value / rhs.value,
            grad: [0.0; N],
            hess: [0.0; MAX_HESS_STORAGE],
        };

        if O >= 1 {
            for i in 0..N {
                result.grad[i] = ((rhs.value * self.grad[i]) - (self.value * rhs.grad[i]))
                    / (rhs.value * rhs.value);
            }
        }

        // TODO: Implement Hessian division
        result
    }
}
