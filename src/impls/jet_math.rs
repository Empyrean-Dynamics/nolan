use crate::jets::{Jet1, Jet2, hess_index};

// ─── Jet1 unary helper macro ────────────────────────────────────

macro_rules! impl_unary_jet1 {
    ($method:ident, $self:ident, $phi:expr, $phi_p:expr) => {
        pub fn $method($self) -> Self {
            let phi_p_val = $phi_p;
            let mut result = Jet1 {
                value: $phi,
                grad: [0.0; N],
            };
            for i in 0..N {
                result.grad[i] = phi_p_val * $self.grad[i];
            }
            result
        }
    };
}

// ─── Jet2 unary helper macro ────────────────────────────────────

macro_rules! impl_unary_jet2 {
    ($method:ident, $self:ident, $phi:expr, $phi_p:expr, $phi_pp:expr) => {
        pub fn $method($self) -> Self {
            let phi_p_val = $phi_p;
            let phi_pp_val = $phi_pp;
            let mut result = Jet2 {
                value: $phi,
                grad: [0.0; N],
                hess: [0.0; H],
            };
            for i in 0..N {
                result.grad[i] = phi_p_val * $self.grad[i];
            }
            for i in 0..N {
                for j in 0..=i {
                    let idx = hess_index(i, j).unwrap();
                    result.hess[idx] =
                        phi_p_val * $self.hess[idx] + phi_pp_val * $self.grad[i] * $self.grad[j];
                }
            }
            result
        }
    };
}

// ─── Jet1 math ──────────────────────────────────────────────────

impl<const N: usize> Jet1<N> {
    impl_unary_jet1!(sin, self, self.value.sin(), self.value.cos());
    impl_unary_jet1!(cos, self, self.value.cos(), -self.value.sin());

    impl_unary_jet1!(tan, self, { self.value.tan() }, {
        let t = self.value.tan();
        1.0 + t * t
    });

    impl_unary_jet1!(asin, self, self.value.asin(), {
        1.0 / (1.0 - self.value * self.value).sqrt()
    });

    impl_unary_jet1!(acos, self, self.value.acos(), {
        -1.0 / (1.0 - self.value * self.value).sqrt()
    });

    impl_unary_jet1!(atan, self, self.value.atan(), {
        1.0 / (1.0 + self.value * self.value)
    });

    impl_unary_jet1!(sinh, self, self.value.sinh(), self.value.cosh());
    impl_unary_jet1!(cosh, self, self.value.cosh(), self.value.sinh());

    impl_unary_jet1!(tanh, self, self.value.tanh(), {
        let t = self.value.tanh();
        1.0 - t * t
    });

    impl_unary_jet1!(exp, self, self.value.exp(), self.value.exp());

    impl_unary_jet1!(ln, self, self.value.ln(), 1.0 / self.value);

    impl_unary_jet1!(sqrt, self, self.value.sqrt(), { 0.5 / self.value.sqrt() });

    impl_unary_jet1!(abs, self, self.value.abs(), {
        if self.value > 0.0 {
            1.0
        } else if self.value < 0.0 {
            -1.0
        } else {
            0.0
        }
    });

    pub fn powi(self, n: i32) -> Self {
        let nf = n as f64;
        let phi = self.value.powi(n);
        let phi_p = nf * self.value.powi(n - 1);
        let mut result = Jet1 {
            value: phi,
            grad: [0.0; N],
        };
        for i in 0..N {
            result.grad[i] = phi_p * self.grad[i];
        }
        result
    }

    pub fn powf(self, n: f64) -> Self {
        let phi = self.value.powf(n);
        let phi_p = n * self.value.powf(n - 1.0);
        let mut result = Jet1 {
            value: phi,
            grad: [0.0; N],
        };
        for i in 0..N {
            result.grad[i] = phi_p * self.grad[i];
        }
        result
    }

    pub fn log(self, base: f64) -> Self {
        self.ln() * (1.0 / base.ln())
    }

    pub fn log10(self) -> Self {
        self.log(10.0)
    }

    pub fn atan2(self, other: Self) -> Self {
        let f = self.value;
        let g = other.value;
        let r2 = f * f + g * g;

        let mut result = Jet1 {
            value: f.atan2(g),
            grad: [0.0; N],
        };
        for i in 0..N {
            result.grad[i] = (g * self.grad[i] - f * other.grad[i]) / r2;
        }
        result
    }

    pub fn sin_cos(self) -> (Self, Self) {
        let s = self.value.sin();
        let c = self.value.cos();

        let mut sin_result = Jet1 {
            value: s,
            grad: [0.0; N],
        };
        let mut cos_result = Jet1 {
            value: c,
            grad: [0.0; N],
        };
        for i in 0..N {
            sin_result.grad[i] = c * self.grad[i];
            cos_result.grad[i] = -s * self.grad[i];
        }
        (sin_result, cos_result)
    }
}

// ─── Jet2 math ──────────────────────────────────────────────────

impl<const N: usize, const H: usize> Jet2<N, H> {
    impl_unary_jet2!(
        sin,
        self,
        self.value.sin(),
        self.value.cos(),
        -self.value.sin()
    );

    impl_unary_jet2!(
        cos,
        self,
        self.value.cos(),
        -self.value.sin(),
        -self.value.cos()
    );

