use common::{Float, INFINITY};
use vector::{Vector3f, Point3f};

pub struct Ray {
    pub Origin: Point3f,
    pub Direction: Vector3f,
    pub TMax: Float,
}

impl Ray {
    pub fn Position(&self, t: Float) -> Vector3f {
        return self.Origin + self.Direction * t;
    }
}

impl Default for Ray {
    fn default() -> Ray {
        Ray {
            Origin: Vector3f::default(),
            Direction: Vector3f::default(),
            TMax: INFINITY,
        }
    }
}
