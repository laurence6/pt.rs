use common::{Float, INFINITY};
use vector::{Vector3f, Point3f};

#[derive(Clone)]
pub struct Ray {
    pub origin: Point3f,
    pub direction: Vector3f,
    pub t_max: Float,
}

impl Ray {
    pub fn position(&self, t: Float) -> Point3f {
        self.origin + self.direction * t
    }
}

impl Default for Ray {
    fn default() -> Ray {
        Ray {
            origin: Point3f::new(0., 0., 0.),
            direction: Vector3f::new(0., 0., 0.),
            t_max: INFINITY,
        }
    }
}
