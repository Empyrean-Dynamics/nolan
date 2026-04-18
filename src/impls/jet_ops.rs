use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::jets::{Jet1, Jet2, Jet3, hess_idx, hess_size, tens_idx, tens_size};

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
                let idx = hess_idx(i, j);
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
                let idx = hess_idx(i, j);
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
                let idx = hess_idx(i, j);
                result.hess[idx] = (-result.value * rhs.hess[idx]
                    - result.grad[i] * rhs.grad[j]
                    - result.grad[j] * rhs.grad[i])
                    / rhs.value;
            }
        }
        result
    }
}

// ─── Jet3: Jet + Jet ────────────────────────────────────────────

impl<const N: usize, const H: usize, const T: usize> Add for Jet3<N, H, T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut result = Jet3 {
            value: self.value + rhs.value,
            grad: [0.0; N],
            hess: [0.0; H],
            tens: [0.0; T],
        };
        for i in 0..N {
            result.grad[i] = self.grad[i] + rhs.grad[i];
        }
        for i in 0..hess_size(N) {
            result.hess[i] = self.hess[i] + rhs.hess[i];
        }
        for i in 0..tens_size(N) {
            result.tens[i] = self.tens[i] + rhs.tens[i];
        }
        result
    }
}

impl<const N: usize, const H: usize, const T: usize> Sub for Jet3<N, H, T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut result = Jet3 {
            value: self.value - rhs.value,
            grad: [0.0; N],
            hess: [0.0; H],
            tens: [0.0; T],
        };
        for i in 0..N {
            result.grad[i] = self.grad[i] - rhs.grad[i];
        }
        for i in 0..hess_size(N) {
            result.hess[i] = self.hess[i] - rhs.hess[i];
        }
        for i in 0..tens_size(N) {
            result.tens[i] = self.tens[i] - rhs.tens[i];
        }
        result
    }
}

impl<const N: usize, const H: usize, const T: usize> Mul for Jet3<N, H, T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut result = Jet3 {
            value: self.value * rhs.value,
            grad: [0.0; N],
            hess: [0.0; H],
            tens: [0.0; T],
        };
        for i in 0..N {
            result.grad[i] = self.value * rhs.grad[i] + rhs.value * self.grad[i];
        }
        for i in 0..N {
            for j in 0..=i {
                let idx = hess_idx(i, j);
                result.hess[idx] = self.value * rhs.hess[idx]
                    + rhs.value * self.hess[idx]
                    + self.grad[i] * rhs.grad[j]
                    + self.grad[j] * rhs.grad[i];
            }
        }
        // Third-order Leibniz rule for f*g:
        // ∂³(fg)/∂p_i∂p_j∂p_k = f·∂³g + g·∂³f
        //   + ∂f/∂p_i·∂²g/∂p_j∂p_k + ∂f/∂p_j·∂²g/∂p_i∂p_k + ∂f/∂p_k·∂²g/∂p_i∂p_j
        //   + ∂g/∂p_i·∂²f/∂p_j∂p_k + ∂g/∂p_j·∂²f/∂p_i∂p_k + ∂g/∂p_k·∂²f/∂p_i∂p_j
        for i in 0..N {
            for j in 0..=i {
                let h_ij = hess_idx(i, j);
                for k in 0..=j {
                    let t_idx = tens_idx(i, j, k);
                    let h_ik = hess_idx(i, k);
                    let h_jk = hess_idx(j, k);
                    result.tens[t_idx] = self.value * rhs.tens[t_idx]
                        + rhs.value * self.tens[t_idx]
                        + self.grad[i] * rhs.hess[h_jk]
                        + self.grad[j] * rhs.hess[h_ik]
                        + self.grad[k] * rhs.hess[h_ij]
                        + rhs.grad[i] * self.hess[h_jk]
                        + rhs.grad[j] * self.hess[h_ik]
                        + rhs.grad[k] * self.hess[h_ij];
                }
            }
        }
        result
    }
}

