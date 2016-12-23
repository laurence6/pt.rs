use common::Float;
use common::EPS;
use vector::Vector;
use material::Color;
use ray::Ray;

pub struct Sphere {
    Center: Vector,
    Radius: Float,
    Color: Color,
}

pub fn NewSphere(center: Vector, radius: Float, color: Color) -> Sphere {
    //let min = V(center.X - radius, center.Y - radius, center.Z - radius);
    //let max = V(center.X + radius, center.Y + radius, center.Z + radius);
    return Sphere { Center: center, Radius: radius, Color: color };
}

impl Sphere {
    pub fn Intersect(&self, r: &Ray) -> Float {
        let to = r.Origin - self.Center;
        let b = to.Dot(&r.Direction);
        let mut d = b * b - (to.Dot(&to) - self.Radius * self.Radius);
        if d > 0.0 {
            d = d.sqrt();
            let t = -b - d;
            if t > EPS {
                return t;
            }
            let t = -b + d;
            if t > EPS {
                return t;
            }
        }
        return 0.0;
    }
}
