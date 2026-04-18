use crate::jets::{Jet1, Jet2, Jet3, hess_idx, tens_idx, tens_size};

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
                    let idx = hess_idx(i, j);
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

    pub fn exp(self) -> Self {
        let e = self.value.exp();
        let mut result = Jet1 {
            value: e,
            grad: [0.0; N],
        };
        for i in 0..N {
            result.grad[i] = e * self.grad[i];
        }
        result
    }

    impl_unary_jet1!(ln, self, self.value.ln(), 1.0 / self.value);

    pub fn sqrt(self) -> Self {
        let s = self.value.sqrt();
        let phi_p = 0.5 / s;
        let mut result = Jet1 {
            value: s,
            grad: [0.0; N],
        };
        for i in 0..N {
            result.grad[i] = phi_p * self.grad[i];
        }
        result
    }

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
    pub fn sin(self) -> Self {
        let (s, c) = self.value.sin_cos();
        let phi_p = c;
        let phi_pp = -s;
        let mut result = Jet2 {
            value: s,
            grad: [0.0; N],
            hess: [0.0; H],
        };
        for i in 0..N {
            result.grad[i] = phi_p * self.grad[i];
        }
        for i in 0..N {
            for j in 0..=i {
                let idx = hess_idx(i, j);
                result.hess[idx] = phi_p * self.hess[idx] + phi_pp * self.grad[i] * self.grad[j];
            }
        }
        result
    }

    pub fn cos(self) -> Self {
        let (s, c) = self.value.sin_cos();
        let phi_p = -s;
        let phi_pp = -c;
        let mut result = Jet2 {
            value: c,
            grad: [0.0; N],
            hess: [0.0; H],
        };
        for i in 0..N {
            result.grad[i] = phi_p * self.grad[i];
        }
        for i in 0..N {
            for j in 0..=i {
                let idx = hess_idx(i, j);
                result.hess[idx] = phi_p * self.hess[idx] + phi_pp * self.grad[i] * self.grad[j];
            }
        }
        result
    }

    pub fn tan(self) -> Self {
        let t = self.value.tan();
        let sec2 = 1.0 + t * t;
        let phi_p = sec2;
        let phi_pp = 2.0 * t * sec2;
        let mut result = Jet2 {
            value: t,
            grad: [0.0; N],
            hess: [0.0; H],
        };
        for i in 0..N {
            result.grad[i] = phi_p * self.grad[i];
        }
        for i in 0..N {
            for j in 0..=i {
                let idx = hess_idx(i, j);
                result.hess[idx] = phi_p * self.hess[idx] + phi_pp * self.grad[i] * self.grad[j];
            }
        }
        result
    }

    pub fn asin(self) -> Self {
        let f = self.value;
        let d = (1.0 - f * f).sqrt();
        let phi_p = 1.0 / d;
        let phi_pp = f / (d * d * d);
        let mut result = Jet2 {
            value: f.asin(),
            grad: [0.0; N],
            hess: [0.0; H],
        };
        for i in 0..N {
            result.grad[i] = phi_p * self.grad[i];
        }
        for i in 0..N {
            for j in 0..=i {
                let idx = hess_idx(i, j);
                result.hess[idx] = phi_p * self.hess[idx] + phi_pp * self.grad[i] * self.grad[j];
            }
        }
        result
    }

    pub fn acos(self) -> Self {
        let f = self.value;
        let d = (1.0 - f * f).sqrt();
        let phi_p = -1.0 / d;
        let phi_pp = -f / (d * d * d);
        let mut result = Jet2 {
            value: f.acos(),
            grad: [0.0; N],
            hess: [0.0; H],
        };
        for i in 0..N {
            result.grad[i] = phi_p * self.grad[i];
        }
        for i in 0..N {
            for j in 0..=i {
                let idx = hess_idx(i, j);
                result.hess[idx] = phi_p * self.hess[idx] + phi_pp * self.grad[i] * self.grad[j];
            }
        }
        result
    }

    pub fn atan(self) -> Self {
        let f = self.value;
        let d = 1.0 + f * f;
        let phi_p = 1.0 / d;
        let phi_pp = -2.0 * f / (d * d);
        let mut result = Jet2 {
            value: f.atan(),
            grad: [0.0; N],
            hess: [0.0; H],
        };
        for i in 0..N {
            result.grad[i] = phi_p * self.grad[i];
        }
        for i in 0..N {
            for j in 0..=i {
                let idx = hess_idx(i, j);
                result.hess[idx] = phi_p * self.hess[idx] + phi_pp * self.grad[i] * self.grad[j];
            }
        }
        result
    }

    pub fn sinh(self) -> Self {
        let sh = self.value.sinh();
        let ch = self.value.cosh();
        let mut result = Jet2 {
            value: sh,
            grad: [0.0; N],
            hess: [0.0; H],
        };
        for i in 0..N {
            result.grad[i] = ch * self.grad[i];
        }
        for i in 0..N {
            for j in 0..=i {
                let idx = hess_idx(i, j);
                result.hess[idx] = ch * self.hess[idx] + sh * self.grad[i] * self.grad[j];
            }
        }
        result
    }

    pub fn cosh(self) -> Self {
        let sh = self.value.sinh();
        let ch = self.value.cosh();
        let mut result = Jet2 {
            value: ch,
            grad: [0.0; N],
            hess: [0.0; H],
        };
        for i in 0..N {
            result.grad[i] = sh * self.grad[i];
        }
        for i in 0..N {
            for j in 0..=i {
                let idx = hess_idx(i, j);
                result.hess[idx] = sh * self.hess[idx] + ch * self.grad[i] * self.grad[j];
            }
        }
        result
    }

    pub fn tanh(self) -> Self {
        let t = self.value.tanh();
        let sech2 = 1.0 - t * t;
        let phi_pp = -2.0 * t * sech2;
        let mut result = Jet2 {
            value: t,
            grad: [0.0; N],
            hess: [0.0; H],
        };
        for i in 0..N {
            result.grad[i] = sech2 * self.grad[i];
        }
        for i in 0..N {
            for j in 0..=i {
                let idx = hess_idx(i, j);
                result.hess[idx] = sech2 * self.hess[idx] + phi_pp * self.grad[i] * self.grad[j];
            }
        }
        result
    }

    pub fn exp(self) -> Self {
        let e = self.value.exp();
        let mut result = Jet2 {
            value: e,
            grad: [0.0; N],
            hess: [0.0; H],
        };
        for i in 0..N {
            result.grad[i] = e * self.grad[i];
        }
        for i in 0..N {
            for j in 0..=i {
                let idx = hess_idx(i, j);
                result.hess[idx] = e * self.hess[idx] + e * self.grad[i] * self.grad[j];
            }
        }
        result
    }

    pub fn ln(self) -> Self {
        let v = self.value;
        let phi_p = 1.0 / v;
        let phi_pp = -1.0 / (v * v);
        let mut result = Jet2 {
            value: v.ln(),
            grad: [0.0; N],
            hess: [0.0; H],
        };
        for i in 0..N {
            result.grad[i] = phi_p * self.grad[i];
        }
        for i in 0..N {
            for j in 0..=i {
                let idx = hess_idx(i, j);
                result.hess[idx] = phi_p * self.hess[idx] + phi_pp * self.grad[i] * self.grad[j];
            }
        }
        result
    }

    pub fn sqrt(self) -> Self {
        let s = self.value.sqrt();
        let phi_p = 0.5 / s;
        let phi_pp = -0.25 / (s * s * s);
        let mut result = Jet2 {
            value: s,
            grad: [0.0; N],
            hess: [0.0; H],
        };
        for i in 0..N {
            result.grad[i] = phi_p * self.grad[i];
        }
        for i in 0..N {
            for j in 0..=i {
                let idx = hess_idx(i, j);
                result.hess[idx] = phi_p * self.hess[idx] + phi_pp * self.grad[i] * self.grad[j];
            }
        }
        result
    }

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
        let v = self.value;
        let phi = v.powi(n);
        let phi_p = nf * v.powi(n - 1);
        let phi_pp = nf * (nf - 1.0) * v.powi(n - 2);
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
                let idx = hess_idx(i, j);
                result.hess[idx] = phi_p * self.hess[idx] + phi_pp * self.grad[i] * self.grad[j];
            }
        }
        result
    }

    pub fn powf(self, n: f64) -> Self {
        let v = self.value;
        let phi = v.powf(n);
        let phi_p = n * v.powf(n - 1.0);
        let phi_pp = n * (n - 1.0) * v.powf(n - 2.0);
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
                let idx = hess_idx(i, j);
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
                let idx = hess_idx(i, j);
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
                let idx = hess_idx(i, j);
                sin_result.hess[idx] = c * self.hess[idx] + (-s) * self.grad[i] * self.grad[j];
                cos_result.hess[idx] = (-s) * self.hess[idx] + (-c) * self.grad[i] * self.grad[j];
            }
        }
        (sin_result, cos_result)
    }
}