impl<const N: usize, const H: usize, const T: usize> Div for Jet3<N, H, T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let mut result = Jet3 {
            value: self.value / rhs.value,
            grad: [0.0; N],
            hess: [0.0; H],
            tens: [0.0; T],
        };
        for i in 0..N {
            result.grad[i] = (self.grad[i] - result.value * rhs.grad[i]) / rhs.value;
        }
        for i in 0..N {
            for j in 0..=i {
                let idx = hess_idx(i, j);
                result.hess[idx] = (self.hess[idx]
                    - result.value * rhs.hess[idx]
                    - result.grad[i] * rhs.grad[j]
                    - result.grad[j] * rhs.grad[i])
                    / rhs.value;
            }
        }
        // Third-order: rearrange from f = result * g
        // result.tens[ijk] = (f.tens[ijk] - result.value * g.tens[ijk]
        //   - result.grad[i]*g.hess[jk] - result.grad[j]*g.hess[ik] - result.grad[k]*g.hess[ij]
        //   - result.hess[ij]*g.grad[k] - result.hess[ik]*g.grad[j] - result.hess[jk]*g.grad[i]
        // ) / g.value
        for i in 0..N {
            for j in 0..=i {
                let h_ij = hess_idx(i, j);
                for k in 0..=j {
                    let t_idx = tens_idx(i, j, k);
                    let h_ik = hess_idx(i, k);
                    let h_jk = hess_idx(j, k);
                    result.tens[t_idx] = (self.tens[t_idx]
                        - result.value * rhs.tens[t_idx]
                        - result.grad[i] * rhs.hess[h_jk]
                        - result.grad[j] * rhs.hess[h_ik]
                        - result.grad[k] * rhs.hess[h_ij]
                        - result.hess[h_ij] * rhs.grad[k]
                        - result.hess[h_ik] * rhs.grad[j]
                        - result.hess[h_jk] * rhs.grad[i])
                        / rhs.value;
                }
            }
        }
        result
    }
}

impl<const N: usize, const H: usize, const T: usize> Neg for Jet3<N, H, T> {
    type Output = Self;

    fn neg(self) -> Self {
        let mut result = Jet3 {
            value: -self.value,
            grad: [0.0; N],
            hess: [0.0; H],
            tens: [0.0; T],
        };
        for i in 0..N {
            result.grad[i] = -self.grad[i];
        }
        for i in 0..hess_size(N) {
            result.hess[i] = -self.hess[i];
        }
        for i in 0..tens_size(N) {
            result.tens[i] = -self.tens[i];
        }
        result
    }
}

// ─── Jet3: Jet + f64 / f64 + Jet ────────────────────────────────

impl<const N: usize, const H: usize, const T: usize> Add<f64> for Jet3<N, H, T> {
    type Output = Self;

    fn add(self, rhs: f64) -> Self {
        Jet3 {
            value: self.value + rhs,
            grad: self.grad,
            hess: self.hess,
            tens: self.tens,
        }
    }
}

impl<const N: usize, const H: usize, const T: usize> Add<Jet3<N, H, T>> for f64 {
    type Output = Jet3<N, H, T>;

    fn add(self, rhs: Jet3<N, H, T>) -> Jet3<N, H, T> {
        rhs + self
    }
}

impl<const N: usize, const H: usize, const T: usize> Sub<f64> for Jet3<N, H, T> {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self {
        Jet3 {
            value: self.value - rhs,
            grad: self.grad,
            hess: self.hess,
            tens: self.tens,
        }
    }
}

impl<const N: usize, const H: usize, const T: usize> Sub<Jet3<N, H, T>> for f64 {
    type Output = Jet3<N, H, T>;

    fn sub(self, rhs: Jet3<N, H, T>) -> Jet3<N, H, T> {
        let mut result = Jet3 {
            value: self - rhs.value,
            grad: [0.0; N],
            hess: [0.0; H],
            tens: [0.0; T],
        };
        for i in 0..N {
            result.grad[i] = -rhs.grad[i];
        }
        for i in 0..hess_size(N) {
            result.hess[i] = -rhs.hess[i];
        }
        for i in 0..tens_size(N) {
            result.tens[i] = -rhs.tens[i];
        }
        result
    }
}

