use std::rc::Rc;

use common::ONE_MINUS_EPSILON;
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
    pub fn spawn_ray_to(&self, i: Point3f) -> Ray {
        let d = i - self.p;
        return Ray {
            origin: self.p,
            direction: d,
            t_max: ONE_MINUS_EPSILON,
        };
    }
}
