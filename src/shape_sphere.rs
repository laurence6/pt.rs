use std::rc::Rc;

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

    fn intersect_p(&self, ray: &Ray) -> bool {
        unimplemented!()
    }

    fn intersect(&self, ray: &Ray) -> Option<Interaction> {
        unimplemented!()
    }

    fn material(&self) -> Rc<Material> {
        unimplemented!()
    }
}