impl<const N: usize, const H: usize, const T: usize> Mul<f64> for Jet3<N, H, T> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        let mut result = Jet3 {
            value: self.value * rhs,
            grad: [0.0; N],
            hess: [0.0; H],
            tens: [0.0; T],
        };
        for i in 0..N {
            result.grad[i] = self.grad[i] * rhs;
        }
        for i in 0..hess_size(N) {
            result.hess[i] = self.hess[i] * rhs;
        }
        for i in 0..tens_size(N) {
            result.tens[i] = self.tens[i] * rhs;
        }
        result
    }
}

impl<const N: usize, const H: usize, const T: usize> Mul<Jet3<N, H, T>> for f64 {
    type Output = Jet3<N, H, T>;

    fn mul(self, rhs: Jet3<N, H, T>) -> Jet3<N, H, T> {
        rhs * self
    }
}

impl<const N: usize, const H: usize, const T: usize> Div<f64> for Jet3<N, H, T> {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        let k_inv = 1.0 / rhs;
        self * k_inv
    }
}

impl<const N: usize, const H: usize, const T: usize> Div<Jet3<N, H, T>> for f64 {
    type Output = Jet3<N, H, T>;

    fn div(self, rhs: Jet3<N, H, T>) -> Jet3<N, H, T> {
        let mut result = Jet3 {
            value: self / rhs.value,
            grad: [0.0; N],
            hess: [0.0; H],
            tens: [0.0; T],
        };
        for i in 0..N {
            result.grad[i] = (-result.value * rhs.grad[i]) / rhs.value;
        }
        for i in 0..N {
            for j in 0..=i {
                let idx = hess_idx(i, j);
                result.hess[idx] = (-result.value * rhs.hess[idx]
                    - result.grad[i] * rhs.grad[j]
                    - result.grad[j] * rhs.grad[i])
                    / rhs.value;
            }
        }
        for i in 0..N {
            for j in 0..=i {
                let h_ij = hess_idx(i, j);
                for k in 0..=j {
                    let t_idx = tens_idx(i, j, k);
                    let h_ik = hess_idx(i, k);
                    let h_jk = hess_idx(j, k);
                    result.tens[t_idx] = (-result.value * rhs.tens[t_idx]
                        - result.grad[i] * rhs.hess[h_jk]
                        - result.grad[j] * rhs.hess[h_ik]
                        - result.grad[k] * rhs.hess[h_ij]
                        - result.hess[h_ij] * rhs.grad[k]
                        - result.hess[h_ik] * rhs.grad[j]
                        - result.hess[h_jk] * rhs.grad[i])
                        / rhs.value;
                }
            }
        }
        result
    }
}

// ═══════════════════════════════════════════════════════════════
//  Compound assignment operators
// ═══════════════════════════════════════════════════════════════

// ─── Jet1: Assign ops ──────────────────────────────────────────

impl<const N: usize> AddAssign for Jet1<N> {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
        for i in 0..N {
            self.grad[i] += rhs.grad[i];
        }
    }
}

impl<const N: usize> SubAssign for Jet1<N> {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
        for i in 0..N {
            self.grad[i] -= rhs.grad[i];
        }
    }
}

impl<const N: usize> AddAssign<f64> for Jet1<N> {
    fn add_assign(&mut self, rhs: f64) {
        self.value += rhs;
    }
}

impl<const N: usize> SubAssign<f64> for Jet1<N> {
    fn sub_assign(&mut self, rhs: f64) {
        self.value -= rhs;
    }
}

impl<const N: usize> MulAssign<f64> for Jet1<N> {
    fn mul_assign(&mut self, rhs: f64) {
        self.value *= rhs;
        for i in 0..N {
            self.grad[i] *= rhs;
        }
    }
}

