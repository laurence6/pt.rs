use ray::Ray;
use bbox::BBox;
use material::Color;
use hit::Hit;

pub trait Shape {
    fn BBox(&self) -> BBox {
        unimplemented!()
    }
    fn Intersect(&self, &Ray) -> Option<Hit> {
        return None;
    }
    fn MateralAt(&self) -> Color {
        unimplemented!()
    }
}
