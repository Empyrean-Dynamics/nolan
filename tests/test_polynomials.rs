#[cfg(test)]
mod tests {
    use hyperjet::jets::{Jet2, Jet3, hess_size, tens_size};
    use hyperjet::traits::AutoDiff;

    #[test]
    fn test_univariate_polynomial() {
        // f(x) = 8x^3 + 4x^2 - 2x + 10
        fn f<J: AutoDiff>(x: J) -> J {
            x.powf(3.0) * 8.0 + x.powf(2.0) * 4.0 - x * 2.0 + 10.0
        }

        let x = Jet2::<1, { hess_size(1) }>::variable(1.0, 0);
        let result = f(x);

        // f(x) = f(1) = 8 + 4 - 2 + 10 = 20
        assert_eq!(result.value, 20.0);
        // df(x)/dx = 24x^2 + 8x - 2
        // df(1)/dx = 30
        assert_eq!(result.grad[0], 30.0);
        // d^2f(x)/dx^2 = 48x + 8
        // d^2f(1)/dx^2 = 56
        assert_eq!(result.hess[0], 56.0);

        for i in 1..result.hess.len() {
            assert_eq!(result.hess[i], 0.0);
        }
    }

    #[test]
    fn test_bivariate_polynomial() {
        // f(x,y) = 10x^4 - 3y^2 + 8x^2y^3 + 10x + 4y + 2
        fn f<J: AutoDiff>(x: J, y: J) -> J {
            x.powf(4.0) * 10.0 - y.powf(2.0) * 3.0
                + x.powf(2.0) * y.powf(3.0) * 8.0
                + x * 10.0
                + y * 4.0
                + 2.0
        }

        let x = Jet2::<2, { hess_size(2) }>::variable(1.0, 0);
        let y = Jet2::<2, { hess_size(2) }>::variable(2.0, 1);
        let result = f(x, y);

        // f(x,y) = f(1, 2) = 10 - 12 + 64 + 10 + 8 + 2 = 82
        assert_eq!(result.value, 82.0);
        // df(x,y)/dx = 40x^3 + 16xy^3 + 10
        // df(1, 2)/dx = 40 + 128 + 10
        assert_eq!(result.grad[0], 178.0);
        // df(x,y)/dy = -6y + 24x^2y^2 + 4
        // df(1, 2)/dy = -12 + 96 + 4
        assert_eq!(result.grad[1], 88.0);
        // d^2f(x,y)/dx^2 = 120x^2 + 16y^3
        // d^2f(1,2)/dx^2 = 120 + 128 = 248
        assert_eq!(result.hess[0], 248.0);
        // d^2f(x,y)/dxdy = 48xy^2
        // d^2f(1,2)/dxdy = 192
        assert_eq!(result.hess[1], 192.0);
        // d^2f(x,y)/dy^2 = -6 + 48x^2y
        // d^2f(1,2)/dy^2 = -6 + 96 = 90
        assert_eq!(result.hess[2], 90.0);

        for i in 3..result.hess.len() {
            assert_eq!(result.hess[i], 0.0);
        }
    }