impl<const N: usize> DivAssign<f64> for Jet1<N> {
    fn div_assign(&mut self, rhs: f64) {
        let inv = 1.0 / rhs;
        *self *= inv;
    }
}

// ─── Jet2: Assign ops ──────────────────────────────────────────

impl<const N: usize, const H: usize> AddAssign for Jet2<N, H> {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
        for i in 0..N {
            self.grad[i] += rhs.grad[i];
        }
        for i in 0..hess_size(N) {
            self.hess[i] += rhs.hess[i];
        }
    }
}

impl<const N: usize, const H: usize> SubAssign for Jet2<N, H> {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
        for i in 0..N {
            self.grad[i] -= rhs.grad[i];
        }
        for i in 0..hess_size(N) {
            self.hess[i] -= rhs.hess[i];
        }
    }
}

impl<const N: usize, const H: usize> AddAssign<f64> for Jet2<N, H> {
    fn add_assign(&mut self, rhs: f64) {
        self.value += rhs;
    }
}

impl<const N: usize, const H: usize> SubAssign<f64> for Jet2<N, H> {
    fn sub_assign(&mut self, rhs: f64) {
        self.value -= rhs;
    }
}

impl<const N: usize, const H: usize> MulAssign<f64> for Jet2<N, H> {
    fn mul_assign(&mut self, rhs: f64) {
        self.value *= rhs;
        for i in 0..N {
            self.grad[i] *= rhs;
        }
        for i in 0..hess_size(N) {
            self.hess[i] *= rhs;
        }
    }
}

impl<const N: usize, const H: usize> DivAssign<f64> for Jet2<N, H> {
    fn div_assign(&mut self, rhs: f64) {
        let inv = 1.0 / rhs;
        *self *= inv;
    }
}

// ─── Jet3: Assign ops ──────────────────────────────────────────

impl<const N: usize, const H: usize, const T: usize> AddAssign for Jet3<N, H, T> {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
        for i in 0..N {
            self.grad[i] += rhs.grad[i];
        }
        for i in 0..hess_size(N) {
            self.hess[i] += rhs.hess[i];
        }
        for i in 0..tens_size(N) {
            self.tens[i] += rhs.tens[i];
        }
    }
}

impl<const N: usize, const H: usize, const T: usize> SubAssign for Jet3<N, H, T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
        for i in 0..N {
            self.grad[i] -= rhs.grad[i];
        }
        for i in 0..hess_size(N) {
            self.hess[i] -= rhs.hess[i];
        }
        for i in 0..tens_size(N) {
            self.tens[i] -= rhs.tens[i];
        }
    }
}

impl<const N: usize, const H: usize, const T: usize> AddAssign<f64> for Jet3<N, H, T> {
    fn add_assign(&mut self, rhs: f64) {
        self.value += rhs;
    }
}

impl<const N: usize, const H: usize, const T: usize> SubAssign<f64> for Jet3<N, H, T> {
    fn sub_assign(&mut self, rhs: f64) {
        self.value -= rhs;
    }
}

impl<const N: usize, const H: usize, const T: usize> MulAssign<f64> for Jet3<N, H, T> {
    fn mul_assign(&mut self, rhs: f64) {
        self.value *= rhs;
        for i in 0..N {
            self.grad[i] *= rhs;
        }
        for i in 0..hess_size(N) {
            self.hess[i] *= rhs;
        }
        for i in 0..tens_size(N) {
            self.tens[i] *= rhs;
        }
    }
}

impl<const N: usize, const H: usize, const T: usize> DivAssign<f64> for Jet3<N, H, T> {
    fn div_assign(&mut self, rhs: f64) {
        let inv = 1.0 / rhs;
        *self *= inv;
    }
}

#[cfg(test)]
mod jet3_ops_tests {
    use super::*;
    use crate::jets::{hess_size, tens_size};

    const TOL: f64 = 1e-12;

    fn approx_eq(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() <= tol
    }

