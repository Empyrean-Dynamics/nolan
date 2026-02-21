use crate::jets::{Jet, MAX_HESS_STORAGE, hess_index};

fn unary<const O: usize, const N: usize>(
    f: &Jet<O, N>,
    phi: f64,
    phi_p: f64,
    phi_pp: f64,
) -> Jet<O, N> {
    let mut result = Jet {
        value: phi,
        grad: [0.0; N],
        hess: [0.0; MAX_HESS_STORAGE],
    };

    if O >= 1 {
        for i in 0..N {
            result.grad[i] = phi_p * f.grad[i];
        }
    }

    if O >= 2 {
        for i in 0..N {
            for j in 0..=i {
                let idx = hess_index(i, j).unwrap();
                result.hess[idx] = phi_p * f.hess[idx] + phi_pp * f.grad[i] * f.grad[j];
            }
        }
    }

    result
}

impl<const O: usize, const N: usize> Jet<O, N> {
    pub fn sin(self) -> Self {
        unary(&self, self.value.sin(), self.value.cos(), -self.value.sin())
    }

    pub fn cos(self) -> Self {
        unary(
            &self,
            self.value.cos(),
            -self.value.sin(),
            -self.value.cos(),
        )
    }

    pub fn tan(self) -> Self {
        let tan_h = self.value.tan();
        let sec2_h = 1.0 + tan_h * tan_h;
        unary(&self, tan_h, sec2_h, 2.0 * tan_h * sec2_h)
    }

    pub fn asin(self) -> Self {
        let f = self.value;
        let d = (1.0 - f * f).sqrt();
        unary(&self, f.asin(), 1.0 / d, f / (d * d * d))
    }

    pub fn acos(self) -> Self {
        let f = self.value;
        let d = (1.0 - f * f).sqrt();
        unary(&self, f.acos(), -1.0 / d, -f / (d * d * d))
    }

    pub fn atan(self) -> Self {
        let f = self.value;
        let d = 1.0 + f * f;
        unary(&self, f.atan(), 1.0 / d, -2.0 * f / (d * d))
    }

    pub fn atan2(self, other: Self) -> Self {
        let f = self.value;
        let g = other.value;
        let r2 = f * f + g * g;

        let mut result = Jet {
            value: f.atan2(g),
            grad: [0.0; N],
            hess: [0.0; MAX_HESS_STORAGE],
        };

        if O >= 1 {
            for i in 0..N {
                result.grad[i] = (g * self.grad[i] - f * other.grad[i]) / r2;
            }
        }

        if O >= 2 {
            for i in 0..N {
                for j in 0..=i {
                    let idx = hess_index(i, j).unwrap();
                    let du_ij = other.grad[j] * self.grad[i] + g * self.hess[idx]
                        - self.grad[j] * other.grad[i]
                        - f * other.hess[idx];
                    let dr2_j = 2.0 * (f * self.grad[j] + g * other.grad[j]);
                    result.hess[idx] = (du_ij - result.grad[i] * dr2_j) / r2;
                }
            }
        }

        result
    }

    pub fn sin_cos(self) -> (Self, Self) {
        let s = self.value.sin();
        let c = self.value.cos();
        (unary(&self, s, c, -s), unary(&self, c, -s, -c))
    }

    pub fn sinh(self) -> Self {
        let sinh_h = self.value.sinh();
        let cosh_h = self.value.cosh();
        unary(&self, sinh_h, cosh_h, sinh_h)
    }

    pub fn cosh(self) -> Self {
        let sinh_h = self.value.sinh();
        let cosh_h = self.value.cosh();
        unary(&self, cosh_h, sinh_h, cosh_h)
    }

    pub fn tanh(self) -> Self {
        let tanh_h = self.value.tanh();
        let sech2_h = 1.0 - tanh_h * tanh_h;
        unary(&self, tanh_h, sech2_h, -2.0 * tanh_h * sech2_h)
    }

    pub fn exp(self) -> Self {
        let e = self.value.exp();
        unary(&self, e, e, e)
    }

    pub fn log(self, base: f64) -> Self {
        self.ln() * (1.0 / base.ln())
    }

    pub fn log10(self) -> Self {
        self.log(10.0)
    }

    pub fn ln(self) -> Self {
        let f = self.value;
        unary(&self, f.ln(), 1.0 / f, -1.0 / (f * f))
    }

    pub fn sqrt(self) -> Self {
        let s = self.value.sqrt();
        unary(&self, s, 0.5 / s, -0.25 / (s * s * s))
    }

    pub fn powi(self, n: i32) -> Self {
        let nf = n as f64;
        unary(
            &self,
            self.value.powi(n),
            nf * self.value.powi(n - 1),
            nf * (nf - 1.0) * self.value.powi(n - 2),
        )
    }

    pub fn powf(self, n: f64) -> Self {
        unary(
            &self,
            self.value.powf(n),
            n * self.value.powf(n - 1.0),
            n * (n - 1.0) * self.value.powf(n - 2.0),
        )
    }
}
