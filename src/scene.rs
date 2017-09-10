use std::sync::Arc;

use container::Container;
use interaction::Interaction;
use light::Light;
use ray::Ray;

pub struct Scene<C> where C: Container {
    lights: Box<[Arc<Light>]>,
    shapes: C,
}

impl<C> Scene<C> where C: Container {
    pub fn new(lights: Box<[Arc<Light>]>, shapes: C) -> Scene<C> {
        Scene {
            lights,
            shapes,
        }
    }

    pub fn lights(&self) -> &[Arc<Light>] {
        &self.lights
    }

    pub fn intersect_p(&self, ray: &Ray) -> bool {
        self.shapes.intersect_p(ray)
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Interaction> {
        self.shapes.intersect(ray)
    }
}