    #[test]
    fn test_multivariate_polynomial() {
        // f(x,y,z) = x^2y^2z^3 + 20x^2 + 10y^2 + 5z^3 + 2xy + 3yz + 4xz + 10
        fn f<J: AutoDiff>(x: J, y: J, z: J) -> J {
            x.powf(2.0) * y.powf(2.0) * z.powf(3.0)
                + x.powf(2.0) * 20.0
                + y.powf(2.0) * 10.0
                + z.powf(3.0) * 5.0
                + x * y * 2.0
                + y * z * 3.0
                + x * z * 4.0
                + 10.0
        }

        let x = Jet2::<3, { hess_size(3) }>::variable(1.0, 0);
        let y = Jet2::<3, { hess_size(3) }>::variable(2.0, 1);
        let z = Jet2::<3, { hess_size(3) }>::variable(3.0, 2);
        let result = f(x, y, z);

        // f(x,y,z) = f(1,2,3) = 108 + 20 + 40 + 135 + 4 + 18 + 12 + 10 = 347
        assert_eq!(result.value, 347.0);
        // df(x,y,z)/dx = 2xy^2z^3 + 40x + 2y + 4z
        // df(1,2,3)/dx = 216 + 40 + 4 + 12 = 272
        assert_eq!(result.grad[0], 272.0);
        // df(x,y,z)/dy = 2x^2yz^3 + 20y + 2x + 3z
        // df(1,2,3)/dy = 108 + 40 + 2 + 9 = 159
        assert_eq!(result.grad[1], 159.0);
        // df(x,y,z)/dz = 3x^2y^2z^2 + 15z^2 + 3y + 4x
        // df(1,2,3)/dz = 108 + 135 + 6 + 4 = 253
        assert_eq!(result.grad[2], 253.0);
        // d^2f(x,y,z)/dx^2 = 2y^2z^3 + 40
        // d^2f(1,2,3)/dx^2 = 216 + 40 = 256
        assert_eq!(result.hess[0], 256.0);
        // d^2f(x,y,z)/dxdy = 4xyz^3 + 2
        // d^2f(1,2,3)/dxdy = 216 + 2 = 218
        assert_eq!(result.hess[1], 218.0);
        // d^2f(x,y,z)/dy^2 = 2x^2z^3 + 20
        // d^2f(1,2,3)/dy^2 = 54 + 20 = 74
        assert_eq!(result.hess[2], 74.0);
        // d^2f(x,y,z)/dxdz = 6xy^2z^2 + 4
        // d^2f(1,2,3)/dxdz = 216 + 4 = 220
        assert_eq!(result.hess[3], 220.0);
        // d^2f(x,y,z)/dydz = 6x^2yz^2 + 3
        // d^2f(1,2,3)/dydz = 108 + 3 = 111
        assert_eq!(result.hess[4], 111.0);
        // d^2f(x,y,z)/dz^2 = 6x^2y^2z + 30z
        // d^2f(1,2,3)/dz^2 = 72 + 90 = 162
        assert_eq!(result.hess[5], 162.0);

        for i in 6..result.hess.len() {
            assert_eq!(result.hess[i], 0.0);
        }
    }

    // ─── Jet3 polynomial tests ─────────────────────────────────

    #[test]
    fn test_jet3_univariate_cubic() {
        // f(x) = 8x^3 + 4x^2 - 2x + 10
        fn f<J: AutoDiff>(x: J) -> J {
            x.powf(3.0) * 8.0 + x.powf(2.0) * 4.0 - x * 2.0 + 10.0
        }

        let x = Jet3::<1, { hess_size(1) }, { tens_size(1) }>::variable(1.0, 0);
        let result = f(x);

        assert_eq!(result.value, 20.0);
        assert_eq!(result.grad[0], 30.0);
        assert_eq!(result.hess[0], 56.0);
        // d^3f/dx^3 = 48 (constant)
        assert!((result.tens[0] - 48.0).abs() < 1e-10);
    }

    #[test]
    fn test_jet3_univariate_quartic() {
        // f(x) = x^4 at x=2
        // d^3f/dx^3 = 24x = 48
        let x = Jet3::<1, { hess_size(1) }, { tens_size(1) }>::variable(2.0, 0);
        let result = x.powf(4.0);

        assert_eq!(result.value, 16.0);
        assert_eq!(result.grad[0], 32.0); // 4x^3 = 32
        assert!((result.hess[0] - 48.0).abs() < 1e-10); // 12x^2 = 48
        assert!((result.tens[0] - 48.0).abs() < 1e-10); // 24x = 48
    }

