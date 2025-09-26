use crate::parameters::Parameter;
use crate::traits::{Differentiable, DifferentiableMath, FirstOrder, SecondOrder};

#[derive(Clone)]
pub struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

impl<T: DifferentiableMath> Vec3<T> {
    /// Compute the vector's magnitude.
    pub fn norm(&self) -> T {
        (self.x.clone().powi(2) + self.y.clone().powi(2) + self.z.clone().powi(2)).sqrt()
    }

    /// Compute the dot product with another vector.
    pub fn dot(&self, other: &Self) -> T {
        self.x.clone() * other.x.clone()
            + self.y.clone() * other.y.clone()
            + self.z.clone() * other.z.clone()
    }
}

#[derive(Clone)]
pub struct CartesianState<J: Differentiable> {
    r: Vec3<J>,
    t: J,
}

impl<J: FirstOrder> CartesianState<J> {
    pub fn position(&self) -> [f64; 3] {
        [self.r.x.value(), self.r.y.value(), self.r.z.value()]
    }

    pub fn velocity(&self) -> [f64; 3] {
        let tidx = Parameter::Time.index();
        [
            self.r.x.grad(tidx),
            self.r.y.grad(tidx),
            self.r.z.grad(tidx),
        ]
    }

    fn set_velocity(&mut self, v: [f64; 3]) {
        let tidx = Parameter::Time.index();
        *self.r.x.grad_mut(tidx) = v[0];
        *self.r.y.grad_mut(tidx) = v[1];
        *self.r.z.grad_mut(tidx) = v[2];
    }
}

impl<J: SecondOrder> CartesianState<J> {
    pub fn acceleration(&self) -> [f64; 3] {
        let tidx = Parameter::Time.index();
        [
            self.r.x.hess(tidx, tidx),
            self.r.y.hess(tidx, tidx),
            self.r.z.hess(tidx, tidx),
        ]
    }

    fn set_acceleration(&mut self, a: [f64; 3]) {
        let tidx = Parameter::Time.index();
        *self.r.x.hess_mut(tidx, tidx) = a[0];
        *self.r.y.hess_mut(tidx, tidx) = a[1];
        *self.r.z.hess_mut(tidx, tidx) = a[2];
    }
}
