use std::sync::Arc;

use bbox::BBox3f;
use container::Container;
use interaction::Interaction;
use light::{Light, AreaLight};
use ray::Ray;
use shape::Shape;
use vector::Point3f;

pub struct Scene<C> where C: Container {
    lights: Box<[Arc<Light>]>,
    shapes: C,
}

impl<C> Scene<C> where C: Container {
    fn new(lights: Box<[Arc<Light>]>, shapes: C) -> Scene<C> {
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

pub struct Builder {
    lights: Vec<Arc<Light>>,
    shapes: Vec<Arc<Shape>>,
    area_lights: Vec<Arc<AreaLight>>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            lights: Vec::new(),
            shapes: Vec::new(),
            area_lights: Vec::new(),
        }
    }

    pub fn add_light<T>(&mut self, light: T) where T: 'static + Light {
        self.lights.push(Arc::new(light));
    }

    pub fn add_shape<T>(&mut self, shape: T) where T: 'static + Shape {
        self.shapes.push(Arc::new(shape));
    }

    pub fn add_area_light(&mut self, area_light: AreaLight) {
        self.area_lights.push(Arc::new(area_light));
    }

    pub fn construct<C, F>(mut self, container: F) -> Scene<C> where C: Container, F: FnOnce(Box<[Arc<Shape>]>) -> C {
        let mut bbox = if self.shapes.is_empty() { BBox3f::new(Point3f::default(), Point3f::default()) } else { BBox3f::bbox_of_shapes(&self.shapes) };
        for area_light in self.area_lights.iter() {
            bbox = bbox.union(&area_light.bbox());
        }

        for light in self.lights.iter_mut() {
            let light = Arc::get_mut(light).unwrap();
            light.pre_process(bbox);
        }
        for light in self.area_lights.iter_mut() {
            let light = Arc::get_mut(light).unwrap();
            light.pre_process(bbox);
        }

        for area_light in self.area_lights.into_iter() {
            let light: Arc<Light> = area_light.clone();
            let shape: Arc<Shape> = area_light.clone();
            self.lights.push(light);
            self.shapes.push(shape);
        }

        return Scene::new(
            self.lights.into_boxed_slice(),
            container(self.shapes.into_boxed_slice()),
        );
    }
}
