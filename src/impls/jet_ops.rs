use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::jets::{Jet1, Jet2, hess_index, hess_size};

// ─── Jet1: Jet + Jet ────────────────────────────────────────────

impl<const N: usize> Add for Jet1<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut result = Jet1 {
            value: self.value + rhs.value,
            grad: [0.0; N],
        };
        for i in 0..N {
            result.grad[i] = self.grad[i] + rhs.grad[i];
        }
        result
    }
}

impl<const N: usize> Sub for Jet1<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut result = Jet1 {
            value: self.value - rhs.value,
            grad: [0.0; N],
        };
        for i in 0..N {
            result.grad[i] = self.grad[i] - rhs.grad[i];
        }
        result
    }
}

impl<const N: usize> Mul for Jet1<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut result = Jet1 {
            value: self.value * rhs.value,
            grad: [0.0; N],
        };
        for i in 0..N {
            result.grad[i] = self.value * rhs.grad[i] + rhs.value * self.grad[i];
        }
        result
    }
}

impl<const N: usize> Div for Jet1<N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let mut result = Jet1 {
            value: self.value / rhs.value,
            grad: [0.0; N],
        };
        for i in 0..N {
            result.grad[i] = (self.grad[i] - result.value * rhs.grad[i]) / rhs.value;
        }
        result
    }
}

impl<const N: usize> Neg for Jet1<N> {
    type Output = Self;

    fn neg(self) -> Self {
        let mut result = Jet1 {
            value: -self.value,
            grad: [0.0; N],
        };
        for i in 0..N {
            result.grad[i] = -self.grad[i];
        }
        result
    }
}

// ─── Jet1: Jet + f64 / f64 + Jet ────────────────────────────────

impl<const N: usize> Add<f64> for Jet1<N> {
    type Output = Self;

    fn add(self, rhs: f64) -> Self {
        Jet1 {
            value: self.value + rhs,
            grad: self.grad,
        }
    }
}

impl<const N: usize> Add<Jet1<N>> for f64 {
    type Output = Jet1<N>;

    fn add(self, rhs: Jet1<N>) -> Jet1<N> {
        rhs + self
    }
}

impl<const N: usize> Sub<f64> for Jet1<N> {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self {
        Jet1 {
            value: self.value - rhs,
            grad: self.grad,
        }
    }
}

impl<const N: usize> Sub<Jet1<N>> for f64 {
    type Output = Jet1<N>;

    fn sub(self, rhs: Jet1<N>) -> Jet1<N> {
        let mut result = Jet1 {
            value: self - rhs.value,
            grad: [0.0; N],
        };
        for i in 0..N {
            result.grad[i] = -rhs.grad[i];
        }
        result
    }
}

impl<const N: usize> Mul<f64> for Jet1<N> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        let mut result = Jet1 {
            value: self.value * rhs,
            grad: [0.0; N],
        };
        for i in 0..N {
            result.grad[i] = self.grad[i] * rhs;
        }
        result
    }
}

impl<const N: usize> Mul<Jet1<N>> for f64 {
    type Output = Jet1<N>;

    fn mul(self, rhs: Jet1<N>) -> Jet1<N> {
        rhs * self
    }
}

impl<const N: usize> Div<f64> for Jet1<N> {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        let k_inv = 1.0 / rhs;
        self * k_inv
    }
}

impl<const N: usize> Div<Jet1<N>> for f64 {
    type Output = Jet1<N>;

    fn div(self, rhs: Jet1<N>) -> Jet1<N> {
        let mut result = Jet1 {
            value: self / rhs.value,
            grad: [0.0; N],
        };
        for i in 0..N {
            result.grad[i] = (-result.value * rhs.grad[i]) / rhs.value;
        }
        result
    }
}

// ─── Jet2: Jet + Jet ────────────────────────────────────────────

impl<const N: usize, const H: usize> Add for Jet2<N, H> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut result = Jet2 {
            value: self.value + rhs.value,
            grad: [0.0; N],
            hess: [0.0; H],
        };
        for i in 0..N {
            result.grad[i] = self.grad[i] + rhs.grad[i];
        }
        for i in 0..hess_size(N) {
            result.hess[i] = self.hess[i] + rhs.hess[i];
        }
        result
    }
}

impl<const N: usize, const H: usize> Sub for Jet2<N, H> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut result = Jet2 {
            value: self.value - rhs.value,
            grad: [0.0; N],
            hess: [0.0; H],
        };
        for i in 0..N {
            result.grad[i] = self.grad[i] - rhs.grad[i];
        }
        for i in 0..hess_size(N) {
            result.hess[i] = self.hess[i] - rhs.hess[i];
        }
        result
    }
}

