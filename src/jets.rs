/// Get the linear index of a Hessian matrix stored in lower triangular form.
#[inline(always)]
pub fn hess_index(i: usize, j: usize) -> Option<usize> {
    if i < j {
        hess_index(j, i)
    } else {
        Some(i * (i + 1) / 2 + j)
    }
}

/// Lower-triangular index assuming canonical ordering (i >= j).
#[inline(always)]
pub fn hess_idx(i: usize, j: usize) -> usize {
    i * (i + 1) / 2 + j
}

/// Calculate the size of the flattened hessian matrix given
/// n parameters.
#[inline(always)]
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

/// Get the linear index of a third-order tensor stored in lower-triangular form.
///
/// Canonical ordering: `i >= j >= k`. All permutations map to the same index.
#[inline(always)]
pub fn tens_index(i: usize, j: usize, k: usize) -> Option<usize> {
    let (mut a, mut b, mut c) = (i, j, k);
    if a < b {
        std::mem::swap(&mut a, &mut b);
    }
    if b < c {
        std::mem::swap(&mut b, &mut c);
    }
    if a < b {
        std::mem::swap(&mut a, &mut b);
    }
    Some(a * (a + 1) * (a + 2) / 6 + b * (b + 1) / 2 + c)
}

/// Third-order tensor index assuming canonical ordering (i >= j >= k).
#[inline(always)]
pub fn tens_idx(i: usize, j: usize, k: usize) -> usize {
    i * (i + 1) * (i + 2) / 6 + j * (j + 1) / 2 + k
}

/// Calculate the size of the flattened third-order tensor given
/// n parameters: \\( n(n+1)(n+2)/6 \\).
#[inline(always)]
pub const fn tens_size(n: usize) -> usize {
    n * (n + 1) * (n + 2) / 6
}

/// Third-order jet: value + gradient + hessian + third-order tensor.
///
/// `H` must equal [`hess_size(N)`] = \\( N (N+1) / 2 \\).
/// `T` must equal [`tens_size(N)`] = \\( N (N+1)(N+2) / 6 \\).
///
/// For \\( N = 6, H = 21, T = 56 \\): 672 bytes (1 + 6 + 21 + 56 `f64`s).
#[derive(Clone, Copy)]
pub struct Jet3<const N: usize, const H: usize, const T: usize> {
    /// Function value \\( f \\).
    pub value: f64,
    /// First derivatives \\( \partial f / \partial p_i \\).
    pub grad: [f64; N],
    /// Second derivatives \\( \partial^2 f / \partial p_i \partial p_j \\),
    /// stored in lower-triangular form.
    pub hess: [f64; H],
    /// Third derivatives \\( \partial^3 f / \partial p_i \partial p_j \partial p_k \\),
    /// stored in lower-triangular form (i >= j >= k).
    pub tens: [f64; T],
}

impl<const N: usize, const H: usize, const T: usize> Jet3<N, H, T> {
    /// Constant (all derivatives zero).
    pub fn constant(value: f64) -> Self {
        Self {
            value,
            grad: [0.0; N],
            hess: [0.0; H],
            tens: [0.0; T],
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
pub type HyperHyperDual = Jet3<2, { hess_size(2) }, { tens_size(2) }>;

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

    #[test]
    fn test_tens_index() {
        // (0,0,0) -> 0
        assert_eq!(tens_index(0, 0, 0).unwrap(), 0);
        // (1,0,0) -> 1
        assert_eq!(tens_index(1, 0, 0).unwrap(), 1);
        // (1,1,0) -> 2
        assert_eq!(tens_index(1, 1, 0).unwrap(), 2);
        // (1,1,1) -> 3
        assert_eq!(tens_index(1, 1, 1).unwrap(), 3);
        // (2,0,0) -> 4
        assert_eq!(tens_index(2, 0, 0).unwrap(), 4);
        // (2,1,0) -> 5
        assert_eq!(tens_index(2, 1, 0).unwrap(), 5);
        // (2,1,1) -> 6
        assert_eq!(tens_index(2, 1, 1).unwrap(), 6);
        // (2,2,0) -> 7
        assert_eq!(tens_index(2, 2, 0).unwrap(), 7);
        // (2,2,1) -> 8
        assert_eq!(tens_index(2, 2, 1).unwrap(), 8);
        // (2,2,2) -> 9
        assert_eq!(tens_index(2, 2, 2).unwrap(), 9);

        // All permutations map to the same index
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    assert_eq!(tens_index(i, j, k), tens_index(i, k, j));
                    assert_eq!(tens_index(i, j, k), tens_index(j, i, k));
                    assert_eq!(tens_index(i, j, k), tens_index(j, k, i));
                    assert_eq!(tens_index(i, j, k), tens_index(k, i, j));
                    assert_eq!(tens_index(i, j, k), tens_index(k, j, i));
                }
            }
        }
    }

    #[test]
    fn test_tens_size() {
        assert_eq!(tens_size(1), 1);
        assert_eq!(tens_size(2), 4);
        assert_eq!(tens_size(3), 10);
        assert_eq!(tens_size(6), 56);
        assert_eq!(tens_size(9), 165);
    }
}
