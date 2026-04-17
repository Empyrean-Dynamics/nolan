use crate::traits::DifferentiableMath;

/// Dot product of two 3-vectors: \\(\mathbf{a} \cdot \mathbf{b} = \sum_i a_i b_i\\).
#[inline]
pub fn dot3<T: Copy + DifferentiableMath>(a: &[T; 3], b: &[T; 3]) -> T {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Cross product of two 3-vectors: \\(\mathbf{a} \times \mathbf{b}\\).
#[inline]
pub fn cross3<T: Copy + DifferentiableMath>(a: &[T; 3], b: &[T; 3]) -> [T; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// Squared Euclidean norm of a 3-vector: \\(x_1^2 + x_2^2 + x_3^2\\).
#[inline]
pub fn norm_squared3<T: Copy + DifferentiableMath>(x: &[T; 3]) -> T {
    x[0] * x[0] + x[1] * x[1] + x[2] * x[2]
}

/// Euclidean norm of a 3-vector: \\(\lVert\mathbf{x}\rVert = \sqrt{x_1^2 + x_2^2 + x_3^2}\\).
#[inline]
pub fn norm3<T: Copy + DifferentiableMath>(x: &[T; 3]) -> T {
    norm_squared3(x).sqrt()
}

/// Normalize a 3-vector: \\(\hat{\mathbf{x}} = \mathbf{x} / \lVert\mathbf{x}\rVert\\).
#[inline]
pub fn normalize3<T: Copy + DifferentiableMath>(x: &[T; 3]) -> [T; 3] {
    let n = norm3(x);
    [x[0] / n, x[1] / n, x[2] / n]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jets::Jet1;
    use crate::traits::{Differentiable, FirstOrder};

    #[test]
    fn test_dot3_f64() {
        let a = [1.0, 2.0, 3.0];
        let b = [4.0, 5.0, 6.0];
        assert!((dot3(&a, &b) - 32.0).abs() < 1e-15);
    }

    #[test]
    fn test_cross3_f64() {
        let a = [1.0, 0.0, 0.0];
        let b = [0.0, 1.0, 0.0];
        let c = cross3(&a, &b);
        assert!((c[0]).abs() < 1e-15);
        assert!((c[1]).abs() < 1e-15);
        assert!((c[2] - 1.0).abs() < 1e-15);
    }

    #[test]
    fn test_norm3_f64() {
        let x = [3.0, 4.0, 0.0];
        assert!((norm3(&x) - 5.0).abs() < 1e-15);
    }

    #[test]
    fn test_normalize3_f64() {
        let x = [3.0, 4.0, 0.0];
        let n = normalize3(&x);
        assert!((n[0] - 0.6).abs() < 1e-15);
        assert!((n[1] - 0.8).abs() < 1e-15);
        assert!((n[2]).abs() < 1e-15);
    }

    #[test]
    fn test_dot3_jet1() {
        // dot(a, b) = a0*b0 + a1*b1 + a2*b2
        // d(dot)/d(a0) = b0
        let a = [
            Jet1::<3>::variable(1.0, 0),
            Jet1::<3>::variable(2.0, 1),
            Jet1::<3>::variable(3.0, 2),
        ];
        let b = [
            Jet1::<3>::constant(4.0),
            Jet1::<3>::constant(5.0),
            Jet1::<3>::constant(6.0),
        ];
        let d = dot3(&a, &b);
        assert!((d.value() - 32.0).abs() < 1e-15);
        assert!((d.grad(0) - 4.0).abs() < 1e-15); // d/da0 = b0
        assert!((d.grad(1) - 5.0).abs() < 1e-15); // d/da1 = b1
        assert!((d.grad(2) - 6.0).abs() < 1e-15); // d/da2 = b2
    }

    #[test]
    fn test_cross3_jet1() {
        // cross([1,0,0], [0,1,0]) = [0,0,1]
        // With a0 as variable: cross([a0, 0, 0], [0, 1, 0]) = [0*0 - 0*1, 0*0 - a0*0, a0*1 - 0*0] = [0, 0, a0]
        // d(c2)/d(a0) = 1
        let a = [
            Jet1::<1>::variable(1.0, 0),
            Jet1::<1>::constant(0.0),
            Jet1::<1>::constant(0.0),
        ];
        let b = [
            Jet1::<1>::constant(0.0),
            Jet1::<1>::constant(1.0),
            Jet1::<1>::constant(0.0),
        ];
        let c = cross3(&a, &b);
        assert!((c[2].value() - 1.0).abs() < 1e-15);
        assert!((c[2].grad(0) - 1.0).abs() < 1e-15);
    }

    #[test]
    fn test_norm3_jet1() {
        // norm([3, 4, 0]) = 5
        // d(norm)/d(x0) = x0 / norm = 3/5 = 0.6
        let x = [
            Jet1::<3>::variable(3.0, 0),
            Jet1::<3>::variable(4.0, 1),
            Jet1::<3>::variable(0.0, 2),
        ];
        let n = norm3(&x);
        assert!((n.value() - 5.0).abs() < 1e-15);
        assert!((n.grad(0) - 0.6).abs() < 1e-14);
        assert!((n.grad(1) - 0.8).abs() < 1e-14);
    }

    #[test]
    fn test_normalize3_jet1() {
        let x = [
            Jet1::<3>::variable(3.0, 0),
            Jet1::<3>::variable(4.0, 1),
            Jet1::<3>::variable(0.0, 2),
        ];
        let n = normalize3(&x);
        assert!((n[0].value() - 0.6).abs() < 1e-14);
        assert!((n[1].value() - 0.8).abs() < 1e-14);
        assert!((n[2].value()).abs() < 1e-14);
    }

    #[test]
    fn test_cross3_orthogonality() {
        let a = [1.0, 2.0, 3.0];
        let b = [4.0, 5.0, 6.0];
        let c = cross3(&a, &b);
        assert!(dot3(&a, &c).abs() < 1e-14);
        assert!(dot3(&b, &c).abs() < 1e-14);
    }
}
