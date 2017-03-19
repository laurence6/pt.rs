use std::mem::swap;
use std::ops;

use axis::Axis;
use common::{Float, EPSILON};
use ray::Ray;
use shape::Shape;
use vector::{Vector2, Vector3f, Point2, Point3f};

#[derive(Clone, Copy)]
pub struct BBox3f {
    pub Min: Point3f,
    pub Max: Point3f,
}

impl BBox3f {
    pub fn New(min: Point3f, max: Point3f) -> BBox3f {
        BBox3f { Min: min, Max: max }
    }

    pub fn BBoxOfShapes(shapes: &Vec<Box<Shape>>) -> BBox3f {
        let mut bbox = BBox3f::New(Point3f::default(), Point3f::default());
        for shape in shapes {
            bbox = bbox.Union(&shape.BBox());
        }
        return bbox;
    }

    pub fn Diagonal(&self) -> Vector3f {
        self.Max - self.Min
    }

    pub fn SurfaceArea(&self) -> Float {
        let d = self.Diagonal();
        return (d.X * d.Y + d.X * d.Z + d.Y * d.Z) * 2.0;
    }

    pub fn MaximumExtent(&self) -> Axis {
        let d = self.Diagonal();
        match (d.X <= d.Y, d.Y <= d.Z) {
            (true,  true) => return Axis::X,
            (false, true) => return Axis::Y,
            _             => return Axis::Z,
        }
    }

    pub fn BoundingSphere(&self) -> (Point3f, Float) {
        let center = (self.Min + self.Max) / 2.0;
        let radius = if self.PointInside(center) {
            center.Distance(self.Max)
        } else {
            0.0
        };
        return (center, radius);
    }

    pub fn Overlaps(&self, b: &BBox3f) -> bool {
        (b.Min.X <= self.Max.X) && (self.Min.X <= b.Max.X) &&
        (b.Min.Y <= self.Max.Y) && (self.Min.Y <= b.Max.Y) &&
        (b.Min.Z <= self.Max.Z) && (self.Min.Z <= b.Max.Z)
    }

    pub fn PointInside(&self, p: Point3f) -> bool {
        (self.Min.X <= p.X) && (p.X <= self.Max.X) &&
        (self.Min.Y <= p.Y) && (p.Y <= self.Max.Y) &&
        (self.Min.Z <= p.Z) && (p.Z <= self.Max.Z)
    }

    pub fn Union(&self, b: &BBox3f) -> BBox3f {
        BBox3f {
            Min: self.Min.Min(b.Min),
            Max: self.Max.Max(b.Max),
        }
    }

    pub fn IntersectP(&self, ray: &Ray) -> Option<(Float, Float)> {
        let mut t0 = 0.0;
        let mut t1 = ray.TMax;

        let mut axis = Axis::X;
        for _ in 0..3 {
            let invRayDir = 1.0 / ray.Direction[axis];
            let mut tNear = (self.Min[axis] - ray.Origin[axis]) * invRayDir;
            let mut tFar  = (self.Max[axis] - ray.Origin[axis]) * invRayDir;
            if tNear > tFar {
                swap(&mut tNear, &mut tFar);
            }
            // to avoid epsilon
            tFar *= 1.0 + 2.0 * Gamma(3.0);

            // notice that tNear and tFar could be NaN
            t0 = if tNear > t0 { tNear } else { t0 };
            t1 = if tFar  < t1 { tFar  } else { t1 };

            if t0 > t1 {
                return None;
            }

            axis = axis.NextAxis();
        }
        return Some((t0, t1));
    }
}

#[derive(Clone, Copy)]
pub struct BBox2<T> where T: Copy {
    pub Min: Point2<T>,
    pub Max: Point2<T>,
}

impl<T> BBox2<T> where T: Copy + ops::Sub<Output = T> + ops::Mul<Output = T> {
    pub fn Diagonal(&self) -> Vector2<T> {
        self.Max - self.Min
    }

    pub fn Area(&self) -> T {
        let d = self.Diagonal();
        return d.X * d.Y;
    }
}

pub type BBox2u = BBox2<u32>;
pub type BBox2f = BBox2<Float>;

fn Gamma(x: Float) -> Float {
    (x * EPSILON) / (1.0 - x * EPSILON)
}