    fn assert_close(actual: f64, expected: f64, what: &str) {
        assert!(
            approx_eq(actual, expected, TOL),
            "{}: expected {}, got {} (|diff| = {:e})",
            what,
            expected,
            actual,
            (actual - expected).abs()
        );
    }

    // Jet3 with N=2 parameters: H=3, T=4.
    type J3_2 = Jet3<2, { hess_size(2) }, { tens_size(2) }>;
    // Jet3 with N=1 parameter: H=1, T=1.
    type J3_1 = Jet3<1, { hess_size(1) }, { tens_size(1) }>;

    // ─── Add / Sub: linearity — non-grad derivatives pass through unchanged ──

    #[test]
    fn add_preserves_derivatives() {
        // f(x, y) = 3*x + 2*y + 7
        // ∂f/∂x = 3, ∂f/∂y = 2, all higher derivatives = 0.
        let x = J3_2::variable(1.5, 0);
        let y = J3_2::variable(-0.3, 1);
        let f = x * 3.0 + y * 2.0 + 7.0;
        assert_close(f.value, 3.0 * 1.5 + 2.0 * -0.3 + 7.0, "value");
        assert_close(f.grad[0], 3.0, "grad[0]");
        assert_close(f.grad[1], 2.0, "grad[1]");
        for h in f.hess.iter() {
            assert_close(*h, 0.0, "hess");
        }
        for t in f.tens.iter() {
            assert_close(*t, 0.0, "tens");
        }
    }

    #[test]
    fn sub_preserves_derivatives() {
        // f(x, y) = x - y
        let x = J3_2::variable(2.0, 0);
        let y = J3_2::variable(0.5, 1);
        let f = x - y;
        assert_close(f.value, 1.5, "value");
        assert_close(f.grad[0], 1.0, "grad[0]");
        assert_close(f.grad[1], -1.0, "grad[1]");
        for h in f.hess.iter() {
            assert_close(*h, 0.0, "hess");
        }
        for t in f.tens.iter() {
            assert_close(*t, 0.0, "tens");
        }
    }

    // ─── Mul: Leibniz product rule through order 3 ──────────────────────────

    #[test]
    fn mul_xy_zero_third_derivative() {
        // f(x, y) = x*y
        // ∂f/∂x = y, ∂f/∂y = x
        // ∂²f/∂x∂y = 1, diagonal Hessians = 0
        // All third derivatives = 0
        let a = 1.7;
        let b = -2.3;
        let x = J3_2::variable(a, 0);
        let y = J3_2::variable(b, 1);
        let f = x * y;

        assert_close(f.value, a * b, "value");
        assert_close(f.grad[0], b, "∂(xy)/∂x = y");
        assert_close(f.grad[1], a, "∂(xy)/∂y = x");

        // hess_idx: (0,0)=0, (1,0)=1, (1,1)=2
        assert_close(f.hess[0], 0.0, "∂²(xy)/∂x² = 0");
        assert_close(f.hess[1], 1.0, "∂²(xy)/∂x∂y = 1");
        assert_close(f.hess[2], 0.0, "∂²(xy)/∂y² = 0");

        for (idx, t) in f.tens.iter().enumerate() {
            assert_close(*t, 0.0, &format!("tens[{idx}] of xy"));
        }
    }

