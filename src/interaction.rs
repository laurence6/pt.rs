use std::rc::Rc;

use common::{INFINITY, ONE_MINUS_EPSILON};
use ray::Ray;
use shape::Shape;
use vector::{Vector3f, Normal3f, Point3f};

#[derive(Default, Clone)]
pub struct Interaction {
    pub p: Point3f,
    pub n: Normal3f,
    pub wo: Vector3f,
    pub shape: Option<Rc<Shape>>,
}

impl Interaction {
    pub fn spawn_ray(&self, direction: Vector3f) -> Ray {
        Ray {
            origin: self.p,
            direction,
            t_max: INFINITY,
        }
    }

    pub fn spawn_ray_to(&self, i: Point3f) -> Ray {
        Ray {
            origin: self.p,
            direction: i - self.p,
            t_max: ONE_MINUS_EPSILON,
        }
    }
}
