use std::mem::swap;
use vector::Vector;
use vector::ZERO_VECTOR;
use common::Float;
use common::EPSILON;
use axis::Axis;
use shape::Shape;
use ray::Ray;

#[derive(Clone, Copy)]
pub struct BBox {
    pub Min: Vector,
    pub Max: Vector,
}

impl BBox {
    pub fn New(min: Vector, max: Vector) -> BBox {
        return BBox { Min: min, Max: max };
    }

    pub fn BBoxOfShapes(shapes: &Vec<Box<Shape>>) -> BBox {
        let mut bbox = BBox::New(ZERO_VECTOR, ZERO_VECTOR);
        for shape in shapes {
            bbox = bbox.Union(&shape.BBox());
        }
        return bbox;
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
            tFar *= 1.0 + 2.0 * gamma(3.0); // TODO: make as const

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

    pub fn Diagonal(&self) -> Vector {
        return self.Max - self.Min;
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

    pub fn Overlaps(&self, b: &BBox) -> bool {
        return (self.Max.X >= b.Min.X) && (self.Min.X >= b.Max.X) &&
               (self.Max.Y >= b.Min.Y) && (self.Min.Y >= b.Max.Y) &&
               (self.Max.Z >= b.Min.Z) && (self.Min.Z >= b.Max.Z);
    }

    pub fn Union(&self, b: &BBox) -> BBox {
        return BBox {
            Min: self.Min.Min(&b.Min),
            Max: self.Max.Max(&b.Max),
        };
    }
}

fn gamma(x: Float) -> Float {
    return (x * EPSILON) / (1.0 - x * EPSILON);
}