    impl_unary_jet2!(
        tan,
        self,
        { self.value.tan() },
        {
            let t = self.value.tan();
            1.0 + t * t
        },
        {
            let t = self.value.tan();
            let sec2 = 1.0 + t * t;
            2.0 * t * sec2
        }
    );

    impl_unary_jet2!(
        asin,
        self,
        self.value.asin(),
        {
            let d = (1.0 - self.value * self.value).sqrt();
            1.0 / d
        },
        {
            let f = self.value;
            let d = (1.0 - f * f).sqrt();
            f / (d * d * d)
        }
    );

    impl_unary_jet2!(
        acos,
        self,
        self.value.acos(),
        {
            let d = (1.0 - self.value * self.value).sqrt();
            -1.0 / d
        },
        {
            let f = self.value;
            let d = (1.0 - f * f).sqrt();
            -f / (d * d * d)
        }
    );

    impl_unary_jet2!(
        atan,
        self,
        self.value.atan(),
        {
            let d = 1.0 + self.value * self.value;
            1.0 / d
        },
        {
            let f = self.value;
            let d = 1.0 + f * f;
            -2.0 * f / (d * d)
        }
    );

    impl_unary_jet2!(
        sinh,
        self,
        self.value.sinh(),
        self.value.cosh(),
        self.value.sinh()
    );
    impl_unary_jet2!(
        cosh,
        self,
        self.value.cosh(),
        self.value.sinh(),
        self.value.cosh()
    );

    impl_unary_jet2!(
        tanh,
        self,
        self.value.tanh(),
        {
            let t = self.value.tanh();
            1.0 - t * t
        },
        {
            let t = self.value.tanh();
            let sech2 = 1.0 - t * t;
            -2.0 * t * sech2
        }
    );

    impl_unary_jet2!(
        exp,
        self,
        self.value.exp(),
        self.value.exp(),
        self.value.exp()
    );

    impl_unary_jet2!(ln, self, self.value.ln(), { 1.0 / self.value }, {
        -1.0 / (self.value * self.value)
    });

    impl_unary_jet2!(
        sqrt,
        self,
        self.value.sqrt(),
        { 0.5 / self.value.sqrt() },
        {
            let s = self.value.sqrt();
            -0.25 / (s * s * s)
        }
    );

    impl_unary_jet2!(
        abs,
        self,
        self.value.abs(),
        {
            if self.value > 0.0 {
                1.0
            } else if self.value < 0.0 {
                -1.0
            } else {
                0.0
            }
        },
        0.0
    );

    pub fn powi(self, n: i32) -> Self {
        let nf = n as f64;
        let phi = self.value.powi(n);
        let phi_p = nf * self.value.powi(n - 1);
        let phi_pp = nf * (nf - 1.0) * self.value.powi(n - 2);
        let mut result = Jet2 {
            value: phi,
            grad: [0.0; N],
            hess: [0.0; H],
        };
        for i in 0..N {
            result.grad[i] = phi_p * self.grad[i];
        }
        for i in 0..N {
            for j in 0..=i {
                let idx = hess_index(i, j).unwrap();
                result.hess[idx] = phi_p * self.hess[idx] + phi_pp * self.grad[i] * self.grad[j];
            }
        }
        result
    }

    pub fn powf(self, n: f64) -> Self {
        let phi = self.value.powf(n);
        let phi_p = n * self.value.powf(n - 1.0);
        let phi_pp = n * (n - 1.0) * self.value.powf(n - 2.0);
        let mut result = Jet2 {
            value: phi,
            grad: [0.0; N],
            hess: [0.0; H],
        };
        for i in 0..N {
            result.grad[i] = phi_p * self.grad[i];
        }
        for i in 0..N {
            for j in 0..=i {
                let idx = hess_index(i, j).unwrap();
                result.hess[idx] = phi_p * self.hess[idx] + phi_pp * self.grad[i] * self.grad[j];
            }
        }
        result
    }

    pub fn log(self, base: f64) -> Self {
        self.ln() * (1.0 / base.ln())
    }

    pub fn log10(self) -> Self {
        self.log(10.0)
    }

    pub fn atan2(self, other: Self) -> Self {
        let f = self.value;
        let g = other.value;
        let r2 = f * f + g * g;

        let mut result = Jet2 {
            value: f.atan2(g),
            grad: [0.0; N],
            hess: [0.0; H],
        };
        for i in 0..N {
            result.grad[i] = (g * self.grad[i] - f * other.grad[i]) / r2;
        }
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
        result
    }

    pub fn sin_cos(self) -> (Self, Self) {
        let s = self.value.sin();
        let c = self.value.cos();

        let mut sin_result = Jet2 {
            value: s,
            grad: [0.0; N],
            hess: [0.0; H],
        };
        let mut cos_result = Jet2 {
            value: c,
            grad: [0.0; N],
            hess: [0.0; H],
        };
        for i in 0..N {
            sin_result.grad[i] = c * self.grad[i];
            cos_result.grad[i] = -s * self.grad[i];
        }
        for i in 0..N {
            for j in 0..=i {
                let idx = hess_index(i, j).unwrap();
                sin_result.hess[idx] = c * self.hess[idx] + (-s) * self.grad[i] * self.grad[j];
                cos_result.hess[idx] = (-s) * self.hess[idx] + (-c) * self.grad[i] * self.grad[j];
            }
        }
        (sin_result, cos_result)
    }
}
