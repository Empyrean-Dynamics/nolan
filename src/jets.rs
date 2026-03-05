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

/// Calculate the size of the flattened hessian matrix given
/// n parameters.
pub const fn hess_size(n: usize) -> usize {
    n * (n + 1) / 2
}

/// First-order jet: value + gradient. No hessian storage.
///
/// For \\( N = 6 \\): 56 bytes (1 `f64` value + 6 `f64` gradient).
#[derive(Clone, Copy)]
pub struct Jet1<const N: usize> {
    /// Function value \\( f \\).
    pub value: f64,
    /// First derivatives \\( \partial f / \partial p_i \\).
    pub grad: [f64; N],
}

/// Second-order jet: value + gradient + hessian.
///
/// `H` must equal [`hess_size(N)`] = \\( N (N+1) / 2 \\).
/// Use `Jet2::<N, { hess_size(N) }>::variable(...)` for correctly-sized instances.
///
/// For \\( N = 6, H = 21 \\): 224 bytes (1 + 6 + 21 `f64`s).
#[derive(Clone, Copy)]
pub struct Jet2<const N: usize, const H: usize> {
    /// Function value \\( f \\).
    pub value: f64,
    /// First derivatives \\( \partial f / \partial p_i \\).
    pub grad: [f64; N],
    /// Second derivatives \\( \partial^2 f / \partial p_i \partial p_j \\),
    /// stored in lower-triangular form.
    pub hess: [f64; H],
}

impl<const N: usize> Jet1<N> {
    /// Constant (all derivatives zero).
    pub fn constant(value: f64) -> Self {
        Self {
            value,
            grad: [0.0; N],
        }
    }

    /// Variable seeded at `param_idx` (unit gradient there).
    pub fn variable(value: f64, param_idx: usize) -> Self {
        let mut jet = Self::constant(value);
        jet.grad[param_idx] = 1.0;
        jet
    }
}

impl<const N: usize, const H: usize> Jet2<N, H> {
    /// Constant (all derivatives zero).
    pub fn constant(value: f64) -> Self {
        Self {
            value,
            grad: [0.0; N],
            hess: [0.0; H],
        }
    }

    /// Variable seeded at `param_idx` (unit gradient there).
    pub fn variable(value: f64, param_idx: usize) -> Self {
        let mut jet = Self::constant(value);
        jet.grad[param_idx] = 1.0;
        jet
    }
}

pub type Dual = Jet1<1>;
pub type HyperDual = Jet2<2, { hess_size(2) }>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hess_index() {
        assert_eq!(hess_index(0, 0).unwrap(), 0);
        assert_eq!(hess_index(1, 0).unwrap(), 1);
        assert_eq!(hess_index(1, 1).unwrap(), 2);
        assert_eq!(hess_index(2, 0).unwrap(), 3);
        assert_eq!(hess_index(2, 1).unwrap(), 4);
        assert_eq!(hess_index(2, 2).unwrap(), 5);

        for i in 0..5 {
            for j in 0..5 {
                assert_eq!(hess_index(i, j), hess_index(j, i))
            }
        }
    }

    #[test]
    fn test_hess_size() {
        assert_eq!(hess_size(1), 1);
        assert_eq!(hess_size(2), 3);
        assert_eq!(hess_size(3), 6);
        assert_eq!(hess_size(4), 10);
    }
}