    #[test]
    fn test_jet3_bivariate_product() {
        // f(x,y) = x^2 * y^3 at (1, 2)
        fn f<J: AutoDiff>(x: J, y: J) -> J {
            x.powf(2.0) * y.powf(3.0)
        }

        let x = Jet3::<2, { hess_size(2) }, { tens_size(2) }>::variable(1.0, 0);
        let y = Jet3::<2, { hess_size(2) }, { tens_size(2) }>::variable(2.0, 1);
        let result = f(x, y);

        // f(1,2) = 8
        assert_eq!(result.value, 8.0);

        // Third derivatives:
        // d^3f/dx^3 = 0 (x^2 only has 2nd order)
        assert!((result.tens[0]).abs() < 1e-10); // tens_index(0,0,0) = 0

        // d^3f/dx^2dy = 6y^2 = 24
        let idx_xxy = hyperjet::jets::tens_index(1, 0, 0).unwrap(); // (1,0,0) = d/dy d/dx d/dx
        assert!((result.tens[idx_xxy] - 24.0).abs() < 1e-10);

        // d^3f/dxdy^2 = 12xy = 24
        let idx_xyy = hyperjet::jets::tens_index(1, 1, 0).unwrap(); // (1,1,0)
        assert!((result.tens[idx_xyy] - 24.0).abs() < 1e-10);

        // d^3f/dy^3 = 6x^2 = 6
        let idx_yyy = hyperjet::jets::tens_index(1, 1, 1).unwrap(); // (1,1,1)
        assert!((result.tens[idx_yyy] - 6.0).abs() < 1e-10);
    }

    #[test]
    fn test_jet3_trivariate_product() {
        // f(x,y,z) = x * y * z at (2, 3, 5)
        // d^3f/dxdydz = 1, all other third partials are 0
        let x = Jet3::<3, { hess_size(3) }, { tens_size(3) }>::variable(2.0, 0);
        let y = Jet3::<3, { hess_size(3) }, { tens_size(3) }>::variable(3.0, 1);
        let z = Jet3::<3, { hess_size(3) }, { tens_size(3) }>::variable(5.0, 2);
        let result = x * y * z;

        assert_eq!(result.value, 30.0);

        // d^3f/dxdydz = 1
        let idx_xyz = hyperjet::jets::tens_index(2, 1, 0).unwrap();
        assert!((result.tens[idx_xyz] - 1.0).abs() < 1e-10);

        // All pure third derivatives are zero
        assert!((result.tens[hyperjet::jets::tens_index(0, 0, 0).unwrap()]).abs() < 1e-10);
        assert!((result.tens[hyperjet::jets::tens_index(1, 1, 1).unwrap()]).abs() < 1e-10);
        assert!((result.tens[hyperjet::jets::tens_index(2, 2, 2).unwrap()]).abs() < 1e-10);
    }

    #[test]
    fn test_jet3_tensor_symmetry() {
        // f(x,y,z) = x^2*y*z + x*y^2*z at (1, 2, 3)
        fn f<J: AutoDiff>(x: J, y: J, z: J) -> J {
            x.powf(2.0) * y * z + x * y.powf(2.0) * z
        }

        let x = Jet3::<3, { hess_size(3) }, { tens_size(3) }>::variable(1.0, 0);
        let y = Jet3::<3, { hess_size(3) }, { tens_size(3) }>::variable(2.0, 1);
        let z = Jet3::<3, { hess_size(3) }, { tens_size(3) }>::variable(3.0, 2);
        let result = f(x, y, z);

        use hyperjet::traits::ThirdOrder;
        // Verify symmetry: t(i,j,k) should be invariant under all permutations
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    let v = result.tens(i, j, k);
                    assert_eq!(v, result.tens(i, k, j), "symmetry fail at ({i},{j},{k})");
                    assert_eq!(v, result.tens(j, i, k), "symmetry fail at ({i},{j},{k})");
                    assert_eq!(v, result.tens(k, j, i), "symmetry fail at ({i},{j},{k})");
                }
            }
        }
    }
}
