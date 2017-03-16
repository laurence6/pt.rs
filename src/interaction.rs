use std::rc::Rc;

use vector::{Vector, Point3f, Point2f};
use shape::Shape;

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
