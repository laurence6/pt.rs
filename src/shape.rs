use std::rc::Rc;

use bbox::BBox3f;
use common::Float;
use interaction::Interaction;
use material::Material;
use ray::Ray;

pub trait Shape {
    fn bbox(&self) -> BBox3f;
    /// intersect_p returns whether or not the ray intersects this shape.
    fn intersect_p(&self, ray: &Ray) -> bool;
    /// intersect determines whether the ray intersects this shape and if an intersection occurs, returns the details of the intersection and t value of the ray at the intersection point.
    fn intersect(&self, ray: &Ray) -> Option<(Interaction, Float)>;
    fn material(&self) -> Rc<Material>;
}
