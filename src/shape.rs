use bbox::BBox;
use ray::Ray;
use interaction::Interaction;

pub trait Shape {
    fn BBox(&self) -> BBox;
    fn IntersectP(&self, &Ray) -> bool;
    fn Intersect(&self, &Ray) -> Option<Interaction>;
}
