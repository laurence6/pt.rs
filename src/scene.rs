use container::Container;
use interaction::Interaction;
use light::Light;
use ray::Ray;

pub struct Scene<C> where C: Container {
    lights: Box<[Box<Light>]>,
    shapes: C,
}

impl<C> Scene<C> where C: Container {
    pub fn new(mut lights: Box<[Box<Light>]>, shapes: C) -> Scene<C> {
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
