use bbox::BBox3f;
use common::Float;
use interaction::Interaction;
use ray::Ray;
use shape::Shape;
use vector::Point3f;

pub struct Sphere {
    Center: Point3f,
    Radius: Float,

    BBox: BBox3f,
}

impl Sphere {
    pub fn New(center: Point3f, radius: Float) -> Sphere {
        let min = Point3f::New(center.X - radius, center.Y - radius, center.Z - radius);
        let max = Point3f::New(center.X + radius, center.Y + radius, center.Z + radius);
        let bbox = BBox3f { min: min, max: max };
        return Sphere { Center: center, Radius: radius, BBox: bbox };
    }
}

impl Shape for Sphere {
    fn BBox(&self) -> BBox3f {
        return self.BBox;
    }

    fn IntersectP(&self, r: &Ray) -> bool {
        unimplemented!()
    }

    fn Intersect(&self, r: &Ray) -> Option<Interaction> {
        unimplemented!()
    }
}
