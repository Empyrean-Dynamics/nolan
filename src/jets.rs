pub const MAX_HESS_STORAGE: usize = 256;

/// Get the linear index of a Hessian matrix stored in lower triangular form.
pub fn hess_index(i: usize, j: usize) -> Option<usize> {
    // Lower triangular form
    // i,j
    // 0,0
    // 1,0  1,1
    // 2,0, 2,1, 2,2
    // 3,0, 3,1, 3,2, 3,3
    // ->   0      1      2      3      4      5      6      7      8      9
    // -> (0,0), (1,0), (1,1), (2,0), (2,1), (2,2), (3,0), (3,1), (3,2), (3,3)
    if i < j {
        hess_index(j, i)
    } else {
        Some(i * (i + 1) / 2 + j)
    }
}

#[derive(Clone, Copy)]
pub struct Jet<const ORDER: usize, const N: usize> {
    /// Function value f.
    pub value: f64,
    /// First derivatives `∂f/∂p_i`.
    pub grad: [f64; N],
    /// Second derivatives `∂²f/(∂p_i ∂p_j)` in lower triangular storage.
    pub hess: [f64; MAX_HESS_STORAGE],
}

impl<const O: usize, const N: usize> Jet<O, N> {
    /// Constant (all derivatives zero). Available for any ORDER.
    pub fn constant(value: f64) -> Self {
        Self {
            value,
            grad: [0.0; N],
            hess: [0.0; MAX_HESS_STORAGE],
        }
    }

    /// Variable seeded at `param_idx` (unit gradient there). Requires O ≥ 1.
    pub fn variable(value: f64, param_idx: usize) -> Self {
        let mut jet = Self::constant(value);
        jet.grad[param_idx] = 1.0;
        jet
    }
}

pub type Jet1<const N: usize> = Jet<1, N>;
pub type Jet2<const N: usize> = Jet<2, N>;
pub type Dual = Jet1<1>;
pub type HyperDual = Jet2<2>;
