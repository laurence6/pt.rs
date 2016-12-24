use vector::Vector;
use ray::Ray;

//struct Camera {
//
// }

pub fn NewCamera(o: Vector, d: Vector) -> Ray {
    return Ray::New(o, d);
}
