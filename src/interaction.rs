use std::sync::Arc;

use common::{INFINITY, next_float_down, next_float_up};
use ray::Ray;
use reflection::BSDF;
use shape::Shape;
use spectrum::Spectrum;
use vector::{Vector3f, Normal3f, Point3f};

#[derive(Default, Clone)]
pub struct Interaction {
    pub p: Point3f,
    pub p_err: Vector3f,

    pub n: Normal3f,
    pub sn: Normal3f, // shading normal
    pub dpdu: Vector3f,
    pub dpdv: Vector3f,

    pub wo: Vector3f,
    pub shape: Option<Arc<Shape>>,
}

impl Interaction {
    fn offset_ray_origin(&self, v: Vector3f) -> Point3f {
        let n = Vector3f::from(self.n);
        let d = n.abs().dot(self.p_err);
        let mut offset = n * d;
        if n.dot(v) < 0. {
            offset *= -1.;
        }
        let mut p = self.p + offset;
        for i in 0..3 {
            if offset[i] > 0. {
                p[i] = next_float_up(p[i]);
            } else if offset[i] < 0. {
                p[i] = next_float_down(p[i]);
            }
        }
        return p;
    }

    pub fn spawn_ray(&self, direction: Vector3f) -> Ray {
        Ray {
            origin: self.offset_ray_origin(direction),
            direction,
            t_max: INFINITY,
        }
    }

    pub fn spawn_ray_to(&self, i: &Interaction) -> Ray {
        let d = i.p - self.p;
        let origin = self.offset_ray_origin(d);
        let p = i.offset_ray_origin(-d);
        let direction = p - origin;
        return Ray {
            origin,
            direction,
            t_max: 1. - 0.0001,
        };
    }

    pub fn compute_scattering(&self) -> BSDF {
        if let Some(shape) = self.shape.clone() {
            shape.compute_scattering(self)
        } else {
            panic!("shape is None")
        }
    }

    pub fn le(&self, w: Vector3f) -> Spectrum {
        if let Some(shape) = self.shape.clone() {
            shape.l(self, w)
        } else {
            panic!("shape is None")
        }
    }
}
