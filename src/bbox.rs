use std::ops;
use std::rc::Rc;
use std::mem::swap;

use axis::Axis;
use common::{Float, EPSILON};
use ray::Ray;
use shape::Shape;
use vector::{Vector2, Vector3f, Point2, Point3f};

#[derive(Clone, Copy)]
pub struct BBox3f {
    pub min: Point3f,
    pub max: Point3f,
}

impl BBox3f {
    pub fn new(min: Point3f, max: Point3f) -> BBox3f {
        BBox3f { min: min, max: max }
    }

    pub fn bbox_of_shapes(shapes: &Vec<Rc<Shape>>) -> BBox3f {
        let mut bbox = BBox3f::new(Point3f::default(), Point3f::default());
        for shape in shapes {
            bbox = bbox.union(&shape.bbox());
        }
        return bbox;
    }

    pub fn diagonal(&self) -> Vector3f {
        self.max - self.min
    }

    pub fn surface_area(&self) -> Float {
        let d = self.diagonal();
        return (d.X * d.Y + d.X * d.Z + d.Y * d.Z) * 2.0;
    }

    pub fn maximum_extent(&self) -> Axis {
        let d = self.diagonal();
        match (d.X <= d.Y, d.Y <= d.Z) {
            (true,  true) => return Axis::X,
            (false, true) => return Axis::Y,
            _             => return Axis::Z,
        }
    }

    pub fn bounding_sphere(&self) -> (Point3f, Float) {
        let center = (self.min + self.max) / 2.0;
        let radius = if self.point_inside(center) {
            center.Distance(self.max)
        } else {
            0.0
        };
        return (center, radius);
    }

    pub fn overlaps(&self, b: &BBox3f) -> bool {
        (b.min.X <= self.max.X) && (self.min.X <= b.max.X) &&
        (b.min.Y <= self.max.Y) && (self.min.Y <= b.max.Y) &&
        (b.min.Z <= self.max.Z) && (self.min.Z <= b.max.Z)
    }

    pub fn point_inside(&self, p: Point3f) -> bool {
        (self.min.X <= p.X) && (p.X <= self.max.X) &&
        (self.min.Y <= p.Y) && (p.Y <= self.max.Y) &&
        (self.min.Z <= p.Z) && (p.Z <= self.max.Z)
    }

    pub fn union(&self, b: &BBox3f) -> BBox3f {
        BBox3f {
            min: self.min.Min(b.min),
            max: self.max.Max(b.max),
        }
    }

    pub fn intersect_p(&self, ray: &Ray) -> Option<(Float, Float)> {
        let mut t0 = 0.0;
        let mut t1 = ray.t_max;

        let mut axis = Axis::X;
        for _ in 0..3 {
            let inv_ray_dir = 1.0 / ray.direction[axis];
            let mut t_near = (self.min[axis] - ray.origin[axis]) * inv_ray_dir;
            let mut t_far  = (self.max[axis] - ray.origin[axis]) * inv_ray_dir;
            if t_near > t_far {
                swap(&mut t_near, &mut t_far);
            }
            // to avoid epsilon
            t_far *= 1.0 + 2.0 * gamma(3.0);

            // notice that t_near and t_far could be NaN
            t0 = if t_near > t0 { t_near } else { t0 };
            t1 = if t_far  < t1 { t_far  } else { t1 };

            if t0 > t1 {
                return None;
            }

            axis = axis.next_axis();
        }
        return Some((t0, t1));
    }
}

#[derive(Clone, Copy)]
pub struct BBox2<T> where T: Copy {
    pub min: Point2<T>,
    pub max: Point2<T>,
}

impl<T> BBox2<T> where T: Copy + ops::Sub<Output = T> + ops::Mul<Output = T> {
    pub fn diagonal(&self) -> Vector2<T> {
        self.max - self.min
    }

    pub fn area(&self) -> T {
        let d = self.diagonal();
        return d.X * d.Y;
    }
}

pub type BBox2u = BBox2<u32>;
pub type BBox2f = BBox2<Float>;

fn gamma(x: Float) -> Float {
    (x * EPSILON) / (1.0 - x * EPSILON)
}
