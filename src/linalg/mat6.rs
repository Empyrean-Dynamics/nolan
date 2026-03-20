use crate::traits::DifferentiableMath;

/// Multiply two \\(6 \times 6\\) matrices: \\(C = A \, B\\).
#[inline]
pub fn mat6_mul<T: Copy + DifferentiableMath>(a: &[[T; 6]; 6], b: &[[T; 6]; 6]) -> [[T; 6]; 6] {
    let zero = T::constant(0.0);
    let mut c = [[zero; 6]; 6];
    for i in 0..6 {
        for j in 0..6 {
            let mut sum = zero;
            for k in 0..6 {
                sum = sum + a[i][k] * b[k][j];
            }
            c[i][j] = sum;
        }
    }
    c
}

/// Transpose a \\(6 \times 6\\) matrix: \\(T_{ij} = A_{ji}\\).
#[inline]
pub fn mat6_transpose<T: Copy + DifferentiableMath>(a: &[[T; 6]; 6]) -> [[T; 6]; 6] {
    let zero = T::constant(0.0);
    let mut t = [[zero; 6]; 6];
    for i in 0..6 {
        for j in 0..6 {
            t[i][j] = a[j][i];
        }
    }
    t
}

/// Multiply a \\(6 \times 6\\) matrix by a 6-vector: \\(\mathbf{y} = A \, \mathbf{x}\\).
#[inline]
pub fn mat6_vec_mul<T: Copy + DifferentiableMath>(a: &[[T; 6]; 6], x: &[T; 6]) -> [T; 6] {
    let zero = T::constant(0.0);
    let mut y = [zero; 6];
    for i in 0..6 {
        let mut sum = zero;
        for k in 0..6 {
            sum = sum + a[i][k] * x[k];
        }
        y[i] = sum;
    }
    y
}

/// Add two \\(6 \times 6\\) matrices: \\(C = A + B\\).
#[inline]
pub fn mat6_add<T: Copy + DifferentiableMath>(a: &[[T; 6]; 6], b: &[[T; 6]; 6]) -> [[T; 6]; 6] {
    let zero = T::constant(0.0);
    let mut c = [[zero; 6]; 6];
    for i in 0..6 {
        for j in 0..6 {
            c[i][j] = a[i][j] + b[i][j];
        }
    }
    c
}

/// Symmetrize a \\(6 \times 6\\) matrix: \\(S = \tfrac{1}{2}(A + A^\top)\\).
#[inline]
pub fn mat6_symmetrize<T: Copy + DifferentiableMath>(a: &[[T; 6]; 6]) -> [[T; 6]; 6] {
    let zero = T::constant(0.0);
    let mut s = [[zero; 6]; 6];
    for i in 0..6 {
        for j in i..6 {
            let avg = (a[i][j] + a[j][i]) * 0.5;
            s[i][j] = avg;
            s[j][i] = avg;
        }
    }
    s
}

