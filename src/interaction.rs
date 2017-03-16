use std::rc::Rc;

use vector::{Vector, Point3f, Point2f};
use shape::Shape;

pub trait Interaction {
    fn GetP(&self) -> Point3f;
}

#[derive(Default)]
pub struct BaseInteraction {
    pub p: Point3f,
    pub n: Vector,
    pub wo: Vector,
}

impl Interaction for BaseInteraction {
    fn GetP(&self) -> Point3f {
        self.p
    }
}

pub struct SurfaceInteraction {
    base: BaseInteraction,

    uv: Point2f,
    dpdu: Vector,
    dpdv: Vector,
    dndu: Vector,
    dndv: Vector,
    shape: Rc<Shape>,
}
