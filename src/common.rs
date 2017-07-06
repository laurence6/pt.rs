use std::f32;

pub const FLOAT_MAX: f32     = f32::MAX;
pub const FLOAT_MIN_POS: f32 = f32::MIN_POSITIVE;
pub const EPSILON: f32       = f32::EPSILON * 0.5;
pub const INFINITY: f32      = f32::INFINITY;
/// Max number less than 1.
pub const ONE_MINUS_EPSILON: f32 = 1. - EPSILON;

pub const PI: f32 = f32::consts::PI;

pub fn clamp<T: PartialOrd>(x: T, low: T, high: T) -> T {
    debug_assert!(low <= high);
    if x < low {
        return low;
    } else if x > high {
        return high;
    } else {
        return x;
    }
}

pub fn lerp(t: f32, v1: f32, v2: f32) -> f32 {
    (1. - t) * v1 + t * v2
}

pub fn gamma(x: f32) -> f32 {
    (x * EPSILON) / (1. - x * EPSILON)
}
