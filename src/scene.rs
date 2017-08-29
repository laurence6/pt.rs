use container::Container;
use interaction::Interaction;
use light::Light;
use ray::Ray;

pub struct Scene {
    lights: Box<[Box<Light>]>,
    shapes: Box<Container>,
}

impl Scene {
    pub fn new(mut lights: Box<[Box<Light>]>, shapes: Box<Container>) -> Scene {
        let bbox = shapes.bbox();
        for light in lights.iter_mut() {
            light.pre_process(bbox);
        }
        return Scene {
            lights,
            shapes,
        };
    }

    pub fn lights(&self) -> &[Box<Light>] {
        &self.lights
    }

    pub fn intersect_p(&self, ray: &Ray) -> bool {
        self.shapes.intersect_p(ray)
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Interaction> {
        self.shapes.intersect(ray)
    }
}
