use bbox::BBox3f;
use interaction::Interaction;
use light::Light;
use ray::Ray;
use shape::Shape;

pub struct Scene {
    lights: Box<[Box<Light>]>,
    aggregate: Box<Shape>,
}

impl Scene {
    pub fn new(lights: Box<[Box<Light>]>, aggregate: Box<Shape>) -> Scene {
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