impl<const N: usize, const H: usize> Mul for Jet2<N, H> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut result = Jet2 {
            value: self.value * rhs.value,
            grad: [0.0; N],
            hess: [0.0; H],
        };
        for i in 0..N {
            result.grad[i] = self.value * rhs.grad[i] + rhs.value * self.grad[i];
        }
        for i in 0..N {
            for j in 0..=i {
                let idx = hess_index(i, j).unwrap();
                result.hess[idx] = self.value * rhs.hess[idx]
                    + rhs.value * self.hess[idx]
                    + self.grad[i] * rhs.grad[j]
                    + self.grad[j] * rhs.grad[i];
            }
        }
        result
    }
}

impl<const N: usize, const H: usize> Div for Jet2<N, H> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let mut result = Jet2 {
            value: self.value / rhs.value,
            grad: [0.0; N],
            hess: [0.0; H],
        };
        for i in 0..N {
            result.grad[i] = (self.grad[i] - result.value * rhs.grad[i]) / rhs.value;
        }
        for i in 0..N {
            for j in 0..=i {
                let idx = hess_index(i, j).unwrap();
                result.hess[idx] = (self.hess[idx]
                    - result.value * rhs.hess[idx]
                    - result.grad[i] * rhs.grad[j]
                    - result.grad[j] * rhs.grad[i])
                    / rhs.value;
            }
        }
        result
    }
}

impl<const N: usize, const H: usize> Neg for Jet2<N, H> {
    type Output = Self;

    fn neg(self) -> Self {
        let mut result = Jet2 {
            value: -self.value,
            grad: [0.0; N],
            hess: [0.0; H],
        };
        for i in 0..N {
            result.grad[i] = -self.grad[i];
        }
        for i in 0..hess_size(N) {
            result.hess[i] = -self.hess[i];
        }
        result
    }
}

// ─── Jet2: Jet + f64 / f64 + Jet ────────────────────────────────

impl<const N: usize, const H: usize> Add<f64> for Jet2<N, H> {
    type Output = Self;

    fn add(self, rhs: f64) -> Self {
        Jet2 {
            value: self.value + rhs,
            grad: self.grad,
            hess: self.hess,
        }
    }
}

impl<const N: usize, const H: usize> Add<Jet2<N, H>> for f64 {
    type Output = Jet2<N, H>;

    fn add(self, rhs: Jet2<N, H>) -> Jet2<N, H> {
        rhs + self
    }
}

impl<const N: usize, const H: usize> Sub<f64> for Jet2<N, H> {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self {
        Jet2 {
            value: self.value - rhs,
            grad: self.grad,
            hess: self.hess,
        }
    }
}

impl<const N: usize, const H: usize> Sub<Jet2<N, H>> for f64 {
    type Output = Jet2<N, H>;

    fn sub(self, rhs: Jet2<N, H>) -> Jet2<N, H> {
        let mut result = Jet2 {
            value: self - rhs.value,
            grad: [0.0; N],
            hess: [0.0; H],
        };
        for i in 0..N {
            result.grad[i] = -rhs.grad[i];
        }
        for i in 0..hess_size(N) {
            result.hess[i] = -rhs.hess[i];
        }
        result
    }
}

impl<const N: usize, const H: usize> Mul<f64> for Jet2<N, H> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        let mut result = Jet2 {
            value: self.value * rhs,
            grad: [0.0; N],
            hess: [0.0; H],
        };
        for i in 0..N {
            result.grad[i] = self.grad[i] * rhs;
        }
        for i in 0..hess_size(N) {
            result.hess[i] = self.hess[i] * rhs;
        }
        result
    }
}

impl<const N: usize, const H: usize> Mul<Jet2<N, H>> for f64 {
    type Output = Jet2<N, H>;

    fn mul(self, rhs: Jet2<N, H>) -> Jet2<N, H> {
        rhs * self
    }
}

impl<const N: usize, const H: usize> Div<f64> for Jet2<N, H> {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        let k_inv = 1.0 / rhs;
        self * k_inv
    }
}

impl<const N: usize, const H: usize> Div<Jet2<N, H>> for f64 {
    type Output = Jet2<N, H>;

    fn div(self, rhs: Jet2<N, H>) -> Jet2<N, H> {
        let mut result = Jet2 {
            value: self / rhs.value,
            grad: [0.0; N],
            hess: [0.0; H],
        };
        for i in 0..N {
            result.grad[i] = (-result.value * rhs.grad[i]) / rhs.value;
        }
        for i in 0..N {
            for j in 0..=i {
                let idx = hess_index(i, j).unwrap();
                result.hess[idx] = (-result.value * rhs.hess[idx]
                    - result.grad[i] * rhs.grad[j]
                    - result.grad[j] * rhs.grad[i])
                    / rhs.value;
            }
        }
        result
    }
}