/// Solve \\(A \mathbf{x} = \mathbf{b}\\) for a \\(6 \times 6\\) system via Gauss–Jordan
/// elimination with partial pivoting.
///
/// Returns `None` if the matrix is singular (pivot magnitude below \\(10^{-30}\\)).
#[allow(clippy::needless_range_loop)]
pub fn mat6_solve<T: Copy + DifferentiableMath>(a: &[[T; 6]; 6], b: &[T; 6]) -> Option<[T; 6]> {
    let zero = T::constant(0.0);
    // Augmented matrix [A | b]
    let mut m = [[zero; 7]; 6];
    for i in 0..6 {
        for j in 0..6 {
            m[i][j] = a[i][j];
        }
        m[i][6] = b[i];
    }

    for col in 0..6 {
        // Partial pivoting: find row with largest |value| in this column.
        let mut max_row = col;
        let mut max_val = m[col][col].value().abs();
        for row in (col + 1)..6 {
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

        // Scale pivot row
        let pivot = m[col][col];
        for j in 0..7 {
            m[col][j] = m[col][j] / pivot;
        }

        // Eliminate column in all other rows
        for row in 0..6 {
            if row == col {
                continue;
            }
            let factor = m[row][col];
            for j in 0..7 {
                m[row][j] = m[row][j] - factor * m[col][j];
            }
        }
    }

    let mut x = [zero; 6];
    for i in 0..6 {
        x[i] = m[i][6];
    }
    Some(x)
}

/// Invert a \\(6 \times 6\\) matrix via Gauss–Jordan elimination with partial pivoting.
///
/// Returns `None` if the matrix is singular (pivot magnitude below \\(10^{-30}\\)).
#[allow(clippy::needless_range_loop)]
pub fn mat6_inv<T: Copy + DifferentiableMath>(a: &[[T; 6]; 6]) -> Option<[[T; 6]; 6]> {
    let zero = T::constant(0.0);
    let one = T::constant(1.0);
    // Augmented matrix [A | I]
    let mut m = [[zero; 12]; 6];
    for i in 0..6 {
        for j in 0..6 {
            m[i][j] = a[i][j];
        }
        m[i][i + 6] = one;
    }

    for col in 0..6 {
        let mut max_row = col;
        let mut max_val = m[col][col].value().abs();
        for row in (col + 1)..6 {
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
        for j in 0..12 {
            m[col][j] = m[col][j] / pivot;
        }

        for row in 0..6 {
            if row == col {
                continue;
            }
            let factor = m[row][col];
            for j in 0..12 {
                m[row][j] = m[row][j] - factor * m[col][j];
            }
        }
    }

    let mut inv = [[zero; 6]; 6];
    for i in 0..6 {
        for j in 0..6 {
            inv[i][j] = m[i][j + 6];
        }
    }
    Some(inv)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jets::Jet1;
    use crate::traits::{Differentiable, FirstOrder};

    fn identity6<T: Copy + DifferentiableMath>() -> [[T; 6]; 6] {
        let zero = T::constant(0.0);
        let one = T::constant(1.0);
        let mut id = [[zero; 6]; 6];
        for i in 0..6 {
            id[i][i] = one;
        }
        id
    }

    fn diagonal6(vals: &[f64; 6]) -> [[f64; 6]; 6] {
        let mut m = [[0.0; 6]; 6];
        for i in 0..6 {
            m[i][i] = vals[i];
        }
        m
    }

    #[test]
    fn test_mat6_mul_identity() {
        let id = identity6::<f64>();
        let a = diagonal6(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let c = mat6_mul(&a, &id);
        for i in 0..6 {
            for j in 0..6 {
                assert!((c[i][j] - a[i][j]).abs() < 1e-15);
            }
        }
    }

    #[test]
    fn test_mat6_transpose() {
        let mut a = [[0.0; 6]; 6];
        for i in 0..6 {
            for j in 0..6 {
                a[i][j] = (i * 6 + j) as f64;
            }
        }
        let t = mat6_transpose(&a);
        for i in 0..6 {
            for j in 0..6 {
                assert!((t[i][j] - a[j][i]).abs() < 1e-15);
            }
        }
    }

    #[test]
    fn test_mat6_vec_mul_identity() {
        let id = identity6::<f64>();
        let x = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let y = mat6_vec_mul(&id, &x);
        for i in 0..6 {
            assert!((y[i] - x[i]).abs() < 1e-15);
        }
    }

    #[test]
    fn test_mat6_add() {
        let a = diagonal6(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let b = diagonal6(&[6.0, 5.0, 4.0, 3.0, 2.0, 1.0]);
        let c = mat6_add(&a, &b);
        for i in 0..6 {
            assert!((c[i][i] - 7.0).abs() < 1e-15);
        }
    }

    #[test]
    fn test_mat6_symmetrize() {
        let mut a = [[0.0; 6]; 6];
        a[0][1] = 2.0;
        a[1][0] = 4.0;
        let s = mat6_symmetrize(&a);
        assert!((s[0][1] - 3.0).abs() < 1e-15);
        assert!((s[1][0] - 3.0).abs() < 1e-15);
    }

    #[test]
    fn test_mat6_inv_diagonal() {
        let a = diagonal6(&[2.0, 3.0, 4.0, 5.0, 6.0, 7.0]);
        let inv = mat6_inv(&a).unwrap();
        for i in 0..6 {
            assert!((inv[i][i] - 1.0 / a[i][i]).abs() < 1e-14);
        }
    }

    #[test]
    fn test_mat6_inv_roundtrip() {
        // SPD matrix: A = D + I where D is diagonal with distinct values
        let mut a = [[0.0; 6]; 6];
        for i in 0..6 {
            a[i][i] = (i + 2) as f64;
            for j in 0..6 {
                if i != j {
                    a[i][j] = 0.1;
                }
            }
        }
        let inv = mat6_inv(&a).unwrap();
        let prod = mat6_mul(&a, &inv);
        let id = identity6::<f64>();
        for i in 0..6 {
            for j in 0..6 {
                assert!(
                    (prod[i][j] - id[i][j]).abs() < 1e-12,
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
    fn test_mat6_solve_f64() {
        let a = diagonal6(&[2.0, 3.0, 4.0, 5.0, 6.0, 7.0]);
        let b = [2.0, 6.0, 12.0, 20.0, 30.0, 42.0];
        let x = mat6_solve(&a, &b).unwrap();
        for i in 0..6 {
            assert!((x[i] - b[i] / a[i][i]).abs() < 1e-14);
        }
    }

    #[test]
    fn test_mat6_solve_roundtrip() {
        let mut a = [[0.0; 6]; 6];
        for i in 0..6 {
            a[i][i] = (i + 2) as f64;
            for j in 0..6 {
                if i != j {
                    a[i][j] = 0.1;
                }
            }
        }
        let b = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let x = mat6_solve(&a, &b).unwrap();
        let y = mat6_vec_mul(&a, &x);
        for i in 0..6 {
            assert!(
                (y[i] - b[i]).abs() < 1e-12,
                "y[{}] = {}, b[{}] = {}",
                i,
                y[i],
                i,
                b[i]
            );
        }
    }

    #[test]
    fn test_mat6_inv_jet1() {
        let mut a = [[Jet1::<6>::constant(0.0); 6]; 6];
        for i in 0..6 {
            a[i][i] = Jet1::<6>::variable((i + 2) as f64, i);
        }
        let inv = mat6_inv(&a).unwrap();
        let prod = mat6_mul(&a, &inv);
        for i in 0..6 {
            for j in 0..6 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!(
                    (prod[i][j].value() - expected).abs() < 1e-12,
                    "prod[{}][{}] = {}, expected {}",
                    i,
                    j,
                    prod[i][j].value(),
                    expected
                );
            }
        }
        // d(inv[0][0])/d(a[0][0]) = d(1/a00)/da00 = -1/a00^2 = -1/4
        assert!(
            (inv[0][0].grad(0) - (-1.0 / 4.0)).abs() < 1e-12,
            "d(inv[0][0])/d(a00) = {}, expected {}",
            inv[0][0].grad(0),
            -1.0 / 4.0
        );
    }

    #[test]
    fn test_mat6_solve_jet1() {
        let mut a = [[Jet1::<6>::constant(0.0); 6]; 6];
        for i in 0..6 {
            a[i][i] = Jet1::<6>::variable((i + 2) as f64, i);
        }
        let b = [
            Jet1::<6>::constant(1.0),
            Jet1::<6>::constant(2.0),
            Jet1::<6>::constant(3.0),
            Jet1::<6>::constant(4.0),
            Jet1::<6>::constant(5.0),
            Jet1::<6>::constant(6.0),
        ];
        let x = mat6_solve(&a, &b).unwrap();
        // x[0] = b[0]/a[0][0] = 1/2 = 0.5
        assert!((x[0].value() - 0.5).abs() < 1e-14);
        // d(x[0])/d(a[0][0]) = d(1/a00)/da00 * b[0] = -1/a00^2 = -1/4
        assert!((x[0].grad(0) - (-0.25)).abs() < 1e-14);
    }
}
