use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::jets::{Jet, MAX_HESS_STORAGE, hess_index, hess_size};

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

        if O >= 2 {
            for i in 0..hess_size(N) {
                result.hess[i] = self.hess[i] + rhs.hess[i]
            }
        }
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

        if O >= 2 {
            for i in 0..hess_size(N) {
                result.hess[i] = self.hess[i] - rhs.hess[i]
            }
        }
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

        if O >= 2 {
            for i in 0..N {
                for j in 0..=i {
                    let idx = hess_index(i, j).unwrap();
                    result.hess[idx] = self.value * rhs.hess[idx]
                        + rhs.value * self.hess[idx]
                        + self.grad[i] * rhs.grad[j]
                        + self.grad[j] * rhs.grad[i];
                }
            }
        }
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
                result.grad[i] = (self.grad[i] - result.value * rhs.grad[i]) / rhs.value;
            }
        }

        if O >= 2 {
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
        }
        result
    }
}

impl<const O: usize, const N: usize> Neg for Jet<O, N> {
    type Output = Self;

    fn neg(self) -> Self {
        let mut result = Jet {
            value: -self.value,
            grad: [0.0; N],
            hess: [0.0; MAX_HESS_STORAGE],
        };

        if O >= 1 {
            for i in 0..N {
                result.grad[i] = -self.grad[i];
            }
        }

        if O >= 2 {
            for i in 0..hess_size(N) {
                result.hess[i] = -self.hess[i];
            }
        }

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
        let mut result = Jet {
            value: self / rhs.value,
            grad: [0.0; N],
            hess: [0.0; MAX_HESS_STORAGE],
        };

        if O >= 1 {
            for i in 0..N {
                result.grad[i] = (-result.value * rhs.grad[i]) / rhs.value;
            }
        }

        if O >= 2 {
            for i in 0..N {
                for j in 0..=i {
                    let idx = hess_index(i, j).unwrap();
                    result.hess[idx] = (-result.value * rhs.hess[idx]
                        - result.grad[i] * rhs.grad[j]
                        - result.grad[j] * rhs.grad[i])
                        / rhs.value;
                }
            }
        }
        result
    }
}
