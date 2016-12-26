use vector::Vector;
use common::Float;
use common::Axis;
use shape::Shape;

#[derive(Clone, Copy)]
pub struct BBox {
    pub Min: Vector,
    pub Max: Vector,
}

impl BBox {
    pub fn New(min: Vector, max: Vector) -> BBox {
        return BBox { Min: min, Max: max };
    }

    pub fn BBoxOfShapes(shapes: &Vec<&Shape>) -> BBox {
        let mut bbox = BBox::New(Vector::ZeroVector(), Vector::ZeroVector());
        for shape in shapes {
            bbox = bbox.Union(&shape.BBox());
        }
        return bbox;
    }

    pub fn IntersectP(&self) -> bool {
        unimplemented!()
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
