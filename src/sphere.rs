use bbox::BBox3f;
use common::Float;
use interaction::Interaction;
use material::Material;
use ray::Ray;
use shape::Shape;
use vector::Point3f;

pub struct Sphere {
    center: Point3f,
    radius: Float,

    bbox: BBox3f,
}

impl Sphere {
    pub fn new(center: Point3f, radius: Float) -> Sphere {
        let min = Point3f::new(center.x - radius, center.y - radius, center.z - radius);
        let max = Point3f::new(center.x + radius, center.y + radius, center.z + radius);
        let bbox = BBox3f { min: min, max: max };
        return Sphere { center: center, radius: radius, bbox: bbox };
    }
}

impl Shape for Sphere {
    fn bbox(&self) -> BBox3f {
        return self.bbox;
    }

    fn intersect_p(&self, r: &Ray) -> bool {
        unimplemented!()
    }

    fn intersect(&self, r: &Ray) -> Option<Interaction> {
        unimplemented!()
    }
}

impl Material for Sphere {
}
