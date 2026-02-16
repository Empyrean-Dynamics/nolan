use std::ops::{Add, Div, Mul, Sub};

use crate::jets::{HyperDual, Jet, MAX_HESS_STORAGE, hess_size};

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

impl<const O: usize, const N: usize> Add<f64> for Jet<O, N> {
    type Output = Self;

    fn add(self, rhs: f64) -> Self {
        Jet {
            value: self.value + rhs,
            grad: self.grad,
            hess: self.hess,
        }
    }
}

impl<const O: usize, const N: usize> Add<Jet<O, N>> for f64 {
    type Output = Jet<O, N>;

    fn add(self, rhs: Jet<O, N>) -> Jet<O, N> {
        rhs + self
    }
}

impl<const O: usize, const N: usize> Sub<f64> for Jet<O, N> {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self {
        Jet {
            value: self.value - rhs,
            grad: self.grad,
            hess: self.hess,
        }
    }
}

impl<const O: usize, const N: usize> Sub<Jet<O, N>> for f64 {
    type Output = Jet<O, N>;

    fn sub(self, rhs: Jet<O, N>) -> Jet<O, N> {
        let mut result = Jet {
            value: self - rhs.value,
            grad: [0.0; N],
            hess: [0.0; MAX_HESS_STORAGE],
        };

        if O >= 1 {
            for i in 0..N {
                result.grad[i] = -rhs.grad[i];
            }
        }

        if O >= 2 {
            for i in 0..hess_size(N) {
                result.hess[i] = -rhs.hess[i];
            }
        }

        result
    }
}

impl<const O: usize, const N: usize> Mul<f64> for Jet<O, N> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        let mut result = Jet {
            value: self.value * rhs,
            grad: [0.0; N],
            hess: [0.0; MAX_HESS_STORAGE],
        };

        if O >= 1 {
            for i in 0..N {
                result.grad[i] = self.grad[i] * rhs;
            }
        }

        if O >= 2 {
            for i in 0..hess_size(N) {
                result.hess[i] = self.hess[i] * rhs;
            }
        }

        result
    }
}

impl<const O: usize, const N: usize> Mul<Jet<O, N>> for f64 {
    type Output = Jet<O, N>;

    fn mul(self, rhs: Jet<O, N>) -> Jet<O, N> {
        rhs * self
    }
}

impl<const O: usize, const N: usize> Div<f64> for Jet<O, N> {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        let k_inv = 1.0 / rhs;
        self * k_inv
    }
}

impl<const O: usize, const N: usize> Div<Jet<O, N>> for f64 {
    type Output = Jet<O, N>;

    fn div(self, rhs: Jet<O, N>) -> Jet<O, N> {
        let a_inv = 1.0 / rhs.value;

        let mut result = Jet {
            value: rhs.value * a_inv,
            grad: [0.0; N],
            hess: [0.0; MAX_HESS_STORAGE],
        };

        if O >= 1 {
            for i in 0..N {
                result.grad[i] = rhs.grad[i] * a_inv * a_inv * -self;
            }
        }

        // TODO: Implement Hessian division
        result
    }
}
