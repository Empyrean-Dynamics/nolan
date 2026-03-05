#[cfg(test)]
mod tests {
    use nolan::jets::{Jet2, hess_size};
    use nolan::traits::AutoDiff;

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
}
