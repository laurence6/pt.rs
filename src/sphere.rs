use common::Float;
use vector::Vector;

pub struct Sphere {
    Center: Vector,
    Radius: Float,
}

pub fn NewSphere(center: Vector, radius: Float) -> Sphere {
    //let min = V(center.X - radius, center.Y - radius, center.Z - radius);
    //let max = V(center.X + radius, center.Y + radius, center.Z + radius);
    return Sphere { Center: center, Radius: radius };
}
