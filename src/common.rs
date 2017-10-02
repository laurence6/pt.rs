use std::f32;
use std::mem::{swap, transmute};

pub const FLOAT_MAX:     f32 = f32::MAX;
pub const FLOAT_MIN_POS: f32 = f32::MIN_POSITIVE;
pub const EPSILON:       f32 = f32::EPSILON;
pub const INFINITY:      f32 = f32::INFINITY;

pub const MACHINE_EPSILON: f32 = EPSILON * 0.5;
/// Max number less than 1.
pub const ONE_MINUS_EPSILON: f32 = 1. - EPSILON;

pub const PI: f32 = f32::consts::PI;

pub fn clamp<T>(x: T, low: T, high: T) -> T where T: PartialOrd {
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
    (x * MACHINE_EPSILON) / (1. - x * MACHINE_EPSILON)
}

pub fn next_float_down(mut x: f32) -> f32 {
    if x.is_infinite() && x < 0. {
        return x;
    }
    if x == 0. {
        x = -0.;
    }

    let mut bits = unsafe { transmute::<f32, u32>(x) };
    if x <= 0. {
        bits += 1;
    } else {
        bits -= 1;
    }

    return unsafe { transmute::<u32, f32>(bits) };
}

pub fn next_float_up(mut x: f32) -> f32 {
    if x.is_infinite() && x > 0. {
        return x;
    }
    if x == -0. {
        x = 0.;
    }

    let mut bits = unsafe { transmute::<f32, u32>(x) };
    if x >= 0. {
        bits += 1;
    } else {
        bits -= 1;
    }

    return unsafe { transmute::<u32, f32>(bits) };
}

pub fn quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let discrim = b * b - 4. * a * c;
    if discrim < 0. {
        return None;
    }

    let root_discrim = discrim.sqrt();
    let q = if b < 0. {
        -0.5 * (b - root_discrim)
    } else {
        -0.5 * (b + root_discrim)
    };

    let (mut t0, mut t1) = (q / a, c / q);
    if t0 > t1 {
        swap(&mut t0, &mut t1);
    }

    return Some((t0, t1));
}

fn get_addr<T>(o: &T) -> usize where T: ?Sized {
    o as *const T as *const () as usize
}

pub fn same_addr<T, U>(a: &T, b: &U) -> bool where T: ?Sized, U: ?Sized {
    get_addr(a) == get_addr(b)
}
