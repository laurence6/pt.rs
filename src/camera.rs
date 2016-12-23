use vector::Vector;
use ray::NewRay;
use ray::Ray;

//struct Camera {
//
// }

pub fn NewCamera(o: Vector, d: Vector) -> Ray {
    return NewRay(o, d);
}
