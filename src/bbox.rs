use std::ops;
use std::sync::Arc;
use std::mem::swap;

use axis::Axis;
use common::gamma;
use ray::Ray;
use shape::Shape;
use vector::{Vector3f, Point3f, Vector2, Point2};

#[derive(Clone, Copy)]
pub struct BBox3f {
    pub min: Point3f,
    pub max: Point3f,
}

impl BBox3f {
    pub fn new(min: Point3f, max: Point3f) -> BBox3f {
        BBox3f {
            min: min.min(max),
            max: max.max(min),
        }
    }

    pub fn bbox_of_shapes(shapes: &[Arc<Shape>]) -> BBox3f {
        let mut bbox = shapes[0].bbox();
        for s in shapes[1..].iter() {
            bbox = bbox.union(&s.bbox());
        }
        return bbox;
    }

    pub fn diagonal(&self) -> Vector3f {
        self.max - self.min
    }

    pub fn surface_area(&self) -> f32 {
        let d = self.diagonal();
        return (d.x * d.y + d.x * d.z + d.y * d.z) * 2.;
    }

    pub fn max_extent(&self) -> Axis {
        let Vector3f { x, y, z } = self.diagonal();
        match (x >= y, x >= z, y >= z) {
            ( true,  true,     _) => Axis::X,
            (false,     _,  true) => Axis::Y,
            (    _, false, false) => Axis::Z,
            _                     => panic!(),
        }
    }

    pub fn bounding_sphere(&self) -> (Point3f, f32) {
        let center = (self.min + self.max) / 2.;
        let radius = if self.point_inside(center) {
            center.distance(self.max)
        } else {
            0.
        };
        return (center, radius);
    }

    pub fn overlaps(&self, b: &BBox3f) -> bool {
        (b.min.x <= self.max.x) && (self.min.x <= b.max.x) &&
        (b.min.y <= self.max.y) && (self.min.y <= b.max.y) &&
        (b.min.z <= self.max.z) && (self.min.z <= b.max.z)
    }

    pub fn point_inside(&self, p: Point3f) -> bool {
        (self.min.x <= p.x) && (p.x <= self.max.x) &&
        (self.min.y <= p.y) && (p.y <= self.max.y) &&
        (self.min.z <= p.z) && (p.z <= self.max.z)
    }

    pub fn union(&self, b: &BBox3f) -> BBox3f {
        BBox3f {
            min: self.min.min(b.min),
            max: self.max.max(b.max),
        }
    }

    pub fn union_point(&self, p: Point3f) -> BBox3f {
        BBox3f {
            min: self.min.min(p),
            max: self.max.max(p),
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<(f32, f32)> {
        let mut t0 = 0.;
        let mut t1 = ray.t_max;

        let mut axis = Axis::X;
        for _ in 0..3 {
            // t_near and t_far could be NaN or Infinity
            let mut t_near = (self.min[axis] - ray.origin[axis]) / ray.direction[axis];
            let mut t_far  = (self.max[axis] - ray.origin[axis]) / ray.direction[axis];
            if t_near > t_far {
                swap(&mut t_near, &mut t_far);
            }
            // to avoid epsilon
            t_far *= 1. + 2. * gamma(3.);

            t0 = if t_near > t0 { t_near } else { t0 };
            t1 = if t_far  < t1 { t_far  } else { t1 };

            if t0 > t1 {
                return None;
            }

            axis = axis.next();
        }
        return Some((t0, t1));
    }
}

#[derive(Clone, Copy)]
pub struct BBox2<T> where T: PartialOrd {
    pub min: Point2<T>,
    pub max: Point2<T>,
}

impl<T> BBox2<T> where T: Copy + PartialOrd {
    pub fn new(min: Point2<T>, max: Point2<T>) -> BBox2<T> {
        BBox2 {
            min: min.min(max),
            max: max.max(min),
        }
    }
}

impl<T> BBox2<T> where T: Copy + PartialOrd + ops::Sub<Output = T> + ops::Mul<Output = T> {
    pub fn diagonal(&self) -> Vector2<T> {
        self.max - self.min
    }

    pub fn area(&self) -> T {
        let d = self.diagonal();
        return d.x * d.y;
    }
}

pub type BBox2u = BBox2<u32>;
pub type BBox2i = BBox2<i32>;
pub type BBox2f = BBox2<f32>;

impl BBox2i {
    pub fn iter(&self) -> BBox2iIter {
        BBox2iIter::new(self.min, self.max)
    }
}

macro_rules! impl_bbox2_from {
    ($from: ident, $to: ident) => (
        impl From<$from> for $to {
            fn from(BBox2 { min, max }: $from) -> $to {
                BBox2 {
                    min: Point2::from(min),
                    max: Point2::from(max),
                }
            }
        }
    );
}

impl_bbox2_from!(BBox2f, BBox2u);
impl_bbox2_from!(BBox2i, BBox2u);
impl_bbox2_from!(BBox2f, BBox2i);
impl_bbox2_from!(BBox2u, BBox2f);
impl_bbox2_from!(BBox2i, BBox2f);

pub struct BBox2iIter {
    min: Point2<i32>,
    max: Point2<i32>,
    x: i32,
    y: i32,
    next_none: bool,
}

impl BBox2iIter {
    fn new(min: Point2<i32>, max: Point2<i32>) -> BBox2iIter {
        BBox2iIter {
            min,
            max,
            x: min.x,
            y: min.y,
            next_none: false,
        }
    }
}

impl Iterator for BBox2iIter {
    type Item = Point2<i32>;
    fn next(&mut self) -> Option<Point2<i32>> {
        if self.next_none {
            return None;
        }

        let p = Some(Point2::new(self.x, self.y));

        self.x += 1;
        if self.x >= self.max.x {
            self.x = self.min.x;
            self.y += 1;
            if self.y >= self.max.y {
                self.next_none = true;
            }
        }

        return p;
    }
}

#[cfg(test)]
mod test {
    use bbox::BBox2i;
    use vector::Point2;

    #[test]
    fn test_bbox2i_iter() {
        let bbox = BBox2i::new(Point2::new(2, 2), Point2::new(5, 5));
        let ps = [
            Point2 { x: 2, y: 2 }, Point2 { x: 3, y: 2 }, Point2 { x: 4, y: 2 },
            Point2 { x: 2, y: 3 }, Point2 { x: 3, y: 3 }, Point2 { x: 4, y: 3 },
            Point2 { x: 2, y: 4 }, Point2 { x: 3, y: 4 }, Point2 { x: 4, y: 4 },
        ];

        let mut n = 0;
        for p in bbox.iter() {
            assert_eq!(p, ps[n]);
            n += 1;
        }

        assert_eq!(n as i32, bbox.area());
    }
}
