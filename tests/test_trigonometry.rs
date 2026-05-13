#[cfg(test)]
mod tests {
    use hyperjet::jets::{Jet2, Jet3, hess_size, tens_size};
    use hyperjet::traits::AutoDiff;

    #[test]
    fn test_sin() {
        // f(x) = sin(x)
        fn f<J: AutoDiff>(x: J) -> J {
            x.sin()
        }

        let x = Jet2::<1, { hess_size(1) }>::variable(0.0, 0);
        let result = f(x);

        // f(x) = f(0) = sin(0) = 0
        assert_eq!(result.value, 0.0);
        // df(x)/dx = cos(x)
        // df(0)/dx = cos(0) = 1
        assert_eq!(result.grad[0], 1.0);
        // d^2f(x)/dx^2 = -sin(x)
        // d^2f(0)/dx^2 = -sin(0) = 0
        assert_eq!(result.hess[0], 0.0);

        for i in 1..result.hess.len() {
            assert_eq!(result.hess[i], 0.0);
        }
    }

    #[test]
    fn test_cos() {
        // f(x) = cos(x)
        fn f<J: AutoDiff>(x: J) -> J {
            x.cos()
        }

        let x = Jet2::<1, { hess_size(1) }>::variable(0.0, 0);
        let result = f(x);

        // f(x) = f(0) = cos(0) = 1
        assert_eq!(result.value, 1.0);
        // df(x)/dx = -sin(x)
        // df(0)/dx = -sin(0) = 0
        assert_eq!(result.grad[0], 0.0);
        // d^2f(x)/dx^2 = -cos(x)
        // d^2f(0)/dx^2 = -cos(0) = -1
        assert_eq!(result.hess[0], -1.0);

        for i in 1..result.hess.len() {
            assert_eq!(result.hess[i], 0.0);
        }
    }

    #[test]
    fn test_bivariate_trig() {
        // f(x,y) = sin(x) + cos(y)
        fn f<J: AutoDiff>(x: J, y: J) -> J {
            x.sin() + y.cos()
        }

        let x = Jet2::<2, { hess_size(2) }>::variable(0.0, 0);
        let y = Jet2::<2, { hess_size(2) }>::variable(0.0, 1);
        let result = f(x, y);

        // f(x,y) = f(0, 0) = sin(0) + cos(0) = 0 + 1 = 1
        assert_eq!(result.value, 1.0);
        // df(x,y)/dx = cos(x)
        // df(0,0)/dx = cos(0) = 1
        assert_eq!(result.grad[0], 1.0);
        // df(x,y)/dy = -sin(y)
        // df(0,0)/dy = -sin(0) = 0
        assert_eq!(result.grad[1], 0.0);
        // d^2f(x,y)/dx^2 = -sin(x)
        // d^2f(0,0)/dx^2 = -sin(0) = 0
        assert_eq!(result.hess[0], 0.0);
        // d^2f(x,y)/dxdy = 0
        // d^2f(0,0)/dxdy = 0
        assert_eq!(result.hess[1], 0.0);
        // d^2f(x,y)/dy^2 = -cos(y)
        // d^2f(0,0)/dy^2 = -cos(0) = -1
        assert_eq!(result.hess[2], -1.0);

        for i in 3..result.hess.len() {
            assert_eq!(result.hess[i], 0.0);
        }
    }

    // ─── Jet3 trig tests ───────────────────────────────────────

    #[test]
    fn test_jet3_sin() {
        // d^3sin(x)/dx^3 = -cos(x)
        // At x=0: -cos(0) = -1
        let x = Jet3::<1, { hess_size(1) }, { tens_size(1) }>::variable(0.0, 0);
        let result = x.sin();
        assert_eq!(result.value, 0.0);
        assert_eq!(result.grad[0], 1.0);
        assert_eq!(result.hess[0], 0.0);
        assert!((result.tens[0] - (-1.0)).abs() < 1e-15);
    }

    #[test]
    fn test_jet3_cos() {
        // d^3cos(x)/dx^3 = sin(x)
        // At x=0: sin(0) = 0
        let x = Jet3::<1, { hess_size(1) }, { tens_size(1) }>::variable(0.0, 0);
        let result = x.cos();
        assert_eq!(result.value, 1.0);
        assert_eq!(result.grad[0], 0.0);
        assert_eq!(result.hess[0], -1.0);
        assert!((result.tens[0]).abs() < 1e-15);
    }

