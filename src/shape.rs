use std::rc::Rc;

use bbox::BBox3f;
use interaction::Interaction;
use material::Material;
use ray::Ray;

pub trait Shape {
    fn bbox(&self) -> BBox3f;
    fn intersect_p(&self, ray: &Ray) -> bool;
    fn intersect(&self, ray: &Ray) -> Option<Interaction>;
    fn material(&self) -> Rc<Material>;
}
