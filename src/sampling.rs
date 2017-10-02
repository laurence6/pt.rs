use common::PI;
use vector::{Vector3f, Point2f};

pub fn concentric_sample_disk(u: Point2f) -> Point2f {
    let u = u * 2. + -Point2f::new(1., 1.);

    if u.x == 0. && u.y == 0. {
        return u;
    }

    let (theta, r) = if u.x.abs() > u.y.abs() {
        ((PI / 4.) * (u.y / u.x), u.x)
    } else {
        ((PI / 2.) - (PI / 4.) * (u.x / u.y), u.y)
    };

    return Point2f::new(theta.cos(), theta.sin()) * r;
}

pub fn cosine_sample_hemisphere(u: Point2f) -> Vector3f {
    let Point2f { x, y } = concentric_sample_disk(u);
    let z = (1. - x * x - y * y).max(0.).sqrt();
    return Vector3f::new(x, y, z);
}

pub fn power_heuristic(n1: u32, pdf1: f32, n2: u32, pdf2: f32) -> f32 {
    let f = pdf1 * n1 as f32;
    let g = pdf2 * n2 as f32;
    return (f * f) / (f * f + g * g);
}
