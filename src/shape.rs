use bbox::BBox;
use hit::Hit;
use ray::Ray;

pub trait Shape {
    fn BBox(&self) -> BBox;
    fn IntersectP(&self, &Ray) -> Option<Hit>;
}
