use bbox::BBox;
use common::EPSILON;
use common::Float;
use hit::Hit;
use material::Color;
use ray::Ray;
use shape::Shape;
use vector::Vector;

pub struct Sphere {
    Center: Vector,
    Radius: Float,

    BBox: BBox,

    Color: Color,
}

impl Sphere {
    pub fn New(center: Vector, radius: Float, color: Color) -> Sphere {
        let min = Vector::New(center.X - radius, center.Y - radius, center.Z - radius);
        let max = Vector::New(center.X + radius, center.Y + radius, center.Z + radius);
        let bbox = BBox { Min: min, Max: max };
        return Sphere { Center: center, Radius: radius, BBox: bbox, Color: color };
    }
}

impl Shape for Sphere {
    fn BBox(&self) -> BBox {
        return self.BBox;
    }
    fn IntersectP(&self, r: &Ray) -> Option<Hit> {
        let to = r.Origin - self.Center;
        let b = to.Dot(&r.Direction);
        let mut d = b * b - (to.Dot(&to) - self.Radius * self.Radius);
        if d > 0.0 {
            d = d.sqrt();
            let t = -b - d;
            if t > EPSILON {
                return Some(Hit::New(t));
            }
            let t = -b + d;
            if t > EPSILON {
                return Some(Hit::New(t));
            }
        }
        return None;
    }
    fn MateralAt(&self) -> Color {
        return self.Color;
    }
}
