use bbox::BBox3f;
use ray::Ray;
use interaction::Interaction;
use light::Light;
use primitive::Primitive;

pub struct Scene {
    lights: Box<[Box<Light>]>,
    aggregate: Box<Primitive>,
}

impl Scene {
    pub fn new(lights: Box<[Box<Light>]>, aggregate: Box<Primitive>) -> Scene {
        Scene {
            lights: lights,
            aggregate: aggregate,
        }
    }

    pub fn bbox(&self) -> BBox3f {
        self.aggregate.bbox()
    }

    pub fn intersect_p(&self, ray: &Ray) -> bool {
        self.aggregate.intersect_p(ray)
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Interaction> {
        self.aggregate.intersect(ray)
    }
}
