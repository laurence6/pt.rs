use bbox::BBox3f;
use ray::Ray;
use interaction::Interaction;

pub trait Shape {
    fn bbox(&self) -> BBox3f;
    fn intersect_p(&self, &Ray) -> bool;
    fn intersect(&self, &Ray) -> Option<Interaction>;
}
