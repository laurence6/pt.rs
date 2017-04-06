use std::rc::Rc;

use common::ONE_MINUS_EPSILON;
use vector::{Vector3f, Point3f, Point2f};
use shape::Shape;
use ray::Ray;

#[derive(Default, Clone)]
pub struct Interaction {
    pub p: Point3f,
    n: Vector3f,
    wo: Vector3f,

    uv: Point2f,
    dpdu: Vector3f,
    dpdv: Vector3f,
    dndu: Vector3f,
    dndv: Vector3f,
    shape: Option<Rc<Shape>>,
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
