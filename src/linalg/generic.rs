use crate::traits::DifferentiableMath;

/// Solve \\(A \mathbf{x} = \mathbf{b}\\) for an \\(N \times N\\) system via Gauss–Jordan
/// elimination with partial pivoting.
///
/// Uses `Vec<Vec<T>>` for the augmented matrix since `N+1` cannot be expressed
/// as a const generic on stable Rust. Returns `None` if the matrix is singular
/// (pivot magnitude below \\(10^{-30}\\)).
#[allow(clippy::needless_range_loop)]
pub fn mat_solve<T: Copy + DifferentiableMath, const N: usize>(
    a: &[[T; N]; N],
    b: &[T; N],
) -> Option<[T; N]> {
    let zero = T::constant(0.0);
    // Augmented matrix [A | b]
    let mut m: Vec<Vec<T>> = (0..N)
        .map(|i| {
            let mut row = vec![zero; N + 1];
            row[..N].copy_from_slice(&a[i]);
            row[N] = b[i];
            row
        })
        .collect();

    for col in 0..N {
        let mut max_row = col;
        let mut max_val = m[col][col].value().abs();
        for row in (col + 1)..N {
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
        for j in 0..N + 1 {
            m[col][j] = m[col][j] / pivot;
        }

        for row in 0..N {
            if row == col {
                continue;
            }
            let factor = m[row][col];
            for j in 0..N + 1 {
                m[row][j] = m[row][j] - factor * m[col][j];
            }
        }
    }

    let mut x = [zero; N];
    for i in 0..N {
        x[i] = m[i][N];
    }
    Some(x)
}

/// Invert an \\(N \times N\\) matrix via Gauss–Jordan elimination with partial pivoting.
///
/// Uses `Vec<Vec<T>>` for the augmented matrix since `2*N` cannot be expressed
/// as a const generic on stable Rust. Returns `None` if the matrix is singular
/// (pivot magnitude below \\(10^{-30}\\)).
#[allow(clippy::needless_range_loop)]
pub fn mat_inv<T: Copy + DifferentiableMath, const N: usize>(
    a: &[[T; N]; N],
) -> Option<[[T; N]; N]> {
    let zero = T::constant(0.0);
    let one = T::constant(1.0);
    // Augmented matrix [A | I]
    let mut m: Vec<Vec<T>> = (0..N)
        .map(|i| {
            let mut row = vec![zero; 2 * N];
            row[..N].copy_from_slice(&a[i]);
            row[N + i] = one;
            row
        })
        .collect();

    for col in 0..N {
        let mut max_row = col;
        let mut max_val = m[col][col].value().abs();
        for row in (col + 1)..N {
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
        for j in 0..2 * N {
            m[col][j] = m[col][j] / pivot;
        }

        for row in 0..N {
            if row == col {
                continue;
            }
            let factor = m[row][col];
            for j in 0..2 * N {
                m[row][j] = m[row][j] - factor * m[col][j];
            }
        }
    }

    let mut inv = [[zero; N]; N];
    for i in 0..N {
        for j in 0..N {
            inv[i][j] = m[i][N + j];
        }
    }
    Some(inv)
}

/// Symmetrize an \\(N \times N\\) matrix: \\(S = \tfrac{1}{2}(A + A^\top)\\).
#[inline]
pub fn mat_symmetrize<T: Copy + DifferentiableMath, const N: usize>(
    a: &[[T; N]; N],
) -> [[T; N]; N] {
    let zero = T::constant(0.0);
    let mut s = [[zero; N]; N];
    for i in 0..N {
        for j in i..N {
            let avg = (a[i][j] + a[j][i]) * 0.5;
            s[i][j] = avg;
            s[j][i] = avg;
        }
    }
    s
}

/// Compute the quadratic form \\(\mathbf{x}^\top A \, \mathbf{x}\\).
#[inline]
pub fn mat_quadratic_form<T: Copy + DifferentiableMath, const N: usize>(
    x: &[T; N],
    a: &[[T; N]; N],
) -> T {
    let mut result = T::constant(0.0);
    for i in 0..N {
        for j in 0..N {
            result = result + x[i] * a[i][j] * x[j];
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jets::Jet1;
    use crate::traits::{Differentiable, FirstOrder};

    #[test]
    fn test_mat_solve_3x3() {
        let a = [[2.0, 1.0, 0.0], [1.0, 3.0, 1.0], [0.0, 1.0, 2.0]];
        let b = [1.0, 2.0, 3.0];
        let x = mat_solve::<f64, 3>(&a, &b).unwrap();
        // Verify A*x = b
        for i in 0..3 {
            let mut sum = 0.0;
            for j in 0..3 {
                sum += a[i][j] * x[j];
            }
            assert!((sum - b[i]).abs() < 1e-14);
        }
    }

    #[test]
    fn test_mat_inv_3x3() {
        let a = [[2.0, 1.0, 0.0], [1.0, 3.0, 1.0], [0.0, 1.0, 2.0]];
        let inv = mat_inv::<f64, 3>(&a).unwrap();
        // Verify A * A^-1 = I
        for i in 0..3 {
            for j in 0..3 {
                let mut sum = 0.0;
                for k in 0..3 {
                    sum += a[i][k] * inv[k][j];
                }
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((sum - expected).abs() < 1e-14);
            }
        }
    }

    #[test]
    fn test_mat_solve_6x6_vs_specialized() {
        // Create a diagonally-dominant 6×6 matrix
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

        let x_generic = mat_solve::<f64, 6>(&a, &b).unwrap();
        let x_specialized = crate::linalg::mat6::mat6_solve(&a, &b).unwrap();

        for i in 0..6 {
            assert!(
                (x_generic[i] - x_specialized[i]).abs() < 1e-13,
                "x_generic[{}] = {}, x_specialized[{}] = {}",
                i,
                x_generic[i],
                i,
                x_specialized[i]
            );
        }
    }

    #[test]
    fn test_mat_solve_9x9_vs_specialized() {
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

        let x_generic = mat_solve::<f64, 9>(&a, &b).unwrap();
        let x_specialized = crate::linalg::mat9::mat9_solve(&a, &b).unwrap();

        for i in 0..9 {
            assert!(
                (x_generic[i] - x_specialized[i]).abs() < 1e-12,
                "x_generic[{}] = {}, x_specialized[{}] = {}",
                i,
                x_generic[i],
                i,
                x_specialized[i]
            );
        }
    }

    #[test]
    fn test_mat_symmetrize() {
        let mut a = [[0.0; 4]; 4];
        a[0][1] = 2.0;
        a[1][0] = 4.0;
        a[2][3] = 6.0;
        a[3][2] = 8.0;
        let s = mat_symmetrize(&a);
        assert!((s[0][1] - 3.0).abs() < 1e-15);
        assert!((s[1][0] - 3.0).abs() < 1e-15);
        assert!((s[2][3] - 7.0).abs() < 1e-15);
        assert!((s[3][2] - 7.0).abs() < 1e-15);
    }

    #[test]
    fn test_mat_quadratic_form() {
        // x^T I x = ||x||^2
        let mut id = [[0.0; 3]; 3];
        for i in 0..3 {
            id[i][i] = 1.0;
        }
        let x = [1.0, 2.0, 3.0];
        let q = mat_quadratic_form(&x, &id);
        assert!((q - 14.0).abs() < 1e-15);
    }

    #[test]
    fn test_mat_quadratic_form_jet1() {
        let mut id = [[Jet1::<3>::constant(0.0); 3]; 3];
        for i in 0..3 {
            id[i][i] = Jet1::<3>::constant(1.0);
        }
        let x = [
            Jet1::<3>::variable(1.0, 0),
            Jet1::<3>::variable(2.0, 1),
            Jet1::<3>::variable(3.0, 2),
        ];
        let q = mat_quadratic_form(&x, &id);
        // x^T I x = x0^2 + x1^2 + x2^2 = 14
        assert!((q.value() - 14.0).abs() < 1e-15);
        // d/dx0 = 2*x0 = 2
        assert!((q.grad(0) - 2.0).abs() < 1e-14);
        // d/dx1 = 2*x1 = 4
        assert!((q.grad(1) - 4.0).abs() < 1e-14);
        // d/dx2 = 2*x2 = 6
        assert!((q.grad(2) - 6.0).abs() < 1e-14);
    }

    #[test]
    fn test_mat_solve_jet1() {
        let mut a = [[Jet1::<3>::constant(0.0); 3]; 3];
        a[0][0] = Jet1::<3>::variable(2.0, 0);
        a[0][1] = Jet1::<3>::constant(1.0);
        a[1][0] = Jet1::<3>::constant(1.0);
        a[1][1] = Jet1::<3>::variable(3.0, 1);
        a[1][2] = Jet1::<3>::constant(1.0);
        a[2][1] = Jet1::<3>::constant(1.0);
        a[2][2] = Jet1::<3>::variable(2.0, 2);
        let b = [
            Jet1::<3>::constant(1.0),
            Jet1::<3>::constant(2.0),
            Jet1::<3>::constant(3.0),
        ];
        let x = mat_solve::<Jet1<3>, 3>(&a, &b).unwrap();
        // Verify A*x = b (values)
        for i in 0..3 {
            let mut sum = Jet1::<3>::constant(0.0);
            for j in 0..3 {
                sum = sum + a[i][j] * x[j];
            }
            assert!(
                (sum.value() - b[i].value()).abs() < 1e-13,
                "row {} residual: {}",
                i,
                sum.value() - b[i].value()
            );
        }
    }

    #[test]
    fn test_mat_inv_singular() {
        // Exactly singular matrix (row 3 = row 1)
        let a = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [1.0, 2.0, 3.0]];
        assert!(mat_inv::<f64, 3>(&a).is_none());
    }
}
