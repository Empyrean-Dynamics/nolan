use crate::traits::DifferentiableMath;

/// Multiply two \\(9 \times 9\\) matrices: \\(C = A \, B\\).
#[inline]
pub fn mat9_mul<T: Copy + DifferentiableMath>(a: &[[T; 9]; 9], b: &[[T; 9]; 9]) -> [[T; 9]; 9] {
    let zero = T::constant(0.0);
    let mut c = [[zero; 9]; 9];
    for i in 0..9 {
        for j in 0..9 {
            let mut sum = zero;
            for k in 0..9 {
                sum = sum + a[i][k] * b[k][j];
            }
            c[i][j] = sum;
        }
    }
    c
}

/// Transpose a \\(9 \times 9\\) matrix: \\(T_{ij} = A_{ji}\\).
#[inline]
pub fn mat9_transpose<T: Copy + DifferentiableMath>(a: &[[T; 9]; 9]) -> [[T; 9]; 9] {
    let zero = T::constant(0.0);
    let mut t = [[zero; 9]; 9];
    for i in 0..9 {
        for j in 0..9 {
            t[i][j] = a[j][i];
        }
    }
    t
}

/// Multiply a \\(9 \times 9\\) matrix by a 9-vector: \\(\mathbf{y} = A \, \mathbf{x}\\).
#[inline]
pub fn mat9_vec_mul<T: Copy + DifferentiableMath>(a: &[[T; 9]; 9], x: &[T; 9]) -> [T; 9] {
    let zero = T::constant(0.0);
    let mut y = [zero; 9];
    for i in 0..9 {
        let mut sum = zero;
        for k in 0..9 {
            sum = sum + a[i][k] * x[k];
        }
        y[i] = sum;
    }
    y
}

/// Add two \\(9 \times 9\\) matrices: \\(C = A + B\\).
#[inline]
pub fn mat9_add<T: Copy + DifferentiableMath>(a: &[[T; 9]; 9], b: &[[T; 9]; 9]) -> [[T; 9]; 9] {
    let zero = T::constant(0.0);
    let mut c = [[zero; 9]; 9];
    for i in 0..9 {
        for j in 0..9 {
            c[i][j] = a[i][j] + b[i][j];
        }
    }
    c
}

/// Symmetrize a \\(9 \times 9\\) matrix: \\(S = \tfrac{1}{2}(A + A^\top)\\).
#[inline]
pub fn mat9_symmetrize<T: Copy + DifferentiableMath>(a: &[[T; 9]; 9]) -> [[T; 9]; 9] {
    let zero = T::constant(0.0);
    let mut s = [[zero; 9]; 9];
    for i in 0..9 {
        for j in i..9 {
            let avg = (a[i][j] + a[j][i]) * 0.5;
            s[i][j] = avg;
            s[j][i] = avg;
        }
    }
    s
}

/// Solve \\(A \mathbf{x} = \mathbf{b}\\) for a \\(9 \times 9\\) system via Gauss–Jordan
/// elimination with partial pivoting.
///
/// Returns `None` if the matrix is singular (pivot magnitude below \\(10^{-30}\\)).
#[allow(clippy::needless_range_loop)]
pub fn mat9_solve<T: Copy + DifferentiableMath>(a: &[[T; 9]; 9], b: &[T; 9]) -> Option<[T; 9]> {
    let zero = T::constant(0.0);
    // Augmented matrix [A | b]
    let mut m = [[zero; 10]; 9];
    for i in 0..9 {
        for j in 0..9 {
            m[i][j] = a[i][j];
        }
        m[i][9] = b[i];
    }

    for col in 0..9 {
        let mut max_row = col;
        let mut max_val = m[col][col].value().abs();
        for row in (col + 1)..9 {
            let v = m[row][col].value().abs();
            if v > max_val {
                max_val = v;
                max_row = row;
            }
        }
        if max_val < 1e-30 {
            return None;
        }
        if max_row != col {
            m.swap(col, max_row);
        }

        let pivot = m[col][col];
        for j in 0..10 {
            m[col][j] = m[col][j] / pivot;
        }

        for row in 0..9 {
            if row == col {
                continue;
            }
            let factor = m[row][col];
            for j in 0..10 {
                m[row][j] = m[row][j] - factor * m[col][j];
            }
        }
    }

    let mut x = [zero; 9];
    for i in 0..9 {
        x[i] = m[i][9];
    }
    Some(x)
}

