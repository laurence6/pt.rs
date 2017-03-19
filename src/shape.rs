use bbox::BBox3f;
use ray::Ray;
use interaction::Interaction;

pub trait Shape {
    fn BBox(&self) -> BBox3f;
    fn IntersectP(&self, &Ray) -> bool;
    fn Intersect(&self, &Ray) -> Option<Interaction>;
}