// ─── Jet3 unary helper macro ────────────────────────────────────

/// Third-order chain rule for \\( \phi(x) \\):
/// \\[
///   \frac{\partial^3 \phi}{\partial p_i \partial p_j \partial p_k}
///   = \phi' \frac{\partial^3 x}{\partial p_i \partial p_j \partial p_k}
///   + \phi'' \left[
///       \frac{\partial x}{\partial p_i} \frac{\partial^2 x}{\partial p_j \partial p_k}
///     + \frac{\partial x}{\partial p_j} \frac{\partial^2 x}{\partial p_i \partial p_k}
///     + \frac{\partial x}{\partial p_k} \frac{\partial^2 x}{\partial p_i \partial p_j}
///   \right]
///   + \phi''' \frac{\partial x}{\partial p_i} \frac{\partial x}{\partial p_j} \frac{\partial x}{\partial p_k}
/// \\]
macro_rules! impl_unary_jet3 {
    ($method:ident, $self:ident, $phi:expr, $phi_p:expr, $phi_pp:expr, $phi_ppp:expr) => {
        pub fn $method($self) -> Self {
            let phi_p_val = $phi_p;
            let phi_pp_val = $phi_pp;
            let phi_ppp_val = $phi_ppp;
            let mut result = Jet3 {
                value: $phi,
                grad: [0.0; N],
                hess: [0.0; H],
                tens: [0.0; T],
            };
            for i in 0..N {
                result.grad[i] = phi_p_val * $self.grad[i];
            }
            for i in 0..N {
                for j in 0..=i {
                    let idx = hess_idx(i, j);
                    result.hess[idx] =
                        phi_p_val * $self.hess[idx] + phi_pp_val * $self.grad[i] * $self.grad[j];
                }
            }
            for i in 0..N {
                for j in 0..=i {
                    let h_ij = hess_idx(i, j);
                    for k in 0..=j {
                        let t_idx = tens_idx(i, j, k);
                        let h_ik = hess_idx(i, k);
                        let h_jk = hess_idx(j, k);
                        result.tens[t_idx] = phi_p_val * $self.tens[t_idx]
                            + phi_pp_val
                                * ($self.grad[i] * $self.hess[h_jk]
                                    + $self.grad[j] * $self.hess[h_ik]
                                    + $self.grad[k] * $self.hess[h_ij])
                            + phi_ppp_val * $self.grad[i] * $self.grad[j] * $self.grad[k];
                    }
                }
            }
            result
        }
    };
}