    #[test]
    fn test_jet3_exp() {
        // d^3exp(x)/dx^3 = exp(x)
        // At x=1: e
        let x = Jet3::<1, { hess_size(1) }, { tens_size(1) }>::variable(1.0, 0);
        let result = x.exp();
        let e = 1.0_f64.exp();
        assert!((result.value - e).abs() < 1e-14);
        assert!((result.grad[0] - e).abs() < 1e-14);
        assert!((result.hess[0] - e).abs() < 1e-14);
        assert!((result.tens[0] - e).abs() < 1e-14);
    }

    #[test]
    fn test_jet3_ln() {
        // d^3ln(x)/dx^3 = 2/x^3
        // At x=2: 2/8 = 0.25
        let x = Jet3::<1, { hess_size(1) }, { tens_size(1) }>::variable(2.0, 0);
        let result = x.ln();
        assert!((result.value - 2.0_f64.ln()).abs() < 1e-14);
        assert!((result.grad[0] - 0.5).abs() < 1e-14); // 1/x = 0.5
        assert!((result.hess[0] - (-0.25)).abs() < 1e-14); // -1/x^2 = -0.25
        assert!((result.tens[0] - 0.25).abs() < 1e-14); // 2/x^3 = 0.25
    }

    #[test]
    fn test_jet3_sqrt() {
        // d^3sqrt(x)/dx^3 = 3/(8*x^(5/2))
        // At x=4: 3/(8*32) = 3/256 ≈ 0.01171875
        let x = Jet3::<1, { hess_size(1) }, { tens_size(1) }>::variable(4.0, 0);
        let result = x.sqrt();
        assert!((result.value - 2.0).abs() < 1e-14);
        assert!((result.grad[0] - 0.25).abs() < 1e-14); // 1/(2*2) = 0.25
        assert!((result.hess[0] - (-1.0 / 32.0)).abs() < 1e-14); // -1/(4*8) = -1/32
        assert!((result.tens[0] - 3.0 / 256.0).abs() < 1e-14);
    }

    #[test]
    fn test_jet3_sin_cos_joint() {
        // Verify sin_cos() produces consistent results with sin() and cos()
        let x = Jet3::<2, { hess_size(2) }, { tens_size(2) }>::variable(1.5, 0);
        let y = Jet3::<2, { hess_size(2) }, { tens_size(2) }>::variable(0.7, 1);
        let arg = x * y;
        let (s, c) = arg.sin_cos();
        let s2 = arg.sin();
        let c2 = arg.cos();

        assert!((s.value - s2.value).abs() < 1e-15);
        assert!((c.value - c2.value).abs() < 1e-15);
        for i in 0..2 {
            assert!((s.grad[i] - s2.grad[i]).abs() < 1e-14);
            assert!((c.grad[i] - c2.grad[i]).abs() < 1e-14);
        }
        for i in 0..hess_size(2) {
            assert!((s.hess[i] - s2.hess[i]).abs() < 1e-13);
            assert!((c.hess[i] - c2.hess[i]).abs() < 1e-13);
        }
        for i in 0..tens_size(2) {
            assert!(
                (s.tens[i] - s2.tens[i]).abs() < 1e-12,
                "sin tens[{i}]: {} vs {}",
                s.tens[i],
                s2.tens[i]
            );
            assert!(
                (c.tens[i] - c2.tens[i]).abs() < 1e-12,
                "cos tens[{i}]: {} vs {}",
                c.tens[i],
                c2.tens[i]
            );
        }
    }

    #[test]
    fn test_jet3_chain_rule_sin_x_squared() {
        // f(x) = sin(x^2) at x=1
        // Compute analytically:
        //   u = x^2, u' = 2x, u'' = 2, u''' = 0
        //   φ = sin(u), φ' = cos(u), φ'' = -sin(u), φ''' = -cos(u)
        //
        // d^3f/dx^3 = φ'*u''' + 3*φ''*u'*u'' + φ'''*(u')^3
        //           = cos(1)*0 + 3*(-sin(1))*2*2 + (-cos(1))*8
        //           = -12*sin(1) - 8*cos(1)
        let x = Jet3::<1, { hess_size(1) }, { tens_size(1) }>::variable(1.0, 0);
        let result = (x * x).sin();

        let expected = -12.0 * 1.0_f64.sin() - 8.0 * 1.0_f64.cos();
        assert!(
            (result.tens[0] - expected).abs() < 1e-12,
            "got {}, expected {}",
            result.tens[0],
            expected
        );
    }
}
