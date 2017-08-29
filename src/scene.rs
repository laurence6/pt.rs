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
    pub fn new(mut lights: Box<[Box<Light>]>, shapes: Box<Container>) -> Scene {
        let mut scene = Scene {
            lights: Default::default(),
            shapes,
        };
        for light in lights.iter_mut() {
            light.pre_process(&scene)
        }
        scene.lights = lights;
        return scene;
    }

    pub fn lights(&self) -> &[Box<Light>] {
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
