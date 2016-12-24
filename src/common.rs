pub type Float = f32;

pub const INF: Float = 2e9;
pub const EPS: Float = 1e-9;

pub fn clamp(x: Float) -> Float {
    if x < 0.0 {
        return 0.0;
    } else if x > 1.0 {
        return 1.0;
    } else {
        return x;
    }
}
