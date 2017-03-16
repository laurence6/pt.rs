use bbox::BBox;
use common::Float;
use interaction::Interaction;
use ray::Ray;
use shape::Shape;
use vector::Vector;

pub struct Sphere {
    Center: Vector,
    Radius: Float,

    BBox: BBox,
}

impl Sphere {
    pub fn New(center: Vector, radius: Float) -> Sphere {
        let min = Vector::New(center.X - radius, center.Y - radius, center.Z - radius);
        let max = Vector::New(center.X + radius, center.Y + radius, center.Z + radius);
        let bbox = BBox { Min: min, Max: max };
        return Sphere { Center: center, Radius: radius, BBox: bbox };
    }
}

impl Shape for Sphere {
    fn BBox(&self) -> BBox {
        return self.BBox;
    }

    fn IntersectP(&self, r: &Ray) -> bool {
        unimplemented!()
    }

    fn Intersect(&self, r: &Ray) -> Option<Interaction> {
        unimplemented!()
    }
}