    #[test]
    fn mul_x2y_nonzero_third_derivative() {
        // f(x, y) = x² * y computed as (x*x)*y
        // ∂f/∂x = 2xy, ∂f/∂y = x²
        // ∂²f/∂x² = 2y, ∂²f/∂x∂y = 2x, ∂²f/∂y² = 0
        // ∂³f/∂x³ = 0, ∂³f/∂x²∂y = 2, ∂³f/∂x∂y² = 0, ∂³f/∂y³ = 0
        let a = 1.3;
        let b = 0.7;
        let x = J3_2::variable(a, 0);
        let y = J3_2::variable(b, 1);
        let f = (x * x) * y;

        assert_close(f.value, a * a * b, "value");
        assert_close(f.grad[0], 2.0 * a * b, "∂(x²y)/∂x");
        assert_close(f.grad[1], a * a, "∂(x²y)/∂y");

        assert_close(f.hess[0], 2.0 * b, "∂²(x²y)/∂x²");
        assert_close(f.hess[1], 2.0 * a, "∂²(x²y)/∂x∂y");
        assert_close(f.hess[2], 0.0, "∂²(x²y)/∂y²");

        // tens_idx canonical i≥j≥k
        // (0,0,0) → 0: ∂³/∂x³
        assert_close(f.tens[0], 0.0, "∂³(x²y)/∂x³");
        // (1,0,0) → 1: ∂³/∂y∂x²
        assert_close(f.tens[1], 2.0, "∂³(x²y)/∂y∂x² = 2");
        // (1,1,0) → 2: ∂³/∂y²∂x
        assert_close(f.tens[2], 0.0, "∂³(x²y)/∂y²∂x");
        // (1,1,1) → 3: ∂³/∂y³
        assert_close(f.tens[3], 0.0, "∂³(x²y)/∂y³");
    }

    #[test]
    fn mul_x_cubed_third_derivative_is_six() {
        // f(x) = x*x*x = x³
        // ∂³f/∂x³ = 6
        let a = 2.5;
        let x = J3_1::variable(a, 0);
        let f = x * x * x;

        assert_close(f.value, a.powi(3), "value");
        assert_close(f.grad[0], 3.0 * a * a, "∂x³/∂x = 3x²");
        assert_close(f.hess[0], 6.0 * a, "∂²x³/∂x² = 6x");
        assert_close(f.tens[0], 6.0, "∂³x³/∂x³ = 6");
    }

    // ─── Div: quotient rule through order 3 ─────────────────────────────────

    #[test]
    fn div_x_over_y() {
        // f(x, y) = x / y
        // ∂f/∂x = 1/y, ∂f/∂y = -x/y²
        // ∂²f/∂x² = 0, ∂²f/∂x∂y = -1/y², ∂²f/∂y² = 2x/y³
        // ∂³f/∂x³ = 0, ∂³f/∂y∂x² = 0, ∂³f/∂y²∂x = 2/y³, ∂³f/∂y³ = -6x/y⁴
        let a = 3.0;
        let b = 2.0;
        let x = J3_2::variable(a, 0);
        let y = J3_2::variable(b, 1);
        let f = x / y;

        assert_close(f.value, a / b, "value");
        assert_close(f.grad[0], 1.0 / b, "∂(x/y)/∂x");
        assert_close(f.grad[1], -a / (b * b), "∂(x/y)/∂y");

        assert_close(f.hess[0], 0.0, "∂²(x/y)/∂x²");
        assert_close(f.hess[1], -1.0 / (b * b), "∂²(x/y)/∂x∂y");
        assert_close(f.hess[2], 2.0 * a / b.powi(3), "∂²(x/y)/∂y²");

        assert_close(f.tens[0], 0.0, "∂³(x/y)/∂x³");
        assert_close(f.tens[1], 0.0, "∂³(x/y)/∂y∂x²");
        assert_close(f.tens[2], 2.0 / b.powi(3), "∂³(x/y)/∂y²∂x");
        assert_close(f.tens[3], -6.0 * a / b.powi(4), "∂³(x/y)/∂y³");
    }

    #[test]
    fn div_one_over_x() {
        // f(x) = 1/x = constant(1)/x
        // ∂f/∂x = -1/x², ∂²f/∂x² = 2/x³, ∂³f/∂x³ = -6/x⁴
        let a = 1.25;
        let one = J3_1::constant(1.0);
        let x = J3_1::variable(a, 0);
        let f = one / x;

        assert_close(f.value, 1.0 / a, "value");
        assert_close(f.grad[0], -1.0 / (a * a), "∂(1/x)/∂x");
        assert_close(f.hess[0], 2.0 / a.powi(3), "∂²(1/x)/∂x²");
        assert_close(f.tens[0], -6.0 / a.powi(4), "∂³(1/x)/∂x³");
    }

