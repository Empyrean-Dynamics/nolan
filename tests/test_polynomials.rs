#[cfg(test)]
mod tests {
    use nolan::jets::Jet;
    use nolan::traits::AutoDiff;

    #[test]
    fn test_univariate_polynomial() {
        // f(x) = 8x^3 + 4x^2 - 2x + 10
        fn f<J: AutoDiff>(x: J) -> J {
            x.powf(3.0) * 8.0 + x.powf(2.0) * 4.0 - x * 2.0 + 10.0
        }

        let x = Jet::<2, 1>::variable(1.0, 0);
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

        let x = Jet::<2, 2>::variable(1.0, 0);
        let y = Jet::<2, 2>::variable(2.0, 1);
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

        let x = Jet::<2, 3>::variable(1.0, 0);
        let y = Jet::<2, 3>::variable(2.0, 1);
        let z = Jet::<2, 3>::variable(3.0, 2);
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
}
