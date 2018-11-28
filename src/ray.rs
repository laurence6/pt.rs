use common::INFINITY;
use vector::{Vector3f, Point3f};

#[derive(Clone)]
pub struct Ray {
    pub origin: Point3f,
    pub direction: Vector3f,
    pub t_max: f32,
}

impl Ray {
    pub fn position(&self, t: f32) -> Point3f {
        self.origin + self.direction * t
    }
}

impl Default for Ray {
    fn default() -> Ray {
        Ray {
            origin: Point3f::default(),
            direction: Vector3f::default(),
            t_max: INFINITY,
        }
    }
}