    // ─── Neg ─────────────────────────────────────────────────────────────────

    #[test]
    fn neg_flips_all_derivatives() {
        // f(x) = -(x*x*x) = -x³. ∂³f/∂x³ = -6.
        let a = 1.5;
        let x = J3_1::variable(a, 0);
        let f = -(x * x * x);

        assert_close(f.value, -a.powi(3), "value");
        assert_close(f.grad[0], -3.0 * a * a, "grad");
        assert_close(f.hess[0], -6.0 * a, "hess");
        assert_close(f.tens[0], -6.0, "tens");
    }

    // ─── Mixed f64 arithmetic ────────────────────────────────────────────────

    #[test]
    fn add_f64_preserves_derivatives() {
        // f(x) = x + 5 as Jet3 + f64. Derivatives unchanged from x.
        let a = 0.7;
        let x = J3_1::variable(a, 0);
        let f = x + 5.0;
        assert_close(f.value, a + 5.0, "value");
        assert_close(f.grad[0], 1.0, "grad");
        assert_close(f.hess[0], 0.0, "hess");
        assert_close(f.tens[0], 0.0, "tens");
    }

    #[test]
    fn mul_by_f64_scales_all_derivatives() {
        // f(x) = 3 * x*x*x. ∂³f/∂x³ = 18.
        let a = 1.1;
        let x = J3_1::variable(a, 0);
        let f = (x * x * x) * 3.0;
        assert_close(f.value, 3.0 * a.powi(3), "value");
        assert_close(f.grad[0], 9.0 * a * a, "grad");
        assert_close(f.hess[0], 18.0 * a, "hess");
        assert_close(f.tens[0], 18.0, "tens");
    }

    #[test]
    fn div_by_f64_scales_all_derivatives() {
        // f(x) = (x*x*x) / 2. ∂³f/∂x³ = 3.
        let a = 2.0;
        let x = J3_1::variable(a, 0);
        let f = (x * x * x) / 2.0;
        assert_close(f.value, a.powi(3) / 2.0, "value");
        assert_close(f.grad[0], 3.0 * a * a / 2.0, "grad");
        assert_close(f.hess[0], 6.0 * a / 2.0, "hess");
        assert_close(f.tens[0], 6.0 / 2.0, "tens");
    }

    #[test]
    fn f64_div_jet_matches_closed_form() {
        // f(x) = 2 / x. Using f64 / Jet3.
        // ∂f/∂x = -2/x², ∂²/∂x² = 4/x³, ∂³/∂x³ = -12/x⁴.
        let a = 1.5;
        let x = J3_1::variable(a, 0);
        let f = 2.0 / x;
        assert_close(f.value, 2.0 / a, "value");
        assert_close(f.grad[0], -2.0 / (a * a), "grad");
        assert_close(f.hess[0], 4.0 / a.powi(3), "hess");
        assert_close(f.tens[0], -12.0 / a.powi(4), "tens");
    }

    // ─── AddAssign / SubAssign / MulAssign / DivAssign by f64 ────────────────

    #[test]
    fn assign_ops_match_non_assign() {
        let a = 1.7;
        let mut f = J3_1::variable(a, 0);
        f = f * f * f; // x³
        f += 1.0;
        f *= 2.0;
        f -= 3.0;
        f /= 4.0;
        // f(x) = (2(x³ + 1) - 3) / 4 = (2x³ - 1) / 4
        // ∂/∂x = 6x² / 4 = 3x²/2
        // ∂²/∂x² = 12x / 4 = 3x
        // ∂³/∂x³ = 12 / 4 = 3
        assert_close(f.value, (2.0 * a.powi(3) - 1.0) / 4.0, "value");
        assert_close(f.grad[0], 1.5 * a * a, "grad");
        assert_close(f.hess[0], 3.0 * a, "hess");
        assert_close(f.tens[0], 3.0, "tens");
    }
}
