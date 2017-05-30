use common::PI;
use vector::{Vector3f, Point2f};

pub fn concentric_sample_disk(u: Point2f) -> Point2f {
    let u = u * 2.0 + -Point2f::new(1.0, 1.0);

    if u.x == 0.0 && u.y == 0.0 {
        return u;
    }

    let (theta, r) = if u.x.abs() > u.y.abs() {
        ((PI / 4.0) * (u.y / u.x), u.x)
    } else {
        ((PI / 2.0) - (PI / 4.0) * (u.x / u.y), u.y)
    };

    return Point2f::new(theta.cos(), theta.sin()) * r;
}

pub fn cosine_sample_hemisphere(u: Point2f) -> Vector3f {
    let Point2f { x, y } = concentric_sample_disk(u);
    let z = ((1.0 - x * x - y * y).max(0.0)).sqrt();
    return Vector3f::new(x, y, z);
}