/// Invert a \\(9 \times 9\\) matrix via Gauss–Jordan elimination with partial pivoting.
///
/// Stack-allocated augmented matrix `[[T; 18]; 9]` for performance.
/// Returns `None` if the matrix is singular (pivot magnitude below \\(10^{-30}\\)).
#[allow(clippy::needless_range_loop)]
pub fn mat9_inv<T: Copy + DifferentiableMath>(a: &[[T; 9]; 9]) -> Option<[[T; 9]; 9]> {
    let zero = T::constant(0.0);
    let one = T::constant(1.0);
    // Augmented matrix [A | I]
    let mut m = [[zero; 18]; 9];
    for i in 0..9 {
        for j in 0..9 {
            m[i][j] = a[i][j];
        }
        m[i][i + 9] = one;
    }

    for col in 0..9 {
        let mut max_row = col;
        let mut max_val = m[col][col].value().abs();
        for row in (col + 1)..9 {
            let v = m[row][col].value().abs();
            if v > max_val {
                max_val = v;
                max_row = row;
            }
        }
        if max_val < 1e-30 {
            return None;
        }
        if max_row != col {
            m.swap(col, max_row);
        }

        let pivot = m[col][col];
        for j in 0..18 {
            m[col][j] = m[col][j] / pivot;
        }

        for row in 0..9 {
            if row == col {
                continue;
            }
            let factor = m[row][col];
            for j in 0..18 {
                m[row][j] = m[row][j] - factor * m[col][j];
            }
        }
    }

    let mut inv = [[zero; 9]; 9];
    for i in 0..9 {
        for j in 0..9 {
            inv[i][j] = m[i][j + 9];
        }
    }
    Some(inv)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jets::Jet1;
    use crate::traits::{Differentiable, FirstOrder};

    fn identity9<T: Copy + DifferentiableMath>() -> [[T; 9]; 9] {
        let zero = T::constant(0.0);
        let one = T::constant(1.0);
        let mut id = [[zero; 9]; 9];
        for i in 0..9 {
            id[i][i] = one;
        }
        id
    }

    fn diagonal9(vals: &[f64; 9]) -> [[f64; 9]; 9] {
        let mut m = [[0.0; 9]; 9];
        for i in 0..9 {
            m[i][i] = vals[i];
        }
        m
    }

    #[test]
    fn test_mat9_mul_identity() {
        let id = identity9::<f64>();
        let a = diagonal9(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
        let c = mat9_mul(&a, &id);
        for i in 0..9 {
            for j in 0..9 {
                assert!((c[i][j] - a[i][j]).abs() < 1e-15);
            }
        }
    }

    #[test]
    fn test_mat9_transpose() {
        let mut a = [[0.0; 9]; 9];
        for i in 0..9 {
            for j in 0..9 {
                a[i][j] = (i * 9 + j) as f64;
            }
        }
        let t = mat9_transpose(&a);
        for i in 0..9 {
            for j in 0..9 {
                assert!((t[i][j] - a[j][i]).abs() < 1e-15);
            }
        }
    }

    #[test]
    fn test_mat9_vec_mul_identity() {
        let id = identity9::<f64>();
        let x = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let y = mat9_vec_mul(&id, &x);
        for i in 0..9 {
            assert!((y[i] - x[i]).abs() < 1e-15);
        }
    }

    #[test]
    fn test_mat9_add() {
        let a = diagonal9(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
        let b = diagonal9(&[9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0]);
        let c = mat9_add(&a, &b);
        for i in 0..9 {
            assert!((c[i][i] - 10.0).abs() < 1e-15);
        }
    }

    #[test]
    fn test_mat9_symmetrize() {
        let mut a = [[0.0; 9]; 9];
        a[0][1] = 2.0;
        a[1][0] = 4.0;
        let s = mat9_symmetrize(&a);
        assert!((s[0][1] - 3.0).abs() < 1e-15);
        assert!((s[1][0] - 3.0).abs() < 1e-15);
    }

    #[test]
    fn test_mat9_inv_diagonal() {
        let a = diagonal9(&[2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]);
        let inv = mat9_inv(&a).unwrap();
        for i in 0..9 {
            assert!((inv[i][i] - 1.0 / a[i][i]).abs() < 1e-14);
        }
    }

    #[test]
    fn test_mat9_inv_roundtrip() {
        let mut a = [[0.0; 9]; 9];
        for i in 0..9 {
            a[i][i] = (i + 2) as f64;
            for j in 0..9 {
                if i != j {
                    a[i][j] = 0.1;
                }
            }
        }
        let inv = mat9_inv(&a).unwrap();
        let prod = mat9_mul(&a, &inv);
        let id = identity9::<f64>();
        for i in 0..9 {
            for j in 0..9 {
                assert!(
                    (prod[i][j] - id[i][j]).abs() < 1e-11,
                    "prod[{}][{}] = {}, expected {}",
                    i,
                    j,
                    prod[i][j],
                    id[i][j]
                );
            }
        }
    }

    #[test]
    fn test_mat9_solve_diagonal() {
        let a = diagonal9(&[2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]);
        let b = [2.0, 6.0, 12.0, 20.0, 30.0, 42.0, 56.0, 72.0, 90.0];
        let x = mat9_solve(&a, &b).unwrap();
        for i in 0..9 {
            assert!((x[i] - b[i] / a[i][i]).abs() < 1e-14);
        }
    }

    #[test]
    fn test_mat9_solve_roundtrip() {
        let mut a = [[0.0; 9]; 9];
        for i in 0..9 {
            a[i][i] = (i + 2) as f64;
            for j in 0..9 {
                if i != j {
                    a[i][j] = 0.1;
                }
            }
        }
        let b = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let x = mat9_solve(&a, &b).unwrap();
        let y = mat9_vec_mul(&a, &x);
        for i in 0..9 {
            assert!(
                (y[i] - b[i]).abs() < 1e-11,
                "y[{}] = {}, b[{}] = {}",
                i,
                y[i],
                i,
                b[i]
            );
        }
    }

    #[test]
    fn test_mat9_inv_jet1() {
        let mut a = [[Jet1::<9>::constant(0.0); 9]; 9];
        for i in 0..9 {
            a[i][i] = Jet1::<9>::variable((i + 2) as f64, i);
        }
        let inv = mat9_inv(&a).unwrap();
        let prod = mat9_mul(&a, &inv);
        for i in 0..9 {
            for j in 0..9 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!(
                    (prod[i][j].value() - expected).abs() < 1e-11,
                    "prod[{}][{}] = {}, expected {}",
                    i,
                    j,
                    prod[i][j].value(),
                    expected
                );
            }
        }
        // d(inv[0][0])/d(a[0][0]) = -1/a00^2 = -1/4
        assert!(
            (inv[0][0].grad(0) - (-1.0 / 4.0)).abs() < 1e-11,
            "d(inv[0][0])/d(a00) = {}, expected {}",
            inv[0][0].grad(0),
            -1.0 / 4.0
        );
    }

    #[test]
    fn test_mat9_solve_jet1() {
        let mut a = [[Jet1::<9>::constant(0.0); 9]; 9];
        for i in 0..9 {
            a[i][i] = Jet1::<9>::variable((i + 2) as f64, i);
        }
        let mut b = [Jet1::<9>::constant(0.0); 9];
        for i in 0..9 {
            b[i] = Jet1::<9>::constant((i + 1) as f64);
        }
        let x = mat9_solve(&a, &b).unwrap();
        // x[0] = b[0]/a[0][0] = 1/2
        assert!((x[0].value() - 0.5).abs() < 1e-14);
        // d(x[0])/d(a[0][0]) = -b[0]/a00^2 = -1/4
        assert!((x[0].grad(0) - (-0.25)).abs() < 1e-14);
    }
}
