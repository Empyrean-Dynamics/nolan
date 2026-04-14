use crate::jets::{Jet1, Jet2, Jet3, hess_index, tens_index, tens_size};

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
                    let idx = hess_index(i, j).unwrap();
                    result.hess[idx] =
                        phi_p_val * $self.hess[idx] + phi_pp_val * $self.grad[i] * $self.grad[j];
                }
            }
            for i in 0..N {
                for j in 0..=i {
                    let h_ij = hess_index(i, j).unwrap();
                    for k in 0..=j {
                        let t_idx = tens_index(i, j, k).unwrap();
                        let h_ik = hess_index(i, k).unwrap();
                        let h_jk = hess_index(j, k).unwrap();
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
                let idx = hess_index(i, j).unwrap();
                result.hess[idx] = phi_p * self.hess[idx] + phi_pp * self.grad[i] * self.grad[j];
            }
        }
        for i in 0..N {
            for j in 0..=i {
                let h_ij = hess_index(i, j).unwrap();
                for k in 0..=j {
                    let t_idx = tens_index(i, j, k).unwrap();
                    let h_ik = hess_index(i, k).unwrap();
                    let h_jk = hess_index(j, k).unwrap();
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
                let idx = hess_index(i, j).unwrap();
                result.hess[idx] = phi_p * self.hess[idx] + phi_pp * self.grad[i] * self.grad[j];
            }
        }
        for i in 0..N {
            for j in 0..=i {
                let h_ij = hess_index(i, j).unwrap();
                for k in 0..=j {
                    let t_idx = tens_index(i, j, k).unwrap();
                    let h_ik = hess_index(i, k).unwrap();
                    let h_jk = hess_index(j, k).unwrap();
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
                let idx = hess_index(i, j).unwrap();
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
                let idx = hess_index(i, j).unwrap();
                sin_result.hess[idx] = c * self.hess[idx] + (-s) * self.grad[i] * self.grad[j];
                cos_result.hess[idx] = (-s) * self.hess[idx] + (-c) * self.grad[i] * self.grad[j];
            }
        }
        // sin: φ'=c, φ''=-s, φ'''=-c
        // cos: φ'=-s, φ''=-c, φ'''=s
        for i in 0..N {
            for j in 0..=i {
                let h_ij = hess_index(i, j).unwrap();
                for k in 0..=j {
                    let t_idx = tens_index(i, j, k).unwrap();
                    let h_ik = hess_index(i, k).unwrap();
                    let h_jk = hess_index(j, k).unwrap();
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
