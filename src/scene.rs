use bbox::BBox3f;
use container::Container;
use interaction::Interaction;
use light::Light;
use ray::Ray;

pub struct Scene {
    lights: Box<[Box<Light>]>,
    shapes: Box<Container>,
}

impl Scene {
    pub fn new(lights: Box<[Box<Light>]>, shapes: Box<Container>) -> Scene {
        Scene { lights, shapes }
    }

    pub fn lights(&self) -> &Box<[Box<Light>]> {
        &self.lights
    }

    pub fn bbox(&self) -> BBox3f {
        self.shapes.bbox()
    }

    pub fn intersect_p(&self, ray: &Ray) -> bool {
        self.shapes.intersect_p(ray)
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Interaction> {
        self.shapes.intersect(ray)
    }
}
