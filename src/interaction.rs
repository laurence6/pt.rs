use std::rc::Rc;

use common::ONE_MINUS_EPSILON;
use vector::{Vector, Point3f, Point2f};
use shape::Shape;
use ray::Ray;

#[derive(Default, Clone)]
pub struct Interaction {
    pub p: Point3f,
    n: Vector,
    wo: Vector,

    uv: Point2f,
    dpdu: Vector,
    dpdv: Vector,
    dndu: Vector,
    dndv: Vector,
    shape: Option<Rc<Shape>>,
}

impl Interaction {
    pub fn SpawnRayTo(&self, i: Point3f) -> Ray {
        let d = i - self.p;
        return Ray {
            Origin: self.p,
            Direction: d,
            TMax: ONE_MINUS_EPSILON,
        };
    }
}