// ─── Jet3 math ──────────────────────────────────────────────────

impl<const N: usize, const H: usize, const T: usize> Jet3<N, H, T> {
    // sin: φ=sin, φ'=cos, φ''=-sin, φ'''=-cos
    impl_unary_jet3!(
        sin,
        self,
        self.value.sin(),
        self.value.cos(),
        -self.value.sin(),
        -self.value.cos()
    );

    // cos: φ=cos, φ'=-sin, φ''=-cos, φ'''=sin
    impl_unary_jet3!(
        cos,
        self,
        self.value.cos(),
        -self.value.sin(),
        -self.value.cos(),
        self.value.sin()
    );

    impl_unary_jet3!(
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
        },
        {
            let t = self.value.tan();
            let sec2 = 1.0 + t * t;
            2.0 * sec2 * (1.0 + 3.0 * t * t)
        }
    );

    impl_unary_jet3!(
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
        },
        {
            let f = self.value;
            let d2 = 1.0 - f * f;
            let d = d2.sqrt();
            (2.0 * f * f + 1.0) / (d * d * d * d * d)
        }
    );

    impl_unary_jet3!(
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
        },
        {
            let f = self.value;
            let d2 = 1.0 - f * f;
            let d = d2.sqrt();
            -(2.0 * f * f + 1.0) / (d * d * d * d * d)
        }
    );

    impl_unary_jet3!(
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
        },
        {
            let f = self.value;
            let d = 1.0 + f * f;
            (6.0 * f * f - 2.0) / (d * d * d)
        }
    );

    // sinh: φ'=cosh, φ''=sinh, φ'''=cosh
    impl_unary_jet3!(
        sinh,
        self,
        self.value.sinh(),
        self.value.cosh(),
        self.value.sinh(),
        self.value.cosh()
    );

    // cosh: φ'=sinh, φ''=cosh, φ'''=sinh
    impl_unary_jet3!(
        cosh,
        self,
        self.value.cosh(),
        self.value.sinh(),
        self.value.cosh(),
        self.value.sinh()
    );

    impl_unary_jet3!(
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
        },
        {
            let t = self.value.tanh();
            let sech2 = 1.0 - t * t;
            2.0 * sech2 * (3.0 * t * t - 1.0)
        }
    );

    // exp: all derivatives are exp(x)
    impl_unary_jet3!(
        exp,
        self,
        self.value.exp(),
        self.value.exp(),
        self.value.exp(),
        self.value.exp()
    );

    // ln: φ'=1/x, φ''=-1/x², φ'''=2/x³
    impl_unary_jet3!(
        ln,
        self,
        self.value.ln(),
        { 1.0 / self.value },
        { -1.0 / (self.value * self.value) },
        { 2.0 / (self.value * self.value * self.value) }
    );

    // sqrt: φ'=1/(2√x), φ''=-1/(4x^(3/2)), φ'''=3/(8x^(5/2))
    impl_unary_jet3!(
        sqrt,
        self,
        self.value.sqrt(),
        { 0.5 / self.value.sqrt() },
        {
            let s = self.value.sqrt();
            -0.25 / (s * s * s)
        },
        {
            let s = self.value.sqrt();
            0.375 / (s * s * s * s * s)
        }
    );

    impl_unary_jet3!(
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
        0.0,
        0.0
    );

    pub fn powi(self, n: i32) -> Self {
        let nf = n as f64;
        let phi = self.value.powi(n);
        let phi_p = nf * self.value.powi(n - 1);
        let phi_pp = nf * (nf - 1.0) * self.value.powi(n - 2);
        let phi_ppp = nf * (nf - 1.0) * (nf - 2.0) * self.value.powi(n - 3);
        let mut result = Jet3 {
            value: phi,
            grad: [0.0; N],
            hess: [0.0; H],
            tens: [0.0; T],
        };
        for i in 0..N {
            result.grad[i] = phi_p * self.grad[i];
        }
        for i in 0..N {
            for j in 0..=i {
                let idx = hess_idx(i, j);
                result.hess[idx] = phi_p * self.hess[idx] + phi_pp * self.grad[i] * self.grad[j];
            }
        }
        for i in 0..N {
            for j in 0..=i {
                let h_ij = hess_idx(i, j);
                for k in 0..=j {
                    let t_idx = tens_idx(i, j, k);
                    let h_ik = hess_idx(i, k);
                    let h_jk = hess_idx(j, k);
                    result.tens[t_idx] = phi_p * self.tens[t_idx]
                        + phi_pp
                            * (self.grad[i] * self.hess[h_jk]
                                + self.grad[j] * self.hess[h_ik]
                                + self.grad[k] * self.hess[h_ij])
                        + phi_ppp * self.grad[i] * self.grad[j] * self.grad[k];
                }
            }
        }
        result
    }

    pub fn powf(self, n: f64) -> Self {
        let phi = self.value.powf(n);
        let phi_p = n * self.value.powf(n - 1.0);
        let phi_pp = n * (n - 1.0) * self.value.powf(n - 2.0);
        let phi_ppp = n * (n - 1.0) * (n - 2.0) * self.value.powf(n - 3.0);
        let mut result = Jet3 {
            value: phi,
            grad: [0.0; N],
            hess: [0.0; H],
            tens: [0.0; T],
        };
        for i in 0..N {
            result.grad[i] = phi_p * self.grad[i];
        }
        for i in 0..N {
            for j in 0..=i {
                let idx = hess_idx(i, j);
                result.hess[idx] = phi_p * self.hess[idx] + phi_pp * self.grad[i] * self.grad[j];
            }
        }
        for i in 0..N {
            for j in 0..=i {
                let h_ij = hess_idx(i, j);
                for k in 0..=j {
                    let t_idx = tens_idx(i, j, k);
                    let h_ik = hess_idx(i, k);
                    let h_jk = hess_idx(j, k);
                    result.tens[t_idx] = phi_p * self.tens[t_idx]
                        + phi_pp
                            * (self.grad[i] * self.hess[h_jk]
                                + self.grad[j] * self.hess[h_ik]
                                + self.grad[k] * self.hess[h_ij])
                        + phi_ppp * self.grad[i] * self.grad[j] * self.grad[k];
                }
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

        let mut result = Jet3 {
            value: f.atan2(g),
            grad: [0.0; N],
            hess: [0.0; H],
            tens: [0.0; T],
        };
        for i in 0..N {
            result.grad[i] = (g * self.grad[i] - f * other.grad[i]) / r2;
        }
        for i in 0..N {
            for j in 0..=i {
                let idx = hess_idx(i, j);
                let du_ij = other.grad[j] * self.grad[i] + g * self.hess[idx]
                    - self.grad[j] * other.grad[i]
                    - f * other.hess[idx];
                let dr2_j = 2.0 * (f * self.grad[j] + g * other.grad[j]);
                result.hess[idx] = (du_ij - result.grad[i] * dr2_j) / r2;
            }
        }
        // Third-order via atan(y/x) — the quadrant correction is constant
        // so its third derivatives are zero.
        let ratio = self / other;
        let atan_ratio = ratio.atan();
        for i in 0..tens_size(N) {
            result.tens[i] = atan_ratio.tens[i];
        }
        result
    }

    pub fn sin_cos(self) -> (Self, Self) {
        let s = self.value.sin();
        let c = self.value.cos();

        let mut sin_result = Jet3 {
            value: s,
            grad: [0.0; N],
            hess: [0.0; H],
            tens: [0.0; T],
        };
        let mut cos_result = Jet3 {
            value: c,
            grad: [0.0; N],
            hess: [0.0; H],
            tens: [0.0; T],
        };
        for i in 0..N {
            sin_result.grad[i] = c * self.grad[i];
            cos_result.grad[i] = -s * self.grad[i];
        }
        for i in 0..N {
            for j in 0..=i {
                let idx = hess_idx(i, j);
                sin_result.hess[idx] = c * self.hess[idx] + (-s) * self.grad[i] * self.grad[j];
                cos_result.hess[idx] = (-s) * self.hess[idx] + (-c) * self.grad[i] * self.grad[j];
            }
        }
        // sin: φ'=c, φ''=-s, φ'''=-c
        // cos: φ'=-s, φ''=-c, φ'''=s
        for i in 0..N {
            for j in 0..=i {
                let h_ij = hess_idx(i, j);
                for k in 0..=j {
                    let t_idx = tens_idx(i, j, k);
                    let h_ik = hess_idx(i, k);
                    let h_jk = hess_idx(j, k);
                    let gi_hess_jk = self.grad[i] * self.hess[h_jk]
                        + self.grad[j] * self.hess[h_ik]
                        + self.grad[k] * self.hess[h_ij];
                    let gi_gj_gk = self.grad[i] * self.grad[j] * self.grad[k];
                    sin_result.tens[t_idx] =
                        c * self.tens[t_idx] + (-s) * gi_hess_jk + (-c) * gi_gj_gk;
                    cos_result.tens[t_idx] =
                        (-s) * self.tens[t_idx] + (-c) * gi_hess_jk + s * gi_gj_gk;
                }
            }
        }
        (sin_result, cos_result)
    }
}

#[cfg(test)]
mod jet3_math_tests {
    use super::*;
    use crate::jets::{hess_size, tens_size};

    const TOL: f64 = 1e-10;

    type J3_1 = Jet3<1, { hess_size(1) }, { tens_size(1) }>;
    type J3_2 = Jet3<2, { hess_size(2) }, { tens_size(2) }>;

    fn assert_close(actual: f64, expected: f64, what: &str) {
        assert!(
            (actual - expected).abs() <= TOL * expected.abs().max(1.0),
            "{}: expected {}, got {} (|diff| = {:e})",
            what,
            expected,
            actual,
            (actual - expected).abs()
        );
    }

    /// Evaluate a unary op on a single-variable Jet3 seeded at `x_val`
    /// and verify value + grad + hess + tens match the closed-form.
    fn check_unary(
        x_val: f64,
        apply: impl Fn(J3_1) -> J3_1,
        expected_f: f64,
        expected_fp: f64,
        expected_fpp: f64,
        expected_fppp: f64,
        name: &str,
    ) {
        let x = J3_1::variable(x_val, 0);
        let f = apply(x);
        assert_close(f.value, expected_f, &format!("{name}.value"));
        assert_close(f.grad[0], expected_fp, &format!("{name}.grad"));
        assert_close(f.hess[0], expected_fpp, &format!("{name}.hess"));
        assert_close(f.tens[0], expected_fppp, &format!("{name}.tens"));
    }

    // ─── Trig ────────────────────────────────────────────────────────────────

    #[test]
    fn sin_derivatives() {
        let a = 0.7;
        check_unary(a, |x| x.sin(), a.sin(), a.cos(), -a.sin(), -a.cos(), "sin");
    }

    #[test]
    fn cos_derivatives() {
        let a = -0.4;
        check_unary(a, |x| x.cos(), a.cos(), -a.sin(), -a.cos(), a.sin(), "cos");
    }

    #[test]
    fn tan_derivatives() {
        // tan: f' = sec², f'' = 2 tan sec², f''' = 2 sec² (1 + 3 tan²)
        let a: f64 = 0.3;
        let t = a.tan();
        let sec2 = 1.0 + t * t;
        check_unary(
            a,
            |x| x.tan(),
            t,
            sec2,
            2.0 * t * sec2,
            2.0 * sec2 * (1.0 + 3.0 * t * t),
            "tan",
        );
    }

    #[test]
    fn asin_derivatives() {
        // asin: f' = 1/√(1-x²), f'' = x/(1-x²)^(3/2), f''' = (2x²+1)/(1-x²)^(5/2)
        let a: f64 = 0.4;
        let d2 = 1.0 - a * a;
        let d = d2.sqrt();
        check_unary(
            a,
            |x| x.asin(),
            a.asin(),
            1.0 / d,
            a / (d * d * d),
            (2.0 * a * a + 1.0) / d.powi(5),
            "asin",
        );
    }

    #[test]
    fn acos_derivatives() {
        let a: f64 = 0.4;
        let d2 = 1.0 - a * a;
        let d = d2.sqrt();
        check_unary(
            a,
            |x| x.acos(),
            a.acos(),
            -1.0 / d,
            -a / (d * d * d),
            -(2.0 * a * a + 1.0) / d.powi(5),
            "acos",
        );
    }

    #[test]
    fn atan_derivatives() {
        // atan: f' = 1/(1+x²), f'' = -2x/(1+x²)², f''' = (6x²-2)/(1+x²)³
        let a = 0.7;
        let d = 1.0 + a * a;
        check_unary(
            a,
            |x| x.atan(),
            a.atan(),
            1.0 / d,
            -2.0 * a / (d * d),
            (6.0 * a * a - 2.0) / d.powi(3),
            "atan",
        );
    }

    // ─── Hyperbolic ──────────────────────────────────────────────────────────

    #[test]
    fn sinh_derivatives() {
        let a = 0.5;
        check_unary(
            a,
            |x| x.sinh(),
            a.sinh(),
            a.cosh(),
            a.sinh(),
            a.cosh(),
            "sinh",
        );
    }

    #[test]
    fn cosh_derivatives() {
        let a = 0.5;
        check_unary(
            a,
            |x| x.cosh(),
            a.cosh(),
            a.sinh(),
            a.cosh(),
            a.sinh(),
            "cosh",
        );
    }

    #[test]
    fn tanh_derivatives() {
        // tanh: f' = sech² = 1-t², f'' = -2t sech², f''' = 2 sech² (3t²-1)
        let a: f64 = 0.4;
        let t = a.tanh();
        let sech2 = 1.0 - t * t;
        check_unary(
            a,
            |x| x.tanh(),
            t,
            sech2,
            -2.0 * t * sech2,
            2.0 * sech2 * (3.0 * t * t - 1.0),
            "tanh",
        );
    }

    // ─── Exp / Log / Sqrt / Pow ──────────────────────────────────────────────

    #[test]
    fn exp_derivatives() {
        let a: f64 = 0.6;
        let e = a.exp();
        check_unary(a, |x| x.exp(), e, e, e, e, "exp");
    }

    #[test]
    fn ln_derivatives() {
        // ln: f'=1/x, f''=-1/x², f'''=2/x³
        let a = 1.7;
        check_unary(
            a,
            |x| x.ln(),
            a.ln(),
            1.0 / a,
            -1.0 / (a * a),
            2.0 / (a * a * a),
            "ln",
        );
    }

    #[test]
    fn sqrt_derivatives() {
        // sqrt: f'=1/(2√x), f''=-1/(4x^(3/2)), f'''=3/(8x^(5/2))
        let a: f64 = 2.25;
        let s = a.sqrt();
        check_unary(
            a,
            |x| x.sqrt(),
            s,
            0.5 / s,
            -0.25 / (s * s * s),
            0.375 / s.powi(5),
            "sqrt",
        );
    }

    #[test]
    fn powi_derivatives() {
        // x^4: f'=4x³, f''=12x², f'''=24x
        let a = 1.5;
        check_unary(
            a,
            |x| x.powi(4),
            a.powi(4),
            4.0 * a.powi(3),
            12.0 * a * a,
            24.0 * a,
            "x^4",
        );
    }

    #[test]
    fn powf_derivatives() {
        // x^2.5: f'=2.5·x^1.5, f''=3.75·x^0.5, f'''=1.875/x^0.5
        let a = 2.0;
        check_unary(
            a,
            |x| x.powf(2.5),
            a.powf(2.5),
            2.5 * a.powf(1.5),
            3.75 * a.powf(0.5),
            1.875 / a.powf(0.5),
            "x^2.5",
        );
    }

    #[test]
    fn abs_positive_branch() {
        let a = 2.3;
        check_unary(a, |x| x.abs(), a, 1.0, 0.0, 0.0, "|x| at x>0");
    }

    #[test]
    fn abs_negative_branch() {
        let a = -1.7;
        check_unary(a, |x| x.abs(), -a, -1.0, 0.0, 0.0, "|x| at x<0");
    }

    #[test]
    fn atan2_matches_atan_of_ratio() {
        // For x > 0, atan2(y, x) = atan(y/x); verify through order 3.
        let y_val = 0.7;
        let x_val = 1.3;
        let y = J3_2::variable(y_val, 0);
        let x = J3_2::variable(x_val, 1);
        let f = y.atan2(x);
        let ref_ = (y / x).atan();
        assert_close(f.value, ref_.value, "atan2.value");
        for i in 0..2 {
            assert_close(f.grad[i], ref_.grad[i], &format!("atan2.grad[{i}]"));
        }
        for i in 0..hess_size(2) {
            assert_close(f.hess[i], ref_.hess[i], &format!("atan2.hess[{i}]"));
        }
        for i in 0..tens_size(2) {
            assert_close(f.tens[i], ref_.tens[i], &format!("atan2.tens[{i}]"));
        }
    }

    // ─── Chain rule through Mul composition ──────────────────────────────────

    #[test]
    fn sin_of_xy_multivariable_chain_rule() {
        // f(x,y) = sin(xy). With c = cos(xy), s = sin(xy):
        //   ∂/∂x = c·y      ∂/∂y = c·x
        //   ∂²/∂x² = -s·y²  ∂²/∂x∂y = -s·xy + c   ∂²/∂y² = -s·x²
        //   ∂³/∂x³   = -c·y³
        //   ∂³/∂y∂x² = -c·x·y² - 2s·y
        //   ∂³/∂y²∂x = -c·x²·y - 2s·x
        //   ∂³/∂y³   = -c·x³
        let a = 0.8;
        let b = 1.1;
        let x = J3_2::variable(a, 0);
        let y = J3_2::variable(b, 1);
        let f = (x * y).sin();
        let c = (a * b).cos();
        let s = (a * b).sin();

        assert_close(f.value, (a * b).sin(), "value");
        assert_close(f.grad[0], c * b, "∂/∂x");
        assert_close(f.grad[1], c * a, "∂/∂y");
        assert_close(f.hess[0], -s * b * b, "∂²/∂x²");
        assert_close(f.hess[1], -s * a * b + c, "∂²/∂x∂y");
        assert_close(f.hess[2], -s * a * a, "∂²/∂y²");
        assert_close(f.tens[0], -c * b.powi(3), "∂³/∂x³");
        assert_close(f.tens[1], -c * a * b * b - 2.0 * s * b, "∂³/∂y∂x²");
        assert_close(f.tens[2], -c * a * a * b - 2.0 * s * a, "∂³/∂y²∂x");
        assert_close(f.tens[3], -c * a.powi(3), "∂³/∂y³");
    }

    #[test]
    fn gaussian_exp_neg_half_x_squared() {
        // f(x) = exp(-x²/2): f' = -x·f, f'' = (x²-1)·f, f''' = x(3-x²)·f
        let a = 0.5;
        let x = J3_1::variable(a, 0);
        let f = (-(x * x) * 0.5).exp();
        let g = (-a * a / 2.0).exp();
        assert_close(f.value, g, "value");
        assert_close(f.grad[0], -a * g, "grad");
        assert_close(f.hess[0], (a * a - 1.0) * g, "hess");
        assert_close(f.tens[0], a * (3.0 - a * a) * g, "tens");
    }

    #[test]
    fn sin_cos_matches_individual_calls() {
        let a = 0.65;
        let x = J3_1::variable(a, 0);
        let (s_pair, c_pair) = x.sin_cos();
        let s_solo = x.sin();
        let c_solo = x.cos();
        assert_close(s_pair.value, s_solo.value, "sin value");
        assert_close(s_pair.grad[0], s_solo.grad[0], "sin grad");
        assert_close(s_pair.hess[0], s_solo.hess[0], "sin hess");
        assert_close(s_pair.tens[0], s_solo.tens[0], "sin tens");
        assert_close(c_pair.value, c_solo.value, "cos value");
        assert_close(c_pair.grad[0], c_solo.grad[0], "cos grad");
        assert_close(c_pair.hess[0], c_solo.hess[0], "cos hess");
        assert_close(c_pair.tens[0], c_solo.tens[0], "cos tens");
    }
}
